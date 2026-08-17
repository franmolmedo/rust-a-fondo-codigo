fn plus_one(x: i32) -> i32 {
    x + 1;
    // error[E0308]: mismatched types — se esperaba `i32`,
    // pero el `;` hace que el bloque produzca `()`
}
