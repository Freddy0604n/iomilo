mod window;

fn main() {
    let mut window = window::UIWindow::empty();
    window.add_rectangle((0, 0), (10, 10), (0, 0, 0), true);
    window.add_rectangle((0, 20), (10, 10), (255, 0, 0), false);
    window.add_window(
        String::from("Hello"),
        (100, 0),
        (100, 100),
        (20, 20, 20),
        (5, 5, 5),
    );
    'running: loop {
        window.refresh_frame();
        if window.quit_pressed() {
            break 'running;
        }
    }
}
