#![allow(non_snake_case)]
#![forbid(unsafe_code)]

mod display;

use display::{start, PixelsDisplayAdapter, WindowSettings};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    start::<PixelsDisplayAdapter>(WindowSettings::default())
}
