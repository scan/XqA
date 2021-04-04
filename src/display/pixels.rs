use std::collections::HashMap;

use super::{
    video_memory::VideoCell, Display, DisplayAdapter, WindowSettings,
    DISPLAY_COLUMNS, DISPLAY_LINES,
};
use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

const DISPLAY_LOGICAL_WIDTH: usize = 1280;
const DISPLAY_LOGICAL_HEIGHT: usize = 720;

const COLUMN_OFFSET: usize = 10; // 1280 is not evenly divisible by 90
const ROW_OFFSET: usize = 10; // 720 is not evenly disible by 25

const CELL_HEIGHT: usize = 28;
const CELL_WIDTH: usize = 14;

pub struct PixelsDisplayAdapter;

impl DisplayAdapter for PixelsDisplayAdapter {
    fn run(settings: WindowSettings) -> Result<()> {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        let window = {
            let size =
                LogicalSize::new(settings.width as f32, settings.height as f32);
            let fullscreen = if settings.fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            };

            WindowBuilder::new()
                .with_title(settings.title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_fullscreen(fullscreen)
                .build(&event_loop)?
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                &window,
            );

            Pixels::new(
                DISPLAY_LOGICAL_WIDTH as u32,
                DISPLAY_LOGICAL_HEIGHT as u32,
                surface_texture,
            )?
        };

        let mut display = Display::default();

        let mut cell_cache: HashMap<VideoCell, Image> = HashMap::new();

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                draw_display(&display, pixels.get_frame(), &mut cell_cache);

                if let Err(e) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    log::error!("pixels.render() failed: {}", e);
                    return;
                }
            }

            if input.update(&event) {
                if let Some(size) = input.window_resized() {
                    pixels.resize(size.width, size.height);
                }

                if input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                window.request_redraw();
            }
        });
    }
}

fn draw_display(display: &Display, frame: &mut [u8], cache: &mut HashMap<VideoCell, Image>) {
    for row in 0..DISPLAY_LINES {
        for column in 0..DISPLAY_COLUMNS {
            if let Ok(cell) = display.memory.get(column, row) {
                let img = match cache.get(&cell) {
                    Some(img) => img.clone(),
                    None => {
                        let img = Image::from_drawable(cell);
                        cache.insert(cell, img);
                        img.clone()
                    }
                };

                blit(
                    frame,
                    &(
                        column * CELL_WIDTH + COLUMN_OFFSET,
                        row * CELL_WIDTH + ROW_OFFSET,
                    ),
                    &cell,
                );
            }
        }
    }
}

trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl Drawable for VideoCell {
    fn width(&self) -> usize {
        CELL_WIDTH
    }

    fn height(&self) -> usize {
        CELL_HEIGHT
    }

    fn pixels(&self) -> &[u8] {
        let v = vec![self.background; CELL_WIDTH * CELL_HEIGHT]
            .iter()
            .flat_map(|s| -> &[u8] { (*s).into() })
            .map(|b| *b)
            .collect::<Vec<u8>>();
        &v[..]
    }
}

#[derive(Debug, Clone)]
struct Image {
    dimensions: (usize, usize),
    data: Vec<u8>,
}

impl Image {
    fn from_drawable<T: Drawable>(s: T) -> Self {
        Self {
            dimensions: (s.width(), s.height()),
            data: s.pixels().into(),
        }
    }
}

impl Drawable for Image {
    fn width(&self) -> usize {
        self.dimensions.0
    }

    fn height(&self) -> usize {
        self.dimensions.1
    }

    fn pixels(&self) -> &[u8] {
        &(self.data)
    }
}

fn blit<S>(screen: &mut [u8], dest: &(usize, usize), sprite: &S)
where
    S: Drawable,
{
    let (dest_x, dest_y) = *dest;
    assert!(dest_x + sprite.width() <= DISPLAY_LOGICAL_WIDTH);
    assert!(dest_y + sprite.height() <= DISPLAY_LOGICAL_HEIGHT);

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest_x * 4
            + dest_y * DISPLAY_LOGICAL_WIDTH * 4
            + y * DISPLAY_LOGICAL_HEIGHT * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}
