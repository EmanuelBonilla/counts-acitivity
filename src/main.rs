mod counts;

use std::collections::HashSet;

use counts::*;

fn main() {
    // Ejemplo de combinación
    let n = 5;
    let r = 3;
    println!(
        "Combinación de {} elementos tomados de {} en {} = {}",
        n,
        n,
        r,
        combinacion(n, r)
    );

    // Ejemplo de permutación
    println!(
        "Permutación de {} elementos tomados de {} en {} = {}",
        n,
        n,
        r,
        permutacion(n, r)
    );

    // Ejemplo del principio del palomar
    let palomas = 10;
    let palomares = 3;
    println!(
        "Mínimo de palomas en al menos un palomar: {}",
        principio_palomar(palomas, palomares)
    );

    // Ejemplo del principio de inclusión-exclusión
    let conjunto_a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let conjunto_b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();
    println!(
        "Elementos únicos en A y B: {}",
        inclusion_exclusion(&conjunto_a, &conjunto_b)
    )
}
