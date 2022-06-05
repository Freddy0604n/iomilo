extern crate sdl2;

pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

pub struct UIWindow {
    pub width: u32,
    pub height: u32,
    pub content: Vec<Module>,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump
}

impl UIWindow {
    pub fn new(width: u32, height: u32) -> UIWindow {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("demo", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();
        UIWindow {
            width,
            height,
            content: Vec::new(),
            sdl_context,
            video_subsystem,
            canvas,
            event_pump
        }
    }

    pub fn refresh_frame(&mut self) {
        for module in self.content.iter() {
            match module.kind {
                Modules::Rectangle => {
                    self.canvas.set_draw_color(Color::RGB(module.parameters[0] as u8, module.parameters[1] as u8, module.parameters[2] as u8));
                    self.canvas.fill_rect(Rect::new(module.position.0 as i32, module.position.1 as i32, module.parameters[3] as u32, module.parameters[4] as u32)).unwrap();
                }
            }
        }
    }
}
pub enum Modules {
    Rectangle,  // parameters position: [Red, Green, Blue, Widht, Height]
}

pub struct Module {
    kind: Modules,
    position: (u32, u32),
    parameters: Vec<usize>
}

impl Module {
    pub fn new(kind: Modules, position: (u32, u32), parameters: Vec<usize>) -> Module {
        Module {
            kind, 
            position,
            parameters
        }
    }
}