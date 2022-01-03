use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;
include!("../bizcat.xbm.rs");

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

struct Bezier {
    points: Vec<Point>,
    weights: Vec<f64>,
    weight_controls: Vec<Point>,
}

const WEIGHT_CONTROL_FACTOR: i64 = 5;

impl Bezier {
    fn new(points: Vec<Point>) -> Self {
        Bezier {
            weights: vec![1.; points.len()],
            weight_controls: points
                .iter()
                .cloned()
                .map(|(x, y)| (x, y + WEIGHT_CONTROL_FACTOR))
                .collect::<Vec<Point>>(),
            points,
        }
    }

    fn get_point(&self, t: f64) -> Option<Point> {
        draw_curve_point(&self.points, &self.weights, t)
    }
}

fn draw_curve_point(points: &[Point], weights: &[f64], t: f64) -> Option<Point> {
    if points.is_empty() {
        return None;
    }
    if points.len() == 1 {
        //std::dbg!(points[0]);
        return Some(points[0]);
    }
    let mut new_points = Vec::with_capacity(points.len() - 1);
    for (c, chunk) in points.windows(2).enumerate() {
        let p1 = chunk[0];
        let p2 = chunk[1];
        let w1 = weights[c];
        let w2 = weights[c + 1];
        let x = (w1 * (1. - t) * (p1.0 as f64) + w2 * t * (p2.0 as f64)) / (w1 * (1. - t) + w2 * t);
        let y = (w1 * (1. - t) * (p1.1 as f64) + w2 * t * (p2.1 as f64)) / (w1 * (1. - t) + w2 * t);
        new_points.push((x as i64, y as i64));
    }
    assert_eq!(new_points.len(), points.len() - 1);
    draw_curve_point(&new_points, &weights[..weights.len() - 1], t)
}

fn main() {
    let mut bizcat = Image::new(BIZCAT_WIDTH, BIZCAT_HEIGHT, 0, 0);
    bizcat.bytes = bits_to_bytes(BIZCAT_BITS, BIZCAT_WIDTH);
    let bizcat = BitmapFont::new(bizcat, (8, 16), 0, 0);

    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Weighted bezier curves - ESC to exit",
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

    let mut image = Image::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0, 0);
    let curve = Bezier::new(vec![(25, 115), (135, 189), (225, 180), (250, 25)]);
    let mut curves = vec![curve];
    enum DragMode {
        Off { selected: Option<usize> },
        On { b: usize, i: usize, x: i64, y: i64 },
        Weight { b: usize, i: usize, x: i64, y: i64 },
    }
    let is_pressed =
        |p: &Point, (x, y): Point| -> bool { (p.0 - x).abs() < 4 && (p.1 - y).abs() < 4 };

    let mut state = DragMode::Off { selected: None };

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.clear();
        image.draw_grid(10);
        image.write_str(
            &bizcat,
            "Click and drag the points!",
            (WINDOW_WIDTH as i64 / 4, 3 * WINDOW_HEIGHT as i64 / 4),
        );
        image.draw(&mut buffer, BLACK, Some(WHITE), WINDOW_WIDTH);
        match &mut state {
            DragMode::Off { selected } => {
                let mut selected = *selected;
                if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    let x = x as i64;
                    let y = y as i64;
                    let mut set_default = true;
                    'curve_loop: for (b, c) in curves.iter().enumerate() {
                        for (i, p) in c.points.iter().enumerate() {
                            if is_pressed(p, (x, y)) {
                                if window.get_mouse_down(MouseButton::Left) {
                                    state = DragMode::On { b, i, x, y };
                                    selected = Some(b);
                                    window.set_cursor_style(CursorStyle::ClosedHand);
                                } else {
                                    selected = None;
                                    window.set_cursor_style(CursorStyle::OpenHand);
                                }
                                set_default = false;
                                break 'curve_loop;
                            }
                        }
                        for (i, p) in c.weight_controls.iter().enumerate() {
                            if is_pressed(p, (x, y)) {
                                if window.get_mouse_down(MouseButton::Left) {
                                    state = DragMode::Weight { b, i, x, y };
                                    selected = Some(b);
                                    window.set_cursor_style(CursorStyle::ClosedHand);
                                } else {
                                    selected = None;
                                    window.set_cursor_style(CursorStyle::OpenHand);
                                }
                                set_default = false;
                                break 'curve_loop;
                            }
                        }
                    }
                    if set_default {
                        window.set_cursor_style(CursorStyle::Arrow);
                    }
                }
                if let Some(b) = selected {
                    for (i, p) in curves[b].points.iter().enumerate() {
                        image.write_str(
                            &bizcat,
                            &format!("({}, {}) [{:.2}]", p.0, p.1, curves[b].weights[i]),
                            (p.0, p.1 + bizcat.glyph_height as i64),
                        );
                    }
                }
                for c in &mut curves {
                    for (i, p) in c.points.iter().enumerate() {
                        image.plot_square(*p, 3, 0.);
                        image.plot_line_width(*p, c.weight_controls[i], 1.0);
                        image.plot_circle(c.weight_controls[i], 3, 0.);
                    }
                }
            }
            DragMode::On {
                b,
                i,
                ref mut x,
                ref mut y,
            } => {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((xn, yn)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let weight = distance_between_two_points(
                            curves[*b].points[*i],
                            curves[*b].weight_controls[*i],
                        ) as i64;
                        *x = xn as i64;
                        *y = yn as i64;
                        curves[*b].points[*i] = (*x, *y);
                        curves[*b].weight_controls[*i] = (*x, *y + weight);
                    }
                    for (i, p) in curves[*b].points.iter().enumerate() {
                        image.write_str(
                            &bizcat,
                            &format!("({}, {}) [{:.2}]", p.0, p.1, curves[*b].weights[i]),
                            (p.0, p.1 + bizcat.glyph_height as i64),
                        );
                    }
                } else {
                    let b = *b;
                    state = DragMode::Off { selected: Some(b) };
                    window.set_cursor_style(CursorStyle::Arrow);
                }
                for c in &mut curves {
                    for (i, p) in c.points.iter().enumerate() {
                        image.plot_square(*p, 3, 0.);
                        image.plot_line_width(*p, c.weight_controls[i], 1.0);
                        image.plot_circle(c.weight_controls[i], 3, 0.);
                    }
                }
            }
            DragMode::Weight {
                b,
                i,
                ref mut x,
                ref mut y,
            } => {
                let i = *i;
                let mut weight_control_point = (*x, *y);
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((xn, yn)) = window.get_mouse_pos(MouseMode::Clamp) {
                        let new_point = (xn as i64, yn as i64);
                        let new_weight =
                            distance_between_two_points(new_point, curves[*b].points[i])
                                / (WEIGHT_CONTROL_FACTOR) as f64;
                        *x = new_point.0;
                        *y = new_point.1;
                        weight_control_point = (*x, *y);
                        curves[*b].weights[i] = new_weight;
                    }
                    for (i, p) in curves[*b].points.iter().enumerate() {
                        image.write_str(
                            &bizcat,
                            &format!("({}, {}) [{:.2}]", p.0, p.1, curves[*b].weights[i]),
                            (p.0, p.1 + bizcat.glyph_height as i64),
                        );
                    }
                } else {
                    curves[*b].weight_controls[i] = (*x, *y);
                    let b = *b;
                    state = DragMode::Off { selected: Some(b) };
                    window.set_cursor_style(CursorStyle::Arrow);
                }
                for c in &mut curves {
                    for (ii, p) in c.points.iter().enumerate() {
                        image.plot_square(*p, 3, 0.);
                        if ii == i {
                            image.plot_line_width(*p, weight_control_point, 1.0);
                            image.plot_circle(weight_control_point, 3, 0.);
                        }
                    }
                }
            }
        }
        for c in &curves {
            let mut prev_point = c.points[0];
            let mut sample = 0;
            for t in (0..100).step_by(1) {
                let t = (t as f64) / 100.;
                if let Some(new_point) = c.get_point(t) {
                    if sample == 0 {
                        image.plot_line_width(prev_point, new_point, 2.);
                        sample = 5;
                        prev_point = new_point;
                    }
                    sample -= 1;
                }
            }
            image.plot_line_width(prev_point, *c.points.last().unwrap(), 2.);
        }
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
