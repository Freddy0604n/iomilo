extern crate sdl2;

pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

const BGCOLOR: Color = Color::RGB(255, 255, 255);

pub struct UIWindow {
    pub content: Vec<Module>,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump
}

impl UIWindow {
    pub fn new() -> UIWindow {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("demo", 100, 100)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.window_mut().maximize();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();
        UIWindow {
            content: Vec::new(),
            sdl_context,
            video_subsystem,
            canvas,
            event_pump
        }
    }

    pub fn refresh_frame(&mut self) {
        self.canvas.set_draw_color(BGCOLOR);
        self.canvas.clear();
        for module in self.content.iter() {
            match module.kind {
                Modules::Rectangle => {
                    self.canvas.set_draw_color(Color::RGB(module.parameters[0] as u8, module.parameters[1] as u8, module.parameters[2] as u8));
                    self.canvas.fill_rect(Rect::new(module.position.0 as i32, module.position.1 as i32, module.parameters[3] as u32, module.parameters[4] as u32)).unwrap();
                }
            }
        }
        self.canvas.present();
    }
}
pub enum Modules {
    Rectangle,  // parameters: [Red, Green, Blue, Width, Height]
}

pub struct Module {
    kind: Modules,
    pub position: (u32, u32),
    pub parameters: Vec<usize>
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
