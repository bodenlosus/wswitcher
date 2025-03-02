use std::path::PathBuf;
use std::process::Command;

use crate::utils::handle_cmd_result;

pub trait SetWallpaper {
    async fn set_wallpaper(&mut self, path: &PathBuf) -> Result<(), String>;
}

pub trait CleanUp {
    async fn clean_up(&mut self) -> Result<(), String>;
}

pub trait ValidateWallPath {
    fn validate_wall_path(&self, path: &PathBuf) -> Result<(), String>;
}

#[derive(Clone)]
pub struct HyprpaperBackend {
    hyprctl_path: String,
    current_wallpaper_path: Option<String>,
}

impl HyprpaperBackend {
    pub fn new(hyprctl_path: String) -> Self {
        Self {
            hyprctl_path,
            current_wallpaper_path: None,
        }
    }
    async fn preload_wallpaper(&self, pathstr: &str) -> Result<(), String> {
        let res = Command::new(self.hyprctl_path.clone())
            .args(["hyprpaper", "preload", pathstr])
            .output();

        handle_cmd_result(res, Some("preloading wallpaper"))?;

        Ok(())
    }

    async fn unload_wallpaper(&self, pathstr: &Option<String>) -> Result<(), String> {
        println!("{pathstr:?}");
        let pathstr = match pathstr {
            Some(pathstr) => pathstr,
            None => {
                println!("Nothing to unload");
                return Ok(());
            }
        };

        let res = Command::new(self.hyprctl_path.clone())
            .args(["hyprpaper", "unload", pathstr])
            .output();

        handle_cmd_result(res, Some("unloading"))?;

        Ok(())
    }
}

impl ValidateWallPath for HyprpaperBackend {
    fn validate_wall_path(&self, path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err("Path is not a file".to_string());
        }

        if !path.is_absolute() {
            return Err("Path is not absolute".to_string());
        }
        Ok(())
    }
}

impl SetWallpaper for HyprpaperBackend {
    async fn set_wallpaper(&mut self, path: &PathBuf) -> Result<(), String> {
        // Unload the previous wallpaper if there is one;
        println!("{:?}", self.current_wallpaper_path);
        self.unload_wallpaper(&self.current_wallpaper_path).await?;

        self.validate_wall_path(path)?;

        let path_str = path
            .to_str()
            .ok_or_else(|| "Error parsing path to string".to_string())?;

        // Preload the new wallpaper
        self.preload_wallpaper(path_str).await?;

        let res = Command::new(self.hyprctl_path.clone())
            .args(["hyprpaper", "wallpaper", &format!(",{}", path_str)])
            .output();

        // Exit out if faililed
        handle_cmd_result(res, Some("setting wallpaper"))?;
        // Remember which wallpaper is loaded
        self.current_wallpaper_path = Some(path_str.to_string());

        Ok(())
    }
}

impl CleanUp for HyprpaperBackend {
    async fn clean_up(&mut self) -> Result<(), String> {
        self.unload_wallpaper(&self.current_wallpaper_path).await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct SwwwBackend {
    swaybg_path: String,
}

impl SwwwBackend {
    pub fn new(swaybg_path: String) -> Self{
        Self {
            swaybg_path
        }
    }
}

impl SetWallpaper for SwwwBackend {
    async fn set_wallpaper(&mut self, path: &PathBuf) -> Result<(), String> {
        self.validate_wall_path(path)?;

        let path_str = path.to_str().ok_or("cannot conver path to strings")?;
        
        let res = Command::new(self.swaybg_path.clone())
            .args(["img", path_str])
            .output();
        
        handle_cmd_result(res, Some("setting wallpaper"))?;

        Ok(())
    }
}

impl ValidateWallPath for SwwwBackend {
    fn validate_wall_path(&self, path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        if !path.is_file() {
            return Err("Path is not a file".to_string());
        }

        if !path.is_absolute() {
            return Err("Path is not absolute".to_string());
        }
        Ok(())
    }
}