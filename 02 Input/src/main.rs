use std::io;

fn main() {
    println!("Introduce un texto:");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Fallo al leer la línea");

    println!("Has introducido: {text}");
}