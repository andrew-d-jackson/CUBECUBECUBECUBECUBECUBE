use obj::Obj;
use glium::implement_vertex;

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

implement_vertex!(MyVertex, position, color, normal);

pub fn create_buffers(map_data: Vec<Vec<Vec<Option<Color>>>>, obj: Obj) -> (Vec<MyVertex>, Vec<u32>) {
    let mut indices: Vec<u32> = vec![];
    let mut vertexes: Vec<MyVertex> = vec![];

    for x in 0..map_data.len() {
        for y in 0..map_data[0].len() {
            for z in 0..map_data[0][0].len() {
                match &map_data[x as usize][y as usize][z as usize] {
                    None => (),
                    Some(color) => {
                        let vertexes_for_block = obj.vertices.iter().map(|v| {
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