use glium::implement_vertex;

#[derive(Clone, Copy, Debug)]
pub struct QuadVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(QuadVertex, position, tex_coords);

pub fn get_quad_vertexes() -> (Vec<QuadVertex>, Vec<u16>) {
    (
        vec![
            QuadVertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            QuadVertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            QuadVertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            QuadVertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            QuadVertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            QuadVertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
        ],
        vec![0, 1, 2, 5, 4, 3],
    )
}
