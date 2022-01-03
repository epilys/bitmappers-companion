use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

type Line = (i64, i64, i64);
pub fn distance_line_to_point((x, y): Point, (a, b, c): Line) -> f64 {
    let d = f64::sqrt((a * a + b * b) as f64);
    if d == 0.0 {
        0.
    } else {
        (a * x + b * y + c) as f64 / d
    }
}

fn find_line(point_a: Point, point_b: Point) -> (i64, i64, i64) {
    let (xa, ya) = point_a;
    let (xb, yb) = point_b;
    let a = yb - ya;
    let b = xa - xb;
    let c = xb * ya - xa * yb;

    (a, b, c)
}

fn point_perpendicular((a, b, c): Line, p: Point) -> Point {
    let d = (a * a + b * b) as f64;
    if d == 0. {
        return (0, 0);
    }
    let cp = a * p.1 - b * p.0;
    (
        ((-a * c - b * cp) as f64 / d) as i64,
        ((a * cp - b * c) as f64 / d) as i64,
    )
}

fn cross2(v1: Point, v2: Point) -> f64 {
    v1.0 as f64 * v2.1 as f64 - v2.0 as f64 * v1.1 as f64
}

fn dot2(v1: Point, v2: Point) -> f64 {
    let d = f64::sqrt(((v1.0 * v1.0 + v1.1 * v1.1) * (v2.0 * v2.0 + v2.1 * v2.1)) as f64);
    if d != 0. {
        f64::acos((v1.0 * v2.0 + v1.1 * v2.1) as f64 / d)
    } else {
        0.
    }
}

fn draw_arc(image: &mut Image, p: Point, r: f64, startangle: f64, angle: f64) {
    const SINDT: f64 = 0.017452406; // sin 1deg
    const COSDT: f64 = 0.999847695; // cos 1deg

    let mut x = r * f64::cos(startangle);
    let mut y = r * f64::sin(startangle);

    let mut prev_pos = (p.0 + x as i64, p.1 + y as i64);

    let sr = if angle >= 0. { SINDT } else { -SINDT };

    for _ in 1..=angle.abs().floor() as i64 {
        x = x * COSDT - y * sr;
        y = x * sr + y * COSDT;

        let new_pos = (p.0 + x as i64, p.1 + y as i64);
        image.plot_line_width(prev_pos, new_pos, 0.);
        prev_pos = new_pos;
    }
}

fn round_corner(image: &mut Image, (p1, mut p2): (Point, Point), (mut p3, p4): (Point, Point)) {
    const R: f64 = 20.;

    let l1 = find_line(p1, p2);
    let l2 = find_line(p3, p4);

    let (a1, b1, _c1) = l1;
    let (a2, b2, _c2) = l2;

    let m1 = ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2);
    let m2 = ((p3.0 + p4.0) / 2, (p3.1 + p4.1) / 2);

    let d1 = distance_line_to_point(m2, l1);

    let d2 = distance_line_to_point(m1, l2);

    let mut rr = R;
    if d1 <= 0. {
        rr = -rr;
    }
    let c1p = (l1.2 as f64) - rr * f64::sqrt((l1.0 * l1.0 + l1.1 * l1.1) as f64);
    let mut rr = R;
    if d2 <= 0. {
        rr = -rr;
    }
    let c2p = (l2.2 as f64) - rr * f64::sqrt((l2.0 * l2.0 + l2.1 * l2.1) as f64);

    let d = (l1.0 * l2.1 - l2.0 * l1.1) as f64;

    let p_c = (
        ((c2p * (b1 as f64) - c1p * (b2 as f64)) / d) as i64,
        ((c1p * (a2 as f64) - c2p * (a1 as f64)) / d) as i64,
    );

    let (xa, ya) = point_perpendicular(l1, p_c);

    image.plot_circle(std::dbg!(point_perpendicular(l1, p_c)), 3, 1.);
    image.plot_circle(std::dbg!(point_perpendicular(l2, p_c)), 3, 1.);

    let (xb, yb) = point_perpendicular(l2, p_c);

    p2 = (xa, ya);

    p3 = (xb, yb);

    let v1 = (xa - p_c.0, ya - p_c.1);
    let v2 = (xb - p_c.0, yb - p_c.1);

    let pa = f64::atan2(v1.1 as f64, v1.0 as f64);

    let mut aa = dot2(v1, v2) * 57.2958;
    if std::dbg!(cross2(v1, v2)) < 0. {
        aa = -aa;
    }

    image.plot_line_width(p1, p2, 0.);
    image.plot_line_width(p3, p4, 0.);
    draw_arc(image, p_c, R, pa, aa);
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Join with round corner - ESC to exit",
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

    let mut points = [(35, 35), (128, 250), (213, 104), (40, 130)];

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
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        points[i] = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        for p in &points {
            image.plot_circle(*p, 3, 0.);
        }

        round_corner(&mut image, (points[1], points[0]), (points[3], points[2]));

        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
