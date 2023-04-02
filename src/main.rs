use crate::engine::{Engine, DEFAULT_HEIGHT, DEFAULT_WIDTH};

use dolly::prelude::{YawPitch};
use glam::{Mat4, Quat, Vec3};

use std::{mem, slice};
use winit::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod engine;
mod geometry_renderer;
mod util;

fn main() {
    let event_loop = EventLoop::new();
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
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key_code) = input.virtual_keycode {
                        if key_code == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let projection_matrix = Mat4::perspective_lh(
                    90.0f32.to_radians(),
                    DEFAULT_WIDTH as f32 / DEFAULT_HEIGHT as f32,
                    0.1,
                    1000.0,
                );

                let transform = engine.geometry_renderer().camera_rig().final_transform;

                let view_projection_matrix = projection_matrix
                    * Mat4::look_at_lh(
                        transform.position,
                        transform.position + transform.forward(),
                        transform.up(),
                    )
                    * Mat4::from_rotation_translation(Quat::IDENTITY, Vec3::new(0.0, 0.0, 1.0));

                engine.queue().write_buffer(
                    engine.geometry_renderer().uniform_buffer(),
                    0,
                    unsafe {
                        slice::from_raw_parts(
                            &view_projection_matrix as *const Mat4 as *const _,
                            mem::size_of::<Mat4>(),
                        )
                    },
                );

                //TODO: refactor this architecture, just make it work now

                engine.render();
            }
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    let camera_rig = &mut engine.geometry_renderer_mut().camera_rig_mut();
                    camera_rig
                        .driver_mut::<YawPitch>()
                        .rotate_yaw_pitch(0.3 * delta.0 as f32, -0.3 * delta.1 as f32);
                    camera_rig.update(1. / 120.);
                }
            }
            _ => (),
        }
    });
}
