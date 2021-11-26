use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

include!("../dmr.xbm.rs");
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

    let mut original = Image::new(DMR_WIDTH, DMR_HEIGHT, 25, 25);
    original.bytes = bits_to_bytes(DMR_BITS, DMR_WIDTH);
    original.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

    let mut scaled = Image::new(DMR_WIDTH * 5, DMR_HEIGHT * 5, 100, 100);
    let mut sx: i64; //source
    let mut sy: i64; //source
    let mut dx: i64; //destination
    let mut dy: i64 = 0; //destination

    let og_height = original.height as i64;
    let og_width = original.width as i64;
    let scaled_height = scaled.height as i64;
    let scaled_width = scaled.width as i64;

    while dy < scaled_height {
        sy = (dy * og_height) / scaled_height;
        dx = 0;
        while dx < scaled_width {
            sx = (dx * og_width) / scaled_width;
            if original.get(sx, sy) == Some(BLACK) {
                scaled.plot(dx, dy);
            }
            dx += 1;
        }
        dy += 1;
    }
    scaled.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
