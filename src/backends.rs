use std::future::Future;
use std::path::PathBuf;
use std::process::Command;

pub trait SetWallpaper {
    fn set_wallpaper(&self, path:&PathBuf) -> Result<(), String>;

}

pub trait PreloadWallpaper {
    async fn preload_wallpaper(&self, path:&PathBuf) -> Result<(), String>;

}
#[derive(Clone)]
pub struct HyprpaperBackend { 
    hyprctl_path: String,
}

impl HyprpaperBackend { 
    pub fn new(hyprctl_path: String) -> Self {
        Self { hyprctl_path }
    }
}
impl PreloadWallpaper for HyprpaperBackend {
    async fn preload_wallpaper(&self, path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err("Path is not a file".to_string());
        }
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return Err("Invalid path".to_string()),
        };
        let res = Command::new(self.hyprctl_path.clone()).args([
            "hyprpaper",
            "preload",
            path_str,  
        ]).output();

        match res {
            Ok(output) => {
                if output.status.success() {
                    println!("Wallpaper preloaded successfully {:?}", output);
                    Ok(())
                } else {
                    Err(format!("Failed : {:?}", output.stderr))
                }
            }
            Err(err) => {
                Err(format!("Error setting wallpaper: {}", err))
            }
        }
    }
}
impl SetWallpaper for HyprpaperBackend {
    fn set_wallpaper(&self, path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err("Path is not a file".to_string());
        }
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return Err("Invalid path".to_string()),
        };

        let res = Command::new(self.hyprctl_path.clone()).args([
            "hyprpaper",
            "wallpaper",
            &format!(",{}", path_str),  
        ]).output();
        
        match res {
            Ok(output) => {
                if output.status.success() {
                    println!("Wallpaper set successfully {:?}", output);
                    Ok(())
                } else {
                    Err(format!("Failed to set wallpaper: {:?}", output.stderr))
                }
            }
            Err(err) => {
                Err(format!("Error setting wallpaper: {}", err))
            }
        }
    }
}