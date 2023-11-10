// Metodo público para sumar 2 números y regresar su resultado
pub fn suma(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

// Metodo público para restar 2 números y regresar su resultado
pub fn resta(num1: i32, num2: i32) -> i32 {
    return num1 - num2;
}

// Metodo público para multiplicar 2 números y regresar su resultado
pub fn multiplicacion(num1: i32, num2: i32) -> i32 {
    return num1 * num2;
}

// Metodo público para dividir 2 números y regresar su resultado
pub fn division(num1: i32, num2: i32) -> f64 {
    if num1 == 0 || num2 == 0 {
        return 0 as f64;
    } else {
        return num1 as f64 / num2 as f64;
    };
}
