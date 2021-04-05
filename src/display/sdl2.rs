use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::{
    font::NOTO_MONO, video_memory::VideoCell, Display, DisplayAdapter,
    WindowSettings, DISPLAY_COLUMNS, DISPLAY_LINES,
};
use anyhow::{Error, Result};
use fontdue::{Font, FontSettings};
use sdl2::{event::Event, pixels::Color, rect::{Point, Rect}, render::{BlendMode, Texture}};

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
            .resizable()
            .position_centered()
            .build()?;

        let font = Font::from_bytes(NOTO_MONO, FontSettings::default())
            .map_err(|e| Error::msg(e))?;

        let mut canvas = window.into_canvas().present_vsync().build()?;
        canvas.set_logical_size(
            DISPLAY_LOGICAL_WIDTH as u32,
            DISPLAY_LOGICAL_HEIGHT as u32,
        )?;

        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();

        let mut display = Display::default();

        let mut cell_cache: HashMap<VideoCell, Rc<Texture>> = HashMap::new();

        let mut event_pump =
            sdl_context.event_pump().map_err(|e| Error::msg(e))?;

        'running: loop {
            canvas.set_draw_color(Color::BLACK);
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
                    let texture = match cell_cache.entry(cell) {
                        Entry::Occupied(entry) => Rc::clone(entry.get()),
                        Entry::Vacant(entry) => {
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
                                        .set_draw_color(entry.key().background);
                                    texture_canvas.clear();
                                    texture_canvas.set_blend_mode(BlendMode::Blend);

                                    let (metrics, bitmap) =
                                        font.rasterize('g', 17.0);

                                    log::info!("Size: {:?}", metrics);

                                    for fy in 0..metrics.height {
                                        for fx in 0..metrics.width {
                                            let fc =
                                                bitmap[fy * metrics.width + fx];
                                            let colour: &[u8] =
                                                entry.key().foreground.into();
                                            texture_canvas.set_draw_color((
                                                colour[0], colour[1],
                                                colour[2], fc,
                                            ));
                                            texture_canvas.draw_point(
                                                Point::new(
                                                    fx as i32, fy as i32,
                                                ),
                                            );
                                        }
                                    }
                                },
                            )?;

                            log::info!(
                                "Created texture for cell {:?}",
                                entry.key()
                            );

                            Rc::clone(entry.insert(Rc::new(texture)))
                        }
                    };

                    canvas
                        .copy(
                            &texture,
                            None,
                            Some(Rect::new(
                                (column * CELL_WIDTH + COLUMN_OFFSET) as i32,
                                (row * CELL_HEIGHT + ROW_OFFSET) as i32,
                                CELL_WIDTH as u32,
                                CELL_HEIGHT as u32,
                            )),
                        )
                        .map_err(|e| Error::msg(e))?;
                }
            }

            canvas.present();
        }

        Ok(())
    }
}
