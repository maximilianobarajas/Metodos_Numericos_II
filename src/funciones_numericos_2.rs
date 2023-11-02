extern crate num_traits;
use num_traits::pow;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
type MyFunction = fn(f64) -> f64;

pub fn derivada_adelante(function: MyFunction, x_0: f64, h_values: Vec<f64>, output_file: &str, fprima: Option<MyFunction>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "h,derivada aproximada, cota error,abs_diff")?;
    for h in h_values {
        let derivative = (function(x_0 + h) - function(x_0)) / h;
        let cota = h.abs() / (2.0 * (x_0 * x_0));
        let abs_diff = match fprima {
            Some(f) => (derivative - f(x_0)).abs(),
            None => 0.0, 
        };
        writeln!(file, "{},{},{},{}", h, derivative, cota, abs_diff)?;
    }

    Ok(())
}

pub fn derivada_atras(function: MyFunction, x_0: f64, h_values: Vec<f64>, output_file: &str, fprima: Option<MyFunction>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "h,derivada aproximada, cota error,abs_diff")?;
    for h in h_values {
        let derivative = (function(x_0) - function(x_0 - h)) / h;
        let cota = h.abs() / (2.0 * (x_0 * x_0 - h));
        let abs_diff = match fprima {
            Some(f) => (derivative - f(x_0)).abs(),
            None => 0.0, 
        };
        writeln!(file, "{},{},{},{}", h, derivative, cota, abs_diff)?;
    }

    Ok(())
}

pub fn tres_puntos(function: MyFunction, x_0: f64, h_values: Vec<f64>, output_file: &str, fprima: Option<MyFunction>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "h,derivada aproximada, cota error,abs_diff")?;
    for h in h_values {
        let derivative = (-3.0 * function(x_0) + 4.0 * function(x_0 + h) - function(x_0 + 2.0 * h)) / (2.0 * h);
        let cota = h.abs() / (2.0 * (x_0 * x_0 - h));
        let abs_diff = match fprima {
            Some(f) => (derivative - f(x_0)).abs(),
            None => 0.0, 
        };
        writeln!(file, "{},{},{},{}", h, derivative, cota, abs_diff)?;
    }
    Ok(())
}

pub fn tres_puntos_medio(function: MyFunction, x_0: f64, h_values: Vec<f64>, output_file: &str, fprima: Option<MyFunction>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "h,derivada aproximada, cota error,abs_diff")?;
    for h in h_values {
        let derivative = (function(x_0 + h) - function(x_0 - h)) / (2.0 * h);
        let cota = (2.0 * h * h) / (6.0 * pow(x_0 - h, 3));
        let abs_diff = match fprima {
            Some(f) => (derivative - f(x_0)).abs(),
            None => 0.0,
        };
        writeln!(file, "{},{},{},{}", h, derivative, cota, abs_diff)?;
    }
    Ok(())
}

