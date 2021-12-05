use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 800;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RuleTile {
    Blank = 0,
    Filled = 1,
    Smooth = 2,
    Ignore = 3,
}

use RuleTile::*;

type Rule<const D: usize> = [[RuleTile; D]; D];

fn matches<const D: usize>(_self: &Rule<D>, buffer: &Image, (x, y): (i64, i64)) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut ret = true;
    for tilerow in _self {
        for tile in tilerow {
            if *tile as u8 != Ignore as u8 && buffer.get(x + col, y + row).is_none() {
                return false;
            }
            match tile {
                Blank if buffer.get(x + col, y + row) != Some(WHITE) => {
                    return false;
                }
                Filled if buffer.get(x + col, y + row) != Some(BLACK) => {
                    return false;
                }
                Smooth if buffer.get(x + col, y + row) == Some(WHITE) => {}
                Smooth => {
                    return false;
                }
                Blank | Filled | Ignore => {}
            }
            col += 1;
        }
        col = 0;
        row += 1;
    }
    true
}

fn smooth<const D: usize>(_self: &Rule<D>, rule_idx: usize, orig: &Image, buffer: &mut Image) {
    let scale_down = (buffer.height / orig.height) as i64;
    let og_height = orig.height as i64;
    let og_width = orig.width as i64;
    let scaled_height = buffer.height as i64;
    let scaled_width = buffer.width as i64;

    let is_reflection = rule_idx < 4;
    let which_triangle = if is_reflection {
        (rule_idx + 1) % 5
    } else {
        (rule_idx - 3) % 5
    };
    //println!("scale_down = {}", scale_down);
    let mut y = 0;
    let mut x = 0;

    std::dbg!(rule_idx);
    std::dbg!(scaled_height);
    std::dbg!(scaled_width);
    std::dbg!(which_triangle);
    std::dbg!(is_reflection);
    while y < og_height {
        while x < og_width {
            if matches(&_self, &orig, (x, y)) {
                let (dx, dy) = (scale_down * x, scale_down * y);
                let region_width = scale_down * (D as i64);
                std::dbg!(region_width);
                let bw = scale_down;
                std::dbg!(bw);
                let iw = region_width - 2 * bw;
                std::dbg!(iw);
                let (mut a, mut b, mut c) = match which_triangle {
                    1 => {
                        let a = (dx + bw, dy + iw);
                        let b = (a.0, a.1 + iw);
                        let c = (b.0 + iw, b.1);
                        (a, b, c)
                    }
                    2 => {
                        let a = (dx + bw, dy + bw);
                        let b = (a.0, a.1 + bw);
                        let c = (b.0 + iw, b.1);
                        (a, b, c)
                    }
                    3 => {
                        let a = (dx + bw, dy + bw);
                        let b = (a.0 + iw, a.1);
                        let c = (b.0, b.1 + bw);
                        (a, b, c)
                    }
                    4 => {
                        let a = (dx + bw, dy + bw);
                        let b = (a.0, a.1 + bw);
                        let c = (a.0 + iw, a.1);
                        (c, b, a)
                    }
                    other => panic!("which trongle {}", other),
                };
                std::dbg!((a, b, c));

                let d_x_ca = a.0 - c.0;
                let d_y_ca = a.1 - c.1;
                std::dbg!((d_x_ca, d_y_ca));

                a.0 -= 1;
                a.1 -= 1;
                b.0 -= 1;
                b.1 -= 1;
                c.0 -= 1;
                c.1 -= 1;

                if !(which_triangle == 2 && !is_reflection) {
                    buffer.plot_line_width(a, b, 1.);
                    buffer.plot_line_width(b, c, 1.);
                    buffer.plot_line_width(c, a, 1.);
                    let centroid = ((a.0 + b.0 + c.0) / 3, (a.1 + b.1 + c.1) / 3);
                    buffer.fill_triangle(a, b, c);
                }
                //for row in 0..scale_down {
                //    for col in 0..scale_down {
                //        buffer.plot(x+col, y+row);
                //    }
                //}

                //for offset in 0..scale_down {
                //    println!("smoothing {}, {}, {:?}", x+col, y+row, buffer.get(x+col, y+row));
                //    buffer.plot(x+offset*col+2, y+offset*row+2);
                //}
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
}

const RULE_1: Rule<3> = [
    [Ignore, Blank, Blank],
    [Blank, Smooth, Filled],
    [Ignore, Filled, Ignore],
];
const RULE_2: Rule<4> = [
    [Ignore, Ignore, Blank, Blank],
    [Blank, Smooth, Smooth, Filled],
    [Ignore, Filled, Filled, Ignore],
    [Ignore, Ignore, Ignore, Ignore],
];

const RULE_3: Rule<5> = [
    [Ignore, Ignore, Ignore, Blank, Blank],
    [Blank, Smooth, Smooth, Smooth, Filled],
    [Ignore, Filled, Filled, Filled, Ignore],
    [Ignore, Ignore, Ignore, Ignore, Ignore],
    [Ignore, Ignore, Ignore, Ignore, Ignore],
];

fn gen_ruleset<const D: usize>(rule: Rule<D>) -> [Rule<D>; 8] {
    let mut ret = [rule; 8];
    {
        let e = 1;
        let eo = e - 1;
        let mut row = 0;
        let mut col = 0;
        for y in 0..D {
            for x in (0..D).rev() {
                ret[e][row][col] = ret[eo][y][x];
                col += 1;
            }
            col = 0;
            row += 1;
        }
    }
    for e in 2..4 {
        let eo = e - 2;
        let mut row = 0;
        let mut col = 0;
        for y in (0..D).rev() {
            for x in 0..D {
                ret[e][row][col] = ret[eo][y][x];
                col += 1;
            }
            col = 0;
            row += 1;
        }
    }
    for e in 4..8 {
        ret[e] = if e == 4 { ret[0] } else { ret[e - 1] };
        // transpose
        for i in 0..D {
            for j in (i + 1)..D {
                let a = &mut ret[e][i][j] as *mut _;
                let b = &mut ret[e][j][i] as *mut _;
                unsafe { std::ptr::swap(a, b) };
            }
        }
        // reverse rows
        for i in 0..D {
            let mut low = 0;
            let mut high = D - 1;
            while low < high {
                let a = &mut ret[e][i][low] as *mut _;
                let b = &mut ret[e][i][high] as *mut _;
                unsafe { std::ptr::swap(a, b) };
                low += 1;
                high -= 1;
            }
        }
    }
    ret
}

fn main() {
    let rule_1_set: [Rule<3>; 8] = gen_ruleset(RULE_1);
    let rule_2_set: [Rule<4>; 8] = gen_ruleset(RULE_2);
    let rule_3_set: [Rule<5>; 8] = gen_ruleset(RULE_3);
    for rul in rule_2_set {
        for row in rul {
            for el in row {
                print!("{:?}", el as u8);
            }
            println!("");
        }
        println!("");
    }
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
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

    let mut original = Image::from_xbm("./testimages/xface.xbm", 100, 100).unwrap();
    original.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

    let mut scaled = Image::new(original.width * 8, original.width * 8, 0, 100);
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
    scaled.x_offset += 380;

    for (i, rul) in rule_1_set.iter().enumerate() {
        smooth(&rul, i, &original, &mut scaled);
    }
    for (i, rul) in rule_2_set.iter().enumerate() {
        //smooth(&rul, i, &original, &mut scaled);
    }
    for (i, rul) in rule_3_set.iter().enumerate() {
        //smooth(&rul, i, &original, &mut scaled);
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
