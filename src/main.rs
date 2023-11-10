// Importación de los metodos como modulo
mod operaciones;

use std::io;

// Función main principal que se ejecuta en Rust
fn main() {
    println!("Metodos de ejemplo para la calculadora sencilla");

    println!("Ingrese el primer número:");
    let mut entrada1 = String::new();
    io::stdin()
        .read_line(&mut entrada1)
        .expect("Error al leer la entrada");
    let num1: i32 = entrada1.trim().parse().expect("Ingrese un número válido");

    println!("Ingrese el segundo número:");
    let mut entrada2 = String::new();
    io::stdin()
        .read_line(&mut entrada2)
        .expect("Error al leer la entrada");
    let num2: i32 = entrada2.trim().parse().expect("Ingrese un número válido");

    println!(
        "El resultado de la suma es {}",
        operaciones::suma(num1, num2)
    );
    println!(
        "El resultado de la resta es {}",
        operaciones::resta(num1, num2)
    );
    println!(
        "El resultado de la división es {}",
        operaciones::division(num1, num2)
    );
    println!(
        "El resultado de la multiplicación es {}",
        operaciones::multiplicacion(num1, num2)
    );
}
