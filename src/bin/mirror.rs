use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 600;

fn find_mirror(point: Point, l: Line) -> Point {
    let (x, y) = point;
    let (a, b, c) = l;
    let (a, b, c) = (a as f64, b as f64, c as f64);

    let b2a = (b * b) / a;
    let mx = (b2a * x as f64 - c - b * y as f64) / (a + b2a);
    let my = (-a * mx - c) / b;
    let (mx, my) = (mx as i64, my as i64);

    (2 * mx - x, 2 * my - y)
}

fn plot_line(image: &mut Image, (a, b, c): (i64, i64, i64)) {
    let x = if a != 0 { -c / a } else { 0 };
    let mut prev_point = (x, 0);
    for y in 0..(WINDOW_HEIGHT as i64) {
        // ax+by+c =0 =>
        // x=(-c-by)/a

        let x = if a != 0 { -(c + b * y) / a } else { 0 };
        let new_point = (x, y);
        image.plot_line_width(prev_point, new_point, 1.0);
        prev_point = new_point;
        //image.plot(x, y);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Reflection of point on line - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            title: true,
            //borderless: true,
            resize: true,
            //transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut p_m = (35, 35);
    let p_n = (128, 250);
    let l = (-57, 174, -19470);

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    enum DragMode {
        Off,
        M,
    }
    let mut state = DragMode::Off;
    let is_pressed =
        |p: &Point, (x, y): Point| -> bool { (p.0 - x).abs() < 4 && (p.1 - y).abs() < 4 };
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.clear();
        image.draw(&mut buffer, BLACK, Some(WHITE), WINDOW_WIDTH);
        match &mut state {
            DragMode::Off => {
                if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    let x = x as i64;
                    let y = y as i64;
                    if is_pressed(&p_m, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::M;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else {
                        window.set_cursor_style(CursorStyle::Arrow);
                    }
                }
            }
            DragMode::M => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_m = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        image.plot_circle(p_m, 3, 0.);
        image.plot_circle(find_mirror(p_m, l), 3, 0.);

        plot_line(&mut image, l);
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
