extern crate glium;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate rand;
extern crate specs;
use glium::index::PrimitiveType;
use tobj::load_obj;

use std::sync::{Arc, Mutex};

use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::vertex::VertexBufferAny;
use glium::{Display, IndexBuffer, VertexBuffer};
use specs::prelude::*;
use std::time::{Duration, Instant};

mod cube;
mod input;
mod map;
mod misc;
mod quad;
mod shader;
mod systems;
mod vxl;
use systems::*;
mod components;
use components::*;
mod resources;
use resources::*;

fn create_window(event_loop: &EventLoop<()>) -> Display {
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new().with_depth_buffer(24);
    Display::new(wb, cb, event_loop).unwrap()
}

fn load_object(filename: String) -> (Vec<tobj::Model>, Vec<tobj::Material>) {
    load_obj(filename, true).unwrap()
}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F) -> !
where
    F: 'static + FnMut(&Vec<Event<()>>) -> Action,
{
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => match cause {
                StartCause::ResumeTimeReached { .. } | StartCause::Init => true,
                _ => false,
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => false,
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(1666667);
            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            }
            Action::Stop => *control_flow = ControlFlow::Exit,
        }
    })
}
fn main() {
    let event_loop = EventLoop::new();
    let display = create_window(&event_loop);
    let (width, height) = display.get_framebuffer_dimensions();

    let the_rock = TexturedModel::new("./objs/TheRock2.obj".to_string(), &display);
    let (cube_obj, _) = load_object("./objs/cube.obj".to_string());
    let map = vxl::load_map("./maps/London.vxl".to_string(), (512, 512, 512));
    let (vertexes, indices) = map::create_buffers(map.clone(), cube_obj[0].mesh.clone());
    let vertex_buffer = VertexBuffer::new(&display, &vertexes.as_ref()).unwrap();
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices.as_ref()).unwrap();

    let mut shaders = Shaders::new();
    shaders.create_program(&display, "cube_color".to_string());
    shaders.create_program(&display, "cube_depth".to_string());
    shaders.create_program(&display, "2d".to_string());
    shaders.create_program(&display, "shadows".to_string());
    shaders.create_program(&display, "cube_normal".to_string());
    shaders.create_program(&display, "ocean".to_string());
    shaders.create_program(&display, "light".to_string());
    shaders.create_program(&display, "cube_pos".to_string());
    shaders.create_program(&display, "texture".to_string());

    let mut writable_textures = WritableTextures::new();
    writable_textures.insert("camera".to_string(), &display);
    writable_textures.insert("camera_normal".to_string(), &display);
    writable_textures.insert("sun_depth".to_string(), &display);
    writable_textures.insert("sun_distant_depth".to_string(), &display);
    writable_textures.insert("composed".to_string(), &display);
    writable_textures.insert("light".to_string(), &display);
    writable_textures.insert("light_depth".to_string(), &display);

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Model>();
    world.register::<TexturedModel>();
    world.register::<FlyingControls>();
    world.register::<Light>();
    world.register::<RotateRandomly>();

    world.insert(Inputs::new());
    world.insert(ActiveTexture {
        active_texture: "composed".to_string(),
        depth: false,
    });
    world.insert(WindowInfo {
        display: Option::Some(Arc::new(Mutex::new(display))),
        width: width,
        height: height,
        delta_time: 0.0,
        resized: false,
    });
    world.insert(writable_textures);
    world.insert(shaders);

    world
        .create_entity()
        .with(Position::new())
        .with(Model {
            vertex_buffer: Arc::new(Mutex::new(VertexBufferAny::from(vertex_buffer))),
            index_buffer: Arc::new(Mutex::new(index_buffer)),
        })
        .build();

    world
        .create_entity()
        .with(
            Position::new_pos(256.0f32, 380.0, 256.0)
                .scale(0.1, 0.1, 0.1)
                .rotate(0.1, 0.4, 0.3),
        )
        .with(the_rock)
        .with(RotateRandomly {})
        .build();

    world
        .create_entity()
        .with(Position::new_pos(256.0f32, 510.0, 256.0))
        .with(FlyingControls {})
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(UpdateWindowSystem::new())
        .with_thread_local(FPSSystem {
            map: map,
            velocity: glm::vec3(0.0f32, 0.0, 0.0),
        })
        .with_thread_local(RotateRandomlySystem {})
        .with_thread_local(SwitchActiveTextureSystem {})
        .with_thread_local(ReloadShadersSystem {})
        .with_thread_local(ResizeTexturesSystem {})
        .with_thread_local(AddLightSystem {})
        .with_thread_local(RenderSystem {})
        .build();

    start_loop(event_loop, move |events| {
        let mut should_close = false;
        for event in events.iter() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        should_close = true;
                    }
                    _ => (),
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } => (),
                    StartCause::Init => (),
                    _ => (),
                },
                _ => (),
            }
        }

        {
            let mut inputs = world.write_resource::<Inputs>();
            inputs.process_events(events);
        }

        dispatcher.run_now(&world);

        if should_close {
            Action::Stop
        } else {
            Action::Continue
        }
    });
}
