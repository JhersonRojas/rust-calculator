// Usando Operaciones como un módulo
use crate::operaciones;

// Creando la función para elegir el tipo de operación necesitada
pub fn calculadora(operacion: &str, num1: i32, num2: i32) {
    // Recorre las coincidencias para elegir la operación requerida
    match operacion {
        // En caso de coincidir una operación elegida, regresara su resultado
        "suma" => println!("Resultado: {}", operaciones::suma(num1, num2)),
        "resta" => println!("Resultado: {}", operaciones::resta(num1, num2)),
        "multiplicacion" => println!("Resultado: {}", operaciones::multiplicacion(num1, num2)),
        "division" => match operaciones::division(num1, num2) {
            Some(resultado) => println!("Resultado: {:.2}", resultado),
            None => println!("Error: No se puede dividir por cero."),
        },
        // En caso de no coincidir la operación elegida, regresara un mensaje de invalidez
        _ => println!("Operación no válida"),
    }
}
