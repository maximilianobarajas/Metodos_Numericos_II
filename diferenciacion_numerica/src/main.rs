fn diff_forwards<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por la definición hacía adelante
    (f(x + h) - f(x)) / h
}

fn diff_backwards<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por la definición hacía atrás
    (f(x) - f(x - h)) / h
}

fn diff_final_three_points<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por tres puntos recorridos
    (-3.0 * f(x) + 4.0 * f(x + h) - f(x + 2.0 * h)) / (2.0 * h)
}

fn diff_medium_three_points<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por tres puntos medios
    (f(x + h) - f(x - h)) / (2.0 * h)
}

fn diff_medium_five_points<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por cinco puntos medios
    (f(x - 2.0 * h) - 8.0 * f(x - h) + 8.0 * f(x + h) - f(x + 2.0 * h)) / (12.0 * h)
}

fn diff_final_five_points<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    // Derivada aproximada por cinco puntos finales
    (-25.0 * f(x) + 48.0 * f(x + h) - 36.0 * f(x + 2.0 * h)
        + 16.0 * f(x + 3.0 * h) - 3.0 * f(x + 4.0 * h)) / (12.0 * h)
}

fn main() {
    // Define a simple function for testing, e.g., f(x) = x^2
    let f = |x: f64| x * x;

    // Choose a point x and a step size h
    let x = 2.0;
    let h = 0.001;

    // Test each differentiation method
    let forward_diff = diff_forwards(&f, x, h);
    let backward_diff = diff_backwards(&f, x, h);
    let final_three_points_diff = diff_final_three_points(&f, x, h);
    let medium_three_points_diff = diff_medium_three_points(&f, x, h);
    let medium_five_points_diff = diff_medium_five_points(&f, x, h);
    let final_five_points_diff = diff_final_five_points(&f, x, h);

    // Print the results
    println!("Function: f(x) = x^2");
    println!("Point: x = {}", x);
    println!("Step size: h = {}", h);
    println!("Forward Difference: {}", forward_diff);
    println!("Backward Difference: {}", backward_diff);
    println!("Final Three Points Difference: {}", final_three_points_diff);
    println!("Medium Three Points Difference: {}", medium_three_points_diff);
    println!("Medium Five Points Difference: {}", medium_five_points_diff);
    println!("Final Five Points Difference: {}", final_five_points_diff);
}

