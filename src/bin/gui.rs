use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use pixels::{Pixels, SurfaceTexture};

const GB_RESOLUTION: (u32, u32) = (160, 144);
const SCALE: u32 = 3;

fn main() -> Result<(), pixels::Error> {
    let event_loop: EventLoop<()> = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("fpt (winit + pixels)")
        .with_inner_size(LogicalSize::new(
            SCALE * GB_RESOLUTION.0,
            SCALE * GB_RESOLUTION.1,
        ))
        .with_min_inner_size(LogicalSize::new(GB_RESOLUTION.0, GB_RESOLUTION.1))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(GB_RESOLUTION.0, GB_RESOLUTION.1, surface_texture)?
    };

    let mut frame_number = 0u32;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event:
                ref e @ (WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }),
            ..
        } => {
            println!(
                "{reason}; stopping",
                reason = match e {
                    WindowEvent::CloseRequested => "The close button was pressed",
                    WindowEvent::KeyboardInput { .. } => "The ESC key was pressed",
                    _ => "whatever",
                }
            );
            control_flow.set_exit();
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } => {
            if let Err(err) = pixels.resize_surface(size.width, size.height) {
                eprintln!("pixels.resize_surface() error! {err}");
                control_flow.set_exit_with_code(1);
                return;
            }
        }
        Event::MainEventsCleared => {
            for _ in 0..53 {
                draw_something(pixels.frame_mut(), frame_number);
                frame_number += 1;
            }
            if let Err(err) = pixels.render() {
                eprintln!("pixels.render() error! {err}");
                control_flow.set_exit_with_code(2);
                return;
            }
            // window.request_redraw();
        }
        _ => (),
    });
}

fn draw_something(frame: &mut [u8], frame_number: u32) {
    // random arithmetics written at 2 AM
    let pos = (frame_number % (frame.len() as u32 / 4)) as usize;
    let pos = if pos / GB_RESOLUTION.0 as usize % 2 > 0 {
        pos + 4 * GB_RESOLUTION.0 as usize
    } else {
        pos
    };
    let pos = pos % ((frame.len() / 8) - 4) as usize;
    let rgba: [u8; 4] = [
        (frame_number % 0xFF) as u8,
        128_i32
            .wrapping_sub_unsigned(2 * frame_number)
            .rem_euclid(0xFF) as u8,
        ((92 + 3 * frame_number) % 0xFF) as u8,
        0xFF,
    ];
    let pixel = &mut frame[(8 * pos)..(8 * pos + 4)];
    pixel.copy_from_slice(&rgba);
}
