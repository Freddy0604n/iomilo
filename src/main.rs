mod licence;
mod window;

fn main() {
    let mut window = window::UIWindow::new();
    window.content.push(window::Module::new(
        window::Modules::Rectangle,
        (0, 0),
        vec![0, 0, 0, 10, 10],
    )); // should draw a black rectangle on the screen
    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        window.refresh_frame();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("Active licence: {}", licence::check(185866116752));
}
