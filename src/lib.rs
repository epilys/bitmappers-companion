use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
pub type Point = (i64, i64);
pub type Line = (i64, i64, i64);

pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub const AZURE_BLUE: u32 = from_u8_rgb(0, 127, 255);
pub const RED: u32 = from_u8_rgb(157, 37, 10);
pub const WHITE: u32 = from_u8_rgb(255, 255, 255);
pub const GRAY82: u32 = from_u8_rgb(208, 208, 208);
pub const BLACK: u32 = 0;

pub const fn from_u32_rgb(v: u32) -> (u8, u8, u8) {
    let r = v >> 16;
    let g = (v >> 8) & 0xff;
    let b = v & 0xff;
    (r as u8, g as u8, b as u8)
}
#[derive(Clone)]
pub struct Image {
    pub bytes: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub y_offset: usize,
}

impl Image {
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
        Image {
            bytes: vec![WHITE; width * height],
            width,
            height,
            x_offset,
            y_offset,
        }
    }

    pub fn magick_open(
        path: &str,
        x_offset: usize,
        y_offset: usize,
    ) -> Result<Self, Box<dyn ::std::error::Error>> {
        let output = Command::new("identify").args([path]).output()?;

        let re = regex::Regex::new(r"\s*(\d+)x(\d+)\s*")?;
        let identify = String::from_utf8(output.stdout)?;
        let matches = re
            .captures(&identify)
            .ok_or("Could not find dimensions in `identify` output")?;
        let width = matches.get(1).unwrap().as_str().parse::<usize>()?;
        let height = matches.get(2).unwrap().as_str().parse::<usize>()?;
        let output = Command::new("magick")
            .args(["convert", path, "RGB:-"])
            .output()?;

        let bytes = output.stdout;

        let bytes = bytes
            .chunks(3)
            .map(|c| from_u8_rgb(c[0], c[1], c[2]))
            .collect::<Vec<u32>>();
        Ok(Image {
            bytes,
            width,
            height,
            x_offset,
            y_offset,
        })
    }

    pub fn from_xbm(
        path: &str,
        x_offset: usize,
        y_offset: usize,
    ) -> Result<Self, Box<dyn ::std::error::Error>> {
        let mut file = File::open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let re = Regex::new(
            r"(?imx)
  ^\s*\x23\s*define\s+(?P<i>.+?)_width\s+(?P<w>\d\d*)$
  \s*
  ^\s*\x23\s*define\s+.+?_height\s+(?P<h>\d\d*)$
  \s*
  ^\s*static(\s+unsigned){0,1}\s+char\s+.+?_bits..\s*=\s*\{(?P<b>[^}]+)\};
  ",
        )
        .unwrap();
        let caps = re
            .captures(&s)
            .ok_or("Could not open xbm file, regex doesn't match :(")?;
        let width = caps.name("w").unwrap().as_str().parse::<usize>()?;
        let height = caps.name("h").unwrap().as_str().parse::<usize>()?;
        let bits = caps
            .name("b")
            .unwrap()
            .as_str()
            .split(',')
            .map(|h| u8::from_str_radix(&h.trim()["0x".len()..], 16))
            .collect::<Result<Vec<u8>, _>>()?;
        Ok(Image {
            bytes: bits_to_bytes(&bits, width),
            width,
            height,
            x_offset,
            y_offset,
        })
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>, window_width: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bytes[y * self.width + x] == BLACK {
                    buffer[(self.y_offset + y) * window_width + self.x_offset + x] = fg;
                } else if self.bytes[y * self.width + x] != WHITE {
                    buffer[(self.y_offset + y) * window_width + self.x_offset + x] =
                        self.bytes[y * self.width + x];
                } else if let Some(bg) = bg {
                    buffer[(self.y_offset + y) * window_width + self.x_offset + x] = bg;
                }
            }
        }
    }

    pub fn draw_raw(&self, buffer: &mut Vec<u32>, window_width: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                buffer[(self.y_offset + y) * window_width + self.x_offset + x] =
                    self.bytes[y * self.width + x];
            }
        }
    }

    pub fn draw_outline(&mut self) {
        for i in 0..(self.height as _) {
            self.plot(0, i);
            self.plot(self.width as i64 - 1, i);
        }
        for i in 0..(self.width as _) {
            self.plot(i, self.height as i64 - 1);
            self.plot(i, 0);
        }
    }

    pub fn draw_grid(&mut self, step: usize) {
        for i in (0..(self.height as _)).step_by(step) {
            self.plot_line_width_color((0, i), (self.width as i64 - 1, i), 0., Some(GRAY82));
        }
        for i in (0..(self.width as _)).step_by(step) {
            self.plot_line_width_color((i, self.height as i64 - 1), (i, 0), 0., Some(GRAY82));
        }
    }

    pub fn clear(&mut self) {
        for i in self.bytes.iter_mut() {
            *i = WHITE;
        }
    }

    pub fn plot(&mut self, x: i64, y: i64) {
        if x < 0 || y < 0 || y >= (self.height as i64) || x >= (self.width as i64) {
            //eprintln!("invalid plot() coors: ({}, {})", x, y);
            return;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        self.bytes[y * self.width + x] = BLACK;
    }

    pub fn plot_color(&mut self, x: i64, y: i64, color: Option<u32>) {
        if x < 0 || y < 0 || y >= (self.height as i64) || x >= (self.width as i64) {
            //eprintln!("invalid plot() coors: ({}, {})", x, y);
            return;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        self.bytes[y * self.width + x] = color.unwrap_or(BLACK);
    }

    pub fn get(&self, x: i64, y: i64) -> Option<u32> {
        if x < 0 || y < 0 || y >= (self.height as i64) || x >= (self.width as i64) {
            return None;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        Some(self.bytes[y * self.width + x])
    }

    pub fn plot_circle(&mut self, center: Point, r: i64, _wd: f64) {
        self.plot_ellipse(center, (r, r), [true, true, true, true], _wd)
    }

    pub fn plot_square(&mut self, center: Point, r: i64, wd: f64) {
        let (cx, cy) = center;
        let a = (cx - r, cy - r);
        let b = (cx + r, cy - r);
        let c = (cx + r, cy + r);
        let d = (cx - r, cy + r);
        self.plot_line_width(a, b, wd);
        self.plot_line_width(b, c, wd);
        self.plot_line_width(c, d, wd);
        self.plot_line_width(d, a, wd);
    }

    pub fn plot_triangle(&mut self, center: Point, d: i64, wd: f64) {
        let (cx, cy) = center;
        let a = (cx - d, cy - d);
        let b = (cx + d, cy - d);
        let c = (cx + d, cy + d);
        self.plot_line_width(a, b, wd);
        self.plot_line_width(b, c, wd);
        self.plot_line_width(c, a, wd);
    }

    pub fn plot_ellipse(
        &mut self,
        (xm, ym): (i64, i64),
        (a, b): (i64, i64),
        quadrants: [bool; 4],
        _wd: f64,
    ) {
        let mut x = -a;
        let mut y = 0;
        let mut e2 = b;
        let mut dx = (1 + 2 * x) * e2 * e2;
        let mut dy = x * x;
        let mut err = dx + dy;
        loop {
            if quadrants[0] {
                self.plot(xm - x, ym + y); /*   I. Quadrant */
            }
            if quadrants[1] {
                self.plot(xm + x, ym + y); /*  II. Quadrant */
            }
            if quadrants[2] {
                self.plot(xm + x, ym - y); /* III. Quadrant */
            }
            if quadrants[3] {
                self.plot(xm - x, ym - y); /*  IV. Quadrant */
            }
            e2 = 2 * err;
            if e2 >= dx {
                x += 1;
                dx += 2 * b * b;
                err += dx;
                //err += dx += 2*(long)b*b; }    /* x step */
            }
            if e2 <= dy {
                y += 1;
                dy += 2 * a * a;
                err += dy;
                //err += dy += 2*(long)a*a; }    /* y step */
            }
            if x > 0 {
                break;
            }
        }
        while y < b {
            /* to early stop for flat ellipses with a=1, */
            y += 1;
            self.plot(xm, ym + y); /* -> finish tip of ellipse */
            self.plot(xm, ym - y);
        }
    }

    pub fn plot_line_width(&mut self, a: (i64, i64), b: (i64, i64), wd: f64) {
        self.plot_line_width_color(a, b, wd, None)
    }

    pub fn plot_line_width_color(
        &mut self,
        (x1, y1): (i64, i64),
        (x2, y2): (i64, i64),
        wd: f64,
        color: Option<u32>,
    ) {
        /* Bresenham's line algorithm */
        let mut d;
        let mut x: i64;
        let mut y: i64;
        let ax: i64;
        let ay: i64;
        let sx: i64;
        let sy: i64;
        let dx: i64;
        let dy: i64;

        dx = x2 - x1;
        ax = (dx * 2).abs();
        sx = if dx > 0 { 1 } else { -1 };

        dy = y2 - y1;
        ay = (dy * 2).abs();
        sy = if dy > 0 { 1 } else { -1 };

        x = x1;
        y = y1;

        let b = if dy == 0 { -1 } else { dx / dy };
        let a = 1;
        let double_d = (wd * f64::sqrt((a * a + b * b) as f64)) as i64;
        let delta = double_d / 2;

        if ax > ay {
            /* x step */
            d = ay - ax / 2;
            loop {
                self.plot_color(x, y, color);
                {
                    let total = |_x| {
                        if dy == 0 {
                            _x - x1
                        } else {
                            _x - (y * dx) / dy + (y1 * dx) / dy - x1
                        }
                    };
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x += 1;
                        self.plot_color(_x, y, color);
                    }
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x -= 1;
                        self.plot_color(_x, y, color);
                    }
                }
                if x == x2 {
                    return;
                }
                if d >= 0 {
                    y = y + sy;
                    d = d - ax;
                }
                x = x + sx;
                d = d + ay;
            }
        } else {
            /* y step */
            d = ax - ay / 2;
            loop {
                self.plot_color(x, y, color);
                {
                    let total = |_x| {
                        if dy == 0 {
                            _x - x1
                        } else {
                            _x - (y * dx) / dy + (y1 * dx) / dy - x1
                        }
                    };
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x += 1;
                        self.plot_color(_x, y, color);
                    }
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x -= 1;
                        self.plot_color(_x, y, color);
                    }
                }
                if y == y2 {
                    return;
                }
                if d >= 0 {
                    x = x + sx;
                    d = d - ay;
                }
                y = y + sy;
                d = d + ax;
            }
        }
    }

    pub fn fill_triangle(&mut self, q1: Point, q2: Point, q3: Point) {
        let make_equation =
            |p1: Point, p2: Point, p3: Point, a: &mut i64, b: &mut i64, c: &mut i64| {
                *a = p2.1 - p1.1;
                *b = p1.0 - p2.0;
                *c = p1.0 * p2.1 - p1.1 * p2.0;

                if *a * p3.0 + *b * p3.1 + *c < 0 {
                    *a = -*a;
                    *b = -*b;
                    *c = -*c;
                }
            };
        let mut x_min = q1.0;
        let mut y_min = q1.1;
        let mut x_max = q1.0;
        let mut y_max = q1.1;
        let mut a = [0_i64; 3];
        let mut b = [0_i64; 3];
        let mut c = [0_i64; 3];

        // find bounding box
        for q in [q1, q2, q3] {
            x_min = std::cmp::min(x_min, q.0);
            x_max = std::cmp::max(x_max, q.0);

            y_min = std::cmp::min(y_min, q.1);
            y_max = std::cmp::max(y_max, q.1);
        }
        make_equation(q1, q2, q3, &mut a[0], &mut b[0], &mut c[0]);
        make_equation(q1, q3, q2, &mut a[1], &mut b[1], &mut c[1]);
        make_equation(q2, q3, q1, &mut a[2], &mut b[2], &mut c[2]);

        let mut d0 = a[0] * x_min + b[0] * y_min + c[0];
        let mut d1 = a[1] * x_min + b[1] * y_min + c[1];
        let mut d2 = a[2] * x_min + b[2] * y_min + c[2];

        for y in y_min..=y_max {
            let mut f0 = d0;
            let mut f1 = d1;
            let mut f2 = d2;

            d0 += b[0];
            d1 += b[1];
            d2 += b[2];

            for x in x_min..=x_max {
                if f0 >= 0 && f1 >= 0 && f2 >= 0 {
                    self.plot(x, y);
                }
                f0 += a[0];
                f1 += a[1];
                f2 += a[2];
            }
        }
    }

    pub fn flood_fill(&mut self, x: i64, y: i64) {
        if self.get(x, y) != Some(WHITE) {
            return;
        }

        let w = self.width as i64;
        let h = self.height as i64;
        let mut span_above: bool;
        let mut span_below: bool;

        let mut s = VecDeque::new();
        s.push_back((x, y));
        while let Some((x, y)) = s.pop_back() {
            let mut x1 = x;
            while x1 >= 0 && self.get(x1, y).map(|some| some == WHITE).unwrap_or(false) {
                x1 -= 1;
            }
            x1 += 1;
            span_above = false;
            span_below = false;
            while x1 < w && self.get(x1, y).map(|some| some == WHITE).unwrap_or(false) {
                self.plot(x1, y);
                if !span_above
                    && y > 0
                    && self
                        .get(x1, y - 1)
                        .map(|some| some == WHITE)
                        .unwrap_or(false)
                {
                    s.push_back((x1, y - 1));
                    span_above = true;
                } else if span_above
                    && y > 0
                    && self
                        .get(x1, y - 1)
                        .map(|some| some != WHITE)
                        .unwrap_or(false)
                {
                    span_above = false;
                }
                if !span_below
                    && y < h - 1
                    && self
                        .get(x1, y + 1)
                        .map(|some| some == WHITE)
                        .unwrap_or(false)
                {
                    s.push_back((x1, y + 1));
                    span_below = true;
                } else if span_below
                    && y < h - 1
                    && self
                        .get(x1, y + 1)
                        .map(|some| some != WHITE)
                        .unwrap_or(false)
                {
                    span_below = false;
                }
                x1 += 1;
            }
        }
    }

    pub fn copy(
        &mut self,
        source: &Image,
        (x_offset, y_offset): (usize, usize),
        (sx_offset, sy_offset): (usize, usize),
        width: usize,
        height: usize,
    ) {
        for (sy, y) in (y_offset..std::cmp::min(height + y_offset, self.height)).enumerate() {
            for (sx, x) in (x_offset..std::cmp::min(width + x_offset, self.width)).enumerate() {
                if let Some(p) =
                    source.get(sx as i64 + sx_offset as i64, sy as i64 + sy_offset as i64)
                {
                    if p == BLACK {
                        self.plot(x as i64, y as i64);
                    }
                }
            }
        }
    }

    pub fn write_str(&mut self, font: &BitmapFont, s: &str, (x, y): (i64, i64)) {
        for (i, c) in s.chars().enumerate() {
            let glyph = font.glyph(c).unwrap();
            self.copy(
                &glyph,
                (x as usize + i * font.glyph_width, y as usize),
                (0, 0),
                font.glyph_width,
                font.glyph_height,
            );
        }
    }

    pub fn resize(
        &self,
        scaled_width: usize,
        scaled_height: usize,
        x_offset: usize,
        y_offset: usize,
    ) -> Image {
        let mut scaled = Image::new(scaled_width, scaled_height, x_offset, y_offset);
        let mut sx: i64; //source
        let mut sy: i64; //source
        let mut dx: i64; //destination
        let mut dy: i64 = 0; //destination

        let og_height = self.height as i64;
        let og_width = self.width as i64;
        let scaled_height = scaled.height as i64;
        let scaled_width = scaled.width as i64;

        while dy < scaled_height {
            sy = (dy * og_height) / scaled_height;
            dx = 0;
            while dx < scaled_width {
                sx = (dx * og_width) / scaled_width;
                if self.get(sx, sy) == Some(BLACK) {
                    scaled.plot(dx, dy);
                }
                dx += 1;
            }
            dy += 1;
        }
        scaled
    }
}

pub fn bits_to_bytes(bits: &[u8], width: usize) -> Vec<u32> {
    let mut ret = Vec::with_capacity(bits.len() * 8);
    let mut current_row_count = 0;
    for byte in bits {
        for n in 0..8 {
            if byte.rotate_right(n) & 0x01 > 0 {
                ret.push(BLACK);
            } else {
                ret.push(WHITE);
            }
            current_row_count += 1;
            if current_row_count == width {
                current_row_count = 0;
                break;
            }
        }
    }
    assert!(current_row_count == 0);
    ret
}

pub struct BitmapFont {
    pub image: Image,
    pub x_offset: usize,
    pub y_offset: usize,
    pub glyph_width: usize,
    pub glyph_height: usize,
}

impl BitmapFont {
    pub fn new(
        image: Image,
        (glyph_width, glyph_height): (usize, usize),
        x_offset: usize,
        y_offset: usize,
    ) -> Self {
        Self {
            image,
            x_offset,
            y_offset,
            glyph_width,
            glyph_height,
        }
    }

    pub fn glyph(&self, c: char) -> Option<Image> {
        let gwidth = (self.image.width - self.x_offset) / self.glyph_width;

        let idx = c as u32;
        let row = (idx & 0x00FF) as usize;

        let cursor = (
            self.x_offset + (row % gwidth) * self.glyph_width,
            self.y_offset + (row / gwidth) * self.glyph_height,
        );

        let mut glyph = Image::new(self.glyph_width, self.glyph_height, 0, 0);
        glyph.copy(
            &self.image,
            (self.x_offset, self.y_offset),
            (cursor.0, cursor.1),
            self.glyph_width,
            self.glyph_height,
        );
        Some(glyph)
    }
}

pub fn distance_line_to_point((x, y): Point, (a, b, c): Line) -> f64 {
    let d = f64::sqrt((a * a + b * b) as f64);
    if d == 0.0 {
        0.
    } else {
        (a * x + b * y + c) as f64 / d
    }
}

pub fn perpendicular((a, b, c): Line, p: Point) -> Line {
    (b, -1 * a, a * p.1 - b * p.0)
}

pub fn point_perpendicular((a, b, c): Line, p: Point) -> Point {
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
