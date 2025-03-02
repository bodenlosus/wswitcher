use std::path::PathBuf;
use std::sync::Mutex;
use std::{rc::Rc, sync::Arc};
use std::cell::RefCell;
use crate::utils::get_attr;

use gtk::gdk::Backend;
use gtk::ListTabBehavior;
use gtk::{gdk::Key, gio::File, glib, prelude::*, EventControllerKey, GridView, ListView, Orientation, SelectionModel, TextDirection, Widget, Window};

use crate::{backends::SetWallpaper, factory::WallpaperFactory};

type  BackendClone<T: SetWallpaper> = Arc<Mutex<T>>;

pub struct List<T: SetWallpaper + 'static>{
    pub list: ListView,
    backend: BackendClone<T>,
    pub selected: Rc<RefCell<u32>>,
}

impl<T> List<T>
where T: SetWallpaper + 'static
{
    pub fn new<M: IsA<SelectionModel>>(factory: Option<WallpaperFactory>, backend: BackendClone<T>, model: Option<M>) -> Self {
        let list_view = ListView::new(model, factory.and_then(|f| Some(f.factory)));

        list_view.set_css_classes(&["file-grid"]);
        list_view.set_orientation(Orientation::Horizontal);
        list_view.set_single_click_activate(true);
        list_view.set_tab_behavior(ListTabBehavior::All);

        Self { list:  list_view, backend, selected: Rc::new(RefCell::new(0)) }
    }
    pub fn on_activate(&self) {
        self.list.connect_activate({
            let fut = Arc::clone(&self.backend);
            move |list_view, id| {
                let path = match get_item(list_view, id) {
                    None => {return;},
                    Some(path) => path,
                };

                let path = path.clone();
                let fut = Arc::clone(&fut);
                glib::MainContext::default().spawn_local(async move {
                    let mut backend = fut.lock().unwrap();
                    let _ = backend.set_wallpaper(&path).await;
                });

        }});
    }
     

    

}

fn get_item(list_view: &ListView, id: u32) -> Option<PathBuf> {
    let item = list_view.model().unwrap().item(id);
        let file_info = match item.and_downcast_ref::<gtk::gio::FileInfo>() {
            Some(fi) => fi,
            None => return None,
        };

        let file = match get_attr::<_, File>(file_info, "standard::file") {
            Some(path) => path,
            None => {
                return None;
            }
        };
        let path = match file.path() {
            Some(path) => path,
            None => return None,
        };
        
        return Some(path);
}