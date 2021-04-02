mod font;
mod pixels;
mod video_memory;

use anyhow::Result;

use self::video_memory::VideoMemory;

pub use self::pixels::PixelsDisplayAdapter;

pub const DISPLAY_COLUMNS: usize = 90;
pub const DISPLAY_LINES: usize = 25;

#[derive(Debug, Default, Copy, Clone)]
pub struct Display {
    pub memory: VideoMemory,
}

pub trait DisplayAdapter {
    fn run(settings: WindowSettings) -> Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct WindowSettings<'a> {
    title: &'a str,
    width: usize,
    height: usize,
    fullscreen: bool,
}

impl<'a> Default for WindowSettings<'a> {
    fn default() -> Self {
        Self {
            title: "XqA terminal",
            width: 1280,
            height: 720,
            fullscreen: false,
        }
    }
}

pub fn start<T>(settings: WindowSettings) -> Result<()>
where
    T: DisplayAdapter + Sized,
{
    T::run(settings)
}
