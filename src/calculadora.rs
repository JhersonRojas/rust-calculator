use crate::operaciones;

pub fn calculadora(operacion: &str, num1: i32, num2: i32) {
    match operacion {
        "suma" => println!("Resultado: {}", operaciones::suma(num1, num2)),
        "resta" => println!("Resultado: {}", operaciones::resta(num1, num2)),
        "multiplicacion" => println!("Resultado: {}", operaciones::multiplicacion(num1, num2)),
        "division" => match operaciones::division(num1, num2) {
            Some(resultado) => println!("Resultado: {:.2}", resultado),
            None => println!("Error: No se puede dividir por cero."),
        },
        _ => println!("Operación no válida"),
    }
}
