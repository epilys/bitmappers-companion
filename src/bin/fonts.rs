use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;

include!("../bizcat.xbm.rs");
//include!("../unifont.xbm.rs");

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Bitmap fonts - ESC to exit",
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

    let mut image = Image::new(300, 100, 15, 15);

    image.draw_outline();

    let mut bizcat = Image::new(BIZCAT_WIDTH, BIZCAT_HEIGHT, 0, 0);
    bizcat.bytes = bits_to_bytes(BIZCAT_BITS, BIZCAT_WIDTH);
    //bizcat.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    let bizcat = BitmapFont::new(bizcat, (8, 16), 0, 0);
    //let mut glyph = bizcat.glyph('(').unwrap();
    //glyph.x_offset = 150;
    //glyph.y_offset = 55;
    //glyph.draw_outline();
    //glyph.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    //image.write_str(&bizcat, "hello world! this is the bizcat font.", (0,0));
    //let mut unifont = Image::new(UNIFONT_WIDTH, UNIFONT_HEIGHT, 0, 0);
    //unifont.bytes = bits_to_bytes(UNIFONT_BITS, UNIFONT_WIDTH);
    //let unifont = BitmapFont::new(unifont, (16, 16), 0, 0);
    image.write_str(&bizcat, "hello world!", (0, 0));

    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
