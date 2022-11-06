use std::env;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, VirtualKeyCode, ElementState, MouseButton};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use ray_tracing_utils::image::Image;

const WIN_WIDTH: u32 = 512;
const WIN_HEIGHT: u32 = 512;

fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please specify filepath.");
    }

    let filepath = &args[1];
    let img = Image::from_path(filepath);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIN_WIDTH, WIN_HEIGHT);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(
            window_size.width,
            window_size.height,
            &window
        );
        Pixels::new(img.width, img.height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                draw(pixels.get_frame(), &img);
                if pixels.render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },
            Event::WindowEvent { ref event, .. } => {
                match event {
                    // WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        window.drag_window().unwrap()
                    }
                    _ => (),
                }
            },
            _ => (),
        };

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

fn draw(frame: &mut [u8], img: &Image) {
    for (k, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let px = img.get_pixel(k);
        let rgba = [px.r, px.g, px.b, px.a];
        pixel.copy_from_slice(&rgba);
    }
}
