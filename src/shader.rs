use glium::{Program, Display, ProgramCreationError};
use std::fs::File;
use std::io::Read;

pub fn load_shader_string(filename: String) -> String {
    let mut f = File::open(filename).unwrap();
    let mut ret = String::new();
    f.read_to_string(&mut ret).unwrap();
    ret
}

pub fn create_program(display: &Display, vertex_shader: String, fragment_shader: String) -> Result<Program, ProgramCreationError> {
    Program::from_source(display, vertex_shader.as_ref(), fragment_shader.as_ref(), None)
}
