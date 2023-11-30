extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};
use std::fs::File;
use std::io::Write;

fn ode_function(t: f64, y: f64) -> f64 {
    y - t * t + 1.0
}

fn euler_method(
    ode_func: fn(f64, f64) -> f64,
    initial_t: f64,
    initial_y: f64,
    h: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    result.push((initial_t, initial_y));
    let mut t = initial_t;
    let mut y = initial_y;
    for _ in 0..steps {
        let dy = ode_func(t, y) * h;
        y += dy;
        t += h;
        result.push((t, y));
    }
    result
}

fn main() {
    let initial_t = 0.0;
    let initial_y = 1.0 / 2.0;
    let h = 0.2;
    let steps = 10;
    let solution = euler_method(ode_function, initial_t, initial_y, h, steps);

    // Print the values of t and y
    println!("t\t\ty");
    for (t, y) in &solution {
        println!("{:.6}\t{:.6}", t, y);
    }

    // Write values to a file
    let mut file = File::create("solution_values.txt").expect("Error creating file");
    writeln!(file, "t\ty").expect("Error writing to file");
    for (t, y) in &solution {
        writeln!(file, "{:.6}\t{:.6}", t, y).expect("Error writing to file");
    }

    // Create a new gnuplot figure
    let mut fg = Figure::new();

    // Extract t and y values for plotting
    let (t_values, y_values): (Vec<_>, Vec<_>) = solution.iter().cloned().unzip();

    // Plot the solution
    fg.axes2d()
        .lines(&t_values, &y_values, &[Caption("Euler Method Solution"), Color("blue")]);

    // Display the plot
    fg.show();

    println!("Solution values have been printed to the console and saved to 'solution_values.txt'");
}

