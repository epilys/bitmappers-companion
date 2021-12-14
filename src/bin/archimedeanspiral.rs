use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;

pub fn arch(image: &mut Image, center: Point) {
    let a = 1.0_f64;
    let b = 9.0_f64;

    // max_angle = number of spirals * 2pi.
    let max_angle = 5.0_f64 * 2.0_f64 * std::f64::consts::PI;

    let mut theta = 0.0_f64;
    let (dx, dy) = center;
    let mut prev_point = center;
    while theta < max_angle {
        theta = theta + 0.002_f64;

        let r = a + b * theta;
        let x = (r * theta.cos()) as i64 + dx;
        let y = (r * theta.sin()) as i64 + dy;
        image.plot_line_width(prev_point, (x, y), 1.0);
        prev_point = (x, y);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Archimedean spiral - ESC to exit",
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

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    let center = (image.width as i64 / 2, image.height as i64 / 2);
    arch(&mut image, center);
    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
