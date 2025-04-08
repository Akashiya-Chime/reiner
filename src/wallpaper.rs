use windows::{
    core::{Result, HSTRING},
    Win32::{
        System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED}, UI::Shell::{DesktopWallpaper, IDesktopWallpaper}
    }
};

pub struct Wallpaper {
    interface: IDesktopWallpaper
}

impl Wallpaper {
    pub fn new() -> Result<Self> {
        let interface: IDesktopWallpaper = unsafe {
            // Initialize COM, use STA
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
            // Create a DesktkopWallpaper object and return its IDesktopWallpaper interface
            CoCreateInstance(
                &DesktopWallpaper,
                None,
                CLSCTX_ALL
            )?
        };
        Ok(Self { interface })
    }

    pub fn set_wallpaper(&self, wallpaper: &HSTRING) -> Result<()> {
        unsafe {
            // Expand: set different wallpapers for different monitors
            self.interface.SetWallpaper(None, wallpaper)
        }
    }
}
