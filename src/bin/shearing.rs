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

const WINDOW_WIDTH: usize = 150;
const WINDOW_HEIGHT: usize = 150;

fn shear_x((x_p, y_p): (i64, i64), l: f64) -> (i64, i64) {
    (x_p + (l * (y_p as f64)) as i64, y_p)
}
fn shear_y((x_p, y_p): (i64, i64), l: f64) -> (i64, i64) {
    (x_p, (l * (x_p as f64)) as i64 + y_p)
}

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

    let mut image = Image::new(DMR_WIDTH, DMR_HEIGHT, 25, 25);
    image.bytes = bits_to_bytes(DMR_BITS, DMR_WIDTH);
    image.draw_outline();

    let l = -1.047;
    let mut sheared = Image::new(DMR_WIDTH * 2, DMR_HEIGHT * 2, 25, 25);
    for x in 0..DMR_WIDTH {
        for y in 0..DMR_HEIGHT {
            if image.bytes[y * DMR_WIDTH + x] == BLACK {
                //let p = (x as i64 ,y as i64 );
                let p = shear_x((x as i64, y as i64), l);
                //let p = shear_y((x as i64 ,y as i64 ), l);
                sheared.plot(p.0 + (DMR_WIDTH / 2) as i64, p.1 + (DMR_HEIGHT / 2) as i64);
            }
        }
    }
    sheared.draw_outline();

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        sheared.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
