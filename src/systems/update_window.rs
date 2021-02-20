use crate::resources::WindowInfo;
use specs::{System, Write};
use std::time::Instant;

pub struct UpdateWindowSystem {
    last_frame_time: Instant,
}

impl UpdateWindowSystem {
    pub fn new() -> UpdateWindowSystem {
        UpdateWindowSystem {
            last_frame_time: Instant::now(),
        }
    }
}

impl<'a> System<'a> for UpdateWindowSystem {
    type SystemData = Write<'a, WindowInfo>;

    fn run(&mut self, mut window_info: Self::SystemData) {
        let display_mutex = window_info.display.clone().unwrap();
        let display = display_mutex.lock().unwrap();

        let (width, height) = display.get_framebuffer_dimensions();

        if window_info.width != width || window_info.height != height {
            window_info.resized = true;
            window_info.width = width;
            window_info.height = height;
        } else {
            window_info.resized = false;
        }

        let current_frame_time = Instant::now();
        let delta_time = current_frame_time - self.last_frame_time;
        self.last_frame_time = current_frame_time;
        window_info.delta_time =
            delta_time.as_secs() as f32 + delta_time.subsec_nanos() as f32 * 1e-9;

        display
            .gl_window()
            .window()
            .set_title((format!("{}", 1.0f32 / window_info.delta_time)).as_str());
    }
}
