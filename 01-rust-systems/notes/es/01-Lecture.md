# Índice

- [Instalación](#instalación)
- [Rust Docs](#rust-docs)
- [Hola Mundo](#hola-mundo)
- [Cargo](#cargo)
- [std::io](#stdio)
- [unwrap()](#unwrap)
- [match](#match)
- [Ordering](#ordering)
- [rand crate](#rand-crate)
- [Structs](#structs)
- [Enum](#enum)



## Instalación
- rustup
- rustc --version
---
## Rust Docs

- https://doc.rust-lang.org
- rustup doc

---

## Hola Mundo

```rust
fn main() {
    println!("Hola mundo");
}
```
- fn:
Es la palabra clave para declarar una función

- main:
Es el nombre de la función principal

- ()
Indica los parámetros de la función
En este caso está vacío. la función no recibe argumentos

- { ... }
Delimitan el bloque de código de la función
Todo lo que está dentro se ejecuta cuando se llama main

- println!
Es un macro de Rust (no una función normal)
Sirve para imprimir texto en la consola
El ! indica que es un macro

> [!NOTA]
Un macro en Rust es código que genera otro código antes de que el programa se ejecute, y por eso usa ! : https://doc.rust-lang.org/book/ch20-05-macros.html 

- "Hola mundo"
Es una cadena de texto
Es lo que se va a mostrar en pantalla

- ;
Indica el fin de la instrucción

## Cargo
### Comando
```
cargo new project
cargo run
cargo build
```
## std::io
- Permite leer input del usuario
- Ejemplo:
```
  use std::io;
```

## unwrap()
- Extrae el valor oculto o hace que el programa explote (crashee)s

## match
- Control de flujo tipo switch

## Ordering
- Es un [Enum](#enum) (tipo con opciones) que se usa para comparar valores

```rust
use std::cmp::Ordering;

let secreto = 10;
let intento = 5;

// cmp() compara y devuelve un Ordering
match intento.cmp(&secreto) {
    Ordering::Less => println!("Muy pequeño"),
    Ordering::Greater => println!("Muy grande"),
    Ordering::Equal => println!("Acertaste!"),
}
```

## rand crate
- La librería principal para números aleatorios

```rust
use rand::Rng; // Importamos las herramientas del crate rand

let numero_secreto = rand::thread_rng().gen_range(1..=100);
```

## Structs
- forma de crear nuevos tipos de datos agrupando otros bajo un mismo techo

```rust
// Definimos la estructura
struct Usuario {
    nombre: String,
    edad: u8,
    activo: bool,
}

// Creamos una instancia (un usuario real)
let user1 = Usuario {
    nombre: String::from("Joel"),
    edad: 19,
    activo: true,
};
```

## Enum
- Un tipo de dato que puede ser una de varias opciones distintas

```rust
enum Mensaje Web {
    CargarPagina,  // No tiene datos  extra
    ClickBoton(i32, i32), //Contiene coordenada X e Y
    EscribirTexto(String),  // Contiene un texto
}

// Un mensaje solo puede ser uno de los tres a la vez
let accion = MensajeWeb::EscribirTexto(String::from("Hola Rust"));
```