use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_decimal::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

fn truchet(image: &mut Image, size: i64) {
    let mut x = 0;
    let mut y = 0;
    #[repr(u8)]
    enum Tile {
        A = 0,
        B,
        C,
        D,
    }
    let tiles = [Tile::A, Tile::B, Tile::C, Tile::D];
    let width = image.width as i64;
    let height = image.height as i64;
    let mut rng = thread_rng();
    while y < height {
        while x < width {
            let t = tiles.choose(&mut rng).unwrap();
            let (a, b, c) = match t {
                Tile::A => {
                    let a = (x, y + size);
                    let b = (x + size, y + size);
                    let c = (x + size, y);
                    (a, b, c)
                }
                Tile::B => {
                    let a = (x, y);
                    let b = (x, y + size);
                    let c = (x + size, y + size);
                    (a, b, c)
                }
                Tile::C => {
                    let a = (x, y);
                    let b = (x + size, y);
                    let c = (x, y + size);
                    (a, b, c)
                }
                Tile::D => {
                    let a = (x, y);
                    let b = (x + size, y);
                    let c = (x + size, y + size);
                    (a, b, c)
                }
            };
            image.plot_line_width(a, b, 1.);
            image.plot_line_width(b, c, 1.);
            image.plot_line_width(c, a, 1.);
            let c = ((a.0 + b.0 + c.0) / 3, (a.1 + b.1 + c.1) / 3);
            image.flood_fill(c.0, c.1);
            x += size;
        }
        x = 0;
        y += size;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Truchet tiling - ESC to exit",
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
    truchet(&mut image, 55);

    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
