extern crate glium;
extern crate rand;
extern crate nalgebra_glm as glm;
use std::io::BufReader;
use obj::{load_obj, Obj};
use std::fs::File;
use glium::index::PrimitiveType;

use glium::{uniform, Surface};
use glium::glutin::dpi::LogicalPosition;

use glium::{Display, VertexBuffer, IndexBuffer, DrawParameters, Depth, Program};
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
use std::collections::HashMap;
use rand::Rng;


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


struct DepthFrameBuffer {
    depth_texture: DepthTexture2d,
}
impl DepthFrameBuffer {
    fn new(display: &Display) -> DepthFrameBuffer {
        let (width, height) = display.get_framebuffer_dimensions();
        DepthFrameBuffer {
            depth_texture: DepthTexture2d::empty(display, width, height).unwrap(),
        }
    }

    fn get_frame_buffer(&mut self, display: &Display) -> SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::depth_only(display, &self.depth_texture).unwrap()
    }

    fn resize(&mut self, display: &Display) -> () {
        let (width, height) = display.get_framebuffer_dimensions();
        self.depth_texture = DepthTexture2d::empty(display, width, height).unwrap();
    }
}

struct ColorFrameBuffer {
    color_texture: Texture2d,
}
impl ColorFrameBuffer {
    fn new(display: &Display) -> ColorFrameBuffer {
        let (width, height) = display.get_framebuffer_dimensions();
        ColorFrameBuffer {
            color_texture: Texture2d::empty(display, width, height).unwrap()
        }
    }

    fn get_frame_buffer(&mut self, f: &Facade) -> SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::new(f, &self.color_texture).unwrap()
    }

    fn resize(&mut self, display: &Display) -> () {
        let (width, height) = display.get_framebuffer_dimensions();
        self.color_texture = Texture2d::empty(display, width, height).unwrap();
    }
}

struct ColorDepthFrameBuffer {
    color_texture: Texture2d,
    depth_texture: DepthTexture2d,
}
impl ColorDepthFrameBuffer {
    fn new(display: &Display) -> ColorDepthFrameBuffer {
        let (width, height) = display.get_framebuffer_dimensions();
        ColorDepthFrameBuffer {
            color_texture: Texture2d::empty(display, width, height).unwrap(),
            depth_texture: DepthTexture2d::empty(display, width, height).unwrap(),
        }
    }

    fn get_frame_buffer(&self, f: &Facade) -> SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(f, &self.color_texture, &self.depth_texture).unwrap()
    }

    fn resize(&mut self, display: &Display) -> () {
        let (width, height) = display.get_framebuffer_dimensions();
        self.color_texture = Texture2d::empty(display, width, height).unwrap();
        self.depth_texture = DepthTexture2d::empty(display, width, height).unwrap();
    }
}

struct Shaders {
    programs: HashMap<String, Program>,
}

impl Shaders {
    fn create_program(&mut self, display: &Display, name: String) -> () {
        println!("Reloading 1");
        let result = shader::create_program(
            &display,
            shader::load_shader_string(format!("./shaders/{}_v.glsl", name)),
            shader::load_shader_string(format!("./shaders/{}_f.glsl", name)),
        );

        match result {
            Ok(program) => { self.programs.insert(name, program); }
            Err(err) => { println!("{}", err); }
        };
    }

    fn new() -> Shaders {
        Shaders { programs: HashMap::default() }
    }

    fn get(&mut self, name: String) -> &Program {
        self.programs.get(&name).unwrap()
    }

    fn reload_all(&mut self, display: &Display) -> () {
        println!("Reloading");
        let keys: Vec<String> = self.programs.keys().map(|k| k.clone()).collect();
        println!("{:?}", keys);
        keys.into_iter().for_each(|key| self.create_program(display, key));
    }
}

struct Light {
    position: glm::Vec3,
    look_at: glm::Vec3,
    color: glm::Vec3,
}

fn main() {
    let mut rng = rand::thread_rng();
    let event_loop = EventLoop::new();
    let display = create_window(&event_loop);
  //  display.gl_window().window().set_cursor_grab(true);
  //  display.gl_window().window().set_cursor_position(LogicalPosition::new(50,50));

    let cube_obj = load_object("./objs/cube.obj".to_string());
    let map = vxl::load_map("./maps/CityOfChicago.vxl".to_string(), (512, 512, 512));
    let (vertexes, indices) = map::create_buffers(map, cube_obj);
    let vertex_buffer = VertexBuffer::new(&display, &vertexes.as_ref()).unwrap();
    let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices.as_ref()).unwrap();

    let (quad_vertexs, quad_indexes) = quad::get_quad_vertexes();
    let quad_vertex_buffer = VertexBuffer::new(&display, &quad_vertexs.as_ref()).unwrap();
    let quad_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &quad_indexes.as_ref()).unwrap();

    let (ocean_vertexs, ocean_indexes) = map::create_ocean_buffer();
    let ocean_vertex_buffer = VertexBuffer::new(&display, &ocean_vertexs.as_ref()).unwrap();
    let ocean_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &ocean_indexes.as_ref()).unwrap();

    let mut shaders = Shaders::new();

    let mut camera_frame_buffer = ColorDepthFrameBuffer::new(&display);
    let mut camera_normal_buffer = ColorDepthFrameBuffer::new(&display);
    let mut sun_depth = DepthFrameBuffer::new(&display);
    let mut far_sun_depth = DepthFrameBuffer::new(&display);
    let mut composed_frame_buffer = ColorDepthFrameBuffer::new(&display);
    let mut light_frame_buffer = ColorDepthFrameBuffer::new(&display);
    let mut light_depth_buffer = DepthFrameBuffer::new(&display);
    let mut camera_world_buffer = ColorDepthFrameBuffer::new(&display);

    let mut keyboard_inputs: input::KeyboardState = input::KeyboardState::new();

    let mut last_frame_time = std::time::Instant::now();
    let mut last_mouse_x = 0f32;
    let mut last_mouse_y = -200f32;

    let mut camera_position = glm::vec3(2f32, 1f32, -2f32);
    let mut camera_rotation_abs = glm::vec2(0.0, 0.0f32);
    shaders.create_program(&display, "cube_color".to_string());
    shaders.create_program(&display, "cube_depth".to_string());
    shaders.create_program(&display, "2d".to_string());
    shaders.create_program(&display, "shadows".to_string());
    shaders.create_program(&display, "cube_normal".to_string());
    shaders.create_program(&display, "ocean".to_string());
    shaders.create_program(&display, "light".to_string());
    shaders.create_program(&display, "cube_pos".to_string());

    let mut x: f32 = 0.0;
    let mut active_shader: String = "player".to_string();

    let up_vector = glm::vec3(0.0f32, 1.0, 0.0);

    let (mut width, mut height) = display.get_framebuffer_dimensions();

    let mut lights: Vec<Light> = vec![];

    start_loop(event_loop, move |events| {
        //display.gl_window().window().set_cursor_position(LogicalPosition::new(50,50));

        let (current_width, current_height) = display.get_framebuffer_dimensions();
        if current_width != width || current_height != height {
            println!("resized");
            camera_frame_buffer.resize(&display);
            camera_normal_buffer.resize(&display);
            sun_depth.resize(&display);
            far_sun_depth.resize(&display);
            composed_frame_buffer.resize(&display);
            width = current_width;
            height = current_height;
        }

        let mut should_close = false;
        let current_time =  std::time::Instant::now();
        let next_frame_time = current_time + std::time::Duration::from_nanos(16_666_667);
        let delta_time: f32 = (current_time - last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        keyboard_inputs.reset_presses();

        let camera_direction = glm::vec3(
            camera_rotation_abs.x.cos(), camera_rotation_abs.y.sin(), camera_rotation_abs.x.sin()
        );
        let camera_target = camera_position + camera_direction;

        let camera = glm::look_at(
            &camera_position, &camera_target, &up_vector,
        );
        let camera_matrix: &[[f32; 4]; 4] = camera.as_ref();

        let projection_matrix = glm::perspective(width as f32 / height as f32, 55f32, 0.01f32, 10f32);
        let projection = projection_matrix.as_ref();

        let sun_projection = glm::ortho(-0.5f32, 0.5, -0.5, 0.5, 0.01, 3.0);
        let sun_projection_matrix = sun_projection.as_ref();

        let sun_position = &camera_position + glm::vec3(0.2f32, -1.0, 0.2);
        let sun_look = glm::look_at(
            &sun_position, &camera_position, &up_vector,
        );
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
            shaders.reload_all(&display);
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
            lights.push(Light {
                position: camera_position.clone(),
                look_at: camera_target.clone(),
                color: glm::Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
            });
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

        let mut sun_buffer = sun_depth.get_frame_buffer(&display);
        sun_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
        sun_buffer.clear_depth(1.0);
        sun_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            shaders.get("cube_depth".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *sun_camera_matrix,
                projection: *sun_projection_matrix,
            },
            &sun_parameters
        ).unwrap();

        let mut camera_world_buff = camera_world_buffer.get_frame_buffer(&display);
        camera_world_buff.clear_color(0.0, 0.0, 0.0, 1.0);
        camera_world_buff.clear_depth(1.0);
        camera_world_buff.draw(
            &vertex_buffer,
            &index_buffer,
            shaders.get("cube_pos".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
            },
            &draw_parameters
        ).unwrap();

        let mut far_sun_depth_buffer = far_sun_depth.get_frame_buffer(&display);
        far_sun_depth_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
        far_sun_depth_buffer.clear_depth(1.0);
        far_sun_depth_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            shaders.get("cube_depth".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *sun_camera_matrix,
                projection: *sun_projection2_matrix,
            },
            &sun_parameters
        ).unwrap();

        let mut color_buffer = camera_frame_buffer.get_frame_buffer(&display);
        color_buffer.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
        color_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            shaders.get("cube_color".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
            },
            &draw_parameters
        ).unwrap();

        color_buffer.draw(
            &ocean_vertex_buffer,
            &ocean_index_buffer,
            &shaders.get("ocean".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
            },
            &draw_parameters
        ).unwrap();

        let mut normal_buffer = camera_normal_buffer.get_frame_buffer(&display);
        normal_buffer.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
        normal_buffer.draw(
            &vertex_buffer,
            &index_buffer,
            shaders.get("cube_normal".to_string()),
            &uniform! {
                model: model_matrix,
                camera: *camera_matrix,
                projection: *projection,
            },
            &draw_parameters
        ).unwrap();


        {
            let mut composed = composed_frame_buffer.get_frame_buffer(&display);
            composed.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
            composed.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("shadows".to_string()),
                &uniform! {

                    sunDepth: glium::uniforms::Sampler::new(&sun_depth.depth_texture),
                    cameraDepth: glium::uniforms::Sampler::new(&camera_frame_buffer.depth_texture),
                    cameraColor: glium::uniforms::Sampler::new(&camera_frame_buffer.color_texture),
                    cameraNormals: glium::uniforms::Sampler::new(&camera_normal_buffer.color_texture),
                    sunPosition: *sun_position.as_ref(),

                    sunProjection: *sun_projection_matrix,
                    sunView: *sun_camera_matrix,
                    cameraView: *camera_matrix,
                    cameraProjection: *projection,
                },
                &draw_parameters
            ).unwrap();
        }

        //let mut light_depth_buffer = light_frame_buffer.get_frame_buffer(&display);
        let light_projection_matrix = glm::ortho(-0.5f32, 0.5, -0.5, 0.5, 0.01, 3.0);
        for light in lights.iter() {

            let light_look = glm::look_at(
                &light.position, &light.look_at, &up_vector,
            );


            let mut light_depth_buf = light_depth_buffer.get_frame_buffer(&display);
            light_depth_buf.clear_depth(1.0);
            light_depth_buf.draw(
                &vertex_buffer,
                &index_buffer,
                shaders.get("cube_depth".to_string()),
                &uniform! {
                    model: model_matrix,
                    camera: *light_look.as_ref(),
                    projection: *light_projection_matrix.as_ref(),
                },
                &sun_parameters
            ).unwrap();

            let mut composedBuf = composed_frame_buffer.get_frame_buffer(&display);
            let stren = 0.2f32;
            composedBuf.clear_depth(1.0);
            composedBuf.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("light".to_string()),
                &uniform! {
                lightDepth: glium::uniforms::Sampler::new(&light_depth_buffer.depth_texture),
                currentColor: glium::uniforms::Sampler::new(&composed_frame_buffer.color_texture),
                cameraDepth: glium::uniforms::Sampler::new(&camera_frame_buffer.depth_texture),
                cameraColor: glium::uniforms::Sampler::new(&camera_frame_buffer.color_texture),
                cameraNormals: glium::uniforms::Sampler::new(&camera_normal_buffer.color_texture),

                lightProjection: *light_projection_matrix.as_ref(),
                lightView: *light_look.as_ref(),
 
                cameraView: *camera_matrix,
                cameraProjection: *projection,

                lightPosition: *light.position.as_ref(),
                lightColor: *light.color.as_ref(),
                lightStrength: stren,
            },
                &draw_parameters
            ).unwrap();
        }


        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);

        if active_shader == "player".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("2d".to_string()),
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&camera_frame_buffer.color_texture)
            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "sun2".to_string() {
          target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("2d".to_string()),
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&far_sun_depth.depth_texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)

            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "sun".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("2d".to_string()),
                &uniform! {
                texFramebuffer: glium::uniforms::Sampler::new(&sun_depth.depth_texture)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)

            },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "shadow".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("2d".to_string()),
                &uniform! {
                    texFramebuffer: glium::uniforms::Sampler::new(&composed_frame_buffer.color_texture)
                },
                &draw_parameters
            ).unwrap();
        } else if active_shader == "normal".to_string() {
            target.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("2d".to_string()),
                &uniform! {
                    texFramebuffer: glium::uniforms::Sampler::new(&camera_world_buffer.color_texture)
                },
                &draw_parameters
            ).unwrap();
        }

        target.finish().unwrap();

        if (should_close) {
            Action::Stop
        } else {
            Action::Continue
        }
    });
}