use gtk::SignalListItemFactory;

pub struct WallpaperFactory {
    factory: SignalListItemFactory,
}

pub impl WallpaperFactory {
    pub fn new() -> Self{
        let factory = SignalListItemFactory::new();
        Self { factory: factory }
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
            image.set_css_classes(&["wallpaper"]);

            list_item.set_child(Some(&image));
        });
    }
    pub fn bind_children(&self) {
        self.factory.connect_bind(|f, obj| {
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

            let file = match get_str_attr(&file_info, FILE_ATTRIBUTE_THUMBNAIL_PATH) {
                Some(path) => File::for_path(path),
                None => return,
            };

            image.set_file(Some(&file));
            
        });
    }
}