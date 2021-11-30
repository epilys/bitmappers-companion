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
        //img.plot_circle(d_prev.0, d_prev.1, 0.);
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
        //img.plot_circle(d_prev.0, d_prev.1, 0.);
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
    let mut points = points.to_vec();

    let d_0 = (
        (((q1.0 + q2.0) / 2), (q1.1 + q2.1) / 2),
        (distance_between_two_points(q1, q2) / 2.0),
    );

    let mut d_prev = d_0;
    for k in 0..points.len() {
        //img.plot_circle(d_prev.0, d_prev.1, 0.);
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
    //std::dbg!(q1);
    //std::dbg!(q2);
    //std::dbg!(q3);
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

    let (ax, ay) = (q1.0 as f64, q1.1 as f64);
    let (bx, by) = (q2.0 as f64, q2.1 as f64);
    let (cx, cy) = (q3.0 as f64, q3.1 as f64);

    let mut d = 2. * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
    if d == 0.0 {
        d = std::cmp::max(
            std::cmp::max(
                distance_between_two_points(q1, q2) as i64,
                distance_between_two_points(q2, q3) as i64,
            ),
            distance_between_two_points(q1, q3) as i64,
        ) as f64
            / 2.;
    }
    let ux = ((ax * ax + ay * ay) * (by - cy)
        + (bx * bx + by * by) * (cy - ay)
        + (cx * cx + cy * cy) * (ay - by))
        / d;
    let uy = ((ax * ax + ay * ay) * (cx - bx)
        + (bx * bx + by * by) * (ax - cx)
        + (cx * cx + cy * cy) * (bx - ax))
        / d;
    let mut center = (ux as i64, uy as i64);

    if center.0 < 0 {
        center.0 = 0;
    }
    if center.1 < 0 {
        center.1 = 0;
    }
    let d = distance_between_two_points(center, q1);
    //img.plot_circle(center, d, 1.);
    (center, d)
}

/*
fn bounding_circle(image: &mut Image) {
    loop {
        // First,  determine  a  point  P  with  the smallest  Py.
        let mut P = (0, 0);

        let mut found = false;
        'l: for y in 0..(image.height as i64) {
            for x in 0..(image.width as i64) {
                if image.get(x, y) == Some(BLACK) {
                    P = (x,y);
                    found = true;
                    break 'l;
                }
            }
        }
        if !found {
            return;
        }
        //Then  find  a  point  Q  such  that  the  angle  of  the  line  segment PQ  with  the  x  axis  is  minimal
        let mut Q = (0,0);
        let mut min_Q = 2.*PI;
        for y in 0..(image.height as i64) {
            for x in 0..(image.width as i64) {
                if Q == P {
                    continue;
                }
                if image.get(x, y) == Some(BLACK) {
                    let q = (x,y);
                    let k = (P.0 - x).abs() as f64;
                    let a = f64::acos(f64::cos(k/distance_between_two_points(q, P)));
                    if a < min_Q {
                        Q = q;
                        min_Q = a;
                    }
                }
            }
        }

        //Now  find  R  such  that  the  absolute  value of  the  angle  ∠PRQ  is  minimal
        let mut R = (0,0);

        let mut min_PRQ = 2.*PI;

        let c = distance_between_two_points(Q, P);
        std::dbg!(c);
        // PRQ will be a generic triangle and the angle PRQ will be given by
        // a = dist(P, R)
        // b = dist(Q,R)
        // c=dist(Q,P)
        //
        //angle = acos((a^2+b^2-c^2)/(2ab))
        for y in 0..(image.height as i64) {
            for x in 0..(image.width as i64) {
                if image.get(x, y) == Some(BLACK) {
                    let r = (x, y);
                    if r == P || r == Q || (x == P.0 && P.0 == Q.0){
                        continue;
                    }
                    let a = distance_between_two_points(r, P);
                    let b = distance_between_two_points(r, Q);
                    let a = f64::acos((a.powi(2)+b.powi(2)-c.powi(2))/(2.*a*b));
                    if a < min_PRQ {
                        R = r;
                        min_PRQ = a;
                    }
                }
            }
        }

        std::dbg!(P);
        std::dbg!(Q);
        std::dbg!(min_Q);
        std::dbg!(R);
        std::dbg!(min_PRQ);
        image.clear();
        image.plot(P.0, P.1);
        image.plot(Q.0, Q.1);
        image.plot(R.0, R.1);
        if min_PRQ < FRAC_PI_2 {
            let radius = ((P.0-Q.0).abs()/2, (P.1-Q.1).abs()/2);
            let center = ((P.0-Q.0)/2, (P.1-Q.1)/2);
            std::dbg!(radius);
            std::dbg!(center);
            break;
        }
    }
}
*/

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
    /*let (center, r) = min_circle_w_3_points(&mut full, (
        6,
        21,
    ),
     (
        27,
        2,
    ),
     (
        44,
        43,
    )
        );*/
    image.draw_outline();

    full.plot_circle((center.0 + 45, center.1 + 45), r as i64, 0.);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        image.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
        full.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
