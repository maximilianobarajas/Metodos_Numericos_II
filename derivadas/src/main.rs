// Import the functions from my_functions.rs
mod funciones_numericos_2;
mod imprimir;
fn main() {
    //Definimos una funcion como ejemplo
    fn f(x: f64) -> f64 {
        x.ln()
    }
    let x_0 = 1.8; //Definimos el punto en el cual aproximar la derivada
    let h_values = vec![0.1, 0.01, 0.001,0.0001,0.00001,0.000001,0.0000001]; //Determinamos una serie de valores de h a utilizar en la aproximacion de la derivada
    match funciones_numericos_2::approximate_derivative_csv(f, x_0, h_values, "datos_derivadas.csv") {
        Ok(_) => println!("El archivo csv esta creado correctamente:"),
        Err(e) => eprintln!("Error: {}", e),
    }
    let _ =imprimir::impresion();
}
