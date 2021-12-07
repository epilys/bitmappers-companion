use bitmappers_companion::*;
use minifb::{Key, Window, WindowOptions};
use rust_decimal::prelude::*;
use std::collections::VecDeque;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

fn gosper(img: &mut Image, x_offset: i64, y_offset: i64, order: usize, step_size: Decimal) {
    enum Rules {
        A,
        B,
        Plus,
        Minus,
    }
    use Rules::*;

    macro_rules! lsystem {
        (A) => {
            A
        };
        (B) => {
            B
        };
        (-) => {
            Minus
        };
        (+) => {
            Plus
        };
        ($($input:ident),*;$o:ident) => {
            [$((lsystem!($input), $o)),*]
        };
        ($($input:tt),*;$o:ident) => {
            [$((lsystem!($input), $o)),*]
        };
    }
    let a_production = |o: usize| lsystem!(A,-,B,-,-,B,+,A,+,+,A,A,+,B,-;o);
    let b_production = |o: usize| lsystem!(+,A,-,B,B,-,-,B,-,A,+,+,A,+,B;o);
    let mut stack = VecDeque::from(a_production(order));

    let mut angle: Decimal = -Decimal::HALF_PI;
    let step_size: Decimal = Decimal::from(step_size);

    let sixty_degrees: Decimal = Decimal::from_str("1.047197551196597746154214").unwrap();

    let mut prev_pos = (x_offset, y_offset);
    while let Some((rule, o)) = stack.pop_front() {
        if o == 0 {
            let (s, c) = (angle.sin(), angle.cos());
            let new_point = (
                prev_pos.0 + (c * step_size).to_i64().unwrap(),
                prev_pos.1 + (s * step_size).to_i64().unwrap(),
            );

            img.plot_line_width(new_point, prev_pos, 80.0);
            prev_pos = new_point;
        }

        if o > 0 {
            match rule {
                A => {
                    for r in a_production(o - 1).into_iter().rev() {
                        stack.push_front(r);
                    }
                }
                B => {
                    for r in b_production(o - 1).into_iter().rev() {
                        stack.push_front(r);
                    }
                }
                Plus => {
                    angle -= sixty_degrees;
                }
                Minus => {
                    angle += sixty_degrees;
                }
            }
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Gosper Curve - ESC to exit",
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

    let mut image = Image::new(20 * WINDOW_WIDTH, 20 * WINDOW_WIDTH, 0, 0);
    gosper(
        &mut image,
        (20 * WINDOW_WIDTH) as i64 / 12,
        (20 * WINDOW_HEIGHT) as i64 / 3,
        4,
        Decimal::from(20),
    );

    let small = image.resize(WINDOW_WIDTH, WINDOW_HEIGHT, 0, 0);
    small.draw(&mut buffer, BLACK, None, WINDOW_WIDTH);
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
