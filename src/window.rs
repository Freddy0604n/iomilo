extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

const BGCOLOR: Color = Color::WHITE;

mod modules;

pub struct UIWindow {
    rectangles: Vec<modules::Rectangle>,
    windows: Vec<modules::Window>,
    clicks: Vec<modules::ClickArea>,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
}

impl UIWindow {
    pub fn empty() -> UIWindow {
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
            rectangles: Vec::new(),
            windows: Vec::new(),
            clicks: Vec::new(),
            canvas,
            event_pump,
        }
    }

    pub fn add_rectangle(
        &mut self,
        position: (i32, i32),
        size: (u32, u32),
        color: (u8, u8, u8),
        resize: bool,
    ) {
        self.rectangles
            .push(modules::Rectangle::new(position, size, color, resize));
    }

    pub fn add_window(
        &mut self,
        title: String,
        position: (i32, i32),
        size: (u32, u32),
        bg_color: (u8, u8, u8),
        bar_color: (u8, u8, u8),
    ) {
        self.windows.push(modules::Window::new(
            title, position, size, bg_color, bar_color,
        ));
    }
    pub fn key_down(&mut self, key: Keycode) -> bool {
        self.event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::from_keycode(key).unwrap())
    }

    pub fn quit_pressed(&mut self) -> bool {
        match self.event_pump.poll_event() {
            Some(e) => e.is_same_kind_as(&Event::Quit { timestamp: 0 }),
            None => false,
        }
    }

    pub fn refresh_frame(&mut self) {
        let size: (u32, u32) = self.canvas.window().size();
        self.canvas.set_draw_color(BGCOLOR);
        self.canvas.clear();
        for rectangle in &self.rectangles {
            self.canvas.set_draw_color(rectangle.color);
            if rectangle.rescale {
                self.canvas
                    .fill_rect(Rect::new(
                        to_screen(rectangle.position.0 as u32, size.0) as i32,
                        to_screen(rectangle.position.1 as u32, size.1) as i32,
                        to_screen(rectangle.size.0 as u32, size.0),
                        to_screen(rectangle.size.1 as u32, size.1),
                    ))
                    .unwrap();
            } else {
                self.canvas
                    .fill_rect(Rect::new(
                        rectangle.position.0,
                        rectangle.position.1,
                        rectangle.size.0 as u32,
                        rectangle.size.1 as u32,
                    ))
                    .unwrap()
            }
        }
        for window in &self.windows {
            self.canvas.set_draw_color(window.bar_color);
            self.canvas
                .fill_rect(Rect::new(
                    window.position.0,
                    window.position.1,
                    window.size.0,
                    10,
                ))
                .unwrap();
            self.canvas.set_draw_color(window.bg_color);
            self.canvas
                .fill_rect(Rect::new(
                    window.position.0,
                    window.position.1 + 10,
                    window.size.0,
                    window.size.1 - 10,
                ))
                .unwrap();
            for rectangle in &window.rectangles {
                self.canvas.set_draw_color(rectangle.color);
                if rectangle.rescale {
                    self.canvas
                        .fill_rect(Rect::new(
                            to_screen(rectangle.position.0 as u32, size.0) as i32,
                            to_screen(rectangle.position.1 as u32, size.1) as i32,
                            to_screen(rectangle.size.0 as u32, size.0),
                            to_screen(rectangle.size.1 as u32, size.1),
                        ))
                        .unwrap();
                } else {
                    self.canvas
                        .fill_rect(Rect::new(
                            rectangle.position.0,
                            rectangle.position.1,
                            rectangle.size.0 as u32,
                            rectangle.size.1 as u32,
                        ))
                        .unwrap()
                }
            }
        }
        self.canvas.present();
    }
}

fn to_screen(promille: u32, full: u32) -> u32 {
    // calculates the value of a promille of the entire value
    (promille as f32 / 1000.0 * full as f32).round() as u32
}
