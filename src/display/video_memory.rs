use super::{DISPLAY_COLUMNS, DISPLAY_LINES};
use anyhow::{ensure, Result};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<Colour> for &[u8] {
    fn from(c: Colour) -> Self {
        match c {
            Colour::Black => &[0x00, 0x00, 0x00, 0xff],
            Colour::White => &[0xff, 0xff, 0xff, 0xff],
            Colour::Red => &[0x88, 0x00, 0x00, 0xff],
            Colour::Cyan => &[0xaa, 0xff, 0xee, 0xff],
            Colour::Violet => &[0xcc, 0x44, 0xcc, 0xff],
            Colour::Green => &[0x00, 0xcc, 0x55, 0xff],
            Colour::Blue => &[0x00, 0x00, 0xaa, 0xff],
            Colour::Yellow => &[0xee, 0xee, 0x77, 0xff],
            Colour::Orange => &[0xdd, 0x88, 0x55, 0xff],
            Colour::Brown => &[0x66, 0x44, 0x00, 0xff],
            Colour::LightRed => &[0xff, 0x77, 0x77, 0xff],
            Colour::DarkGrey => &[0x33, 0x33, 0x33, 0xff],
            Colour::Grey => &[0x77, 0x77, 0x77, 0xff],
            Colour::LightGreen => &[0xaa, 0xff, 0x66, 0xff],
            Colour::LightBlue => &[0x00, 0x88, 0xff, 0xff],
            Colour::LightGrey => &[0xbb, 0xbb, 0xbb, 0xff],
        }
    }
}

impl From<Colour> for u32 {
    fn from(c: Colour) -> Self {
        match c {
            Colour::Black => 0xff000000,
            Colour::White => 0xffffffff,
            Colour::Red => 0xff880000,
            Colour::Cyan => 0xffaaffee,
            Colour::Violet => 0xffcc44cc,
            Colour::Green => 0xff00cc55,
            Colour::Blue => 0xff0000aa,
            Colour::Yellow => 0xffeeee77,
            Colour::Orange => 0xffdd8855,
            Colour::Brown => 0xff664400,
            Colour::LightRed => 0xffff7777,
            Colour::DarkGrey => 0xff333333,
            Colour::Grey => 0xff777777,
            Colour::LightGreen => 0xffaaff66,
            Colour::LightBlue => 0xff0088ff,
            Colour::LightGrey => 0xffbbbbbb,
        }
    }
}

impl From<Colour> for sdl2::pixels::Color {
    fn from(c: Colour) -> Self {
        let tmp: &[u8] = c.into();

        Self::RGBA(tmp[0], tmp[1], tmp[2], tmp[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        let mut tmp = VideoMemory([VideoCell::default(); DISPLAY_COLUMNS * DISPLAY_LINES]);
        let mut rnd = rand::thread_rng();

        for c in tmp.0.iter_mut() {
            c.content = rnd.gen_range('A'..'z');
        }

        tmp
    }
}

impl VideoMemory {
    pub fn set(
        &mut self,
        column: usize,
        row: usize,
        content: char,
        foreground: Colour,
        background: Colour,
    ) -> Result<()> {
        ensure!(
            column < DISPLAY_COLUMNS,
            "column {} exceeds allowed columns",
            column
        );
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
        ensure!(
            column < DISPLAY_COLUMNS,
            "column {} exceeds allowed columns",
            column
        );
        ensure!(row < DISPLAY_LINES, "row {} exceeds allowed rows", row);

        let position = (row * DISPLAY_COLUMNS) + column;
        Ok(self.0[position].clone())
    }

    pub fn clear(&mut self) {
        self.0 = [VideoCell::default(); DISPLAY_COLUMNS * DISPLAY_LINES]
    }
}
