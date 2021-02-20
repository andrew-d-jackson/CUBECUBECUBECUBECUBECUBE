use glium::index::PrimitiveType;
use glium::{implement_vertex, Display, IndexBuffer, VertexBuffer};
use specs::{Component, VecStorage};
use std::sync::{Arc, Mutex};
use tobj::load_obj;

#[derive(Debug, Clone, Copy)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoords: [f32; 2],
}

implement_vertex!(TexturedVertex, position, normal, texcoords);

pub struct IndividualModel {
    pub vertex_buffer: VertexBuffer<TexturedVertex>,
    pub index_buffer: IndexBuffer<u32>,
    pub texture: glium::texture::SrgbTexture2d,
}

#[derive(Clone, Default)]
pub struct TexturedModel {
    pub models: Arc<Mutex<Vec<IndividualModel>>>,
}

impl Component for TexturedModel {
    type Storage = VecStorage<Self>;
}

unsafe impl Send for TexturedModel {}
unsafe impl Sync for TexturedModel {}

impl TexturedModel {
    pub fn new(file: String, display: &Display) -> Self {
        let (models, materials) = load_obj(file, true).unwrap();
        let mut individual_models: Vec<IndividualModel> = vec![];
        for m in models.iter() {
            let mesh = &m.mesh;
            let mut vertexes: Vec<TexturedVertex> = vec![];
            for i in 0..mesh.positions.len() / 3 {
                vertexes.push(TexturedVertex {
                    position: [
                        mesh.positions[i * 3] as f32,
                        mesh.positions[i * 3 + 1] as f32,
                        mesh.positions[i * 3 + 2] as f32,
                    ],
                    normal: [0.0f32, 1f32, 0f32],
                    texcoords: [
                        mesh.texcoords[i * 2] as f32,
                        mesh.texcoords[i * 2 + 1] as f32,
                    ],
                });
            }
            let vertex_buffer = VertexBuffer::new(display, &vertexes.as_ref()).unwrap();
            let index_buffer = IndexBuffer::new(
                display,
                PrimitiveType::TrianglesList,
                &mesh.indices.as_ref(),
            )
            .unwrap();

            let material = &materials[mesh.material_id.unwrap()];
            let img = image::open(format!("objs/{}", material.diffuse_texture))
                .unwrap()
                .into_rgba();
            let img_dimensions = img.dimensions();
            let img_raw =
                glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dimensions);
            let diffuse_texture = glium::texture::SrgbTexture2d::new(display, img_raw).unwrap();

            individual_models.push(IndividualModel {
                vertex_buffer,
                index_buffer,
                texture: diffuse_texture,
            });
        }
        Self {
            models: Arc::new(Mutex::new(individual_models)),
        }
    }
}
