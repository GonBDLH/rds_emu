#![allow(clippy::single_match)]

mod hardware;

use crate::hardware::bus::Bus;

#[cfg(not(web_platform))]
fn main() -> Result<(), impl std::error::Error> {
    use std::{sync::Arc, thread, time};

    use winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::Window,
    };

    let mut bus = Bus::default();

    // SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let window = Window::new(&event_loop).unwrap();
        Arc::new(window)
    };

    thread::spawn({
        let window = window.clone();
        move || loop {
            thread::sleep(time::Duration::from_secs(1));
            window.request_redraw();
        }
    });

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            bus.cycle();
        }
        _ => (),
    })
}
