use std::collections::HashSet;

/// Calcula el factorial de un número.
///
/// # Argumentos
///
/// * `n` - El número para calcular su factorial.
///
/// # Retorna
///
/// El factorial de `n`.
///
/// # Ejemplo
///
/// ```
/// let resultado = factorial(5);
/// assert_eq!(resultado, 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Calcula el número de combinaciones de n elementos tomados r a la vez.
///
/// # Argumentos
///
/// * `n` - El número total de elementos.
/// * `r` - El número de elementos a seleccionar.
///
/// # Retorna
///
/// El número de combinaciones posibles.
///
/// # Ejemplo
///
/// ```
/// let resultado = combinacion(5, 3);
/// assert_eq!(resultado, 10);
/// ```
pub fn combinacion(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

/// Calcula el número de permutaciones de n elementos tomados r a la vez.
///
/// # Argumentos
///
/// * `n` - El número total de elementos.
/// * `r` - El número de elementos a permutar.
///
/// # Retorna
///
/// El número de permutaciones posibles.
///
/// # Ejemplo
///
/// ```
/// let resultado = permutacion(5, 3);
/// assert_eq!(resultado, 60);
/// ```
pub fn permutacion(n: u64, r: u64) -> u64 {
    factorial(n) / factorial(n - r)
}

/// Aplica el principio del palomar para calcular el mínimo de elementos en al menos un conjunto.
///
/// # Argumentos
///
/// * `palomas` - El número total de elementos (palomas).
/// * `palomares` - El número de conjuntos (palomares).
///
/// # Retorna
///
/// El mínimo número de elementos que deben estar en al menos un conjunto.
///
/// # Ejemplo
///
/// ```
/// let resultado = principio_palomar(10, 3);
/// assert_eq!(resultado, 4);
/// ```
pub fn principio_palomar(palomas: usize, palomares: usize) -> usize {
    (palomas + palomares - 1) / palomares
}

/// Aplica el principio de inclusión-exclusión para dos conjuntos.
///
/// # Argumentos
///
/// * `conjunto_a` - El primer conjunto.
/// * `conjunto_b` - El segundo conjunto.
///
/// # Retorna
///
/// El número de elementos únicos en la unión de ambos conjuntos.
///
/// # Ejemplo
///
/// ```
/// let conjunto_a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
/// let conjunto_b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();
/// let resultado = inclusion_exclusion(&conjunto_a, &conjunto_b);
/// assert_eq!(resultado, 6);
/// ```
pub fn inclusion_exclusion(conjunto_a: &HashSet<i32>, conjunto_b: &HashSet<i32>) -> usize {
    let union = conjunto_a.union(conjunto_b).count();
    let interseccion = conjunto_a.intersection(conjunto_b).count();
    union - interseccion
}
