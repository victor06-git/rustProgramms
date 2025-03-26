use std::{thread, time};

fn main() {
    let height = 10; // Altura del triángulo
    let delay = time::Duration::from_millis(200); // Retraso entre cada línea

    for i in 0..height {
        // Imprimir espacios
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        // Imprimir estrellas
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        // Nueva línea
        println!();
        // Esperar un momento antes de imprimir la siguiente línea
        thread::sleep(delay);
    }

    // Mensaje final
    println!("\n¡Triángulo de estrellas generado!");
}