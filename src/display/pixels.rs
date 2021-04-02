use super::{DisplayAdapter, WindowSettings, DISPLAY_LOGICAL_HEIGHT, DISPLAY_LOGICAL_WIDTH};
use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

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

        event_loop.run(move |event, _, control_flow| {
            log::info!("Event: {:?}", event);

            if let Event::RedrawRequested(_) = event {
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
