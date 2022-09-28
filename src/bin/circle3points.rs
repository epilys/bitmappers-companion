use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

fn perp_bisector((x_a, y_a): Point, (x_b, y_b): Point) -> (i64, i64, i64) {
    let m_a = if x_b != y_b { (y_a - y_b) as f64 } else { -1.0 };
    let m_b = (x_b - x_a) as f64;

    let (x_m, y_m) = ((x_b + x_a) as f64 / 2.0, (y_b + y_a) as f64 / 2.0);
    // slope form y=mx+b
    // m_og = (y_m - y_n / x_m - x_n)
    // m_og * m = -1 => m = (x_n - x_m) / (y_m - y_n) = m_b / m_a
    //
    // y = mx+b => y_m = m*x_m + b => b = y_m - m * x_m
    //
    // slope form y=mx+b -> implicit form αx+βy=γ
    // y = m*x + y_m - m* x_m
    (
        m_b as i64,
        -m_a as i64,
        (((y_m * m_a) - (m_b * x_m)) as i64),
    )
}

fn find_intersection((a1, b1, c1): (i64, i64, i64), (a2, b2, c2): (i64, i64, i64)) -> Point {
    let denom = a1 * b2 - a2 * b1;

    if denom == 0 {
        return (0, 0);
    }

    ((b1 * c2 - b2 * c1) / denom, (a2 * c1 - a1 * c2) / denom)
}

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Circle from 3 given points - ESC to exit",
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

    let mut p_a = (35, 35);
    let mut p_b = (128, 250);
    let mut p_c = (179, 220);

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    enum DragMode {
        Off,
        A,
        B,
        C,
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
                    if is_pressed(&p_a, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::A;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else if is_pressed(&p_b, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::B;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else if is_pressed(&p_c, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::C;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else {
                        window.set_cursor_style(CursorStyle::Arrow);
                    }
                }
            }
            DragMode::A => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_a = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
            DragMode::B => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_b = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
            DragMode::C => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_c = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        image.plot_circle(p_a, 3, 0.);
        image.plot_circle(p_b, 3, 0.);
        image.plot_circle(p_c, 3, 0.);

        let perp1 = perp_bisector(p_a, p_b);
        let perp2 = perp_bisector(p_b, p_c);

        let centre = find_intersection(perp1, perp2);
        let radius = distance_between_two_points(centre, p_a);

        image.plot_line_width(p_a, p_b, 2.5);
        image.plot_line_width(p_b, p_c, 2.5);
        image.plot_line_width(p_c, p_a, 2.5);
        image.plot_circle(centre, radius as i64, 2.0);
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
