// Importación de los metodos como modulo
mod calculadora;
mod operaciones;

use std::io;

// Función main principal que se ejecuta en Rust
fn main() {
    // Primer mensaje para el usuario
    println!("Calculadora creada con Rust");

    // Mensaje para capturar que operación requiere el usuario
    println!("Ingrese la operación (suma, resta, multiplicacion, division):");
    let mut operacion = String::new();
    io::stdin()
        .read_line(&mut operacion)
        .expect("Error al leer la entrada");

    // Mensaje para captura el primer número a operar
    println!("Ingrese el primer número:");
    let mut entrada1 = String::new();
    io::stdin()
        .read_line(&mut entrada1)
        .expect("Error al leer la entrada");
    let num1: i32 = entrada1.trim().parse().expect("Ingrese un número válido");

    // Mensaje para captura el segundo número a operar
    println!("Ingrese el segundo número:");
    let mut entrada2 = String::new();
    io::stdin()
        .read_line(&mut entrada2)
        .expect("Error al leer la entrada");
    let num2: i32 = entrada2.trim().parse().expect("Ingrese un número válido");

    // Llama a la función calculadora en el módulo calculadora
    calculadora::calculadora(&operacion.trim(), num1, num2);
}
