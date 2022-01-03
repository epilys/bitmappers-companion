use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            title: true,
            //borderless: true,
            //resize: false,
            //transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut image = Image::new(250, 250, 50, 50);
    image.draw_outline();

    /* beam A */
    let b0 = (40, 23);
    let b1 = (95, 96);
    image.plot_line_width((1, 50), b0, 1.0);
    image.plot_line_width(b0, b1, 1.0);
    image.plot_line_width((1, 160), b1, 1.0);

    /* beam B */
    let b2 = (98, 23);
    let b3 = (60, 96);
    image.plot_line_width((150, 50), b2, 1.0);
    image.plot_line_width(b2, b3, 1.0);
    image.plot_line_width((150, 148), b3, 1.0);

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
