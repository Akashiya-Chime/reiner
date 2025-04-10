use std::{ffi::OsString, path::{Path, PathBuf}};
use clap::Parser;
use walkdir::WalkDir;
use windows::core::HSTRING;

use crate::wallpaper::Wallpaper;

#[derive(Parser, Debug)]
#[command(
    version("0.1.0"),
    about("A simple cli tool to manage and configure your windows wallpaper."),
    long_about = None
)]
struct Args {
    /// Set current wallpaper
    #[arg(long)]
    set_wallpaper: Option<OsString>,

    /// Next picture
    #[arg(short, long)]
    next: bool,

    /// Previous picture
    #[arg(short, long)]
    previous: bool,
}

fn get_all_wallpapers(folder: &PathBuf) -> Vec<OsString> {
    let image_exts = ["jpg", "jpeg", "png"];
    WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| image_exts.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|e| e.file_name().to_os_string())
        .collect()
}

struct Configuration {
    folder: PathBuf,
    current_wallpaper: OsString,
    wallpapers: Vec<OsString>
}

impl Configuration {
    fn new(path: &OsString) -> Self {
        let wallpaper_path = Path::new(path);
        let folder = wallpaper_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();
        let current_wallpaper = wallpaper_path
            .file_name()
            .map(OsString::from)
            .unwrap();
        let wallpapers = get_all_wallpapers(&folder);
        Self { folder, current_wallpaper, wallpapers }
    }

    fn get_next_wallpaper(&self) -> &OsString {
        let current_wallpaper = &self.current_wallpaper;
        let images = &self.wallpapers;

        let current_index = images.iter().position(|image| image == current_wallpaper).unwrap();
        let next_index = if current_index == images.len() - 1 { 0 } else { current_index + 1 };
        images.get(next_index).unwrap()
    }

    fn get_previous_wallpaper(&self) -> &OsString {
        let current_wallpaper = &self.current_wallpaper;
        let images = &self.wallpapers;

        let current_index = images.iter().position(|image| image == current_wallpaper).unwrap();
        let previous_index = if current_index == 0 { images.len() - 1 } else { current_index - 1 };
        images.get(previous_index).unwrap()
    }
}

pub fn get_args(wp: Wallpaper) {
    let args = Args::parse();
    let config: Configuration = Configuration::new(&wp.get_wallpaper());
    println!("folder: {:?}\nwallpaper: {:?}", config.folder, config.current_wallpaper);

    if let Some(wallpaper) = args.set_wallpaper {
        println!("Set wallpaper: {:?}", wallpaper);
        wp.set_wallpaper(&HSTRING::from(wallpaper)).unwrap();
    }

    if args.next {
        let next_wallpaper = config.folder.join(config.get_next_wallpaper()).into_os_string();
        println!("set next wallpaper: {:?}", next_wallpaper);
        wp.set_wallpaper(&HSTRING::from(next_wallpaper)).unwrap();
    }

    if args.previous {
        let previous_wallpaper = config.folder.join(config.get_previous_wallpaper()).into_os_string();
        println!("set previous wallpaper: {:?}", previous_wallpaper);
        wp.set_wallpaper(&HSTRING::from(previous_wallpaper)).unwrap();
    }
}
