use std::thread;

use fpt::lr35902::LR35902;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let mut lr = LR35902::new();

    loop {
        lr.step();
    }

}

