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
            Black => [0x00, 0x00, 0x00, 0xff],
            White => [0xff, 0xff, 0xff, 0xff],
            Red => [0x88, 0x00, 0x00, 0xff],
            Cyan => [0xaa, 0xff, 0xee, 0xff],
            Violet => [0xcc, 0x44, 0xcc, 0xff],
            Green => [0x00, 0xcc, 0x55, 0xff],
            Blue => [0x00, 0x00, 0xaa, 0xff],
            Yellow => [0xee, 0xee, 0x77, 0xff],
            Orange => [0xdd, 0x88, 0x55, 0xff],
            Brown => [0x66, 0x44, 0x00, 0xff],
            LightRed => [0xff, 0x77, 0x77, 0xff],
            DarkGrey => [0x33, 0x33, 0x33, 0xff],
            Grey => [0x77, 0x77, 0x77, 0xff],
            LightGreen => [0xaa, 0xff, 0x66, 0xff],
            LightBlue => [0x00, 0x88, 0xff, 0xff],
            LightGrey => [0xbb, 0xbb, 0xbb, 0xff],
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
