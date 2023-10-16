use fpt::lr35902::LR35902;

use std::{
    sync::{Arc, Mutex},
    thread, time,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    //let lr: Arc<Mutex<LR35902>> = Arc::new(Mutex::new(LR35902::new()));
    //let lr_for_the_thing: Arc<Mutex<LR35902>> = Arc::clone(&lr);

    //let the_thing = thread::spawn(move || {
    //    let mut loop_cycle: u64 = 0;
    //    loop {
    //        loop_cycle += 1;
    //        println!("---[Loop cycle: {:#04}]---", loop_cycle);

    //        lr_for_the_thing.lock().unwrap().step();

    //        println!();
    //        thread::sleep(time::Duration::from_millis(100));
    //    }
    //});

    //the_loop(lr.clone());
    //the_thing.join().unwrap();
}

fn the_loop(lr: Arc<Mutex<LR35902>>) {
    let event_loop: EventLoop<()> = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                // Application update code.
                lr.lock().unwrap().step();

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            _ => (),
        }
    });
}
