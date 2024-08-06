/*
- fn main(): La función principal del programa.

- es_primo(n: u32) -> bool: Esta función determina si un número n es primo.
La función realiza varias verificaciones: si el número es menor o igual a 1, si es divisible por
2 o 3, y luego utiliza un bucle para verificar los posibles factores restantes.

- for i in 2..=limite: Un bucle que itera sobre los números desde 2 hasta el límite dado,
comprobando si cada número es primo y, si lo es, lo imprime.
*/


//use std::io;
//use std::io::Write;
use std::io::{self,Write};

fn main() {
    /*
    // Aqui agregamos toda la funcionalidad utilizando la funcion es_primo
    let numero: i32 = 29;
    if es_primo(numero) {
        println!("{} es un numero primo", numero);
    } else {
        println!("{} no es un numero primo", numero);
    }

    let limite = 50; // solo nos motrara un total de 50 numero primos
    println!("Numeros primos hasta {}", limite);
    for i in 2..=limite {
        if es_primo(i) {
            print!("{}", i);
        }
    }
    //println!()

     */
    // Multiple ingreso de valores, para saber si son primos o no
    let mut input = String::new();

    println!("Introduce uno o mas numeros separados por espacios");

    // Aseguramos que el mensaje se imprima antes de leer la entrada
    io::stdout().flush().expect("Error al intentar limpiar el bufer");

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la linea");

    let numeros: Vec<i32> = input
        .trim() // Eliminamos los espacios en blanco y el salto de línea al final
        .split_whitespace()// Dividimos la entrada en palabras separadas por espacios
        .filter_map(|s| s.parse().ok())// Convertimos cada palabra en un número u32, ignorando las que no se pueden convertir
        .collect(); // Recolectamos los números en un vector

    println!("Numeros introducidos");
    for numero in &numeros {
        if es_primo(*numero){
            println!("{} es un numero primo", numero);
        }else{
            println!("{} no es un numero primo", numero);
        }
    }
}

// funcion para validar si nuestro numero es primo o no
//
fn es_primo(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i:i32= 5;
    while i*i <=n{
        if n % i == 0 || n % (i+2)==0{
            return false;
        }
        i *=6;
    }
true}
