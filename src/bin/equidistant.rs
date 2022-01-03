use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

fn find_equidistant(point_a: Point, point_b: Point) -> (i64, i64, i64) {
    let (xa, ya) = point_a;
    let (xb, yb) = point_b;
    let midpoint = ((xa + xb) / 2, (ya + yb) / 2);

    let al = ya - yb;
    let bl = xb - xa;

    // If we had subpixel accuracy, we could do:
    //assert_eq!(al*midpoint.0+bl*midpoint.1, -cl);

    let a = bl;
    let b = -al;
    let c = (al * midpoint.1) - (bl * midpoint.0);

    (a, b, c)
}

fn plot_line(image: &mut Image, (a, b, c): (i64, i64, i64)) {
    let x = if a != 0 { -c / a } else { 0 };
    let mut prev_point = (x, 0);
    for y in 0..(WINDOW_HEIGHT as i64) {
        // ax+by+c =0 =>
        // x=(-c-by)/a

        let x = if a != 0 { -(c + b * y) / a } else { 0 };
        let new_point = (x, y);
        image.plot_line_width(prev_point, new_point, 1.0);
        prev_point = new_point;
        //image.plot(x, y);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Line equidistant from two points - ESC to exit",
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

    let mut p_m = (35, 35);
    let mut p_n = (128, 250);

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_WIDTH, 0, 0);

    enum DragMode {
        Off,
        M,
        N,
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
                    if is_pressed(&p_m, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::M;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else if is_pressed(&p_n, (x, y)) {
                        if window.get_mouse_down(MouseButton::Left) {
                            state = DragMode::N;
                            window.set_cursor_style(CursorStyle::ClosedHand);
                        } else {
                            window.set_cursor_style(CursorStyle::OpenHand);
                        }
                    } else {
                        window.set_cursor_style(CursorStyle::Arrow);
                    }
                }
            }
            DragMode::M => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_m = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
            DragMode::N => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let x = x as i64;
                        let y = y as i64;
                        p_n = (x, y);
                    }
                } else {
                    window.set_cursor_style(CursorStyle::Arrow);
                    state = DragMode::Off;
                }
            }
        }

        image.plot_circle(p_m, 3, 0.);
        image.plot_circle(p_n, 3, 0.);

        plot_line(&mut image, find_equidistant(p_m, p_n));
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
