# Rust Introduction

## Basic Syntax

-  Variables

- Data Types

- Control Flow


## Functions

- Defining Functions

- Parameters and Return Types

- Closures


## How to Start with Rust

- Installing Rust

- Setting Up a Project

- Running Your First Program


## Structures

- Defining Structures

- Using Structures

- Methods

## Enums

- Defining Enums

- Pattern Matching

- Use Cases


## Error Handling

- Result Type

- Option Type

- Panic

## Extra

- Tell people what they can do to learn more rust

- Make the video with a bit of coding, a bit of slides and finally images

- Install Rust on Ubuntu, Windows and Mac

## Println is a macro?

![Image](/rust_practice/macro.png)

## Ejemplos

```
let x: i32 = 10;

let activo: bool = true;

if x > 0 {
    println!("x es positivo");
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

let resultado = sumar(5, 7);

struct Persona {
    nombre: String,
    edad: u32,
}

let persona = Persona {
    nombre: String::from("Carlos"),
    edad: 28,
}

enum Color {
    Rojo,
    Verde,
    Azul,
}

fn describir_color(color: Color) {
    match color {
        Color::Rojo => println!("Es rojo"),
        Color::Verde => println!("Es verde"),
        Color::Azul => println!("Es azul"),
    }
}

fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("No se puede dividir por cero"))
    } else {
        Ok(a / b)
    }
}
```

## Iterators

![loop](/rust_practice/)
