#![feature(array_chunks)]

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use pixels::{Pixels, SurfaceTexture};

const GB_RESOLUTION: (u32, u32) = (160, 144);
const SCALE: u32 = 3;
const PALETTE: [[u8; 4]; 4] = [
    [0, 63, 0, 255],
    [46, 115, 32, 255],
    [140, 191, 10, 255],
    [160, 207, 10, 255],
];

fn main() -> Result<(), pixels::Error> {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.connect("tcp://127.0.0.1:5000").unwrap();
    let topic = "frame".to_owned().into_bytes();
    socket.set_subscribe(&topic).unwrap();

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

    event_loop.run(move |event, _, control_flow| {
        match event {
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
                let _topic = socket.recv_msg(0).unwrap();
                let data = socket.recv_msg(0).unwrap();
                let frame = data.iter().copied().collect::<Vec<u8>>();
                draw(pixels.frame_mut(), &frame.try_into().unwrap());

                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() error! {err}");
                    control_flow.set_exit_with_code(2);
                    return;
                }
            }
            _ => (),
        }
    });
}

fn draw(pixels_frame: &mut [u8], gb_frame: &fpt::ppu::Frame) {
    for (i, chunk) in pixels_frame.array_chunks_mut::<4>().enumerate() {
        chunk.copy_from_slice(&PALETTE[gb_frame[i] as usize]);
    }
}
