cargo// Define the ODE dy/dx = f(x, y)
fn f(x: f64, y: f64) -> f64 {
    // Example ODE: dy/dx = -y
    y-x*x + 1.0
}

// Heun's method for numerical integration
fn heuns_method(x0: f64, y0: f64, h: f64, num_steps: usize) -> Vec<(f64, f64)> {
    let mut results = Vec::with_capacity(num_steps + 1);
    let mut x = x0;
    let mut y = y0;

    results.push((x, y));

    for _ in 0..num_steps {
        y = y + h/4.0 * (f(x,y) + 3.0*f(x+2.0*h/3.0,y+2.0*h*f(x,y)/3.0));
        x = x + h;

        results.push((x, y));
    }

    results
}

fn main() {
    // Initial conditions
    let x0 = 0.0;
    let y0 = 0.5;

    // Step size and number of steps
    let h = 0.2;
    let num_steps = 10;

    // Perform numerical integration using Heun's method
    let results = heuns_method(x0, y0, h, num_steps);
    println!("heun");
    // Print or save the results
    for (x, y) in results {
        println!("x = {:.2}, y = {:.6}", x, y);
    }
}
