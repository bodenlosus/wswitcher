use std::path::PathBuf;
use std::str::FromStr;

use adw::subclass::window;
use gtk::gio::ffi::{GIcon, GThemedIcon};
use gtk::gio::{self, prelude::*, Cancellable, File, FileAttributeType, FileInfo, Menu, MenuItem, ThemedIcon, FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE, FILE_ATTRIBUTE_STANDARD_EDIT_NAME, FILE_ATTRIBUTE_STANDARD_ICON, FILE_ATTRIBUTE_STANDARD_NAME, FILE_ATTRIBUTE_THUMBNAIL_IS_VALID, FILE_ATTRIBUTE_THUMBNAIL_PATH};
use gtk::glib::bitflags::Flags;
use gtk::{Application, BuilderListItemFactory, DirectoryList, FileLauncher, GestureClick, GridView, IconSize, IconTheme, Image, ListItemFactory, PopoverMenu, PopoverMenuFlags, ScrolledWindow, SelectionModel, SingleSelection};
use gtk::gdk::ffi::GDK_KEY_Escape;
use gtk::gdk::{Display, BUTTON_SECONDARY};
use gtk::glib::closure::IntoClosureReturnValue;
use gtk::glib::translate::IntoGlib;
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use gtk::ffi::{GtkBox, GtkButton, GtkSignalListItemFactory};
use gtk::glib::property::PropertyGet;
use gtk::glib::{GStr, GString, Object, Propagation};
use gtk::{prelude::*, CssProvider, EventControllerKey, ListItem, Shortcut, ShortcutTrigger, SignalListItemFactory};
use gtk::{Window, ApplicationWindow, Builder, Button, Box, Label};
use gtk4_sys::GtkGridView;

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

struct WallpaperGrid {
    grid_view: GridView,
    factory: SignalListItemFactory,
    dirpath: PathBuf,
}

impl WallpaperGrid {
    pub fn new(dir: PathBuf,) -> Self {
        let attrs = attrs_to_str([
            FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE,
            FILE_ATTRIBUTE_THUMBNAIL_PATH,
            FILE_ATTRIBUTE_THUMBNAIL_IS_VALID,
            FILE_ATTRIBUTE_STANDARD_NAME,
        ]);
        let file = File::for_path(&dir);
        let model = DirectoryList::new(Some(&attrs), Some(&file));
        let selection_model = SingleSelection::new(Some(model));
        let factory = SignalListItemFactory::new();
        let grid_view = GridView::new(Some(selection_model), None::<ListItemFactory>);
        Self {grid_view, factory, dirpath:dir }
    }

    pub fn setup_children(&self) {
        self.factory.connect_setup(|_, obj| {
            let list_item = match obj.downcast_ref::<ListItem>() {
                Some(item) => item,
                None => return,
            };
            
            let image = Image::builder().vexpand(true).hexpand(true).build();
            let boxchild = Box::new(gtk::Orientation::Vertical, 0);
            boxchild.append(&image);
            list_item.set_child(Some(&boxchild));
        });
    }
    pub fn bind_children(&self) {
        self.factory.connect_bind(|_, obj| {
            let list_item = match obj.downcast_ref::<ListItem>() {
                Some(item) => item,
                None => return,
            };
            let child = list_item.child().and_downcast::<Box>();
            let file_info = list_item.item().and_downcast::<FileInfo>();

            let (child, file_info) = match (child, file_info) {
                (Some(child), Some(file_info)) => (child, file_info),
                _ => return,
            };
            let content_type = file_info.content_type();

            println!("a");
            is_image(&file_info);            
            
        });
    }
    pub fn factory_ready(&self ) {
        self.grid_view.set_factory(Some(&self.factory));
    }
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
        let grid:WallpaperGrid = WallpaperGrid::new( PathBuf::from_str("home/johannes/Pictures/wallpapers").unwrap());
        grid.setup_children();
        grid.bind_children();
        grid.factory_ready();
        scrolled_window.set_child(Some(&grid.grid_view));
        window.present();
    });

    // Run the application
    app.run();
}

fn is_image(fi:&FileInfo) -> bool {
    let content_type = fi.content_type();
    println!("{:?}", content_type);
    return false;
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn get_attr<T: ToString, R: ObjectType + IsA<Object>>(fi: &FileInfo,  attr: T) -> Option<R> {
    let attrstr = attr.to_string();
    if !fi.has_attribute(&attrstr) {
        return None;
    }

    let icon = match fi.attribute_object(&attrstr).and_downcast::<R>() {
        Some(obj) => obj,
        None => return None,
    };
    Some(icon)

}

fn get_str_attr<T: ToString>(fi: &FileInfo, attr: T) -> Option<String> {
    let attrstr = attr.to_string();
    if !fi.has_attribute(&attrstr) {
        return None;
    }

    let value = fi.attribute_as_string(&attrstr)?;
    Some(value.to_string())
}

fn right_click_menu<T: WidgetExt>(widget: &T) {
    let menu = Menu::new();
    let quit_item = MenuItem::new(Some("hi"), None);
    menu.append_item(&quit_item);
    let popover_menu = PopoverMenu::from_model_full(&menu, PopoverMenuFlags::SLIDING);
    popover_menu.set_parent(widget);
    
}

fn attrs_to_str<T: IntoIterator<Item: ToString>>(attributes: T) -> String{
    let binding = attributes
            .into_iter()
            .fold(
                String::new(), 
                |a, b| a + &b.to_string() + ",");
    let attrs = binding
        .trim_end();
    return attrs.to_string();
}