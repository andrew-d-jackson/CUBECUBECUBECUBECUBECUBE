use crate::components::{FlyingControls, Light, Model, Position, TexturedModel};
use crate::quad::*;
use crate::resources::{ActiveTexture, Shaders, WindowInfo, WritableTextures};
use glium::index::PrimitiveType;
use glium::uniform;
use glium::Depth;
use glium::DepthTest;
use glium::DrawParameters;
use glium::Surface;
use glium::{IndexBuffer, VertexBuffer};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct RenderSystem {}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, Shaders>,
        Write<'a, WindowInfo>,
        Write<'a, WritableTextures>,
        WriteStorage<'a, Model>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, FlyingControls>,
        ReadStorage<'a, TexturedModel>,
        ReadStorage<'a, Light>,
        Read<'a, ActiveTexture>,
    );

    fn run(
        &mut self,
        (
            mut shaders,
            window_info,
            writable_textures,
            models,
            mut positions,
            flying_controls,
            textured_model,
            light,
            active_texture,
        ): Self::SystemData,
    ) {
        let y = window_info.display.clone().unwrap();
        let display = y.lock().unwrap();

        let (width, height) = display.get_framebuffer_dimensions();

        let mut camera_direction = glm::vec3(1.0f32, 0.0f32, 0.0f32);
        let mut camera_position = glm::vec3(-1.0f32, 0.0f32, 0.0f32);

        for (position, _) in (&mut positions, &flying_controls).join() {
            camera_position = position.get_pos_vec();
            camera_direction =
                glm::quat_rotate_vec3(&position.get_rot_as_quat(), &glm::vec3(0.0f32, 0.0, 1.0));
        }

        let up_vector = glm::vec3(0.0f32, 1.0f32, 0.0f32);
        let camera_target = camera_position + camera_direction;

        let camera = glm::look_at(&camera_position, &camera_target, &up_vector);
        let camera_matrix: &[[f32; 4]; 4] = camera.as_ref();

        let projection_matrix =
            glm::perspective(width as f32 / height as f32, 1.5708f32, 0.01f32, 600f32);
        let projection = projection_matrix.as_ref();

        let sun_projection = glm::ortho(-50f32, 50.0, -50.0, 50.0, 0.01, 400.0);
        let sun_projection_matrix = sun_projection.as_ref();

        let sun_distant_projection = glm::ortho(-350f32, 350.0, -350.0, 350.0, 0.01, 400.0);
        let sun_distant_projection_matrix = sun_distant_projection.as_ref();

        let sun_position = &camera_position + glm::vec3(100.0f32, 200.0, 100.0);
        let sun_look = glm::look_at(&sun_position, &camera_position, &up_vector);
        let sun_camera_tmp = sun_look.clone();
        let sun_camera_matrix = sun_camera_tmp.as_ref();

        let (quad_vertexs, quad_indexes) = get_quad_vertexes();
        let quad_vertex_buffer = VertexBuffer::new(&*display, &quad_vertexs.as_ref()).unwrap();
        let quad_index_buffer = IndexBuffer::new(
            &*display,
            PrimitiveType::TrianglesList,
            &quad_indexes.as_ref(),
        )
        .unwrap();

        let mut target = display.draw();

        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let sun_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        {
            let (sun_depth_color_texture, sun_depth_depth_texture) =
                writable_textures.get_textures("sun_depth".to_string());
            let (sun_distant_depth_color_texture, sun_distant_depth_depth_texture) =
                writable_textures.get_textures("sun_distant_depth".to_string());
            let (camera_color_texture, camera_depth_texture) =
                writable_textures.get_textures("camera".to_string());
            let (camera_normal_color_texture, camera_normal_depth_texture) =
                writable_textures.get_textures("camera_normal".to_string());
            let (composed_color_texture, composed_depth_texture) =
                writable_textures.get_textures("composed".to_string());
            let (light_depth_color_texture, light_depth_depth_texture) =
                writable_textures.get_textures("light_depth".to_string());

            let mut sun_depth_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &*display,
                &*sun_depth_color_texture,
                &*sun_depth_depth_texture,
            )
            .unwrap();
            let mut sun_distant_depth_buffer =
                glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                    &*display,
                    &*sun_distant_depth_color_texture,
                    &*sun_distant_depth_depth_texture,
                )
                .unwrap();
            let mut camera_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &*display,
                &*camera_color_texture,
                &*camera_depth_texture,
            )
            .unwrap();
            let mut camera_normal_buffer =
                glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                    &*display,
                    &*camera_normal_color_texture,
                    &*camera_normal_depth_texture,
                )
                .unwrap();
            let mut composed_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &*display,
                &*composed_color_texture,
                &*composed_depth_texture,
            )
            .unwrap();
            let mut light_depth_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &*display,
                &*light_depth_color_texture,
                &*light_depth_depth_texture,
            )
            .unwrap();

            sun_depth_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
            sun_depth_buffer.clear_depth(1.0);
            for (position, model) in (&positions, &models).join() {
                let vertex_buffer = &*model.vertex_buffer.lock().unwrap();
                let index_buffer = &*model.index_buffer.lock().unwrap();
                let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
                let translated =
                    glm::translate(&model_mat, &glm::vec3(position.x, position.y, position.z));
                sun_depth_buffer
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &shaders.get("cube_depth".to_string()).lock().unwrap(),
                        &uniform! {
                            model: *translated.as_ref(),
                            camera: *sun_camera_matrix,
                            projection: *sun_projection_matrix,
                        },
                        &sun_parameters,
                    )
                    .unwrap();
            }

            for (position, model) in (&positions, &textured_model).join() {
                let translation = position.get_transform_matrix();
                let models = model.models.lock().unwrap();
                for individual_model in models.iter() {
                    sun_depth_buffer
                        .draw(
                            &individual_model.vertex_buffer,
                            &individual_model.index_buffer,
                            &shaders.get("cube_depth".to_string()).lock().unwrap(),
                            &uniform! {
                                model: *translation.as_ref(),
                                camera: *sun_camera_matrix,
                                projection: *sun_projection_matrix,
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
            }

            sun_distant_depth_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
            sun_distant_depth_buffer.clear_depth(1.0);
            for (position, model) in (&positions, &models).join() {
                let vertex_buffer = &*model.vertex_buffer.lock().unwrap();
                let index_buffer = &*model.index_buffer.lock().unwrap();
                let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
                let translated =
                    glm::translate(&model_mat, &glm::vec3(position.x, position.y, position.z));
                sun_distant_depth_buffer
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &shaders.get("cube_depth".to_string()).lock().unwrap(),
                        &uniform! {
                            model: *translated.as_ref(),
                            camera: *sun_camera_matrix,
                            projection: *sun_distant_projection_matrix,
                        },
                        &sun_parameters,
                    )
                    .unwrap();
            }

            for (position, model) in (&positions, &textured_model).join() {
                let translation = position.get_transform_matrix();
                let models = model.models.lock().unwrap();
                for individual_model in models.iter() {
                    sun_distant_depth_buffer
                        .draw(
                            &individual_model.vertex_buffer,
                            &individual_model.index_buffer,
                            &shaders.get("cube_depth".to_string()).lock().unwrap(),
                            &uniform! {
                                model: *translation.as_ref(),
                                camera: *sun_camera_matrix,
                                projection: *sun_distant_projection_matrix,
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
            }

            camera_buffer.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
            for (position, model) in (&positions, &models).join() {
                let vertex_buffer = &*model.vertex_buffer.lock().unwrap();
                let index_buffer = &*model.index_buffer.lock().unwrap();
                let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
                let translated =
                    glm::translate(&model_mat, &glm::vec3(position.x, position.y, position.z));
                camera_buffer
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &shaders.get("cube_color".to_string()).lock().unwrap(),
                        &uniform! {
                            model: *translated.as_ref(),
                            camera: *camera_matrix,
                            projection: *projection,
                        },
                        &draw_parameters,
                    )
                    .unwrap();
            }

            for (position, model) in (&positions, &textured_model).join() {
                let translation = position.get_transform_matrix();
                let models = model.models.lock().unwrap();
                for individual_model in models.iter() {
                    camera_buffer.draw(
                        &individual_model.vertex_buffer,
                        &individual_model.index_buffer,
                        &shaders.get("texture".to_string()).lock().unwrap(),
                        &uniform! {
                            model: *translation.as_ref(),
                            camera: *camera_matrix,
                            projection: *projection,
                            diffuse_textrue: glium::uniforms::Sampler::new(&individual_model.texture),
                        },
                        &draw_parameters
                    ).unwrap();
                }
            }

            camera_normal_buffer.clear_color_and_depth((0.0f32, 0.3, 1.0, 0.0), 1.0);
            for (position, model) in (&positions, &models).join() {
                let vertex_buffer = &*model.vertex_buffer.lock().unwrap();
                let index_buffer = &*model.index_buffer.lock().unwrap();
                let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
                let translated =
                    glm::translate(&model_mat, &glm::vec3(position.x, position.y, position.z));
                camera_normal_buffer
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &shaders.get("cube_normal".to_string()).lock().unwrap(),
                        &uniform! {
                            model: *translated.as_ref(),
                            camera: *camera_matrix,
                            projection: *projection,
                        },
                        &draw_parameters,
                    )
                    .unwrap();
            }

            for (position, model) in (&positions, &textured_model).join() {
                let translation = position.get_transform_matrix();
                let models = model.models.lock().unwrap();
                for individual_model in models.iter() {
                    camera_normal_buffer
                        .draw(
                            &individual_model.vertex_buffer,
                            &individual_model.index_buffer,
                            &shaders.get("cube_normal".to_string()).lock().unwrap(),
                            &uniform! {
                                model: *translation.as_ref(),
                                camera: *camera_matrix,
                                projection: *projection,
                            },
                            &draw_parameters,
                        )
                        .unwrap();
                }
            }

            composed_buffer.clear_color_and_depth((0.0f32, 0.0, 0.0, 0.0), 1.0);
            composed_buffer.draw(
                &quad_vertex_buffer,
                &quad_index_buffer,
                &shaders.get("shadows".to_string()).lock().unwrap(),
                &uniform! {
                    sunDepth: glium::uniforms::Sampler::new(&*sun_depth_depth_texture),
                    sunDistantDepth: glium::uniforms::Sampler::new(&*sun_distant_depth_depth_texture),
                    cameraDepth: glium::uniforms::Sampler::new(&*camera_depth_texture),
                    cameraColor: glium::uniforms::Sampler::new(&*camera_color_texture),
                    cameraNormals: glium::uniforms::Sampler::new(&*camera_normal_color_texture),
                    sunPosition: *sun_position.as_ref(),

                    sunProjection: *sun_projection_matrix,
                    sunDistantProjection: *sun_distant_projection_matrix,
                    sunView: *sun_camera_matrix,
                    cameraView: *camera_matrix,
                    cameraProjection: *projection,
                },
                &draw_parameters
            ).unwrap();

            let light_projection_matrix = glm::ortho(-50f32, 50.0, -50.0, 50.0, 0.01, 300.0);

            for (light_pos, light) in (&positions, &light).join() {
                let light_position = glm::vec3(light_pos.x, light_pos.y, light_pos.z);
                let light_direction = glm::quat_rotate_vec3(
                    &light_pos.get_rot_as_quat(),
                    &glm::vec3(0.0f32, 0.0, 1.0),
                );
                let light_look = glm::look_at(
                    &light_position,
                    &(light_position + light_direction),
                    &up_vector,
                );

                light_depth_buffer.clear_depth(1.0);
                for (position, model) in (&positions, &models).join() {
                    let vertex_buffer = &*model.vertex_buffer.lock().unwrap();
                    let index_buffer = &*model.index_buffer.lock().unwrap();
                    let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
                    let translated =
                        glm::translate(&model_mat, &glm::vec3(position.x, position.y, position.z));
                    light_depth_buffer
                        .draw(
                            vertex_buffer,
                            index_buffer,
                            &shaders.get("cube_depth".to_string()).lock().unwrap(),
                            &uniform! {
                                model: *translated.as_ref(),
                                camera: *light_look.as_ref(),
                                projection: *light_projection_matrix.as_ref(),
                            },
                            &sun_parameters,
                        )
                        .unwrap();
                }

                let light_strength = 0.2f32;

                composed_buffer.clear_depth(1.0);
                composed_buffer.draw(
                    &quad_vertex_buffer,
                    &quad_index_buffer,
                    &shaders.get("light".to_string()).lock().unwrap(),
                    &uniform! {
                        lightDepth: glium::uniforms::Sampler::new(&*light_depth_depth_texture),
                        currentColor: glium::uniforms::Sampler::new(&*composed_color_texture),
                        cameraDepth: glium::uniforms::Sampler::new(&*camera_depth_texture),
                        cameraColor: glium::uniforms::Sampler::new(&*camera_color_texture),
                        cameraNormals: glium::uniforms::Sampler::new(&*camera_normal_color_texture),

                        lightProjection: *light_projection_matrix.as_ref(),
                        lightView: *light_look.as_ref(),

                        cameraView: *camera_matrix,
                        cameraProjection: *projection,

                        lightPosition: *light_position.as_ref(),
                        lightColor: *light.color.as_ref(),
                        lightStrength: light_strength,
                    },
                    &draw_parameters
                ).unwrap();
            }
        }

        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);

        let shad = active_texture.active_texture.clone();

        let (tex, dep_tex) = writable_textures.get_textures(shad);
        if active_texture.depth {
            target
                .draw(
                    &quad_vertex_buffer,
                    &quad_index_buffer,
                    &shaders.get("2d".to_string()).lock().unwrap(),
                    &uniform! {
                        texFramebuffer: glium::uniforms::Sampler::new(&*dep_tex)
                    },
                    &draw_parameters,
                )
                .unwrap();
        } else {
            target
                .draw(
                    &quad_vertex_buffer,
                    &quad_index_buffer,
                    &shaders.get("2d".to_string()).lock().unwrap(),
                    &uniform! {
                        texFramebuffer: glium::uniforms::Sampler::new(&*tex)
                    },
                    &draw_parameters,
                )
                .unwrap();
        }

        target.finish().unwrap();
    }
}
