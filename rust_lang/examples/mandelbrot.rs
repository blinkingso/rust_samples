extern crate ansi_term;
extern crate num;

use ansi_term::Colour::*;
use num::Complex;

/// mandelbrot with ansi_term;
fn main() {
    let mandelbrot = calculate_mandelbrot(10000, -3_f64, -3_f64, 2_f64, 3_f64, 80, 16);
    render_mandelbrot(mandelbrot);
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows = Vec::with_capacity(height);
    // pixel
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(width);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }

    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        // loop to cal point;
        if z.norm() > 2_f64 {
            return i;
        }

        z = z * z + c;
    }

    max_iters
}

fn render_mandelbrot(escape_values: Vec<Vec<usize>>) {
    for row in escape_values {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => RGB(208, 208, 0).paint("#").to_string(),
                3..=5 => Purple.paint("B").to_string(),
                6..=10 => White.paint("C").to_string(),
                11..=30 => Green.paint("D").to_string(),
                31..=100 => Red.paint("E").to_string(),
                101..=200 => Yellow.paint("F").to_string(),
                201..=400 => Cyan.paint("G").to_string(),
                401..=700 => RGB(200, 200, 200).paint("H").to_string(),
                // 700..=800 => RGB(128, 128, 128).paint("S").to_string(),
                _ => Black.paint("I").to_string(),
            };

            line.push_str(&val);
        }

        println!("{}", line);
    }
}
