use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

fn zcurve(img: &mut Image, x_offset: i64, y_offset: i64) {
    const STEP_SIZE: i64 = 8;
    let mut sx: i64 = 0;
    let mut sy: i64 = 0;
    let mut b: u64 = 0;

    let mut prev_pos = (sx + x_offset, sy + y_offset);
    loop {
        let next = b + 1;
        sx = 0;
        if (next & 1) as i64 > 0 {
            sx += STEP_SIZE;
        }
        if next & 0b100 > 0 {
            sx += 2 * STEP_SIZE;
        }
        if next & 0b10_000 > 0 {
            sx += 4 * STEP_SIZE;
        }
        if next & 0b1_000_000 > 0 {
            sx += 8 * STEP_SIZE;
        }
        if next & 0b100_000_000 > 0 {
            sx += 16 * STEP_SIZE;
        }
        if next & 0b10_000_000_000 > 0 {
            sx += 32 * STEP_SIZE;
        }
        if next & 0b1_000_000_000_000 > 0 {
            sx += 64 * STEP_SIZE;
        }
        if next & 0b100_000_000_000_000 > 0 {
            sx += 128 * STEP_SIZE;
        }
        if next & 0b10_000_000_000_000_000 > 0 {
            sx += 256 * STEP_SIZE;
        }
        if next & 0b1_000_000_000_000_000_000 > 0 {
            sx += 512 * STEP_SIZE;
        }
        sy = 0;
        if (next & 0b10) as i64 > 0 {
            sy += STEP_SIZE;
        }
        if next & 0b1_000 > 0 {
            sy += 2 * STEP_SIZE;
        }
        if next & 0b100_000 > 0 {
            sy += 4 * STEP_SIZE;
        }
        if next & 0b10_000_000 > 0 {
            sy += 8 * STEP_SIZE;
        }
        if next & 0b1_000_000_000 > 0 {
            sy += 16 * STEP_SIZE;
        }
        if next & 0b100_000_000_000 > 0 {
            sy += 32 * STEP_SIZE;
        }
        if next & 0b10_000_000_000_000 > 0 {
            sy += 64 * STEP_SIZE;
        }
        if next & 0b1_000_000_000_000_000 > 0 {
            sy += 128 * STEP_SIZE;
        }
        if next & 0b100_000_000_000_000_000 > 0 {
            sy += 256 * STEP_SIZE;
        }
        if next & 0b10_000_000_000_000_000_000 > 0 {
            sy += 512 * STEP_SIZE;
        }
        img.plot_line_width(prev_pos, (sx + x_offset, sy + y_offset), 1.0);

        if next == 0b111_111_111_111_111_111_111_111 {
            break;
        }
        if sx as usize > img.width && sy as usize > img.height {
            break;
        }

        prev_pos = (sx + x_offset, sy + y_offset);
        b = next;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Z curve - ESC to exit",
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
    zcurve(&mut image, 0, 0);

    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
