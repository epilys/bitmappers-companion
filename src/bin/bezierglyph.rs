use bitmappers_companion::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window, WindowOptions};

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;
include!("../bizcat.xbm.rs");

struct Bezier {
    points: Vec<Point>,
}

impl Bezier {
    fn new(points: Vec<Point>) -> Self {
        Bezier { points }
    }

    fn get_point(&self, t: f64) -> Option<Point> {
        draw_curve_point(&self.points, t)
    }
}

fn draw_curve_point(points: &[Point], t: f64) -> Option<Point> {
    if points.is_empty() {
        return None;
    }
    if points.len() == 1 {
        //std::dbg!(points[0]);
        return Some(points[0]);
    }
    let mut new_points = Vec::with_capacity(points.len() - 1);
    for chunk in points.windows(2) {
        let p1 = chunk[0];
        let p2 = chunk[1];
        let x = (1. - t) * (p1.0 as f64) + t * (p2.0 as f64);
        let y = (1. - t) * (p1.1 as f64) + t * (p2.1 as f64);
        new_points.push((x as i64, y as i64));
    }
    assert_eq!(new_points.len(), points.len() - 1);
    draw_curve_point(&new_points, t)
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if !args.is_empty() && args.iter().any(|s| s == "--help") {
        if args.iter().any(|s| s != "--help") {
            eprintln!(
                "WARNING: Ignoring other arguments and startup because --help was specified."
            );
        }
        println!("Usage: ./bezierglyph [--svg [FILE]|--help], if FILE is not specified or is \"-\" the SVG is written in stdout.");
        return;
    }
    let svg_output = !args.is_empty() && args.iter().any(|s| s == "--svg");

    let svg_path: Option<std::path::PathBuf> = if let Some(path) = args
        .iter()
        .position(|s| s == "--svg")
        .and_then(|pos| args.get(pos + 1).filter(|p| p.as_str() != "-"))
    {
        let p = std::path::PathBuf::from(path);
        if p.exists() {
            eprintln!("{} already exists.", path);
            return;
        }
        Some(p)
    } else {
        None
    };

    let mut bizcat = Image::new(BIZCAT_WIDTH, BIZCAT_HEIGHT, 0, 0);
    bizcat.bytes = bits_to_bytes(BIZCAT_BITS, BIZCAT_WIDTH);
    let bizcat = BitmapFont::new(bizcat, (8, 16), 0, 0);

    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Bezier curves - ESC to exit",
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
    /*
    /* Construct an I glyph: */
    let mut curves = vec![
        Bezier::new(vec![(180, 75), (180, 350)]),
        Bezier::new(vec![(130, 65), (166, 60), (180, 75)]),
        Bezier::new(vec![(230, 75), (230, 350)]),
        Bezier::new(vec![(230, 75), (235, 60), (280, 65)]),
        Bezier::new(vec![(280, 50), (130, 50)]),
        Bezier::new(vec![(280, 370), (130, 370)]),
        Bezier::new(vec![(133, 360), (185, 365), (180, 350)]),
        Bezier::new(vec![(230, 350), (230, 365), (280, 360)]),
        Bezier::new(vec![(130, 65), (130, 50)]),
        Bezier::new(vec![(280, 65), (280, 50)]),
        Bezier::new(vec![(130, 360), (130, 370)]),
        Bezier::new(vec![(280, 360), (280, 370)]),
    ];
    */
    /* Construct an R glyph: */
    let mut curves = vec![
        Bezier::new(vec![(54, 72), (55, 298)]),
        Bezier::new(vec![(27, 328), (61, 333), (55, 299)]),
        Bezier::new(vec![(26, 328), (27, 338)]),
        Bezier::new(vec![(27, 339), (124, 339)]),
        Bezier::new(vec![(98, 306), (97, 209)]),
        Bezier::new(vec![(97, 301), (98, 334), (123, 330)]),
        Bezier::new(vec![(123, 330), (124, 337)]),
        Bezier::new(vec![(12, 53), (54, 55), (53, 72)]),
        Bezier::new(vec![(11, 52), (174, 53)]),
        Bezier::new(vec![(174, 55), (251, 63), (266, 124)]),
        Bezier::new(vec![(183, 192), (265, 182), (266, 127)]),
        Bezier::new(vec![(100, 180), (101, 78)]),
        Bezier::new(vec![(100, 79), (125, 78)]),
        Bezier::new(vec![(126, 79), (209, 67), (216, 120)]),
        Bezier::new(vec![(136, 177), (217, 178), (218, 122)]),
        Bezier::new(vec![(105, 176), (135, 176)]),
        Bezier::new(vec![(96, 209), (138, 209)]),
        Bezier::new(vec![(140, 210), (183, 201), (203, 243)]),
        Bezier::new(vec![(205, 245), (215, 296), (241, 327)]),
        Bezier::new(vec![(187, 192), (244, 197), (252, 237)]),
        Bezier::new(vec![(253, 241), (263, 304), (290, 317)]),
        Bezier::new(vec![(241, 327), (287, 359), (339, 301)]),
        Bezier::new(vec![(292, 317), (316, 318), (332, 294)]),
        Bezier::new(vec![(335, 295), (339, 303)]),
    ];
    enum DragMode {
        Off { selected: Option<usize> },
        On { b: usize, i: usize, x: i64, y: i64 },
    }
    let is_pressed =
        |p: &Point, (x, y): Point| -> bool { (p.0 - x).abs() < 4 && (p.1 - y).abs() < 4 };

    let mut state = DragMode::Off { selected: None };

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.clear();
        image.draw_grid(10);
        image.draw(&mut buffer, BLACK, Some(WHITE), WINDOW_WIDTH);
        if window.is_key_down(Key::Key3) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let x = x as i64;
                let y = y as i64;
                curves.push(Bezier::new(vec![(x, x), (x + 50, y), (x + 150, y)]));
            }
        }
        if window.is_key_down(Key::Key2) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let x = x as i64;
                let y = y as i64;
                curves.push(Bezier::new(vec![(x, x), (x + 50, y)]));
            }
        }
        if window.is_key_down(Key::Delete) {
            match &mut state {
                DragMode::Off { ref mut selected } if selected.is_some() => {
                    curves.remove(selected.unwrap());
                    *selected = None;
                }
                DragMode::On { ref mut b, .. } => {
                    curves.remove(*b);
                    state = DragMode::Off { selected: None };
                }
                _ => {}
            }
        }
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
                    }
                    if set_default {
                        window.set_cursor_style(CursorStyle::Arrow);
                    }
                }
                if let Some(b) = selected {
                    for p in &curves[b].points {
                        image.write_str(
                            &bizcat,
                            &format!("({}, {})", p.0, p.1),
                            (p.0, p.1 + bizcat.glyph_height as i64),
                        );
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
                        *x = xn as i64;
                        *y = yn as i64;
                        curves[*b].points[*i] = (*x, *y);
                    }
                    for p in &curves[*b].points {
                        image.write_str(
                            &bizcat,
                            &format!("({}, {})", p.0, p.1),
                            (p.0, p.1 + bizcat.glyph_height as i64),
                        );
                    }
                } else {
                    let b = *b;
                    state = DragMode::Off { selected: Some(b) };
                    window.set_cursor_style(CursorStyle::Arrow);
                }
            }
        }
        for c in &curves {
            for p in &c.points {
                image.plot_square(*p, 3, 0.);
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

    println!("Final geometry:");
    for c in &curves {
        println!("{:?}", c.points);
    }

    if svg_output {
        let mut output = vec![];
        output.push(format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            WINDOW_WIDTH, WINDOW_HEIGHT
        ));
        for c in &curves {
            match c.points.len() {
                3 => {
                    output.push(format!(
                        r#"  <path d="M {} {} Q {} {} {} {}" stroke="black" fill="transparent"/>"#,
                        c.points[0].0,
                        c.points[0].1,
                        c.points[1].0,
                        c.points[1].1,
                        c.points[2].0,
                        c.points[2].1
                    ));
                }
                2 => {
                    output.push(format!(
                        r#"  <path d="M {} {} L {} {}" stroke="black" fill="transparent"/>"#,
                        c.points[0].0, c.points[0].1, c.points[1].0, c.points[1].1
                    ));
                }
                _ => {}
            }
        }

        output.push("</svg>".to_string());
        match svg_path {
            Some(path) => {
                use std::fs::File;
                use std::io::prelude::*;
                let mut file = match File::create(&path) {
                    Err(err) => panic!("\nCouldn't create {}: {}.", path.display(), err),
                    Ok(file) => file,
                };

                match file.write_all(output.join("\n").as_bytes()) {
                    Err(err) => panic!("\nCouldn't write to {}: {}.", path.display(), err),
                    Ok(_) => println!("\nSVG output saved at {}.", path.display()),
                }
            }
            None => {
                println!("\nSVG output:");
                for o in &output {
                    println!("{}", o);
                }
            }
        }
    }
}
