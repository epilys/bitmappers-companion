use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

fn atkinson(image: &mut Image) {
    let w = image.width;
    let mut e = vec![0.0; 2 * w];
    let m = [0, 1, w - 2, w - 1, w, 2 * w - 1];
    for byte in image.bytes.iter_mut() {
        let (r, g, b) = from_u32_rgb(*byte);
        let g: f64 = (0.299 * (r as f64)) + (0.587_f64 * (g as f64)) + (0.114 * (b as f64));
        let pix = g / 255.0 + {
            e.push(0.);
            e.remove(0)
        };
        let col = if pix > 0.5 { 1. } else { 0. };
        let err = (pix - col) / 8.;
        for m in m.iter() {
            e[*m] += err;
        }
        *byte = if col.floor() as u32 == 1 {
            WHITE
        } else {
            BLACK
        };
    }
}

fn main() {
    const INPUT_FILE: &str = "./figures/peppers.png";
    let mut image = Image::magick_open(INPUT_FILE, 0, 0).unwrap();
    let width = image.width;
    let height = image.height;
    let mut buffer: Vec<u32> = vec![WHITE; width * height];

    let mut window = Window::new(
        "Atkinson Dithering - ESC to exit",
        width,
        height,
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

    atkinson(&mut image);
    image.draw_raw(&mut buffer, width);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(&buffer, width, height).unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
