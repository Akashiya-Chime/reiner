use std::ffi::OsString;
use clap::Parser;
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

    /// Set wallpaper directory
    #[arg(long, default_value = "./wallpaper")]
    set_folder: Option<OsString>,

    /// Set change interval(minutes)
    #[arg(long)]
    interval: Option<u32>,

    /// Next picture
    #[arg(long)]
    next: bool,

    /// Previous picture
    #[arg(long)]
    previous: bool,
}

pub fn get_args(wp: Wallpaper) {
    let args = Args::parse();
    let folder = args.set_folder.unwrap();
    println!("Set folder: {folder:?}");
    if let Some(wallpaper) = args.set_wallpaper {
        println!("Set wallpaper: {wallpaper:?}");
        wp.set_wallpaper(&HSTRING::from(wallpaper)).unwrap();
    }
}
