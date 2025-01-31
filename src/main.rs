

mod utils;
mod filter;
mod factory;
use crate::factory::WallpaperFactory;
use crate::utils::{attrs_to_str, get_str_attr, is_image, load_css};

use std::path::PathBuf;
use std::str::FromStr;

use filter::custom_filter;
use gtk::gio::{prelude::*, File, FileInfo, Menu, MenuItem, FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE, FILE_ATTRIBUTE_STANDARD_NAME, FILE_ATTRIBUTE_THUMBNAIL_IS_VALID, FILE_ATTRIBUTE_THUMBNAIL_PATH};
use gtk::{Application, ContentFit, CustomFilter, DirectoryList, Filter, FilterListModel, GridView, Image, Label, ListItemFactory, Picture, PopoverMenu, PopoverMenuFlags, ScrolledWindow, SelectionModel, SingleSelection};
use gtk::gdk::Display;
use gtk::glib::Object;
use gtk::{prelude::*, CssProvider, ListItem, SignalListItemFactory};
use gtk::{Window, Builder, Box};


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


fn wallpaper_dir_model(dir: PathBuf) -> Option<SingleSelection>{
    if !dir.exists() {return None;};
    if !dir.is_dir() {return None;};

    let file = File::for_path(dir);
    let attrs = attrs_to_str([
        FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE,
        FILE_ATTRIBUTE_THUMBNAIL_PATH,
        FILE_ATTRIBUTE_THUMBNAIL_IS_VALID,
        FILE_ATTRIBUTE_STANDARD_NAME,
    ]);
    let model = DirectoryList::new(Some(&attrs), Some(&file));
    let filtered_model = FilterListModel::new(Some(model), Some(custom_filter()));
    let selection_model = SingleSelection::new(Some(filtered_model));
    Some(selection_model)
}



pub fn wallpaper_grid<T: IsA<SelectionModel>, U: IsA<ListItemFactory>>(model: Option<T>, factory: Option<U>) -> GridView {
    let grid_view = GridView::new(model, factory);
    grid_view.set_css_classes(&["file-grid"]);
    grid_view
}

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();
    app.connect_startup(|_| load_css());
    // Connect to the "activate" signal to set up the UI
    app.connect_activate(|app| {
        // Create a new builder and add the UI definition from the file
        let builder = Builder::from_string(include_str!("resources/window.ui"));
        let window = AppWindow::new(app, builder.object("window").unwrap());
        let scrolled_window: ScrolledWindow = builder.object("scrolled-window").unwrap();
        let model= wallpaper_dir_model( PathBuf::from_str("/home/johannes/Pictures/wallpapers").unwrap());
        let factory = WallpaperFactory::new();
        factory.setup_children();
        factory.bind_children();
        let grid = wallpaper_grid(model, Some(factory.factory));
        scrolled_window.set_child(Some(&grid));
        window.present();
    });

    // Run the application
    app.run();
}

