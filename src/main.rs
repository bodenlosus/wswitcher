mod backends;
mod dirmodel;
mod factory;
mod filter;
mod list;
mod utils;
use backends::{CleanUp, SwwwBackend};
use backends::{HyprpaperBackend, SetWallpaper};
use dirmodel::wallpaper_dir_model;
use gtk::gdk::{Backend, Key};
use gtk::glib::Propagation;
use list::List;
use tokio::runtime::Runtime;
use tokio::spawn;

use crate::factory::WallpaperFactory;
use crate::utils::load_css;
use utils::get_attr;

use std::env::{self, Args};
use std::future::Future;
use std::path::PathBuf;

use adw::Window;
use adw::{prelude::*, HeaderBar, ToolbarView};
use gtk::gio::{prelude::*, Cancellable, File, FileInfo};
use gtk::{
    glib, Application, Builder, Button, EventControllerKey, FileChooserDialog, FileDialog,
    GridView, ListItemFactory, ScrolledWindow, SelectionModel,
};
use gtk4_layer_shell::{self, Edge, KeyboardMode, Layer, LayerShell};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

struct AppWindow {
    window: Window,
}

impl AppWindow {
    fn new(app: &Application, window: Window) -> Self {
        window.set_application(Some(app));
        Self { window }
    }

    fn close(&self) {
        self.window.close();
    }

    fn present(&self) {
        self.window.present();
    }
    fn close_on_escape(&self) {
        let controller = EventControllerKey::new();

        controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                Key::Escape => {
                    std::process::exit(0);
                }
                _ => (),
            }

            Propagation::Proceed
        });

        self.window.add_controller(controller);
    }
    fn on_close<F, Fut>(&self, func: F)
    where
        F: Fn() -> Fut + 'static,
        Fut: Future<Output = ()> + std::marker::Send + 'static,
    {
        self.window.connect_close_request({
            let func = func;
            move |_| {
                let rt = match Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        eprintln!("Failed to create runtime: {:?}", e);
                        return glib::Propagation::Proceed;
                    }
                };
                let fut = func();
                rt.spawn(fut);

                glib::Propagation::Proceed
            }
        });
    }
    fn init_layer_shell(&self) {
        self.window.init_layer_shell();
        self.window.set_anchor(Edge::Left, true);
        self.window.set_anchor(Edge::Right, true);
        self.window.set_anchor(Edge::Bottom, true);
        self.window.set_layer(Layer::Overlay);
        self.window.set_keyboard_mode(KeyboardMode::Exclusive);
    }
}

// self.window.connect_close_request({
//     let fut = Arc::clone(&backend);
//     move |_| {
//     let fut = Arc::clone(&fut);
//     glib::MainContext::default().spawn_local(async move {
//         let mut backend = fut.lrock().unwrap();
//         let _ = backend.clean_up().await;
//     });
//     glib::Propagation::Proceed
// }});

pub fn wallpaper_grid<T: IsA<SelectionModel>, U: IsA<ListItemFactory>>(
    model: Option<T>,
    factory: Option<U>,
) -> GridView {
    let grid_view = GridView::new(model, factory);
    grid_view.set_css_classes(&["file-grid"]);
    grid_view.set_single_click_activate(true);
    grid_view
}

#[tokio::main]
async fn main() {
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

fn build_ui(app: &Application) {
    let args: Vec<String> = env::args().collect();

    let default = "~/Pictures".to_string();
    let dir = args.get(1).unwrap_or(&default);

    let backend = SwwwBackend::new("swww".to_string());
    let backend = Arc::new(Mutex::new(backend));
    // Create a new builder and add the UI definition from the file
    let builder = Builder::from_string(include_str!("resources/window.ui"));
    let window = AppWindow::new(app, builder.object("window").unwrap());
    window.init_layer_shell();

    let scrolled_window: ScrolledWindow = builder.object("scrolled-window").unwrap();

    let model = wallpaper_dir_model(PathBuf::from_str(dir).unwrap());
    let factory = WallpaperFactory::new();
    factory.setup_children();
    factory.bind_children();
    let list = List::new(Some(factory), Arc::clone(&backend), model);
    scrolled_window.set_child(Some(&list.list));

    window.close_on_escape();
    window.present();

    list.on_activate();
}
