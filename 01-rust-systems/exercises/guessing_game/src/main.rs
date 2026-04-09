use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*struct Estudiante{
    matricula: i32,
    nombre: String,
}

enum Direccion{
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}
*/

fn main() {
    println!("Adivina el numero");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let numero = input.trim().parse::<i32>().unwrap();

        match numero.cmp(&numero_secreto) {
            Ordering::Less => {
                println!("El número secreto es más alto");
            }
            Ordering::Greater => {
                println!("El número secreto es más bajo");
            }
            Ordering::Equal => {
                println!("Ese es el número, que duro eres!");
                break;
            }
        }
    }
}