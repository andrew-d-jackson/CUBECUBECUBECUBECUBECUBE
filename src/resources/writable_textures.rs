use glium::texture::depth_texture2d::DepthTexture2d;
use glium::Display;
use glium::Texture2d;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Default)]
pub struct WritableTextures {
    pub color_textures: HashMap<String, Arc<Mutex<Texture2d>>>,
    pub depth_textures: HashMap<String, Arc<Mutex<DepthTexture2d>>>,
}

unsafe impl Send for WritableTextures {}
unsafe impl Sync for WritableTextures {}

impl WritableTextures {
    pub fn new() -> WritableTextures {
        WritableTextures {
            color_textures: HashMap::new(),
            depth_textures: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, display: &Display) -> () {
        let (width, height) = display.get_framebuffer_dimensions();
        self.color_textures.insert(
            name.clone(),
            Arc::new(Mutex::new(
                Texture2d::empty(display, width, height).unwrap(),
            )),
        );
        self.depth_textures.insert(
            name.clone(),
            Arc::new(Mutex::new(
                DepthTexture2d::empty(display, width, height).unwrap(),
            )),
        );
    }

    /*   pub fn get_frame_buffer(name: String, display: &Display) -> SimpleFrameBuffer {
        let color = self.color_textures.get(&name).unwrap().lock().unwrap();
        let depth = self.depth_textures.get(&name).unwrap().lock().unwrap();
        glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(display, color, depth).unwrap()
    }*/

    pub fn resize(&mut self, name: String, display: &Display) -> () {
        let (width, height) = display.get_framebuffer_dimensions();
        let color_texture = Texture2d::empty(display, width, height).unwrap();
        let depth_texture = DepthTexture2d::empty(display, width, height).unwrap();
        self.color_textures
            .insert(name.clone(), Arc::new(Mutex::new(color_texture)));
        self.depth_textures
            .insert(name.clone(), Arc::new(Mutex::new(depth_texture)));
    }

    pub fn resize_all(&mut self, display: &Display) -> () {
        let keys: Vec<String> = self.color_textures.keys().map(|k| k.clone()).collect();
        keys.into_iter().for_each(|key| self.resize(key, display));
    }

    pub fn get_textures(
        &self,
        name: String,
    ) -> (MutexGuard<Texture2d>, MutexGuard<DepthTexture2d>) {
        (
            self.color_textures.get(&name).unwrap().lock().unwrap(),
            self.depth_textures.get(&name).unwrap().lock().unwrap(),
        )
    }
}
