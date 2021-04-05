use std::{collections::HashMap, rc::Rc};

use super::{
    video_memory::VideoCell, Display, DisplayAdapter, WindowSettings,
    DISPLAY_COLUMNS, DISPLAY_LINES,
};
use anyhow::{Error, Result};
use sdl2::{event::Event, rect::Rect, render::Texture};

const DISPLAY_LOGICAL_WIDTH: usize = 1280;
const DISPLAY_LOGICAL_HEIGHT: usize = 720;

const COLUMN_OFFSET: usize = 10; // 1280 is not evenly divisible by 90
const ROW_OFFSET: usize = 10; // 720 is not evenly disible by 25

const CELL_HEIGHT: usize = 28;
const CELL_WIDTH: usize = 14;

pub struct SDL2DisplayAdapter;

impl DisplayAdapter for SDL2DisplayAdapter {
    fn run(settings: WindowSettings) -> Result<()> {
        let sdl_context = sdl2::init().map_err(|e| Error::msg(e))?;
        let video_subsystem = sdl_context.video().map_err(|e| Error::msg(e))?;

        let window = video_subsystem
            .window(
                settings.title,
                settings.width as u32,
                settings.height as u32,
            )
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().present_vsync().build()?;
        canvas.set_logical_size(
            DISPLAY_LOGICAL_WIDTH as u32,
            DISPLAY_LOGICAL_HEIGHT as u32,
        );

        canvas.clear();
        canvas.present();

        let mut texture_creator = canvas.texture_creator();

        let mut display = Display::default();

        let mut cell_cache: HashMap<VideoCell, Rc<Texture>> = HashMap::new();

        let mut event_pump =
            sdl_context.event_pump().map_err(|e| Error::msg(e))?;

        'running: loop {
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            for row in 0..DISPLAY_LINES {
                for column in 0..DISPLAY_COLUMNS {
                    let cell = display.memory.get(column, row)?;
                    let texture = match cell_cache.get(&cell) {
                        Some(texture) => texture,
                        None => {
                            let mut texture = texture_creator
                                .create_texture_target(
                                    None,
                                    CELL_WIDTH as u32,
                                    CELL_HEIGHT as u32,
                                )?;

                            canvas.with_texture_canvas(
                                &mut texture,
                                |texture_canvas| {
                                    texture_canvas
                                        .set_draw_color(cell.background);
                                    texture_canvas.clear();
                                },
                            );

                            let tex_ref = Rc::new(texture);

                            cell_cache.insert(cell, tex_ref);

                            &tex_ref.clone()
                        }
                    };

                    canvas.copy(
                        texture,
                        None,
                        Some(Rect::new(
                            (column * CELL_WIDTH) as i32,
                            (row * CELL_HEIGHT) as i32,
                            CELL_WIDTH as u32,
                            CELL_HEIGHT as u32,
                        )),
                    );
                }
            }

            canvas.present();
        }

        Ok(())
    }
}
