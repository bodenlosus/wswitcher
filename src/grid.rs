use std::sync::{Arc, Mutex};
use crate::utils::get_attr;

use gtk::{gio::File, glib, prelude::*, GridView, SelectionModel, Widget, Window};

use crate::{backends::SetWallpaper, factory::WallpaperFactory};

type  BackendClone<T: SetWallpaper> = Arc<Mutex<T>>;

pub struct Grid<T: SetWallpaper + 'static>{
    pub grid: GridView,
    backend: BackendClone<T>,
}

impl<T> Grid<T>
where T: SetWallpaper + 'static
{
    pub fn new<M: IsA<SelectionModel>>(factory: Option<WallpaperFactory>, backend: BackendClone<T>, model: Option<M>) -> Self {
        let grid_view = GridView::new(model, factory.and_then(|f| Some(f.factory)));
        
        grid_view.set_css_classes(&["file-grid"]);
        grid_view.set_single_click_activate(true);

        Self { grid: grid_view , backend}
    }
    pub fn on_activate(&self) {
        self.grid.connect_activate({
            let fut = Arc::clone(&self.backend);
            move |grid_view, id| {
            let item = grid_view.model().unwrap().item(id);
            let file_info = match item.and_downcast_ref::<gtk::gio::FileInfo>() {
                Some(fi) => fi,
                None => return,
            };
    
            let file = match get_attr::<_, File>(file_info, "standard::file") {
                Some(path) => path,
                None => {
                    return;
                }
            };
            let path = match file.path() {
                Some(path) => path,
                None => return,
            };
            let fut = Arc::clone(&fut);
            glib::MainContext::default().spawn_local(async move {
                let mut backend = fut.lock().unwrap();
                let _ = backend.set_wallpaper(&path).await;
            });
        }});
    }

}