use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};
use std::process::Command;

fn floyd(image: &mut Image) {
    let w = image.width;
    let m = [(0, 7), (w - 2, 3), (w - 1, 5), (w, 1)];
    let mut e = vec![0.0; w + 1];
    let bytes = image
        .bytes
        .iter()
        .map(|&byte| {
            let (r, g, b) = from_u32_rgb(byte);
            let g: f64 = (0.299 * (r as f64)) + (0.587_f64 * (g as f64)) + (0.114 * (b as f64));
            let pix = g / 255.0 + {
                e.push(0.);
                e.remove(0)
            };
            let col = if pix > 0.5 { 1. } else { 0. };
            let err = (pix - col) / 16.;
            for (x, y) in m.iter() {
                e[*x] += err * (*y as f64);
            }
            if col.floor() as u32 == 1 {
                WHITE
            } else {
                BLACK
            }
        })
        .collect::<Vec<u32>>();
    image.bytes = bytes;
}

fn main() {
    const INPUT_FILE: &str = "4.2.06.tiff";
    let output = Command::new("identify")
        .args([INPUT_FILE])
        .output()
        .expect("failed to execute identify");

    let re = regex::Regex::new(r"\s*(\d+)x(\d+)\s*").unwrap();
    let identify = String::from_utf8(output.stdout).unwrap();
    let matches = re.captures(&identify).unwrap();
    let width = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let height = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let mut buffer: Vec<u32> = vec![WHITE; width * height];
    let output = Command::new("magick")
        .args(["convert", INPUT_FILE, "RGB:-"])
        .output()
        .expect("failed to execute magick");

    let bytes = output.stdout;

    let bytes = bytes
        .chunks(3)
        .map(|c| from_u8_rgb(c[0], c[1], c[2]))
        .collect::<Vec<u32>>();

    let mut window = Window::new(
        "Floyd-Steinberg Dithering - ESC to exit",
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

    let mut image = Image::new(width, height, 0, 0);
    image.bytes = bytes;

    floyd(&mut image);
    image.draw_raw(&mut buffer, width);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(&buffer, width, height).unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
