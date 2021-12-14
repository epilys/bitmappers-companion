use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;
include!("../bizcat.xbm.rs");

fn parellarc(image: &mut Image, p: Point, q: Point, k: Point, t: f64) {
    if t <= 0. || t > 1. {
        return;
    }

    let mut v = ((k.0 - q.0) as f64, (k.1 - q.1) as f64);

    let mut u = ((k.0 - p.0) as f64, (k.1 - p.1) as f64);

    let j = ((p.0 as f64 - v.0 + 0.5), (p.1 as f64 - v.1 + 0.5));

    u = (
        (u.0 * f64::sqrt(1. - t * t * 0.25) - v.0 * t * 0.5),
        (u.1 * f64::sqrt(1. - t * t * 0.25) - v.1 * t * 0.5),
    );

    let n = (std::f64::consts::FRAC_PI_2 / t).floor() as u64;

    let mut prev_pos = p;
    for _ in 0..n {
        let x = (v.0 + j.0).round() as i64;
        let y = (v.1 + j.1).round() as i64;
        let new_point = (x, y);
        image.plot_line_width(prev_pos, new_point, 1.);
        prev_pos = new_point;

        u.0 -= v.0 * t;
        v.0 += u.0 * t;
        u.1 -= v.1 * t;
        v.1 += u.1 * t;
    }
}

fn main() {
    let mut bizcat = Image::new(BIZCAT_WIDTH, BIZCAT_HEIGHT, 0, 0);
    bizcat.bytes = bits_to_bytes(BIZCAT_BITS, BIZCAT_WIDTH);
    let bizcat = BitmapFont::new(bizcat, (8, 16), 0, 0);

    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Parametric elliptical arc - ESC to exit",
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

    let mut points = [(31, 146), (183, 258), (257, 19)];

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    enum DragMode {
        Off,
        Drag { i: usize },
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
                    for (i, p) in points.iter().enumerate() {
                        if is_pressed(p, (x, y)) {
                            if window.get_mouse_down(MouseButton::Left) {
                                state = DragMode::Drag { i };
                                window.set_cursor_style(CursorStyle::ClosedHand);
                            } else {
                                window.set_cursor_style(CursorStyle::OpenHand);
                            }
                        } else {
                            window.set_cursor_style(CursorStyle::Arrow);
                        }
                    }
                }
            }
            DragMode::Drag { i } => {
                let i = *i;
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        points[i] = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        for (p, label) in points.iter().zip(["P", "Q", "K"].iter()) {
            image.plot_circle(*p, 3, 0.);
            image.write_str(&bizcat, label, *p);
        }

        image.plot_line_width(points[0], points[1], 0.);
        image.plot_line_width(points[2], points[1], 0.);
        image.plot_line_width(points[2], points[0], 0.);

        parellarc(&mut image, points[0], points[1], points[2], 0.01);

        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
