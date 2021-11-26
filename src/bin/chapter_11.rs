use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};
use std::f64::consts::FRAC_PI_2;

pub fn bits_to_bytes(bits: &[u8], width: usize) -> Vec<u32> {
    let mut ret = Vec::with_capacity(bits.len() * 8);
    let mut current_row_count = 0;
    for byte in bits {
        for n in 0..8 {
            if byte.rotate_right(n) & 0x01 > 0 {
                ret.push(BLACK);
            } else {
                ret.push(WHITE);
            }
            current_row_count += 1;
            if current_row_count == width {
                current_row_count = 0;
                break;
            }
        }
    }
    ret
}

include!("../dmr.xbm.rs");

const WINDOW_WIDTH: usize = 100;
const WINDOW_HEIGHT: usize = 100;

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

    let angle = 0.5; //FRAC_PI_2;

    let c = f64::cos(angle);
    let s = f64::sin(angle);

    let mut image = Image::new(DMR_WIDTH, DMR_HEIGHT, 25, 25);
    let dmr = bits_to_bytes(DMR_BITS, DMR_WIDTH);
    let center_point = ((DMR_WIDTH / 2) as i64, (DMR_HEIGHT / 2) as i64);
    for y in 0..DMR_HEIGHT {
        for x in 0..DMR_WIDTH {
            if dmr[y * DMR_WIDTH + x] == BLACK {
                let x = (x as i64 - center_point.0) as f64;
                let y = (y as i64 - center_point.1) as f64;
                let xr = x * c - y * s;
                let yr = x * s + y * c;
                image.plot(xr as i64 + center_point.0, yr as i64 + center_point.1);
            }
        }
    }
    image.draw_outline();

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
