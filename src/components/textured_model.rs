use glium::index::PrimitiveType;
use glium::{implement_vertex, Display, IndexBuffer, VertexBuffer};
use specs::{Component, VecStorage};
use std::collections::HashMap;
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
    pub texture: usize,
}

#[derive(Clone, Default)]
pub struct TexturedModel {
    pub textures: Arc<Mutex<Vec<glium::texture::SrgbTexture2d>>>,
    pub models: Arc<Mutex<Vec<IndividualModel>>>,
}

impl Component for TexturedModel {
    type Storage = VecStorage<Self>;
}

unsafe impl Send for TexturedModel {}
unsafe impl Sync for TexturedModel {}

impl TexturedModel {
    pub fn new(name: String, display: &Display) -> Self {
        let (models, materials) = load_obj(format!("objs/{}/{}.obj", name, name), true).unwrap();
        let mut individual_models: Vec<IndividualModel> = vec![];
        let mut texture_indexes: HashMap<String, usize> = HashMap::new();
        let mut textures: Vec<glium::texture::SrgbTexture2d> = vec![];

        let mut material_vertexes: HashMap<String, Vec<TexturedVertex>> = HashMap::new();
        let mut material_indexes: HashMap<String, Vec<u32>> = HashMap::new();

        for m in models.iter() {
            let mesh = &m.mesh;
            if let Some(material_id) = mesh.material_id {
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

                let material = &materials[material_id];
                if !material_vertexes.contains_key(&material.diffuse_texture) {
                    material_vertexes.insert(material.diffuse_texture.clone(), vertexes);
                    material_indexes.insert(material.diffuse_texture.clone(), mesh.indices.clone());
                } else {
                    let starting_index = material_vertexes
                        .get(&material.diffuse_texture)
                        .unwrap()
                        .len() as u32;
                    material_vertexes
                        .get_mut(&material.diffuse_texture)
                        .unwrap()
                        .append(&mut vertexes);
                    let new_indexes: Vec<u32> =
                        mesh.indices.iter().map(|i| i + starting_index).collect();
                    material_indexes
                        .get_mut(&material.diffuse_texture)
                        .unwrap()
                        .append(&mut new_indexes.clone());
                }
            }
        }

        for (material_name, vertexes) in material_vertexes {
            let indices = material_indexes.get(&material_name).unwrap();
            let vertex_buffer = VertexBuffer::new(display, &vertexes.as_ref()).unwrap();
            let index_buffer =
                IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices.as_ref()).unwrap();

            if !texture_indexes.contains_key(&material_name) {
                let img = image::open(format!("objs/{}/{}", name, &material_name))
                    .unwrap()
                    .into_rgba();
                let img_dimensions = img.dimensions();
                let img_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &img.into_raw(),
                    img_dimensions,
                );
                let diffuse_texture = glium::texture::SrgbTexture2d::new(display, img_raw).unwrap();
                textures.push(diffuse_texture);
                texture_indexes.insert(material_name.clone(), textures.len() - 1);
            }

            individual_models.push(IndividualModel {
                vertex_buffer,
                index_buffer,
                texture: texture_indexes.get(&material_name).unwrap().clone(),
            });
        }

        Self {
            models: Arc::new(Mutex::new(individual_models)),
            textures: Arc::new(Mutex::new(textures)),
        }
    }
}
