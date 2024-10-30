use num::Complex;

pub(crate) fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(height);
    for y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(width);
        for x in 0..width {

            let x_percent = (x as f64 / width as f64);
            let y_percent = (y as f64 / height as f64);


            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let value_at = mandelbrot_at_point(cx, cy, max_iters);

            row.push(value_at)
        }
        rows.push(row)
    }

    rows
}

fn mandelbrot_at_point(x: f64, y: f64, max_iters: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c
    }
    max_iters
}

pub(crate) fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {


}
