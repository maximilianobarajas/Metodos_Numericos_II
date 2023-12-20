fn midpoint_runge_kutta(f: fn(f64, f64) -> f64, y0: f64, h: f64, start_time: f64, end_time: f64) -> Vec<f64> {
    let num_steps = ((end_time - start_time) / h).round() as usize;
    let mut result = Vec::with_capacity(num_steps + 1);
    result.push(y0);
    let mut y = y0;
    let mut t = start_time;

    for _ in 0..num_steps {
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        y = y + k2;
        t = t + h;
        result.push(y);
    }
    result
}
fn main() {
    let f = |_, y| y;
    let y0 = 1.0;
    let h = 0.1;
    let start_time = 0.0;
    let end_time = 2.0;
    let result = midpoint_runge_kutta(f, y0, h, start_time, end_time);
    println!("t\t\t y");
    println!("---------------------");
    for (i, y) in result.iter().enumerate() {
        let t = i as f64 * h + start_time;
        println!("{:.2}\t\t {:.6}", t, y);
    }
}
