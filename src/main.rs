use reiner::{cli::get_args, wallpaper::Wallpaper};

fn main() {
    let wallpaper = Wallpaper::new().unwrap();
    get_args(wallpaper);
}
