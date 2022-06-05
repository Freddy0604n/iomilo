extern crate sdl2;

pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub content: Vec<Module>,
}

impl Window {
    pub fn start(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("demo", self.width, self.height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
    }
}
pub enum Modules {
    Rectangle,
}

pub struct Module {
    kind: Modules,
}
