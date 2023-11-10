// Función main principal que se ejecuta en Rust
fn main() {
    println!("Metodos de ejemplo para la calculadora sencilla");

    println!("La suma es {}", suma(3, 5));
    println!("La resta es {}", resta(3, 5));
    println!("La división es {}", division(3, 5));
    println!("La multiplicación es {}", multiplicacion(3, 5));
}

// Metodo para sumar 2 números y regresar su resultado
fn suma(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

// Metodo para restar 2 números y regresar su resultado
fn resta(num1: i32, num2: i32) -> i32 {
    return num1 - num2;
}

// Metodo para multiplicar 2 números y regresar su resultado
fn multiplicacion(num1: i32, num2: i32) -> i32 {
    return num1 * num2;
}

// Metodo para dividir 2 números y regresar su resultado
fn division(num1: i32, num2: i32) -> f64 {
    if num1 == 0 || num2 == 0 {
        return 0 as f64;
    } else {
        return num1 as f64 / num2 as f64;
    };
}
