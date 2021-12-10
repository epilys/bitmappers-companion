use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

fn find_intersection((a1, b1, c1): (i64, i64, i64), (a2, b2, c2): (i64, i64, i64)) -> Point {
    let denom = a1 * b2 - a2 * b1;

    if denom == 0 {
        return (0, 0);
    }

    ((b1 * c2 - b2 * c1) / denom, (a2 * c1 - a1 * c2) / denom)
}

fn find_line(point_a: Point, point_b: Point) -> (i64, i64, i64) {
    let (xa, ya) = point_a;
    let (xb, yb) = point_b;
    let a = ya - yb;
    let b = xb - xa;
    let c = xa * yb - xb * ya;

    (a, b, c)
}

fn plot_line(image: &mut Image, (a, b, c): (i64, i64, i64)) {
    let x = if a != 0 { -1 * (c) / a } else { 0 };
    let mut prev_point = (x, 0);
    for y in 0..(WINDOW_HEIGHT as i64) {
        // ax+by+c =0 =>
        // x=(-c-by)/a

        let x = if a != 0 { -1 * (c + b * y) / a } else { 0 };
        let new_point = (x, y);
        image.plot_line_width(prev_point, new_point, 1.0);
        prev_point = new_point;
        //image.plot(x, y);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Point intersection of two lines - ESC to exit",
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

        let l1 = find_line(points[0], points[1]);
        let l2 = find_line(points[2], points[3]);
        plot_line(&mut image, l1);
        plot_line(&mut image, l2);

        let p = find_intersection(l1, l2);
        image.plot_circle(p, 5, 3.);

        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
