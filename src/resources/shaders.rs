use glium::{Display, Program, ProgramCreationError};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn load_shader_string(filename: String) -> String {
    let mut f = File::open(filename).unwrap();
    let mut ret = String::new();
    f.read_to_string(&mut ret).unwrap();
    ret
}

pub fn create_program(
    display: &Display,
    vertex_shader: String,
    fragment_shader: String,
) -> Result<Program, ProgramCreationError> {
    Program::from_source(
        display,
        vertex_shader.as_ref(),
        fragment_shader.as_ref(),
        None,
    )
}

#[derive(Clone, Default)]
pub struct Shaders {
    pub programs: HashMap<String, Arc<Mutex<Program>>>,
}

unsafe impl Send for Shaders {}
unsafe impl Sync for Shaders {}

impl Shaders {
    pub fn create_program(&mut self, display: &Display, name: String) -> () {
        let result = create_program(
            &display,
            load_shader_string(format!("./shaders/{}_v.glsl", name)),
            load_shader_string(format!("./shaders/{}_f.glsl", name)),
        );

        match result {
            Ok(program) => {
                self.programs.insert(name, Arc::new(Mutex::new(program)));
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    }

    pub fn new() -> Shaders {
        Shaders {
            programs: HashMap::default(),
        }
    }

    pub fn get(&mut self, name: String) -> Arc<Mutex<Program>> {
        self.programs.get(&name).unwrap().clone()
    }

    pub fn reload_all(&mut self, display: &Display) -> () {
        let keys: Vec<String> = self.programs.keys().map(|k| k.clone()).collect();
        keys.into_iter()
            .for_each(|key| self.create_program(display, key));
    }
}
