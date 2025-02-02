use gtk::gdk::Texture;
use gdk_pixbuf::Pixbuf;
use gtk::gio::{File, FileInfo, FILE_ATTRIBUTE_THUMBNAIL_IS_VALID, FILE_ATTRIBUTE_THUMBNAIL_PATH};
use gtk::{glib::object::Cast, ContentFit, ListItem, Picture, SignalListItemFactory};
use gtk::prelude::*;

use crate::utils::{get_attr, get_str_attr, is_image};
pub struct WallpaperFactory {
    pub factory: SignalListItemFactory,
}

impl WallpaperFactory {
    pub fn new() -> Self{
        let factory = SignalListItemFactory::new();
        Self { factory }
    }
    pub fn setup_children(&self) {
        self.factory.connect_setup(|_, obj| {
            
            let list_item = match obj.downcast_ref::<ListItem>() {
                Some(item) => item,
                None => return,
            };
            let image = Picture::new();
            image.set_content_fit(ContentFit::Cover);
            image.set_size_request(200, 130);
            image.set_can_shrink(true);
            image.set_css_classes(&["wallpaper"]);

            list_item.set_child(Some(&image));
        });
    }
    pub fn bind_children(&self) {
        self.factory.connect_bind(|_, obj| {
            let list_item = match obj.downcast_ref::<ListItem>() {
                Some(item) => item,
                None => return,
            };

            
            let file_info = match list_item.item().and_downcast::<FileInfo>() {
                Some(info) => info,
                None => return,
            };
            
            if !is_image(&file_info) {
                return;
            }
            let binding = list_item.child();
            let image = match binding.and_downcast_ref::<Picture>() {
                Some(image) => image,
                None => return,
            };

            if let Some(thumbnail_file) = retrieve_thumbnail(&file_info) {
                image.set_file(Some(&thumbnail_file));
                return
            };

            let texture = match retrieve_texture(&file_info, Some(200), None) {
                Some(tex) => tex,
                None => return,
            };

            image.set_paintable(Some(&texture));
            
        });
    }
}

fn retrieve_thumbnail (file_info: &FileInfo) -> Option<File> {
    match get_str_attr(&file_info, FILE_ATTRIBUTE_THUMBNAIL_IS_VALID) {
        Some(val) => {
            if val == "FALSE" {
                return None;
            }
        },
        None => {
            return None;
        },
    }

    let thumbnail_path = match get_str_attr(&file_info, FILE_ATTRIBUTE_THUMBNAIL_PATH) {
        Some(path) => path,
        None => return None,
    };
    let thumbnail_file = File::for_path(&thumbnail_path);
    Some(thumbnail_file)
}

fn retrieve_texture(file_info: &FileInfo, width: Option<i32>, height: Option<i32>) -> Option<Texture>{
    
    let path = match get_attr::<_, File>(&file_info, "standard::file").and_then(|file| file.path()) {
        Some(path) => path,
        None => return None,
    };

    let pixbuf = Pixbuf::from_file_at_scale(&path, width.unwrap_or(-1), height.unwrap_or(-1), true);

    let pixbuf = match pixbuf {
        Ok(pixbuf) => pixbuf,
        Err(_) => {println!("pixbuf didnt work for {path:?}");return None},
    };

    Some(Texture::for_pixbuf(&pixbuf))
}