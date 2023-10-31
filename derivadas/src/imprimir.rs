use std::fs::File;
use std::io::{self, Read};

pub fn impresion() -> io::Result<()> {
    let file_path = "datos_derivadas.csv"; // Replace with the actual path to your CSV file
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in content.lines() {
        let row: Vec<&str> = line.split(',').collect();
        rows.push(row);
    }

    let max_lengths: Vec<usize> = (0..rows[0].len())
        .map(|col| rows.iter().map(|row| row[col].len()).max().unwrap_or(0))
        .collect();

    for row in &rows {
        for (col, cell) in row.iter().enumerate() {
            print!("{:<width$}", cell, width = max_lengths[col] + 2);
        }
        println!();
    }

    Ok(())
}
