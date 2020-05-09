#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
use obj::*;
use std::io::BufReader;
use glm::U4;
use std::marker::Copy;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Debug;
use glium::index::PrimitiveType;

#[derive(Clone, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn load_shader(filename: String) -> String {
    let mut f = File::open(filename).unwrap();
    let mut ret = String::new();
    f.read_to_string(&mut ret);
    ret
}

fn load_map(filename: String) -> Vec<Vec<Vec<Option<Color>>>> {
    let mut f = File::open(filename).unwrap();
    let map_file: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();

    let mut result : Vec<Vec<Vec<Option<Color>>>> = vec![vec![vec![None; 512]; 512]; 512];


    let mut bytePosition: u32 = 0;

    for x in 0..512 {
        for y in 0..512 {
            loop {
                let numberOfChunks = map_file[bytePosition as usize];
                let topColorStart = map_file[(bytePosition + 1) as usize];
                let topColorEnd = map_file[(bytePosition + 2) as usize];
                let lengthOfBottom = topColorEnd + 1 - topColorStart;
                let mut colorPosition = bytePosition + 4;

                for z in topColorStart..topColorEnd+1 {
                    let b = colorPosition;
                    colorPosition = colorPosition + 1;
                    let g = colorPosition;
                    colorPosition = colorPosition + 1;
                    let r = colorPosition;
                    colorPosition = colorPosition + 1;
                    result[x as usize][y as usize][z as usize] = Some(Color { r: map_file[r as usize], g: map_file[g as usize], b: map_file[b as usize] });

                    colorPosition = colorPosition + 1
                }

                if numberOfChunks == 0 {
                    bytePosition = bytePosition + (4 * (lengthOfBottom as u32 + 1));
                    break;
                }

                bytePosition += numberOfChunks as u32 * 4;

                let lengthOfTop = (numberOfChunks - 1) - lengthOfBottom;
                let bottomColorEnd = map_file[(bytePosition + 3) as usize];
                let bottomColorStart = bottomColorEnd - lengthOfTop;

                for z in bottomColorStart..bottomColorEnd {
                    let b = colorPosition;
                    colorPosition = colorPosition + 1;
                    let g = colorPosition;
                    colorPosition = colorPosition + 1;
                    let r = colorPosition;
                    colorPosition = colorPosition + 1;

                    result[x as usize][y as usize][z as usize] = Some(Color { r: map_file[r as usize], g: map_file[g as usize], b: map_file[b as usize] });
                    colorPosition = colorPosition + 1;
                }
            }
        }
    }

    result
}

#[derive(Clone, Copy, Debug)]
struct MyVertex {
    position: [f32; 3],
    color: [u8; 3],
    normal: [f32; 3],
}

implement_vertex!(MyVertex, position, color, normal);


fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};
    let map = load_map("./src/TokyoNeon.vxl".to_string());

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let input = BufReader::new(File::open("./src/cube.objs").unwrap());
    let obj: Obj = load_obj(input).unwrap();

    let mut vertexes: Vec<MyVertex> = vec![];
    let mut indexes: Vec<u32> = vec![];

    for x in 0..512 {
        for y in 0..512 {
            for z in 0..512 {
                match &map[x as usize][y as usize][z as usize] {
                    None => (),
                    Some(color) => {

                        let verts = obj.vertices.iter().map(|v| {
                            MyVertex {
                                position: [
                                    v.position[0] + x as f32,
                                    v.position[1] + z as f32,
                                    v.position[2] + y as f32,
                                ],
                                normal: v.normal,
                                color: [
                                    color.r as u8,
                                    color.g as u8,
                                    color.b as u8,
                                ],
                            }
                        }).collect::<Vec<MyVertex>>();

                        for i in obj.indices.iter() {
                            indexes.push(vertexes.len() as u32 + *i as u32);
                        }
                        for i in verts.into_iter() {
                            vertexes.push(i);
                        }

                    }
                }
            }
        }
    }


    let vertex_buffer = glium::VertexBuffer::new(&display, &vertexes.as_ref()).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indexes.as_ref()).unwrap();

    print!("ad");
/*
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;
        in vec3 color;

        out vec3 Normal;
        out vec3 Color;

        uniform mat4 projection;
        uniform mat4 matrix;
        uniform mat4 camera;

        void main() {
            Normal = normal;
            Color = color;
            gl_Position = projection * camera * matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 Normal;
        in vec3 Color;

        out vec4 color;
        void main() {
            float brightness = dot(normalize(Normal), normalize(vec3(-50.0, -70.0, -30.0)));
            vec3 reg_color = vec3(Color.x / 255, Color.y / 255, Color.z / 255);
            vec3 dark_color = reg_color / 2;
            color = vec4(mix(dark_color, reg_color, brightness), 1.0);
        }
    "#;

    */

    let mut x = 1.0f32;


    let mut last_frame = std::time::Instant::now();
    let mut last_x = 0f32;
    let mut last_y = -200f32;

    let mut x_rotation = 0f32;
    let mut z_rotation = 200f32;
    let mut y_rotation = 0f32;

    let mut camera_pos = glm::vec3(2f32, 1f32, -2f32);
    let mut x_pos = 2f32;
    let mut y_pos = 1f32;
    let mut z_pos = -2f32;

    let mut frag_shader_str = load_shader("./src/cube_f.glsl".to_string());
    let mut vert_shader_str = load_shader("./src/cube_v.glsl".to_string());
    let mut current_vert_shader_str = frag_shader_str.clone();
    let mut current_frag_shader_str = vert_shader_str.clone();

    let mut program = glium::Program::from_source(&display, vert_shader_str.as_ref(), frag_shader_str.as_ref(), None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let current_time =  std::time::Instant::now();
        let dt: f32 = (current_time - last_frame).as_secs_f32();
        last_frame = current_time;

        x = x + dt;


      //  println!("{}, {}, {}", x_pos + x_rotation.cos(), y_pos + y_rotation.sin(), z_pos + x_rotation.sin());

        let cameraDir = glm::vec3(x_rotation.cos(), y_rotation.sin(), x_rotation.sin());
        let cameraTarget = camera_pos + cameraDir;
        let up = glm::vec3(0.0f32, 1.0, 0.0);

       // let view = glm::look_at(&cameraPos, &cameraTarget, &up);

        let radius = 1f32;
        let camera = glm::look_at(
            &camera_pos,
            &cameraTarget,
            &up
        );
        let cameraMatrix: &[[f32; 4]; 4] = camera.as_ref();
        let projectionMatrix = glm::perspective(1f32, 55f32, 0.01f32, 100f32);
        let projection = projectionMatrix.as_ref();


        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::CursorMoved { device_id, position, modifiers } => {
                    x_rotation = x_rotation + ((position.x as f32 - last_x) / 100f32);
                    y_rotation = (y_rotation + (position.y as f32 - last_y) / 100f32).min(1f32).max(-1f32);
                    last_x = position.x as f32;
                    last_y = position.y as f32;
                },
                glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if input.virtual_keycode.is_none() {
                    } else
                    if input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::W {
                        camera_pos = &camera_pos + (&cameraDir / 100f32);
                    } else
                    if input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::S {
                        camera_pos = &camera_pos - (&cameraDir / 100f32);
                    } else
                    if input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::A {
                        let twod = &glm::vec2(cameraDir.x, cameraDir.z);
                        let rotated = glm::rotate_vec2(twod, 1.57f32);
                        let right_vector = glm::vec3(rotated.x, 0.0f32, rotated.y);
                        camera_pos = &camera_pos + (right_vector / 100f32)
                    } else
                    if input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::D {
                        let twod = &glm::vec2(cameraDir.x, cameraDir.z);
                        let rotated = glm::rotate_vec2(twod, 1.57f32);
                        let right_vector = glm::vec3(rotated.x, 0.0f32, rotated.y);
                        camera_pos = &camera_pos - (right_vector / 100f32)
                    } else
                    if input.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::P {
                        println!("Hello");
                        let mut frag_shader_str = load_shader("./src/cube_f.glsl".to_string());
                        let mut vert_shader_str = load_shader("./src/cube_v.glsl".to_string());
                        let mut current_vert_shader_str = frag_shader_str.clone();
                        let mut current_frag_shader_str = vert_shader_str.clone();
                        program = glium::Program::from_source(&display, vert_shader_str.as_ref(), frag_shader_str.as_ref(), None).unwrap();
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);


        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };


        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                                    matrix: matrix,
                                    camera: *cameraMatrix,
                                    projection: *projection
                                },
            &params
        ).unwrap();

      /*  let vertex_buffer = glium::VertexBuffer::new(&display, &[
            Vertex { position: [0.0,  0.0, 0.0], texcoords: [0.0, 1.0] },
            Vertex { position: [5.0, -3.0, 2.0], texcoords: [1.0, 0.0] },
        ]);
*/
/*
        for x in 0..512 {
            println!("x {}", x);
            for y in 0..512 {
                for z in 0..512 {
                    match &map[x as usize][y as usize][z as usize] {
                        None => (),
                        Some(color) => {
                            let id = glm::identity::<f32, U4>();
                            let mut matrix1 = glm::scale(&id, &glm::vec3(0.1f32, 0.1, 0.1));
                            matrix1 = glm::translate(&matrix1, &glm::vec3(x as f32, y as f32, z as f32));
                            let matrix2 = matrix1.as_ref();

                            target.draw(
                                &positions,
                                &indices,
                                &program,
                                &uniform! {
                                    matrix: *matrix2,
                                    camera: *cameraMatrix,
                                    projection: *projection
                                },
                    &params
                            ).unwrap();
                        }
                    }
                }
            }
        }*/
        target.finish().unwrap();
    });
}