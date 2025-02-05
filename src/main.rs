

mod utils;
mod filter;
mod factory;
mod dirmodel;
mod backends;
use backends::{HyprpaperBackend, SetWallpaper};
use dirmodel::wallpaper_dir_model;

use utils::{get_attr};
use crate::factory::WallpaperFactory;
use crate::utils::{load_css};


use std::path::PathBuf;

use std::str::FromStr;
use std::sync::{Arc, Mutex};
use gtk::gio::{prelude::*, File, FileInfo};
use gtk::{glib, Application, Builder, GridView, ListItemFactory, ScrolledWindow, SelectionModel };
use adw::prelude::*;
use adw::{Window};


struct AppWindow {
    window: Window
}

impl AppWindow {
    fn new(app: &Application, window: Window) -> Self {
        window.set_application(Some(app));
        Self { window}
    }

    fn close (&self) {
        self.window.close();
    }

    fn present(&self) {
        self.window.present();
    }

}

pub fn wallpaper_grid<T: IsA<SelectionModel>, U: IsA<ListItemFactory>>(model: Option<T>, factory: Option<U>) -> GridView {
    let grid_view = GridView::new(model, factory);
    grid_view.set_css_classes(&["file-grid"]);
    grid_view.set_single_click_activate(true);
    grid_view
}

fn main() {

    // Create a new application
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();
    app.connect_startup(|_| load_css());
    // Connect to the "activate" signal to set up the UI
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application){
    let backend = HyprpaperBackend::new("hyprctl".to_string());
    let backend = Arc::new(Mutex::new(backend));
        // Create a new builder and add the UI definition from the file
    let builder = Builder::from_string(include_str!("resources/window.ui"));
    let window = AppWindow::new(app, builder.object("window").unwrap());
    let scrolled_window: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let model= wallpaper_dir_model( PathBuf::from_str("/home/johannes/Pictures/wallpapers").unwrap());
    let factory = WallpaperFactory::new();
    factory.setup_children();
    factory.bind_children();
    let grid = wallpaper_grid(model, Some(factory.factory));
    
    grid.connect_activate(move |grid_view, id| {
        
        let item = grid_view.model().unwrap().item(id);
        let file_info = match item.and_downcast_ref::<FileInfo>(){
            Some(fi) => fi,
            None => return,
        };

        let file = match get_attr::<_, File>(file_info, "standard::file") {
            Some(path) => path,
            None => {println!("hell");return;},
        };
        let path = match file.path() {
            Some(path) => path,
            None => return,
        };
        let fut = Arc::clone(&backend);
        glib::MainContext::default().spawn_local(async move {
            let mut backend = fut.lock().unwrap();
            let _ = backend.set_wallpaper(&path).await;
        });
                    
        
    });
    scrolled_window.set_child(Some(&grid));
    window.present();
}

