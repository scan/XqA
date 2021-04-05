#![allow(non_snake_case)]
#![forbid(unsafe_code)]

mod display;

use display::{start, SDL2DisplayAdapter, WindowSettings};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    start::<SDL2DisplayAdapter>(WindowSettings::default())
}
