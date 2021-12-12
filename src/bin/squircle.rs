use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;
include!("../bizcat.xbm.rs");

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

pub fn plot_squircle(
    image: &mut Image,
    (xm, ym): (i64, i64),
    width: i64,
    height: i64,
    n: i32,
    _wd: f64,
) {
    let r = width / 2;
    let w = width / 2;
    let h = height / 2;

    let mut prev_pos = (xm - w, xm - h);

    for i in 0..(2 * r + 1) {
        let x: i64 = (i - r) + w;
        let y: i64 = ((r as f64).powi(n) - (i as f64 - r as f64).abs().powi(n)).powf(1. / n as f64)
            as i64
            + h;
        if i != 0 {
            image.plot_line_width(prev_pos, (xm - x as i64, ym - y), _wd);
        }
        prev_pos = (xm - x as i64, ym - y);
    }
    for i in (2 * r)..(4 * r + 1) {
        let x: i64 = (3 * r - i) + w;
        let y = -1
            * (((r as f64).powi(n) - ((3 * r - i) as f64).abs().powi(n)).powf(1. / n as f64))
                as i64
            + h;
        image.plot_line_width(prev_pos, (xm - x as i64, ym - y), _wd);
        prev_pos = (xm - x as i64, ym - y);
    }
}

fn main() {
    let mut bizcat = Image::new(BIZCAT_WIDTH, BIZCAT_HEIGHT, 0, 0);
    bizcat.bytes = bits_to_bytes(BIZCAT_BITS, BIZCAT_WIDTH);
    let bizcat = BitmapFont::new(bizcat, (8, 16), 0, 0);

    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Squircle - ESC to exit",
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

    let mut points = [(20, 10), (210, 10), (20, 270), (140, 270)];

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    enum DragMode {
        Off,
        Drag { i: usize },
    }
    let mut state = DragMode::Off;
    let is_pressed =
        |p: &Point, (x, y): Point| -> bool { (p.0 - x).abs() < 4 && (p.1 - y).abs() < 4 };

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.clear();
        image.draw(&mut buffer, BLACK, Some(WHITE), WINDOW_WIDTH);
        match &mut state {
            DragMode::Off => {
                if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    let x = x as i64;
                    let y = y as i64;
                    for (i, p) in points.iter().enumerate() {
                        if is_pressed(p, (x, y)) {
                            if window.get_mouse_down(MouseButton::Left) {
                                state = DragMode::Drag { i };
                                window.set_cursor_style(CursorStyle::ClosedHand);
                            } else {
                                window.set_cursor_style(CursorStyle::OpenHand);
                            }
                        } else {
                            window.set_cursor_style(CursorStyle::Arrow);
                        }
                    }
                }
            }
            DragMode::Drag { i } => {
                let i = *i;
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, _)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = points[i].1;

                        let mut new = (x, y);
                        if i == 0 {
                            new = points[i];
                        }
                        if i == 1 {
                            new.0 = std::cmp::max(points[0].0 + 5, new.0);
                        }
                        if i == 2 {
                            new = points[i];
                        }
                        if i == 3 {
                            new.0 = std::cmp::max(points[2].0 + 5, new.0);
                        }
                        points[i] = new;
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        image.plot_circle(points[0], 2, 0.);
        image.plot_square(points[1], 3, 0.);
        image.plot_line_width(points[0], points[1], 0.);

        image.plot_circle(points[2], 2, 0.);
        image.plot_triangle(points[3], 3, 0.);
        image.plot_line_width(points[2], points[3], 0.);

        const N_STEP_SIZE: f64 = (WINDOW_WIDTH / 10) as f64;
        let n =
            (distance_between_two_points(points[2], points[3]) / N_STEP_SIZE).clamp(1., 20.) as i32;
        let width = distance_between_two_points(points[0], points[1]) as i64;
        plot_squircle(&mut image, (250, 250), width, width, n, 1.);

        //let degrees = 57.2958 * find_angle(l1, l2);
        let width_s = format!("width={}", width);
        image.write_str(
            &bizcat,
            &width_s,
            ((points[0].0 + points[1].0) / 2 - width_s.len() as i64, 20),
        );
        image.write_str(
            &bizcat,
            &format!("n={}", n),
            (WINDOW_WIDTH as i64 / 2, 7 * WINDOW_HEIGHT as i64 / 8 - 10),
        );

        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
