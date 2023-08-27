use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    reference::{Image, Library},
    renderer::State,
};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("RustyRef")
        .build(&event_loop)
        .unwrap();

    let mut ctx = State::new(window).await;
    let mut cursor_position: winit::dpi::PhysicalPosition<f64> = (0., 0.).into();
    let mut hovered_image_id: uuid::Uuid = uuid::Uuid::new_v4();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == ctx.window().id() => {
                if !ctx.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => ctx.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            ctx.resize(**new_inner_size)
                        }
                        WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _,
                        } => {
                            cursor_position = *position;
                            println!("{:?}", cursor_position);
                        }
                        WindowEvent::HoveredFile(path_buff) => {
                            let maybe_image = std::fs::read(path_buff.as_path());
                            match maybe_image {
                                Ok(image) => {
                                    let goldorak = Image::new(
                                        [
                                            (cursor_position.x as f32 / ctx.size.width as f32) * 2.
                                                - 1.,
                                            (cursor_position.y as f32 / ctx.size.height as f32)
                                                * -2.
                                                + 1.,
                                        ],
                                        image,
                                    );
                                    let maybe_image_id = ctx.add_image_to_library(goldorak);
                                    match maybe_image_id {
                                        Some(image_id) => hovered_image_id = image_id,
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        WindowEvent::DroppedFile(_) => {
                            ctx.draw(hovered_image_id);
                        }
                        WindowEvent::MouseInput {
                            device_id: _,
                            state: _,
                            button,
                            modifiers: _,
                        } if *button == MouseButton::Left => {}
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == ctx.window().id() => {
                ctx.update();
                match ctx.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => ctx.resize(ctx.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                ctx.window().request_redraw();
            }
            _ => (),
        }
    });
}
