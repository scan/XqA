use super::{Display, DisplayAdapter, WindowSettings, DISPLAY_COLUMNS, DISPLAY_LINES};
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
            let size = LogicalSize::new(settings.width as f32, settings.height as f32);
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
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

            Pixels::new(DISPLAY_LOGICAL_WIDTH as u32, DISPLAY_LOGICAL_HEIGHT as u32, surface_texture)?
        };

        let mut display = Display::default();

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                draw_display(&display, pixels.get_frame());

                if let Err(e) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    log::error!("pixels.render() failed: {}", e);
                    return;
                }
            }

            if input.update(&event) {
                log::info!("Event: {:?}", event);

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

#[inline]
fn pos_to_pix(column: usize, row: usize) -> (usize, usize) {
    (row * CELL_HEIGHT + ROW_OFFSET, column * CELL_WIDTH + COLUMN_OFFSET)
}

#[inline]
fn pix_to_mem(x: usize, y: usize) -> usize {
    (y * DISPLAY_LOGICAL_WIDTH) + x
}

fn draw_display(display: &Display, frame: &mut [u8]) {
    for row in 0..DISPLAY_LINES {
        for column in 0..DISPLAY_COLUMNS {
            let (start_x, start_y) = pos_to_pix(column, row);
            let (end_x, end_y) = (start_x + CELL_WIDTH, start_y + CELL_HEIGHT);
            let col: [u8; 4] = display.memory.get(column, row).unwrap().background.into();

            for y in start_y..end_y {
                for x in start_x..end_x {
                    let mem_pos = pix_to_mem(x, y) * 4;

                    if mem_pos < frame.len() - 4 {
                        frame[mem_pos] = col[0];
                        frame[mem_pos + 1] = col[1];
                        frame[mem_pos + 2] = col[2];
                        frame[mem_pos + 3] = col[3];
                    }
                }
            }
        }
    }
}
