// Importación de los metodos como modulo
mod operaciones;

// Función main principal que se ejecuta en Rust
fn main() {
    println!("Metodos de ejemplo para la calculadora sencilla");

    println!("La suma es {}", operaciones::suma(9, 5));
    println!("La resta es {}", operaciones::resta(2, 5));
    println!("La división es {}", operaciones::division(7, 5));
    println!("La multiplicación es {}", operaciones::multiplicacion(8, 5));
}
