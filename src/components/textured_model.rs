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
    pub texindex: u32,
}

implement_vertex!(TexturedVertex, position, normal, texcoords, texindex);

pub struct IndividualModel {
    pub vertex_buffer: VertexBuffer<TexturedVertex>,
    pub index_buffer: IndexBuffer<u32>,
}

#[derive(Clone, Default)]
pub struct TexturedModel {
    pub textures: Arc<Mutex<Option<glium::texture::Texture2dArray>>>,
    pub model: Arc<Mutex<Option<IndividualModel>>>,
}

impl Component for TexturedModel {
    type Storage = VecStorage<Self>;
}

unsafe impl Send for TexturedModel {}
unsafe impl Sync for TexturedModel {}

impl TexturedModel {
    pub fn new(name: String, display: &Display) -> Self {
        let (models, materials) = load_obj(format!("objs/{}/{}.obj", name, name), true).unwrap();
        let mut material_order: Vec<String> = vec![];
        let mut vertexes: Vec<TexturedVertex> = vec![];
        let mut indexes: Vec<u32> = vec![];

        let textureArray = {
            let mut raw_images = vec![];
            for material in materials.iter() {
                let material_name = &material.diffuse_texture;
                material_order.push(material_name.clone());
                let img = image::open(format!("objs/{}/{}", name, &material_name))
                    .unwrap()
                    .into_rgba();
                let n_img = image::imageops::resize(&img, 1024, 1024, image::imageops::FilterType::Nearest);
                let img_dimensions = n_img.dimensions();
                let img_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &n_img.into_raw(),
                    img_dimensions,
                );
                raw_images.push(img_raw);
            }
            glium::texture::Texture2dArray::new(display, raw_images).unwrap()
        };

        for m in models.iter() {
            let mesh = &m.mesh;

            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];
                let material_index = material_order
                    .iter()
                    .position(|m| m == &material.diffuse_texture)
                    .unwrap() as u32;
                let starting_index = vertexes.len() as u32;

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
                        texindex: material_index,
                    });
                }

                let mut new_indexes: Vec<u32> =
                    mesh.indices.iter().map(|i| i + starting_index).collect();
                indexes.append(&mut new_indexes);
            }
        }
        let vertex_buffer = VertexBuffer::new(display, &vertexes.as_ref()).unwrap();
        let index_buffer =
            IndexBuffer::new(display, PrimitiveType::TrianglesList, &indexes.as_ref()).unwrap();
        Self {
            model: Arc::new(Mutex::new(Some(IndividualModel {
                vertex_buffer,
                index_buffer,
            }))),
            textures: Arc::new(Mutex::new(Some(textureArray))),
        }
    }
}
