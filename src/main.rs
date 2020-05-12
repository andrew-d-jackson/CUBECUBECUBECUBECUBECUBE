extern crate glium;
extern crate nalgebra_glm as glm;
use std::io::BufReader;
use obj::{load_obj, Obj};
use std::fs::File;
use glium::index::PrimitiveType;

use glium::{uniform, Surface};
use glium::glutin::dpi::LogicalPosition;

use glium::{Display, VertexBuffer, IndexBuffer, DrawParameters, Depth};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::draw_parameters::DepthTest;
use glium::glutin::ContextBuilder;
use glium::glutin::event::{Event, WindowEvent, VirtualKeyCode, StartCause};
use crate::input::KeyboardState;
use std::iter;
use glium::backend::Facade;
use glium::texture::DepthTexture2d;
use glium::framebuffer::SimpleFrameBuffer;
use std::rc::Rc;
use glium::texture::texture2d::Texture2d;
use std::time::{Instant, Duration};
use crate::Action::{Continue, Stop};

mod cube;
mod shader;
mod vxl;
mod map;
mod input;
mod quad;

fn create_window(event_loop: &EventLoop<()>) -> Display {
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new().with_depth_buffer(24);
    Display::new(wb, cb, event_loop).unwrap()
}

fn load_object(filename: String) -> Obj {
    let input = BufReader::new(File::open(filename).unwrap());
    let obj: Obj = load_obj(input).unwrap();
    obj
}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)->! where F: 'static + FnMut(&Vec<Event<()>>) -> Action {
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => {
                match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                        true
                    },
                    _ => false
                }
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            },
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(1666667);
            // TODO: Add back the old accumulator loop in some way

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            },
            Action::Stop => *control_flow = ControlFlow::Exit
        }
    })
}


fn main() {
    let event_loop = EventLoop::new();
    let display = create_window(&event_loop);
  //  display.gl_window().window().set_cursor_grab(true);
  //  display.gl_window().window().set_cursor_position(LogicalPosition::new(50,50));

    let cube_obj = load_object("./src/objs/cube.obj".to_string());
    let map = vxl::load_map("./src/maps/London.vxl".to_string(), (512, 512, 512));
    let (vertexes, indices) = map::create_buffers(map, cube_obj);
    let vertex_buffer = VertexBuffer::new(&display, &vertexes.as_ref()).unwrap();
    let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices.as_ref()).unwrap();

    let (quad_vertexs, quad_indexes) = quad::get_quad_vertexes();
    let quad_vertex_buffer = VertexBuffer::new(&display, &quad_vertexs.as_ref()).unwrap();
    let quad_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &quad_indexes.as_ref()).unwrap();

    let (ocean_vertexs, ocean_indexes) = map::create_ocean_buffer();
    let ocean_vertex_buffer = VertexBuffer::new(&display, &ocean_vertexs.as_ref()).unwrap();
    let ocean_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &ocean_indexes.as_ref()).unwrap();

    let (width, height) = display.get_framebuffer_dimensions();

    let depth_texture = (
        DepthTexture2d::empty(&display, width, height).unwrap()
    );

    let sun_depth_texture = (
        DepthTexture2d::empty(&display, width, height).unwrap()
    );

    let sun_depth_texture2 = (
        DepthTexture2d::empty(&display, width, height).unwrap()
    );

    let color_depth_texture = (
        DepthTexture2d::empty(&display, width, height).unwrap()
    );

    let color_texture = (
        Texture2d::empty(&display, width, height).unwrap()
    );

    let shadow_texture = (
        Texture2d::empty(&display, width, height).unwrap()
    );

    let normal_texture = (
        Texture2d::empty(&display, width, height).unwrap()
    );

    let normal_depth_texture = (
        DepthTexture2d::empty(&display, width, height).unwrap()
    );

    let mut keyboard_inputs: input::KeyboardState = input::KeyboardState::new();

    let mut last_frame_time = std::time::Instant::now();
    let mut last_mouse_x = 0f32;
    let mut last_mouse_y = -200f32;

    let mut camera_position = glm::vec3(2f32, 1f32, -2f32);
    let mut camera_rotation_abs = glm::vec2(0.0, 0.0f32);

    let mut program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/cube_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/cube_f.glsl".to_string()),
    ).unwrap();

    let mut depth_program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/cube_depth_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/cube_depth_f.glsl".to_string()),
    ).unwrap();

    let mut quad_program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/2d_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/2d_f.glsl".to_string()),
    ).unwrap();

    let mut shadow_program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/shadows_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/shadows_f.glsl".to_string()),
    ).unwrap();

    let mut normal_program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/cube_normal_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/cube_normal_f.glsl".to_string()),
    ).unwrap();

    let mut ocean_program = shader::create_program(
        &display,
        shader::load_shader_string("./src/shaders/ocean_v.glsl".to_string()),
        shader::load_shader_string("./src/shaders/ocean_f.glsl".to_string()),
    ).unwrap();

    let mut x: f32 = 0.0;
    let mut active_shader: String = "player".to_string();

    let up_vector = glm::vec3(0.0f32, 1.0, 0.0);
    let mut sun_position = &camera_position + glm::vec3(0.2f32, -1.0, 0.2);
    let mut sun_look = glm::look_at(
        &sun_position, &camera_position, &up_vector,
    );
    start_loop(event_loop, move |events| {
        //display.gl_window().window().set_cursor_position(LogicalPosition::new(50,50));

        let mut should_close = false;
        let current_time =  std::time::Instant::now();
        let next_frame_time = current_time + std::time::Duration::from_nanos(16_666_667);
        let delta_time: f32 = (current_time - last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        keyboard_inputs.reset_presses();

        let mut sun_buffer = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &sun_depth_texture).unwrap();
        let mut sun_buffer2 = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &sun_depth_texture2).unwrap();

        let mut depth_buffer = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &depth_texture).unwrap();
        let mut color_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_texture, &color_depth_texture).unwrap();
        let mut shadow_buffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &shadow_texture).unwrap();
        let mut normal_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &normal_texture, &normal_depth_texture).unwrap();

        let camera_direction = glm::vec3(
            camera_rotation_abs.x.cos(), camera_rotation_abs.y.sin(), camera_rotation_abs.x.sin()
        );
        let camera_target = camera_position + camera_direction;

        let camera = glm::look_at(
            &camera_position, &camera_target, &up_vector,
        );
        let camera_matrix: &[[f32; 4]; 4] = camera.as_ref();

        let projection_matrix = glm::perspective(1f32, 55f32, 0.01f32, 10f32);
        let projection = projection_matrix.as_ref();

        let sun_projection = glm::ortho(-0.5f32, 0.5, -0.5, 0.5, 0.01, 3.0);
        let sun_projection_matrix = sun_projection.as_ref();

        let sun_camera_tmp = sun_look.clone();
        let sun_camera_matrix = sun_camera_tmp.as_ref();
        let sun_projection2 = glm::ortho(-5.5f32, 5.5, -5.5, 5.5, 0.01, 3.0);
        let sun_projection2_matrix = sun_projection2.as_ref();

        for event in events.iter() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        should_close = true;
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        camera_rotation_abs = glm::vec2(
                            camera_rotation_abs.x - ((position.x as f32 - last_mouse_x) / 100f32),
                            (camera_rotation_abs.y + (position.y as f32 - last_mouse_y) / 100f32).min(2f32).max(-2f32),
                        );
                        last_mouse_x = position.x as f32;
                        last_mouse_y = position.y as f32;
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        keyboard_inputs.process_keyboard_input(*input);
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

        if keyboard_inputs.is_pressed(VirtualKeyCode::W) {
            camera_position = &camera_position + (&camera_direction * delta_time);
        }

        if keyboard_inputs.is_pressed(VirtualKeyCode::S) {
            camera_position = &camera_position - (&camera_direction * delta_time);
        }

        if keyboard_inputs.is_pressed(VirtualKeyCode::A) {
            let two_dimensional = &glm::vec2(camera_direction.x, camera_direction.z);
            let rotated = glm::rotate_vec2(two_dimensional, 1.57f32);
            let three_dimensional = glm::vec3(rotated.x, 0.0f32, rotated.y);
            let normalised = glm::normalize(&three_dimensional);
            camera_position = &camera_position + (normalised  * delta_time);
        }

        if keyboard_inputs.is_pressed(VirtualKeyCode::D) {
            let two_dimensional = &glm::vec2(camera_direction.x, camera_direction.z);
            let rotated = glm::rotate_vec2(two_dimensional, 1.57f32);
            let three_dimensional = glm::vec3(rotated.x, 0.0f32, rotated.y);
            let normalised = glm::normalize(&three_dimensional);
            camera_position = &camera_position - (normalised * delta_time);
        }

        if keyboard_inputs.was_released(VirtualKeyCode::P) {
            println!("Hello");
            let programres = shader::create_program(
                &display,
                shader::load_shader_string("./src/shaders/cube_v.glsl".to_string()),
                shader::load_shader_string("./src/shaders/cube_f.glsl".to_string()),
            );
            match programres {
                Ok(prog) => program = prog,
                Err(e) => println!("{}", e),
            }

            depth_program = shader::create_program(
                &display,
                shader::load_shader_string("./src/shaders/cube_depth_v.glsl".to_string()),
                shader::load_shader_string("./src/shaders/cube_depth_f.glsl".to_string()),
            ).unwrap();

            quad_program = shader::create_program(
                &display,
                shader::load_shader_string("./src/shaders/2d_v.glsl".to_string()),
                shader::load_shader_string("./src/shaders/2d_f.glsl".to_string()),
            ).unwrap();

            shadow_program = shader::create_program(
                &display,
                shader::load_shader_string("./src/shaders/shadows_v.glsl".to_string()),
                shader::load_shader_string("./src/shaders/shadows_f.glsl".to_string()),
            ).unwrap();

            ocean_program = shader::create_program(
                &display,
                shader::load_shader_string("./src/shaders/ocean_v.glsl".to_string()),
                shader::load_shader_string("./src/shaders/ocean_f.glsl".to_string()),
            ).unwrap();
        }


        if keyboard_inputs.was_released(VirtualKeyCode::B) {
            active_shader = "player".to_string();
        }
        if keyboard_inputs.was_released(VirtualKeyCode::N) {
            active_shader = "sun2".to_string();
        }
        if keyboard_inputs.was_released(VirtualKeyCode::M) {
            active_shader = "sun".to_string();
        }
        if keyboard_inputs.was_released(VirtualKeyCode::V) {
            active_shader = "shadow".to_string();
        }
        if keyboard_inputs.was_released(VirtualKeyCode::C) {
            active_shader = "normal".to_string();
        }

        if keyboard_inputs.was_released(VirtualKeyCode::L) {
            sun_position = camera_position.clone();
            sun_look = glm::look_at(
                &camera_position, &camera_target, &up_vector,
            );
        }

        let mut target = display.draw();
        //target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);

        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let sun_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        let model_matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        sun_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
        sun_buffer.clear_depth(1.0);
        sun_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            &depth_program,
            &uniform! {
                model: model_matrix,
                camera: *sun_camera_matrix,
                projection: *sun_projection_matrix,
            },
            &draw_parameters
        ).unwrap();

        sun_buffer2.clear_color(1.0, 1.0, 1.0, 1.0);
        sun_buffer2.clear_depth(1.0);
        sun_buffer2.draw(
            &vertex_buffer,
            &index_buffer,
            &depth_program,
            &uniform! {
                model: model_matrix,
                camera: *sun_camera_matrix,
                projection: *sun_projection2_matrix,
            },
            &draw_parameters
        ).unwrap();
/*
        depth_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
        depth_buffer.clear_depth(1.0);
        depth_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            &depth_program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection
            },
            &draw_parameters
        ).unwrap();
*/
        color_buffer.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
        color_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
                sunDepth: glium::uniforms::Sampler::new(&sun_depth_texture),
                sunProjection: *sun_projection_matrix,
                sunDepth2: glium::uniforms::Sampler::new(&sun_depth_texture2),
                sunProjection2: *sun_projection2_matrix,
                sunView: *sun_camera_matrix,
            },
            &draw_parameters
        ).unwrap();

        color_buffer.draw(
            &ocean_vertex_buffer,
            &ocean_index_buffer,
            &ocean_program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
                sunDepth: glium::uniforms::Sampler::new(&sun_depth_texture),
                sunProjection: *sun_projection_matrix,
                sunDepth2: glium::uniforms::Sampler::new(&sun_depth_texture2),
                sunProjection2: *sun_projection2_matrix,
                sunView: *sun_camera_matrix,

            },
            &draw_parameters
        ).unwrap();
/*
        normal_buffer.clear_color_and_depth((0.0f32, 0.0, 0.0, 0.0), 1.0);
        normal_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            &normal_program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection
            },
            &draw_parameters
        ).unwrap();
*/
        //target.clear_color_and_depth((0.0f32, 0.0, 0.0, 0.0), 0.0);
        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);
  //      shadow_buffer.clear_color(0.0, 1.0, 1.0, 1.0);
        /*target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection
            },
            &draw_parameters
        ).unwrap();
*/
        if active_shader == "player".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &quad_program,
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&color_texture)
            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "sun2".to_string() {
          target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &quad_program,
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&sun_depth_texture2)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)

            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "sun".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &quad_program,
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&sun_depth_texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)

            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "shadow".to_string() {
          /*target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shadow_program,
                &uniform! {
                    sunDepth: glium::uniforms::Sampler::new(&sun_depth_texture),
                    cameraDepth: glium::uniforms::Sampler::new(&depth_texture),
                    cameraColor: glium::uniforms::Sampler::new(&color_texture),
                    sunProjection: *sun_projection_matrix,
                    sunView: *sun_camera_matrix,
                    cameraProjection: *projection,
                    cameraView: *camera_matrix,
                },
                &draw_parameters
            ).unwrap();*/
        } else if active_shader == "normal".to_string() {
    /*     target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &quad_program,
                &uniform! {
                    texFramebuffer: glium::uniforms::Sampler::new(&normal_texture)
                },
                &draw_parameters
            ).unwrap();*/
        }
/*
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection
            },
            &draw_parameters
        ).unwrap();*/

        target.finish().unwrap();

        if (should_close) {
            Action::Stop
        } else {
            Action::Continue
        }
    });
}