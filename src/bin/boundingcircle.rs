use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::f64::consts::{FRAC_PI_2, PI};

include!("../me.xbm.rs");

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;

pub fn distance_between_two_points(p_k: Point, p_l: Point) -> f64 {
    let (x_k, y_k) = p_k;
    let (x_l, y_l) = p_l;
    let xlk = x_l - x_k;
    let ylk = y_l - y_k;
    f64::sqrt((xlk * xlk + ylk * ylk) as f64)
}

fn image_to_points(image: &Image) -> Vec<Point> {
    let mut ret = Vec::with_capacity(image.bytes.len());
    for y in 0..(image.height as i64) {
        for x in 0..(image.width as i64) {
            if image.get(x, y) == Some(BLACK) {
                ret.push((x, y));
            }
        }
    }
    ret
}

type Circle = (Point, f64);

fn bc(image: &Image) -> Circle {
    let mut points = image_to_points(image);
    points.shuffle(&mut thread_rng());
    min_circle(&points)
}
fn min_circle(points: &[Point]) -> Circle {
    let mut points = points.to_vec();
    points.shuffle(&mut thread_rng());

    let p1 = points[0];
    let p2 = points[1];
    //The  circle  is  determined  by  two  points,  P  and  Q.  The  center  of the  circle  is
    //at  (P  +  Q)/2.0  and  the  radius  is  |(P  –  Q)/2.0|
    let d_2 = (
        (((p1.0 + p2.0) / 2), (p1.1 + p2.1) / 2),
        (distance_between_two_points(p1, p2) / 2.0),
    );

    let mut d_prev = d_2;

    for i in 2..points.len() {
        //if d_prev.1 > ME_WIDTH as _ {
        //println!("\n\n", );
        //std::dbg!(p1);
        //std::dbg!(p2);
        //panic!("i = {} {:#?}", i, d_prev);
        // }
        //image.plot_circle((d_prev.0.0+45, d_prev.0.1+45), d_prev.1 as _, 0.);
        let p_i = points[i];
        if distance_between_two_points(p_i, d_prev.0) <= (d_prev.1) {
            // then d_i = d_(i-1)
        } else {
            let new = min_circle_w_point(&points[..i], p_i);
            if distance_between_two_points(p_i, new.0) <= (new.1) {
                d_prev = new;
            }
        }
    }

    d_prev
}

fn min_circle_w_point(points: &[Point], q: Point) -> Circle {
    let mut points = points.to_vec();

    points.shuffle(&mut thread_rng());
    let p1 = points[0];
    //The  circle  is  determined  by  two  points,  P_1  and  Q.  The  center  of the  circle  is
    //at  (P_1  +  Q)/2.0  and  the  radius  is  |(P_1  –  Q)/2.0|
    let d_1 = (
        (((p1.0 + q.0) / 2), (p1.1 + q.1) / 2),
        (distance_between_two_points(p1, q) / 2.0),
    );

    let mut d_prev = d_1;

    for j in 1..points.len() {
        //image.plot_circle((d_prev.0.0+45, d_prev.0.1+45), d_prev.1 as _, 0.);
        let p_j = points[j];
        if distance_between_two_points(p_j, d_prev.0) <= (d_prev.1) {
            //d_prev = d_prev;
        } else {
            let new = min_circle_w_points(&points[..j], p_j, q);
            if distance_between_two_points(p_j, new.0) <= (new.1) {
                d_prev = new;
            }
        }
    }
    d_prev
}

fn min_circle_w_points(points: &[Point], q1: Point, q2: Point) -> Circle {
    let points = points.to_vec();

    let d_0 = (
        (((q1.0 + q2.0) / 2), (q1.1 + q2.1) / 2),
        (distance_between_two_points(q1, q2) / 2.0),
    );

    let mut d_prev = d_0;
    for k in 0..points.len() {
        //image.plot_circle((d_prev.0.0+45, d_prev.0.1+45), d_prev.1 as _, 0.);
        let p_k = points[k];
        if distance_between_two_points(p_k, d_prev.0) <= (d_prev.1) {
        } else {
            let new = min_circle_w_3_points(q1, q2, p_k);
            if distance_between_two_points(p_k, new.0) <= (new.1) {
                d_prev = new;
            }
        }
    }
    d_prev
}

fn min_circle_w_3_points(q1: Point, q2: Point, q3: Point) -> Circle {
    /*
     * From law of sines:
     *
     * a/sinα = b/sinβ = c/sinγ = 2R = D            A
     *                                              \
     * since δ=γ and sinδ=c/D                      /α\
     *                                            / | -\
     *                                          /-  |   \
     *                                      b  /    |    \  c
     *                                        /     |     -\
     *                                       /      |       \
     *                                      /       \        \
     *                                    /-         |        -\
     *                                   /γ          |        β \
     *                                  /------------|------------B
     *                                C-           α |       ---/
     *                                               |δ  ---/
     *                                               |--/
     *                                              D
     *
     *
     */

    let q12 = distance_between_two_points(q1, q2);
    let q23 = distance_between_two_points(q3, q2);
    let q13 = distance_between_two_points(q3, q1);
    let d_1 = q13 * q12;
    let d_2 = q13 * q12;
    let d_3 = q13 * q23;

    let c1 = d_2 * d_3;
    let c2 = d_3 * d_1;
    let c3 = d_1 * d_2;
    let c = c1 + c2 + c3;
    let c1 = c1 as i64;
    let c2 = c2 as i64;
    let c3 = c3 as i64;

    (
        (
            ((c2 + c3) * q1.0 + (c3 + c1) * q2.0 + (c1 + c2) * q3.0) / ((2. * c) as i64),
            ((c2 + c3) * q1.1 + (c3 + c1) * q2.1 + (c1 + c2) * q3.1) / ((2. * c) as i64),
        ),
        f64::sqrt((d_1 + d_2) * (d_2 + d_3) * (d_3 + d_1) / (4. * c)),
    )
}

fn main() {
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

    let mut full = Image::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0, 0);
    let mut image = Image::new(ME_WIDTH, ME_HEIGHT, 45, 45);
    image.bytes = bits_to_bytes(ME_BITS, ME_WIDTH);
    let (center, r) = bc(&image);
    //image.draw_outline();

    full.plot_circle((center.0 + 45, center.1 + 45), r as i64, 0.);
    image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    full.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
