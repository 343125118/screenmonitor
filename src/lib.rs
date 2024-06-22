use std::collections::HashMap;

pub mod core;
pub mod utils;
use anyhow::Result;

use uuid::{Uuid};
use std::path::PathBuf;
use std::env;
use anyhow::anyhow;
use winreg::RegKey;
use winreg::enums::*;


#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub path: String,
    pub command: String,
    pub arguments: Vec<String>,
}

impl Default for Task {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        let mut arguments = vec![];

        // macos:-f avfoundation -i 1 -r 30 -s 1920x1080 -vcodec libx264 -preset ultrafast -crf 18 -pix_fmt yuv420p output.mp4 -f
        #[cfg(target_os = "macos")]
        arguments.push("-f".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("avfoundation".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-i".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("1".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-r".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("30".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-s".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("1920x1080".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-vcodec".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("libx264".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-preset".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("ultrafast".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-crf".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("18".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-pix_fmt".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("yuv420p".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("output.mp4".to_string());

        #[cfg(target_os = "macos")]
        arguments.push("-y".to_string());


        #[cfg(target_os = "windows")]
        arguments.push("-f".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("gdigrab".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-framerate".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("30".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-i".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("desktop".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-vcodec".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("libx264".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-preset".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("ultrafast".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-crf".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("18".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-pix_fmt".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("yuv420p".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("output.mp4".to_string());

        #[cfg(target_os = "windows")]
        arguments.push("-y".to_string());


        Task {
            id,
            name: "".to_string(),
            path: "".to_string(),
            command: "ffmpeg".to_string(),
            arguments,
        }
    }
}


pub fn register() -> Result<()> {
    #[cfg(target_os = "windows")]
    add_to_startup()?;

    Ok(())
}



fn add_to_startup() -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _disp) = hklm.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;

    let mut path = env::current_exe()?;
    path.set_extension("exe");
    let path = path.to_str().unwrap_or_default();

    key.set_value("MyApp", &path)?;

    Ok(())
}