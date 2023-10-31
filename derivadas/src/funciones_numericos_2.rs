// my_functions.rs
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
type MyFunction = fn(f64) -> f64;
pub fn approximate_derivative_csv(function: MyFunction, x_0: f64, h_values: Vec<f64>, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "h,derivada aproximada, cota error")?;
    for h in h_values {
        let derivative = (function(x_0 + h) - function(x_0)) / h;
        let cota = h.abs()/(2.0 * (x_0 * x_0) );
        writeln!(file, "{},{},{}", h, derivative,cota)?;
    }

    Ok(())
}
