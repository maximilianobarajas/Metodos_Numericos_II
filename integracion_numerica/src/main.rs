fn integrate_trapezoid<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = b - a;
    (delta / 2.0) * (lambda(a) + lambda(a + delta))
}

fn integrate_simpson<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 2.0;
    (delta / 3.0) * (lambda(a) + 4.0 * lambda((a + b) / 2.0) + lambda(b))
}

fn integrate_three_eights<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 3.0;
    (3.0 * delta / 8.0) * (lambda(a) + 3.0 * lambda(a + delta) + 3.0 * lambda(a + 2.0 * delta) + lambda(b))
}

fn integrate_boole<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 4.0;
    (4.0 * delta / 90.0) *
        (7.0 * lambda(a)
            + 32.0 * lambda(a + delta)
            + 12.0 * lambda(a + 2.0 * delta)
            + 32.0 * lambda(a + 3.0 * delta)
            + 7.0 * lambda(b))
}

fn integrate_mid_point<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    (b - a) * lambda((a + b) / 2.0)
}

fn integrate_i_1_a<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 3.0;
    (b - a) / 2.0 * (lambda(a + delta) + lambda(a + 2.0 * delta))
}

fn integrate_i_2_a<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 4.0;
    (b - a) / 3.0 * (2.0 * lambda(a + delta) - lambda(a + 2.0 * delta) + 2.0 * lambda(a + 3.0 * delta))
}

fn integrate_i_3_a<F: Fn(f64) -> f64>(lambda: F, a: f64, b: f64) -> f64 {
    let delta = (b - a) / 5.0;
    (b - a) / 24.0 *
        (11.0 * lambda(a + delta)
            + lambda(a + 2.0 * delta)
            + lambda(a + 3.0 * delta)
            + 11.0 * lambda(a + 4.0 * delta))
}

fn main() {
    // Define a simple function for testing, e.g., f(x) = x^2
    let f = |x: f64| x * x;

    // Specify the integration interval [a, b]
    let a = 0.0;
    let b = 2.0;

    // Test each integration method
    let trapezoid_result = integrate_trapezoid(&f, a, b);
    let simpson_result = integrate_simpson(&f, a, b);
    let three_eights_result = integrate_three_eights(&f, a, b);
    let boole_result = integrate_boole(&f, a, b);
    let mid_point_result = integrate_mid_point(&f, a, b);
    let i_1_a_result = integrate_i_1_a(&f, a, b);
    let i_2_a_result = integrate_i_2_a(&f, a, b);
    let i_3_a_result = integrate_i_3_a(&f, a, b);

    // Print the results
    println!("Function: f(x) = x^2");
    println!("Integration Interval: [{}, {}]", a, b);
    println!("Trapezoid Integration Result: {}", trapezoid_result);
    println!("Simpson Integration Result: {}", simpson_result);
    println!("Three-Eights Integration Result: {}", three_eights_result);
    println!("Boole Integration Result: {}", boole_result);
    println!("Mid-Point Integration Result: {}", mid_point_result);
    println!("I_1_a Integration Result: {}", i_1_a_result);
    println!("I_2_a Integration Result: {}", i_2_a_result);
    println!("I_3_a Integration Result: {}", i_3_a_result);
}

