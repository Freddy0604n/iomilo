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
            .window("demo", 1000, 800)
            .position_centered()
            .resizable()
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
        let size: (u32, u32) = self.canvas.window().size();
        self.canvas.set_draw_color(BGCOLOR);
        self.canvas.clear();
        for module in self.content.iter() {
            match module.kind {
                Modules::Rectangle => {
                    self.canvas.set_draw_color(Color::RGB(module.parameters[0] as u8, module.parameters[1] as u8, module.parameters[2] as u8));
                    self.canvas.fill_rect(Rect::new(to_screen(module.position.0, size.0) as i32, to_screen(module.position.1, size.1) as i32, to_screen(module.parameters[3], size.0), to_screen(module.parameters[4], size.1))).unwrap();
                }
            }
        }
        self.canvas.present();
    }
}

pub enum Modules {
    Rectangle,  // parameters: [Red, Green, Blue, Width, Height] Width and height are in promilles of the width and height of the screen
}

pub struct Module {
    kind: Modules,
    pub position: (u32, u32), // promilles
    pub parameters: Vec<u32>
}

impl Module {
    pub fn new(kind: Modules, position: (u32, u32), parameters: Vec<u32>) -> Module {
        Module {
            kind, 
            position,
            parameters
        }
    }
}

fn to_screen(promille: u32, full: u32) -> u32 {
    (promille as f32 / 1000.0 * full as f32).round() as u32
}