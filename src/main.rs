use crate::engine::Engine;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod engine;
mod geometry_renderer;
mod util;

fn main() {
    let mut event_loop = EventLoop::new();
    let mut engine = Engine::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::Resized(physical_size) => {
                    engine.resize(physical_size.width, physical_size.height)
                }
                WindowEvent::CloseRequested => {
                    if window_id == engine.window().id() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                engine.render();
            }
            _ => (),
        }
    });
}
