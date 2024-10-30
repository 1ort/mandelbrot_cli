mod mandelbrot;
use clap::Parser;

/// Simple program that draws mandelbrot fractal in your terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Minimal x coordinate
    #[arg(long, default_value_t=-2.5, allow_hyphen_values = true)]
    x_min: f64,
    /// Maximal x coordinate
    #[arg(long, default_value_t=1.5, allow_hyphen_values = true)]
    x_max: f64,
    /// Minimal y coordinate
    #[arg(long, default_value_t=-1.5, allow_hyphen_values = true)]
    y_min: f64,
    /// Maximal y coordinate
    #[arg(long, default_value_t=1.5, allow_hyphen_values = true)]
    y_max: f64,

    /// Image width ( in characters)
    #[arg(long, default_value_t=100)]
    width: usize,

    /// Image height ( in characters)
    #[arg(long, default_value_t=24)]
    height: usize,

    /// Maximum number of iterations for each point
    #[arg(long, default_value_t = 10000)]
    n_iters: usize,


}


fn main() {
    let args = Args::parse();

    let mandelbrot = mandelbrot::calculate_mandelbrot(
        args.n_iters,
        args.x_min,
        args.x_max,
        args.y_min,
        args.y_max,
        args.width,
        args.height,
    );
    mandelbrot::render_mandelbrot(mandelbrot);
}
