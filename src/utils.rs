use std::{fs::{}, path::PathBuf};

use gtk::{gdk::Display, gio::FileInfo, glib::{object::{CastNone, IsA, ObjectType}, Object}, CssProvider};
use tokio::fs::read_dir;

use crate::backends::PreloadWallpaper;


pub fn is_image(fi:&FileInfo) -> bool {
    let content_type = fi.content_type();
    if content_type.is_none() {
        return false;
    }
    let content_type = content_type.unwrap().to_string();
    let sep: Vec<&str> = content_type.as_str().split("/").collect();

    return sep[0] == "image";
}

pub fn load_css() {
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

pub fn get_attr<T: ToString, R: ObjectType + IsA<Object>>(fi: &FileInfo,  attr: T) -> Option<R> {
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

pub fn get_str_attr<T: ToString>(fi: &FileInfo, attr: T) -> Option<String> {
    let attrstr = attr.to_string();
    if !fi.has_attribute(&attrstr) {
        return None;
    }

    let value = fi.attribute_as_string(&attrstr)?;
    Some(value.to_string())
}

pub fn attrs_to_str<T: IntoIterator<Item: ToString>>(attributes: T) -> String{
    let binding = attributes
            .into_iter()
            .fold(
                String::new(), 
                |a, b| a + &b.to_string() + ",");
    let attrs = binding
        .trim_end();
    return attrs.to_string();
}

pub async fn preload_dir<T: PreloadWallpaper>(backend: &T, dir: &PathBuf ) -> Result<(), String> {
    if !dir.exists() {return Err(format!("dir doesnt exist {:?}", dir));};
    if !dir.is_dir() {return Err(format!("not a directory {:?}", dir));};
    let mut entries = match read_dir(dir).await {
        Ok(entries) => entries,
        Err(err) => return Err(format!("Error reading directory: {}", err)),
    };
    
    loop { 
        let entry = match entries.next_entry().await {
            Ok(Some(entry)) => entry,
            Ok(None) => break,
            Err(_) => continue,
              
        };
        let file_type = match entry.file_type().await {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };
        if !file_type.is_file() { continue; }
        let path: PathBuf = entry.path();
        let _res = backend.preload_wallpaper(&path).await;
    }

    Ok(())
}