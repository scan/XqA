use super::{DISPLAY_COLUMNS, DISPLAY_LINES};
use anyhow::{ensure, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Black,      // #000000
    White,      // #FFFFFF
    Red,        // #880000
    Cyan,       // #AAFFEE
    Violet,     // #CC44CC
    Green,      // #00CC55
    Blue,       // #0000AA
    Yellow,     // #EEEE77
    Orange,     // #DD8855
    Brown,      // #664400
    LightRed,   // #FF7777
    DarkGrey,   // #333333
    Grey,       // #777777
    LightGreen, // #AAFF66
    LightBlue,  // #0088FF
    LightGrey,  // #BBBBBB
}

impl From<Colour> for [u8; 4] {
    fn from(c: Colour) -> Self {
        match c {
            Colour::Black => [0x00, 0x00, 0x00, 0xff],
            Colour::White => [0xff, 0xff, 0xff, 0xff],
            Colour::Red => [0x88, 0x00, 0x00, 0xff],
            Colour::Cyan => [0xaa, 0xff, 0xee, 0xff],
            Colour::Violet => [0xcc, 0x44, 0xcc, 0xff],
            Colour::Green => [0x00, 0xcc, 0x55, 0xff],
            Colour::Blue => [0x00, 0x00, 0xaa, 0xff],
            Colour::Yellow => [0xee, 0xee, 0x77, 0xff],
            Colour::Orange => [0xdd, 0x88, 0x55, 0xff],
            Colour::Brown => [0x66, 0x44, 0x00, 0xff],
            Colour::LightRed => [0xff, 0x77, 0x77, 0xff],
            Colour::DarkGrey => [0x33, 0x33, 0x33, 0xff],
            Colour::Grey => [0x77, 0x77, 0x77, 0xff],
            Colour::LightGreen => [0xaa, 0xff, 0x66, 0xff],
            Colour::LightBlue => [0x00, 0x88, 0xff, 0xff],
            Colour::LightGrey => [0xbb, 0xbb, 0xbb, 0xff],
        }
    }
}

impl From<Colour> for u32 {
    fn from(c: Colour) -> Self {
        match c {
            Colour::Black => 0x000000ff,
            Colour::White => 0xffffffff,
            Colour::Red => 0x880000ff,
            Colour::Cyan => 0xaaffeeff,
            Colour::Violet => 0xcc44ccff,
            Colour::Green => 0x00cc55ff,
            Colour::Blue => 0x0000aaff,
            Colour::Yellow => 0xeeee77ff,
            Colour::Orange => 0xdd8855ff,
            Colour::Brown => 0x664400ff,
            Colour::LightRed => 0xff7777ff,
            Colour::DarkGrey => 0x333333ff,
            Colour::Grey => 0x777777ff,
            Colour::LightGreen => 0xaaff66ff,
            Colour::LightBlue => 0x0088ffff,
            Colour::LightGrey => 0xbbbbbbff,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VideoCell {
    pub content: char,
    pub background: Colour,
    pub foreground: Colour,
}

impl Default for VideoCell {
    fn default() -> Self {
        Self {
            content: ' ',
            background: Colour::Blue,
            foreground: Colour::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VideoMemory(pub [VideoCell; DISPLAY_COLUMNS * DISPLAY_LINES]);

impl Default for VideoMemory {
    fn default() -> Self {
        VideoMemory([VideoCell::default(); DISPLAY_COLUMNS * DISPLAY_LINES])
    }
}

impl VideoMemory {
    pub fn set(&mut self, column: usize, row: usize, content: char, foreground: Colour, background: Colour) -> Result<()> {
        ensure!(column < DISPLAY_COLUMNS, "column {} exceeds allowed columns", column);
        ensure!(row < DISPLAY_LINES, "row {} exceeds allowed rows", row);

        let position = (row * DISPLAY_COLUMNS) + column;
        self.0[position] = VideoCell {
            content,
            foreground,
            background,
        };

        Ok(())
    }

    pub fn get(&self, column: usize, row: usize) -> Result<VideoCell> {
        ensure!(column < DISPLAY_COLUMNS, "column {} exceeds allowed columns", column);
        ensure!(row < DISPLAY_LINES, "row {} exceeds allowed rows", row);

        let position = (row * DISPLAY_COLUMNS) + column;
        Ok(self.0[position].clone())
    }

    pub fn clear(&mut self) {
        self.0 = [VideoCell::default(); DISPLAY_COLUMNS * DISPLAY_LINES]
    }
}
