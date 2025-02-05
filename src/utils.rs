use std::{fmt::format, fs::{}, io::Error, path::{Path, PathBuf}, process::{Command, Output}};

use gtk::{gdk::Display, gio::FileInfo, glib::{object::{CastNone, IsA, ObjectType}, Object}, CssProvider};
use tokio::fs::read_dir;


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


pub fn handle_cmd_result(res: Result<Output, Error>, error_idf: Option<&str>) -> Result<Output, String>{
    let idf = error_idf.unwrap_or("");
    match res {
        Ok(output) => {
            if output.status.success() {
                println!("{} ran succesfully: {:#}", idf, String::from_utf8_lossy(&output.stdout));
                Ok(output)
            } else {
                let msg = format!("{:?} failed: {:?}", idf, output.stderr);
                eprintln!("msg: {:?}", msg);
                Err(msg)
            }
        }
        Err(err) => {
            Err(format!("{:?} failed: {}", idf, err))
        }
    }
}