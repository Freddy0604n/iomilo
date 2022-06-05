mod window;

fn main() {
    let mut window = window::UIWindow::new(1920, 1020);
    window.content.push(window::Module::new(window::Modules::Rectangle, (0, 0), vec![0, 0, 0, 100, 100])); // should draw a black rectangle on the screen
    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        window.refresh_frame();
        window.canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
