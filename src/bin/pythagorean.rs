use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

fn pythagorean(image: &mut Image, size_a: i64, size_b: i64) {
    let width = image.width as i64;
    let height = image.height as i64;
    let times = 4 * width / (size_a + size_b);
    for i in -times..times {
        let mut x = -width + i * (size_b - size_a);
        let mut y = -height - i * (size_b + size_a);
        while y < 2 * height && x < 2 * width {
            let a = (x, y);
            let b = (x + size_a, y);
            let c = (x + size_a, y + size_a);
            let d = (x, y + size_a);
            image.plot_line_width(a, b, 0.);
            image.plot_line_width(b, c, 0.);
            image.plot_line_width(c, d, 0.);
            image.plot_line_width(d, a, 0.);
            let (cx, cy) = ((a.0 + b.0 + c.0 + d.0) / 4, (a.1 + b.1 + c.1 + d.1) / 4);
            //image.plot_circle((cx, cy), 3, 3.0);
            image.flood_fill(cx, cy);
            x += size_a;
            let a = b;
            let b = (a.0 + size_b, y);
            let c = (a.0 + size_b, y + size_b);
            let d = (a.0, y + size_b);
            image.plot_line_width(a, b, 1.);
            image.plot_line_width(b, c, 1.);
            image.plot_line_width(c, d, 1.);
            image.plot_line_width(d, a, 1.);
            y += size_b;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Pythagorean tiling - ESC to exit",
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
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    pythagorean(&mut image, 50, (50.0 * phi) as i64);

    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
