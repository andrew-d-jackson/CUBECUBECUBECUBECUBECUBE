use glium::implement_vertex;
use tobj::Mesh;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct MyVertex {
    pub position: [f32; 3],
    pub color: [u8; 3],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct OceanVertex {
    pub position: [f32; 3],
}

implement_vertex!(MyVertex, position, color, normal);
implement_vertex!(OceanVertex, position);

pub fn create_ocean_buffer() -> (Vec<OceanVertex>, Vec<u32>) {
    (
        vec![
            OceanVertex {
                position: [0f32, 200.0, 0.0],
            },
            OceanVertex {
                position: [0.0, 200.0, 512.0],
            },
            OceanVertex {
                position: [512.0, 200.0, 0.0],
            },
            OceanVertex {
                position: [512.0, 200.0, 512.0],
            },
            OceanVertex {
                position: [512.0, 200.0, 0.0],
            },
            OceanVertex {
                position: [0.0, 200.0, 512.0],
            },
        ],
        vec![0, 1, 2, 3, 4, 5],
    )
}

pub fn create_buffers(
    map_data: Vec<Vec<Vec<Option<Color>>>>,
    obj: Mesh,
) -> (Vec<MyVertex>, Vec<u32>) {
    let mut indices: Vec<u32> = vec![];
    let mut vertexes: Vec<MyVertex> = vec![];

    for x in 0..map_data.len() {
        for y in 0..map_data[0].len() {
            for z in 0..map_data[0][0].len() {
                match &map_data[x as usize][y as usize][z as usize] {
                    None => (),
                    Some(color) => {
                        let mut vertexes_for_block: Vec<MyVertex> = vec![];
                        for i in 0..obj.positions.len() / 3 {
                            vertexes_for_block.push(MyVertex {
                                position: [
                                    obj.positions[i * 3] + x as f32,
                                    obj.positions[i * 3 + 1] + 512.0 - z as f32,
                                    obj.positions[i * 3 + 2] + y as f32,
                                ],
                                normal: [
                                    obj.normals[i * 3] as f32,
                                    obj.normals[i * 3 + 1] as f32,
                                    obj.normals[i * 3 + 2] as f32,
                                ],
                                color: [color.r as u8, color.g as u8, color.b as u8],
                            });
                        }

                        for i in obj.indices.iter() {
                            indices.push(vertexes.len() as u32 + *i as u32);
                        }
                        for i in vertexes_for_block.into_iter() {
                            vertexes.push(i);
                        }
                    }
                }
            }
        }
    }

    (vertexes, indices)
}
