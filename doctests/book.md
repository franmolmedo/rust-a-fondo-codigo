# Corpus verificable de *Rust a fondo*

Este documento se genera desde `docs/` y alimenta `cargo test --doc`.

## 0.Introducción

### `ch00-b001` — Preparación del entorno

Source: `0.Introducción.md:184` · mode: `reference`

```console
rustc --version
cargo --version
rustup component add rustfmt clippy
cargo new rust-lab
cd rust-lab
cargo run
```

### `ch00-b002` — Preparación del entorno

Source: `0.Introducción.md:195` · mode: `reference`

```console
cargo check
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
cargo doc --no-deps --open
```

## 01.Filosofía-y-modelo-mental

### `ch01-b001` — 1.2 Seguridad sin recolector de basura

Source: `01.Filosofía-y-modelo-mental.md:41` · mode: `run`

```rust
fn main() {
    let text = String::from("hola");
    println!("{text}");
} // `text` se destruye aquí
```

### `ch01-b002` — 1.2 Seguridad sin recolector de basura

Source: `01.Filosofía-y-modelo-mental.md:50` · mode: `run`

```rust
struct ConnectionGuard(&'static str);

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        println!("cerrando {}", self.0);
    }
}

fn main() {
    let _database = ConnectionGuard("base de datos");
    {
        let _cache = ConnectionGuard("cache");
        println!("trabajando con ambas");
    } // la cache se cierra exactamente aquí
    println!("solo queda la base de datos");
} // la base de datos se cierra al final
```

### `ch01-b003` — 1.3 Safe Rust y Unsafe Rust

Source: `01.Filosofía-y-modelo-mental.md:91` · mode: `run`

```rust
fn first_byte(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        None
    } else {
        // SAFETY: acabamos de comprobar que el índice 0 existe.
        Some(unsafe { *bytes.get_unchecked(0) })
    }
}

fn main() {
    assert_eq!(first_byte(b"rust"), Some(b'r'));
    assert_eq!(first_byte(b""), None);
}
```

### `ch01-b004` — 1.4 Abstracciones de coste cero

Source: `01.Filosofía-y-modelo-mental.md:117` · mode: `run`

```rust
fn main() {
    // El compilador usa la representación imposible (puntero nulo)
    // para codificar None: la seguridad no añade ni un byte.
    assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
    assert_eq!(size_of::<Option<Box<u64>>>(), size_of::<Box<u64>>());
    println!("Option sobre punteros: mismo tamaño que el puntero desnudo");
}
```

### `ch01-b005` — 1.6 El compilador como colaborador

Source: `01.Filosofía-y-modelo-mental.md:167` · mode: `compile_fail`

```rust,compile_fail
fn greet(name: String) {
    println!("hola, {name}");
}

fn main() {
    let name = String::from("Ada");
    greet(name); // move: `name` se transfiere a greet
    println!("{name}");
    // error[E0382]: borrow of moved value: `name`
}
```

### `ch01-b006` — 1.6 El compilador como colaborador

Source: `01.Filosofía-y-modelo-mental.md:182` · mode: `run`

```rust
fn greet(name: &str) {
    println!("hola, {name}");
}

fn main() {
    let name = String::from("Ada");
    greet(&name);
    println!("{name}"); // sigue disponible: nadie lo consumió
}
```

### `ch01-b007` — 1.7 El método del curso: tres preguntas

Source: `01.Filosofía-y-modelo-mental.md:206` · mode: `run`

```rust
struct User {
    active: bool,
}

fn first_active(users: &[User]) -> Option<&User> {
    users.iter().find(|user| user.active)
}
```

## 02.Expresiones-bindings-y-control-de-flujo

### `ch02-b001` — 2.1 Rust es principalmente un lenguaje de expresiones

Source: `02.Expresiones-bindings-y-control-de-flujo.md:11` · mode: `illustrative`

```rust,ignore
5 + 6
foo(x)
if condition { 1 } else { 2 }
match value {
    Some(x) => x,
    None => 0,
}
{
    let x = 3;
    x + 1
}
```

### `ch02-b002` — 2.1 Rust es principalmente un lenguaje de expresiones

Source: `02.Expresiones-bindings-y-control-de-flujo.md:27` · mode: `illustrative`

```rust,ignore
let x = 3;
fn calculate() {}
struct User { name: String }
```

### `ch02-b003` — 2.1 Rust es principalmente un lenguaje de expresiones

Source: `02.Expresiones-bindings-y-control-de-flujo.md:35` · mode: `run`

```rust
fn main() {
    let score = 70;

    let label;
    if score >= 50 {
        label = "aprobado";
    } else {
        label = "suspenso";
    }
    println!("{label}");
}
```

### `ch02-b004` — 2.2 El punto y coma cambia el valor

Source: `02.Expresiones-bindings-y-control-de-flujo.md:55` · mode: `run`

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

### `ch02-b005` — 2.2 El punto y coma cambia el valor

Source: `02.Expresiones-bindings-y-control-de-flujo.md:63` · mode: `compile_fail`

```rust,compile_fail
fn plus_one(x: i32) -> i32 {
    x + 1;
    // error[E0308]: mismatched types — se esperaba `i32`,
    // pero el `;` hace que el bloque produzca `()`
}
```

### `ch02-b006` — 2.3 Los bloques producen valores

Source: `02.Expresiones-bindings-y-control-de-flujo.md:84` · mode: `run`

```rust
fn main() {
    let value = {
        let base = 10;
        base * 2
    };

    assert_eq!(value, 20);
}
```

### `ch02-b007` — 2.3 Los bloques producen valores

Source: `02.Expresiones-bindings-y-control-de-flujo.md:97` · mode: `run`

```rust
fn main() {
    let summary = {
        let raw = String::from("  Ada Lovelace  ");
        let clean = raw.trim();
        format!("nombre: {clean}")
    }; // `clean` termina y `raw` se destruye; `summary` posee otro String

    assert_eq!(summary, "nombre: Ada Lovelace");
}
```

### `ch02-b008` — 2.4 `if` como expresión

Source: `02.Expresiones-bindings-y-control-de-flujo.md:115` · mode: `run`

```rust
fn main() {
    let score = 70;
    let label = if score >= 50 {
        "aprobado"
    } else {
        "suspenso"
    };
    assert_eq!(label, "aprobado");
}
```

### `ch02-b009` — 2.4 `if` como expresión

Source: `02.Expresiones-bindings-y-control-de-flujo.md:129` · mode: `run`

```rust
fn main() {
    let condition = true;
    let number = if condition { 1 } else { 2 };
    assert_eq!(number, 1);
}
```

### `ch02-b010` — 2.4 `if` como expresión

Source: `02.Expresiones-bindings-y-control-de-flujo.md:139` · mode: `compile_fail`

```rust,compile_fail
let condition = true;
let value = if condition { 1 } else { "dos" };
// error[E0308]: `if` and `else` have incompatible types
```

### `ch02-b011` — 2.4 `if` como expresión

Source: `02.Expresiones-bindings-y-control-de-flujo.md:149` · mode: `run`

```rust
fn main() {
    let condition = true;
    if condition {
        println!("se cumple");
    }
}
```

### `ch02-b012` — 2.5 `match` como expresión

Source: `02.Expresiones-bindings-y-control-de-flujo.md:164` · mode: `run`

```rust
fn main() {
    let status_code = 404;
    let description = match status_code {
        200 => "ok",
        404 => "not found",
        _ => "other",
    };
    assert_eq!(description, "not found");
}
```

### `ch02-b013` — 2.6 `loop` puede devolver valores

Source: `02.Expresiones-bindings-y-control-de-flujo.md:182` · mode: `compile_only`

```rust,no_run
fn ready() -> bool {
    true
}

fn main() {
    let result = loop {
        if ready() {
            break 42;
        }
    };
    assert_eq!(result, 42);
}
```

### `ch02-b014` — 2.6 `loop` puede devolver valores

Source: `02.Expresiones-bindings-y-control-de-flujo.md:201` · mode: `compile_only`

```rust,no_run
fn main() {
    let mut attempts = 0;

    let value = loop {
        attempts += 1;
        if attempts == 3 {
            break attempts * 10;
        }
    };

    assert_eq!(value, 30);
}
```

### `ch02-b015` — 2.7 Bindings e inmutabilidad por defecto

Source: `02.Expresiones-bindings-y-control-de-flujo.md:222` · mode: `run`

```rust
let x = 5;
```

### `ch02-b016` — 2.7 Bindings e inmutabilidad por defecto

Source: `02.Expresiones-bindings-y-control-de-flujo.md:228` · mode: `run`

```rust
let mut x = 5;
x = 6;
```

### `ch02-b017` — 2.8 Shadowing no es mutación

Source: `02.Expresiones-bindings-y-control-de-flujo.md:241` · mode: `run`

```rust
let input = " 42 ";
let input = input.trim();
let input = input.parse::<u32>().expect("número válido");
```

### `ch02-b018` — 2.8 Shadowing no es mutación

Source: `02.Expresiones-bindings-y-control-de-flujo.md:251` · mode: `run`

```rust
let spaces = "   ";
let spaces = spaces.len();
```

### `ch02-b019` — 2.9 `return` y salida temprana

Source: `02.Expresiones-bindings-y-control-de-flujo.md:264` · mode: `run`

```rust
fn classify_number(x: i32) -> &'static str {
    if x < 0 { "negativo" } else { "no negativo" }
}
```

### `ch02-b020` — 2.9 `return` y salida temprana

Source: `02.Expresiones-bindings-y-control-de-flujo.md:272` · mode: `run`

```rust
fn checked_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    a.checked_div(b)
}
```

### `ch02-b021` — 2.10 Place expressions y value expressions

Source: `02.Expresiones-bindings-y-control-de-flujo.md:288` · mode: `run`

```rust
struct User {
    active: bool,
}

fn main() {
    let mut user = User { active: false };
    user.active = true;
    assert!(user.active);
}
```

### `ch02-b022` — 2.11 Primer puente hacia ownership

Source: `02.Expresiones-bindings-y-control-de-flujo.md:308` · mode: `run`

```rust
let first = String::from("hola");
let second = first;
```

### `ch02-b023` — Ejercicio 1 — valor de un bloque

Source: `02.Expresiones-bindings-y-control-de-flujo.md:348` · mode: `run`

```rust
fn main() {
    let x = {
        let a = 10;
        a * 2
    };

    println!("{x}");
}
```

### `ch02-b024` — Ejercicio 2 — el punto y coma

Source: `02.Expresiones-bindings-y-control-de-flujo.md:363` · mode: `compile_fail`

```rust,compile_fail
fn calculate() -> i32 {
    let x = 3;
    x + 1;
}
```

### `ch02-b025` — Ejercicio 3 — shadowing

Source: `02.Expresiones-bindings-y-control-de-flujo.md:374` · mode: `run`

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 10;
    println!("{x}");
}
```

### `ch02-b026` — Ejercicio 4 — asignación y move

Source: `02.Expresiones-bindings-y-control-de-flujo.md:387` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let text = String::from("abc");
    let other = text;
    println!("{text}");
}
```

### `ch02-b027` — Ejercicio 5 — eliminar estado intermedio

Source: `02.Expresiones-bindings-y-control-de-flujo.md:399` · mode: `run`

```rust
fn label_for(score: u8) -> &'static str {
    let mut label = "suspenso";
    if score >= 50 {
        label = "aprobado";
    }
    label
}

fn main() {
    assert_eq!(label_for(49), "suspenso");
    assert_eq!(label_for(50), "aprobado");
}
```

### `ch02-b028` — Ejercicio 6 — `loop` como productor de un valor

Source: `02.Expresiones-bindings-y-control-de-flujo.md:418` · mode: `run`

```rust
fn first_even(values: &[i32]) -> Option<i32> {
    let mut result = None;
    let mut index = 0;

    while index < values.len() {
        if values[index] % 2 == 0 {
            result = Some(values[index]);
            break;
        }
        index += 1;
    }

    result
}

fn main() {
    assert_eq!(first_even(&[1, 7, 4, 8]), Some(4));
    assert_eq!(first_even(&[1, 7]), None);
}
```

## 03.Tipos-fundamentales-y-compuestos

### `ch03-b001` — 3.1 Tipado estático e inferencia

Source: `03.Tipos-fundamentales-y-compuestos.md:11` · mode: `run`

```rust
let count = 10;          // normalmente i32
let total: u64 = 10;
let mask = 0xff_u8;
```

### `ch03-b002` — 3.1 Tipado estático e inferencia

Source: `03.Tipos-fundamentales-y-compuestos.md:19` · mode: `run`

```rust
let number = "42"
    .parse::<u32>()
    .expect("debe ser un u32");
```

### `ch03-b003` — 3.1 Tipado estático e inferencia

Source: `03.Tipos-fundamentales-y-compuestos.md:27` · mode: `run`

```rust
let number: u32 = "42".parse().expect("debe ser un u32");
```

### `ch03-b004` — 3.1 Tipado estático e inferencia

Source: `03.Tipos-fundamentales-y-compuestos.md:37` · mode: `run`

```rust
let user_id: u64 = 42;
let price_cents: i64 = 1_999;
let retry_count: u8 = 3;
```

### `ch03-b005` — 3.2 Enteros

Source: `03.Tipos-fundamentales-y-compuestos.md:56` · mode: `run`

```rust
fn main() {
    let values = [10, 20, 30];
    let length: usize = values.len();
    assert_eq!(length, 3);
}
```

### `ch03-b006` — 3.2 Enteros

Source: `03.Tipos-fundamentales-y-compuestos.md:68` · mode: `run`

```rust
struct UserId(u64);
```

### `ch03-b007` — 3.3 Literales numéricos

Source: `03.Tipos-fundamentales-y-compuestos.md:78` · mode: `run`

```rust
let decimal = 98_222;
let hexadecimal = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A';
```

### `ch03-b008` — 3.4 Overflow explícito

Source: `03.Tipos-fundamentales-y-compuestos.md:94` · mode: `run`

```rust
let maximum: u8 = 255;
```

### `ch03-b009` — 3.4 Overflow explícito

Source: `03.Tipos-fundamentales-y-compuestos.md:100` · mode: `run`

```rust
fn main() {
    let x: u8 = 255;

    assert_eq!(x.wrapping_add(1), 0);
    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.saturating_add(1), 255);
    assert_eq!(x.overflowing_add(1), (0, true));
}
```

### `ch03-b010` — 3.4 Overflow explícito

Source: `03.Tipos-fundamentales-y-compuestos.md:120` · mode: `run`

```rust
fn add_stock(current: u32, incoming: u32) -> Option<u32> {
    current.checked_add(incoming)
}
```

### `ch03-b011` — 3.5 Conversiones numéricas

Source: `03.Tipos-fundamentales-y-compuestos.md:134` · mode: `compile_fail`

```rust,compile_fail
let a: u32 = 10;
let b: u64 = 20;
let c = a + b;
// error[E0277]: cannot add `u64` to `u32`
```

### `ch03-b012` — 3.5 Conversiones numéricas

Source: `03.Tipos-fundamentales-y-compuestos.md:143` · mode: `run`

```rust
fn main() {
    let a: u32 = 10;
    let b: u64 = 20;
    let c = u64::from(a) + b;
    assert_eq!(c, 30);
}
```

### `ch03-b013` — 3.5 Conversiones numéricas

Source: `03.Tipos-fundamentales-y-compuestos.md:154` · mode: `run`

```rust
use std::convert::TryFrom;

let big: u64 = 300;
let small = u8::try_from(big);
assert!(small.is_err());
```

### `ch03-b014` — 3.6 Coma flotante

Source: `03.Tipos-fundamentales-y-compuestos.md:174` · mode: `run`

```rust
let wide = 2.0;          // f64
let compact: f32 = 3.0;
```

### `ch03-b015` — 3.6 Coma flotante

Source: `03.Tipos-fundamentales-y-compuestos.md:183` · mode: `run`

```rust
struct MoneyCents(i64);
```

### `ch03-b016` — 3.6 Coma flotante

Source: `03.Tipos-fundamentales-y-compuestos.md:189` · mode: `run`

```rust
fn main() {
    let result = 0.1_f64 + 0.2;
    assert!((result - 0.3).abs() < 1e-12);
}
```

### `ch03-b017` — 3.7 `bool`

Source: `03.Tipos-fundamentales-y-compuestos.md:202` · mode: `run`

```rust
let active = true;

if active {
    println!("activo");
}
```

### `ch03-b018` — 3.7 `bool`

Source: `03.Tipos-fundamentales-y-compuestos.md:212` · mode: `run`

```rust
let x = 1;
if x != 0 {
    println!("no cero");
}
```

### `ch03-b019` — 3.8 `char` y Unicode

Source: `03.Tipos-fundamentales-y-compuestos.md:225` · mode: `run`

```rust
let letter = 'a';
let crab = '🦀';
```

### `ch03-b020` — 3.8 `char` y Unicode

Source: `03.Tipos-fundamentales-y-compuestos.md:232` · mode: `run`

```rust
fn main() {
    let composed = "é";
    let decomposed = "e\u{301}";

    assert_eq!(composed.len(), 2);
    assert_eq!(composed.chars().count(), 1);
    assert_eq!(decomposed.len(), 3);
    assert_eq!(decomposed.chars().count(), 2);
}
```

### `ch03-b021` — 3.9 Unit type `()`

Source: `03.Tipos-fundamentales-y-compuestos.md:250` · mode: `run`

```rust
fn log(message: &str) {
    println!("{message}");
}
```

### `ch03-b022` — 3.9 Unit type `()`

Source: `03.Tipos-fundamentales-y-compuestos.md:258` · mode: `run`

```rust
fn log(message: &str) -> () {
    println!("{message}");
}
```

### `ch03-b023` — 3.9 Unit type `()`

Source: `03.Tipos-fundamentales-y-compuestos.md:268` · mode: `run`

```rust
fn validate_nonempty(text: &str) -> Result<(), &'static str> {
    if text.is_empty() {
        Err("texto vacío")
    } else {
        Ok(())
    }
}

fn main() {
    assert_eq!(validate_nonempty("rust"), Ok(()));
    assert!(validate_nonempty("").is_err());
}
```

### `ch03-b024` — 3.10 Never type `!`

Source: `03.Tipos-fundamentales-y-compuestos.md:287` · mode: `compile_only`

```rust,no_run
fn fail(message: &str) -> ! {
    panic!("{message}")
}
```

### `ch03-b025` — 3.10 Never type `!`

Source: `03.Tipos-fundamentales-y-compuestos.md:295` · mode: `compile_only`

```rust,no_run
fn wait_forever() -> ! {
    loop {
        std::hint::spin_loop();
    }
}
```

### `ch03-b026` — 3.10 Never type `!`

Source: `03.Tipos-fundamentales-y-compuestos.md:305` · mode: `compile_only`

```rust,no_run
let value: i32 = match Some(10) {
    Some(number) => number,
    None => panic!("sin valor"),
};
```

### `ch03-b027` — 3.11 Tuplas

Source: `03.Tipos-fundamentales-y-compuestos.md:320` · mode: `run`

```rust
let point: (i32, i32) = (10, 20);
let x = point.0;
let y = point.1;
```

### `ch03-b028` — 3.11 Tuplas

Source: `03.Tipos-fundamentales-y-compuestos.md:328` · mode: `run`

```rust
let rgb = (255, 128, 64);
let (red, green, blue) = rgb;
```

### `ch03-b029` — 3.11 Tuplas

Source: `03.Tipos-fundamentales-y-compuestos.md:337` · mode: `run`

```rust
fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    let minimum = *values.iter().min()?;
    let maximum = *values.iter().max()?;
    Some((minimum, maximum))
}
```

### `ch03-b030` — 3.12 Arrays `[T; N]`

Source: `03.Tipos-fundamentales-y-compuestos.md:351` · mode: `run`

```rust
let values: [i32; 4] = [1, 2, 3, 4];
let zeros = [0; 10];
```

### `ch03-b031` — 3.12 Arrays `[T; N]`

Source: `03.Tipos-fundamentales-y-compuestos.md:358` · mode: `run`

```rust
let matrix: [[i32; 3]; 3] = [
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
];
```

### `ch03-b032` — 3.12 Arrays `[T; N]`

Source: `03.Tipos-fundamentales-y-compuestos.md:370` · mode: `run`

```rust
fn main() {
    let values = [1, 2, 3, 4];
    match values.get(10) {
        Some(value) => println!("{value}"),
        None => println!("índice inexistente"),
    }
}
```

### `ch03-b033` — 3.13 Slices `[T]`, `&[T]` y `&mut [T]`

Source: `03.Tipos-fundamentales-y-compuestos.md:388` · mode: `run`

```rust
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let array = [1, 2, 3, 4];
    let vector = vec![1, 2, 3, 4];

    assert_eq!(sum(&array), 10);
    assert_eq!(sum(&vector), 10);
    assert_eq!(sum(&vector[1..3]), 5);
}
```

### `ch03-b034` — 3.13 Slices `[T]`, `&[T]` y `&mut [T]`

Source: `03.Tipos-fundamentales-y-compuestos.md:405` · mode: `run`

```rust
fn double_all(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}
```

### `ch03-b035` — 3.14 `str` y `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:423` · mode: `run`

```rust
let literal: &'static str = "hola";
```

### `ch03-b036` — 3.14 `str` y `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:431` · mode: `compile_fail`

```rust,compile_fail
let text = "hola";
let first = text[0];
// error[E0277]: the type `str` cannot be indexed by `{integer}`
```

### `ch03-b037` — 3.14 `str` y `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:439` · mode: `run`

```rust
fn main() {
    let text = "éclair";
    let first_char = text.chars().next();
    let first_byte = text.as_bytes().first();

    assert_eq!(first_char, Some('é'));
    assert_eq!(first_byte, Some(&0xc3));
}
```

### `ch03-b038` — 3.14 `str` y `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:454` · mode: `run`

```rust
let text = "é";
let whole = &text[0..2];
assert_eq!(whole, "é");
assert_eq!(text.get(0..1), None);
```

### `ch03-b039` — 3.15 `String`

Source: `03.Tipos-fundamentales-y-compuestos.md:467` · mode: `run`

```rust
let mut text = String::from("hola");
text.push('!');
text.push_str(" mundo");
```

### `ch03-b040` — 3.15 `String`

Source: `03.Tipos-fundamentales-y-compuestos.md:475` · mode: `reference`

```text
String = puntero + longitud + capacidad
```

### `ch03-b041` — 3.15 `String`

Source: `03.Tipos-fundamentales-y-compuestos.md:483` · mode: `run`

```rust
let first = String::from("hola");
let second = first;
// `first` ya no es usable
```

### `ch03-b042` — 3.15 `String`

Source: `03.Tipos-fundamentales-y-compuestos.md:491` · mode: `run`

```rust
let first = String::from("hola");
let second = first.clone();
```

### `ch03-b043` — 3.16 `String` frente a `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:502` · mode: `reference`

```text
String  -> texto owned, growable
&str    -> vista prestada de texto UTF-8
```

### `ch03-b044` — 3.16 `String` frente a `&str`

Source: `03.Tipos-fundamentales-y-compuestos.md:509` · mode: `run`

```rust
fn print_name(name: &str) {
    println!("{name}");
}

fn main() {
    let owned = String::from("Ada");
    print_name(&owned);
    print_name("Grace");
}
```

### `ch03-b045` — 3.17 `Vec<T>`

Source: `03.Tipos-fundamentales-y-compuestos.md:535` · mode: `run`

```rust
let mut values = Vec::new();
values.push(1);
values.push(2);
```

### `ch03-b046` — 3.17 `Vec<T>`

Source: `03.Tipos-fundamentales-y-compuestos.md:543` · mode: `run`

```rust
let values = vec![1, 2, 3];
```

### `ch03-b047` — 3.17 `Vec<T>`

Source: `03.Tipos-fundamentales-y-compuestos.md:549` · mode: `reference`

```text
Vec<T> = puntero + longitud + capacidad
```

### `ch03-b048` — 3.17 `Vec<T>`

Source: `03.Tipos-fundamentales-y-compuestos.md:557` · mode: `run`

```rust
fn main() {
    let mut values = Vec::with_capacity(10);
    values.push(1);
    values.push(2);

    assert_eq!(values.len(), 2);
    assert!(values.capacity() >= 10);
}
```

### `ch03-b049` — 3.18 Realocación e invalidación de referencias

Source: `03.Tipos-fundamentales-y-compuestos.md:574` · mode: `compile_fail`

```rust,compile_fail
let mut values = vec![1, 2, 3];
let first = &values[0];
values.push(4);
// error[E0502]: cannot borrow `values` as mutable
// because it is also borrowed as immutable
println!("{first}");
```

### `ch03-b050` — 3.18 Realocación e invalidación de referencias

Source: `03.Tipos-fundamentales-y-compuestos.md:587` · mode: `run`

```rust
fn main() {
    let mut values = vec![1, 2, 3];
    println!("{}", values[0]);
    values.push(4);
    assert_eq!(values, [1, 2, 3, 4]);
}
```

### `ch03-b051` — 3.18 Realocación e invalidación de referencias

Source: `03.Tipos-fundamentales-y-compuestos.md:598` · mode: `run`

```rust
fn main() {
    let mut values = vec![1, 2, 3];
    let first = values[0];
    values.push(4);
    println!("{first}");
}
```

### `ch03-b052` — 3.19 `Vec<T>` frente a `&[T]`

Source: `03.Tipos-fundamentales-y-compuestos.md:613` · mode: `reference`

```text
Vec<T>  -> buffer owned y growable
&[T]    -> vista prestada de elementos contiguos
```

### `ch03-b053` — 3.19 `Vec<T>` frente a `&[T]`

Source: `03.Tipos-fundamentales-y-compuestos.md:620` · mode: `run`

```rust
fn total(values: &[i32]) -> i32 {
    values.iter().sum()
}
```

### `ch03-b054` — 3.20 Tipos `Copy` y compuestos

Source: `03.Tipos-fundamentales-y-compuestos.md:646` · mode: `run`

```rust
let x = 10;
let y = x;
println!("{x} {y}");
```

### `ch03-b055` — 3.20 Tipos `Copy` y compuestos

Source: `03.Tipos-fundamentales-y-compuestos.md:654` · mode: `run`

```rust
let first = [1, 2, 3];
let second = first;
println!("{first:?} {second:?}");
```

### `ch03-b056` — 3.21 Tipos de tamaño dinámico

Source: `03.Tipos-fundamentales-y-compuestos.md:678` · mode: `run`

```rust
fn main() {
    let text: &str = "rust";
    let numbers: &[u16] = &[10, 20, 30];

    assert_eq!(std::mem::size_of_val(text), 4);
    assert_eq!(std::mem::size_of_val(numbers), 6);
}
```

### `ch03-b057` — 3.22 Alias frente a newtype

Source: `03.Tipos-fundamentales-y-compuestos.md:694` · mode: `run`

```rust
type UserId = u64;
```

### `ch03-b058` — 3.22 Alias frente a newtype

Source: `03.Tipos-fundamentales-y-compuestos.md:702` · mode: `run`

```rust
struct UserId(u64);
struct OrderId(u64);
```

### `ch03-b059` — 3.23 Checklist de diseño

Source: `03.Tipos-fundamentales-y-compuestos.md:723` · mode: `run`

```rust
struct UserId(u64);
struct Email(String);
struct MoneyCents(i64);
struct Quantity(u32);
```

## 04.Ownership

### `ch04-b001` — 4.1 Las tres reglas operativas

Source: `04.Ownership.md:17` · mode: `run`

```rust
fn main() {
    let text = String::from("hola");
    assert_eq!(text.len(), 4);
} // `text` se destruye aquí y libera su buffer
```

### `ch04-b002` — 4.2 Scope, destrucción y orden

Source: `04.Ownership.md:32` · mode: `run`

```rust
struct Tracer(&'static str);

impl Drop for Tracer {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _outer = Tracer("outer");

    {
        let _first = Tracer("first");
        let _second = Tracer("second");
        println!("inside");
    }

    println!("outside");
}
```

### `ch04-b003` — 4.2 Scope, destrucción y orden

Source: `04.Ownership.md:56` · mode: `reference`

```text
inside
drop second
drop first
outside
drop outer
```

### `ch04-b004` — 4.3 Move semantics

Source: `04.Ownership.md:74` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let first = String::from("hola");
    let second = first;

    println!("{second}");
    println!("{first}");
    // error[E0382]: borrow of moved value: `first`
}
```

### `ch04-b005` — 4.3 Move semantics

Source: `04.Ownership.md:89` · mode: `reference`

```text
String
├── puntero ──────► buffer UTF-8 en el heap
├── longitud
└── capacidad
```

### `ch04-b006` — 4.4 Valor movido y reinicialización

Source: `04.Ownership.md:108` · mode: `run`

```rust
fn main() {
    let mut text = String::from("abc");
    let other = text;

    text = String::from("xyz");

    assert_eq!(text, "xyz");
    assert_eq!(other, "abc");
}
```

### `ch04-b007` — 4.5 Pasar argumentos por valor puede consumir

Source: `04.Ownership.md:128` · mode: `run`

```rust
fn consume(text: String) -> usize {
    text.len()
}

fn main() {
    let name = String::from("Ada");
    assert_eq!(consume(name), 3);
}
```

### `ch04-b008` — 4.5 Pasar argumentos por valor puede consumir

Source: `04.Ownership.md:143` · mode: `compile_fail`

```rust,compile_fail
fn consume(text: String) {
    println!("{text}");
}

fn main() {
    let name = String::from("Ada");
    consume(name);
    println!("{name}");
    // error[E0382]: borrow of moved value: `name`
}
```

### `ch04-b009` — 4.6 Retornar transfiere ownership

Source: `04.Ownership.md:170` · mode: `run`

```rust
fn make_name() -> String {
    String::from("Grace")
}

fn identity(text: String) -> String {
    text
}

fn main() {
    let name = make_name();
    let same_name = identity(name);
    assert_eq!(same_name, "Grace");
}
```

### `ch04-b010` — 4.6 Retornar transfiere ownership

Source: `04.Ownership.md:188` · mode: `reference`

```text
make_name → retorno → name → parámetro text → retorno → same_name
```

### `ch04-b011` — 4.7 El patrón torpe sin borrowing

Source: `04.Ownership.md:198` · mode: `run`

```rust
fn length_owned(text: String) -> (String, usize) {
    let length = text.len();
    (text, length)
}

fn main() {
    let text = String::from("rust");
    let (text, length) = length_owned(text);
    assert_eq!(length, 4);
    assert_eq!(text, "rust");
}
```

### `ch04-b012` — 4.7 El patrón torpe sin borrowing

Source: `04.Ownership.md:214` · mode: `run`

```rust
fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let text = String::from("rust");
    assert_eq!(length(&text), 4);
    assert_eq!(text, "rust");
}
```

### `ch04-b013` — 4.8 `Copy`

Source: `04.Ownership.md:232` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let first = Point { x: 3, y: 4 };
    let second = first;

    assert_eq!(first, second);
}
```

### `ch04-b014` — 4.8 `Copy`

Source: `04.Ownership.md:253` · mode: `compile_fail`

```rust,compile_fail
#[derive(Clone, Copy)]
struct Label(String);

fn main() {}
// error[E0204]: the trait `Copy` cannot be implemented for this type
```

### `ch04-b015` — 4.9 `Clone`

Source: `04.Ownership.md:267` · mode: `run`

```rust
fn main() {
    let first = String::from("hola");
    let mut second = first.clone();
    second.push('!');

    assert_eq!(first, "hola");
    assert_eq!(second, "hola!");
}
```

### `ch04-b016` — 4.10 `Drop` y liberación anticipada

Source: `04.Ownership.md:290` · mode: `run`

```rust
struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("guard liberado");
    }
}

fn acquire_lock() -> Guard {
    Guard
}

fn do_protected_work() {}
fn do_unlocked_work() {}

fn main() {
    let guard = acquire_lock();
    do_protected_work();
    drop(guard);
    do_unlocked_work();
}
```

### `ch04-b017` — 4.10 `Drop` y liberación anticipada

Source: `04.Ownership.md:320` · mode: `compile_fail`

```rust,compile_fail
struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {}
}

fn main() {
    let guard = Guard;
    guard.drop();
    // error[E0040]: explicit use of destructor method
}
```

### `ch04-b018` — 4.11 Moves parciales

Source: `04.Ownership.md:342` · mode: `run`

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Ada"),
        age: 36,
    };

    let name = person.name;
    assert_eq!(name, "Ada");
    assert_eq!(person.age, 36);
}
```

### `ch04-b019` — 4.11 Moves parciales

Source: `04.Ownership.md:366` · mode: `compile_fail`

```rust,compile_fail
struct Ticket {
    id: String,
}

impl Drop for Ticket {
    fn drop(&mut self) {
        println!("cerrando {}", self.id);
    }
}

fn main() {
    let ticket = Ticket {
        id: String::from("T-42"),
    };
    let id = ticket.id;
    println!("{id}");
    // error[E0509]: cannot move out of type `Ticket`, which implements `Drop`
}
```

### `ch04-b020` — 4.11 Moves parciales

Source: `04.Ownership.md:389` · mode: `run`

```rust
struct Ticket {
    id: String,
}

impl Drop for Ticket {
    fn drop(&mut self) {}
}

fn main() {
    let mut ticket = Ticket {
        id: String::from("T-42"),
    };

    let id = std::mem::take(&mut ticket.id);
    assert_eq!(id, "T-42");
    assert!(ticket.id.is_empty());
}
```

### `ch04-b021` — 4.12 Métodos como contratos de ownership

Source: `04.Ownership.md:415` · mode: `run`

```rust
struct Counter {
    value: u32,
}

impl Counter {
    fn value(&self) -> u32 {
        self.value
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn finish(self) -> u32 {
        self.value
    }
}

fn main() {
    let mut counter = Counter { value: 1 };
    assert_eq!(counter.value(), 1);

    counter.increment();
    assert_eq!(counter.value(), 2);

    let final_value = counter.finish();
    assert_eq!(final_value, 2);
}
```

### `ch04-b022` — 4.13 Builders que consumen `self`

Source: `04.Ownership.md:458` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Config {
    debug: bool,
    port: u16,
}

struct ConfigBuilder {
    debug: bool,
    port: u16,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            debug: false,
            port: 8080,
        }
    }

    fn debug(mut self, value: bool) -> Self {
        self.debug = value;
        self
    }

    fn port(mut self, value: u16) -> Self {
        self.port = value;
        self
    }

    fn build(self) -> Config {
        Config {
            debug: self.debug,
            port: self.port,
        }
    }
}

fn main() {
    let config = ConfigBuilder::new()
        .debug(true)
        .port(3000)
        .build();

    assert_eq!(
        config,
        Config {
            debug: true,
            port: 3000,
        }
    );
}
```

### `ch04-b023` — 4.14 Ownership compartido explícito

Source: `04.Ownership.md:520` · mode: `run`

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("data"));
    assert_eq!(Rc::strong_count(&shared), 1);

    let first = Rc::clone(&shared);
    {
        let second = Rc::clone(&shared);
        assert_eq!(Rc::strong_count(&shared), 3);
        assert_eq!(second.as_str(), "data");
    }

    assert_eq!(Rc::strong_count(&shared), 2);
    drop(first);
    assert_eq!(Rc::strong_count(&shared), 1);
}
```

### `ch04-b024` — 4.15 Ownership como contrato de dominio

Source: `04.Ownership.md:550` · mode: `run`

```rust
#[derive(Debug)]
struct DraftUser {
    name: String,
}

#[derive(Debug, PartialEq)]
struct RegisteredUser {
    name: String,
}

fn is_valid(user: &DraftUser) -> bool {
    !user.name.trim().is_empty()
}

fn normalize(user: &mut DraftUser) {
    user.name = user.name.trim().to_owned();
}

fn register(user: DraftUser) -> RegisteredUser {
    RegisteredUser { name: user.name }
}

fn main() {
    let mut draft = DraftUser {
        name: String::from("  Ada  "),
    };

    normalize(&mut draft);
    assert!(is_valid(&draft));

    let registered = register(draft);
    assert_eq!(
        registered,
        RegisteredUser {
            name: String::from("Ada"),
        }
    );
}
```

### `ch04-b025` — 4.16 Ownership y closures

Source: `04.Ownership.md:599` · mode: `run`

```rust
fn main() {
    let text = String::from("hola");
    let attempts = 3_u32;

    let describe = move || format!("{text}: intento {attempts}");

    assert_eq!(attempts, 3);
    assert_eq!(describe(), "hola: intento 3");
    assert_eq!(describe(), "hola: intento 3");
}
```

### `ch04-b026` — 4.16 Ownership y closures

Source: `04.Ownership.md:616` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let text = String::from("hola");
    let print = move || println!("{text}");

    println!("{text}");
    print();
    // error[E0382]: borrow of moved value: `text`
}
```

### `ch04-b027` — Ejercicio 7 — captura `move`

Source: `04.Ownership.md:708` · mode: `run`

```rust
fn main() {
    let label = String::from("retry");
    let attempts = 3_u32;
    let describe = move || format!("{label}: {attempts}");

    assert_eq!(attempts, 3);
    assert_eq!(describe(), "retry: 3");
}
```

## 05.Borrowing

### `ch05-b001` — 5.1 Ownership y borrowing responden preguntas distintas

Source: `05.Borrowing.md:13` · mode: `run`

```rust
fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let name = String::from("Ferris");

    assert_eq!(length(&name), 6);
    assert_eq!(name, "Ferris");
}
```

### `ch05-b002` — 5.1 Ownership y borrowing responden preguntas distintas

Source: `05.Borrowing.md:30` · mode: `reference`

```text
T       → poseer; se puede transferir o destruir
&T      → observar temporalmente
&mut T  → observar y modificar temporalmente en exclusiva
```

### `ch05-b003` — 5.2 Referencias compartidas: `&T`

Source: `05.Borrowing.md:42` · mode: `run`

```rust
fn initials(first: &str, last: &str) -> String {
    let first = first.chars().next().unwrap_or('?');
    let last = last.chars().next().unwrap_or('?');
    format!("{first}{last}")
}

fn main() {
    let name = String::from("Ada");
    let first_view = &name;
    let second_view = &name;

    assert_eq!(initials(first_view, second_view), "AA");
    assert_eq!(name, "Ada");
}
```

### `ch05-b004` — 5.2 Referencias compartidas: `&T`

Source: `05.Borrowing.md:65` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let names = vec![String::from("Ada")];
    let view = &names[0];

    let owned: String = *view;
    // error[E0507]: cannot move out through a shared reference

    println!("{owned}");
}
```

### `ch05-b005` — 5.3 Referencias mutables: `&mut T`

Source: `05.Borrowing.md:83` · mode: `run`

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut message = String::from("hola");
    add_exclamation(&mut message);
    add_exclamation(&mut message);

    assert_eq!(message, "hola!!");
}
```

### `ch05-b006` — 5.3 Referencias mutables: `&mut T`

Source: `05.Borrowing.md:99` · mode: `run`

```rust
fn main() {
    let mut count = 1;
    let reference = &mut count;

    *reference += 1;
    assert_eq!(*reference, 2);
}
```

### `ch05-b007` — 5.3 Referencias mutables: `&mut T`

Source: `05.Borrowing.md:115` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut message = String::from("hola");
    let edit = &mut message;

    println!("{message}");
    // error[E0502]: shared access conflicts with the mutable borrow

    edit.push('!');
}
```

### `ch05-b008` — 5.4 Aliasing XOR mutability

Source: `05.Borrowing.md:133` · mode: `reference`

```text
muchos &T             → lectura compartida
un &mut T             → lectura y escritura exclusivas
&T junto a &mut T     → no, si los préstamos se solapan
varios &mut T         → no, si alcanzan datos solapados
```

### `ch05-b009` — 5.5 Los préstamos se aplican a *places*

Source: `05.Borrowing.md:152` · mode: `run`

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 2, y: 3 };
    let x = &mut point.x;
    let y = &mut point.y;

    *x *= 10;
    *y *= 10;

    assert_eq!((point.x, point.y), (20, 30));
}
```

### `ch05-b010` — 5.5 Los préstamos se aplican a *places*

Source: `05.Borrowing.md:177` · mode: `compile_fail`

```rust,compile_fail
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 2, y: 3 };
    let x = &mut point.x;

    println!("{point:?}");
    // error[E0502]: the whole point cannot be read yet

    *x += 1;
}
```

### `ch05-b011` — 5.6 Non-lexical lifetimes y último uso

Source: `05.Borrowing.md:201` · mode: `run`

```rust
fn main() {
    let mut text = String::from("hola");

    let read = &text;
    assert_eq!(read.len(), 4);

    let write = &mut text;
    write.push('!');

    assert_eq!(text, "hola!");
}
```

### `ch05-b012` — 5.6 Non-lexical lifetimes y último uso

Source: `05.Borrowing.md:221` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut text = String::from("hola");
    let read = &text;
    let write = &mut text;
    // error[E0502]: shared and mutable borrows overlap

    write.push('!');
    println!("{read}");
}
```

### `ch05-b013` — 5.6 Non-lexical lifetimes y último uso

Source: `05.Borrowing.md:235` · mode: `run`

```rust
fn main() {
    let mut values = vec![2, 4, 6];

    let sum = {
        let view = &values;
        view.iter().sum::<i32>()
    };

    values.push(sum);
    assert_eq!(values, [2, 4, 6, 12]);
}
```

### `ch05-b014` — 5.7 Elegir vistas: `&str` frente a `&String`

Source: `05.Borrowing.md:255` · mode: `run`

```rust
fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

fn main() {
    let owned = String::from("   ");

    assert!(is_blank(&owned));
    assert!(is_blank(" \n "));
}
```

### `ch05-b015` — 5.7 Elegir vistas: `&str` frente a `&String`

Source: `05.Borrowing.md:272` · mode: `reference`

```text
&String   → normalmente &str
&Vec<T>   → normalmente &[T]
&PathBuf  → normalmente &Path
```

### `ch05-b016` — 5.8 Slices como préstamos de secuencias

Source: `05.Borrowing.md:284` · mode: `run`

```rust
fn contains_name(names: &[String], wanted: &str) -> bool {
    names.iter().any(|name| name == wanted)
}

fn main() {
    let vector = vec![String::from("Ada"), String::from("Grace")];
    let array = [String::from("Linus"), String::from("Margaret")];

    assert!(contains_name(&vector, "Grace"));
    assert!(contains_name(&array, "Linus"));
}
```

### `ch05-b017` — 5.8 Slices como préstamos de secuencias

Source: `05.Borrowing.md:300` · mode: `run`

```rust
fn zero_all(values: &mut [i32]) {
    for value in values {
        *value = 0;
    }
}

fn main() {
    let mut values = vec![1, 2, 3];
    zero_all(&mut values);
    assert_eq!(values, [0, 0, 0]);
}
```

### `ch05-b018` — 5.9 Reborrowing: prestar desde otro préstamo

Source: `05.Borrowing.md:322` · mode: `run`

```rust
fn append_mark(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("hola");
    let reference = &mut text;

    append_mark(reference);
    append_mark(reference);
    reference.make_ascii_uppercase();

    assert_eq!(text, "HOLA!!");
}
```

### `ch05-b019` — 5.9 Reborrowing: prestar desde otro préstamo

Source: `05.Borrowing.md:345` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut value = 10;
    let first = &mut value;
    let second = first;

    *second += 1;
    *first += 1;
    // error[E0382]: first was moved
}
```

### `ch05-b020` — 5.10 Dividir datos para demostrar no solapamiento

Source: `05.Borrowing.md:365` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut values = [10, 20, 30];
    let first = &mut values[0];
    let second = &mut values[1];
    // error[E0499]: indexed mutable borrows may overlap

    *first += 1;
    *second += 1;
}
```

### `ch05-b021` — 5.10 Dividir datos para demostrar no solapamiento

Source: `05.Borrowing.md:379` · mode: `run`

```rust
fn increment_neighbors(values: &mut [i32], middle: usize) {
    let (left, right) = values.split_at_mut(middle);
    left[middle - 1] += 1;
    right[0] += 1;
}

fn main() {
    let mut values = [10, 20, 30];
    increment_neighbors(&mut values, 1);

    assert_eq!(values, [11, 21, 30]);
}
```

### `ch05-b022` — 5.11 Two-phase borrows: ergonomía acotada

Source: `05.Borrowing.md:402` · mode: `run`

```rust
fn main() {
    let mut values = vec![10, 20, 30];
    values.push(values.len());

    assert_eq!(values, [10, 20, 30, 3]);
}
```

### `ch05-b023` — 5.11 Two-phase borrows: ergonomía acotada

Source: `05.Borrowing.md:417` · mode: `compile_fail`

```rust,compile_fail
fn push_length(values: &mut Vec<usize>, length: usize) {
    values.push(length);
}

fn main() {
    let mut values = vec![10, 20, 30];
    push_length(&mut values, values.len());
    // error[E0502]: explicit mutable and shared borrows overlap
}
```

### `ch05-b024` — 5.11 Two-phase borrows: ergonomía acotada

Source: `05.Borrowing.md:431` · mode: `run`

```rust
fn push_length(values: &mut Vec<usize>, length: usize) {
    values.push(length);
}

fn main() {
    let mut values = vec![10, 20, 30];
    let length = values.len();
    push_length(&mut values, length);

    assert_eq!(values, [10, 20, 30, 3]);
}
```

### `ch05-b025` — 5.12 Una referencia no mantiene vivo su referente

Source: `05.Borrowing.md:453` · mode: `compile_fail`

```rust,compile_fail
fn bad() -> &'static str {
    let text = String::from("hola");
    &text
    // error[E0515]: cannot return a reference to local data
}

fn main() {}
```

### `ch05-b026` — 5.12 Una referencia no mantiene vivo su referente

Source: `05.Borrowing.md:465` · mode: `run`

```rust
fn greeting() -> String {
    String::from("hola")
}

fn main() {
    let text = greeting();
    assert_eq!(text, "hola");
}
```

### `ch05-b027` — 5.12 Una referencia no mantiene vivo su referente

Source: `05.Borrowing.md:478` · mode: `run`

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust seguro");
    let word = first_word(&sentence);

    assert_eq!(word, "Rust");
}
```

### `ch05-b028` — 5.13 Borrowing y closures

Source: `05.Borrowing.md:497` · mode: `run`

```rust
fn main() {
    let name = String::from("Ada");
    let describe = || format!("Nombre: {name}");

    assert_eq!(describe(), "Nombre: Ada");
    assert_eq!(describe(), "Nombre: Ada");
    assert_eq!(name, "Ada");
}
```

### `ch05-b029` — 5.13 Borrowing y closures

Source: `05.Borrowing.md:510` · mode: `run`

```rust
fn main() {
    let mut count = 0;
    let mut increment = || count += 1;

    increment();
    increment();

    assert_eq!(count, 2);
}
```

### `ch05-b030` — 5.13 Borrowing y closures

Source: `05.Borrowing.md:526` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut count = 0;
    let mut increment = || count += 1;

    println!("{count}");
    // error[E0502]: count is still mutably borrowed by the closure

    increment();
}
```

### `ch05-b031` — 5.14 Borrowing y pattern matching

Source: `05.Borrowing.md:544` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn text_length(message: &Message) -> Option<usize> {
    match message {
        Message::Text(text) => Some(text.len()),
        Message::Quit => None,
    }
}

fn main() {
    let message = Message::Text(String::from("hola"));

    assert_eq!(text_length(&message), Some(4));
    assert_eq!(message, Message::Text(String::from("hola")));
}
```

### `ch05-b032` — 5.14 Borrowing y pattern matching

Source: `05.Borrowing.md:568` · mode: `compile_only`

```rust,no_run
enum Message {
    Text(String),
    Quit,
}

fn emphasize(message: &mut Message) {
    if let Message::Text(text) = message {
        text.push('!');
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));
    emphasize(&mut message);

    match message {
        Message::Text(text) => assert_eq!(text, "hola!"),
        Message::Quit => panic!("se esperaba texto"),
    }
}
```

### `ch05-b033` — 5.15 Interior mutability: una excepción encapsulada, no un atajo

Source: `05.Borrowing.md:597` · mode: `run`

```rust
use std::cell::Cell;

struct Counter {
    value: Cell<u32>,
}

impl Counter {
    fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }

    fn get(&self) -> u32 {
        self.value.get()
    }
}

fn main() {
    let counter = Counter {
        value: Cell::new(0),
    };

    counter.increment();
    assert_eq!(counter.get(), 1);
}
```

### `ch05-b034` — 5.16 Leer una firma como un contrato de capacidades

Source: `05.Borrowing.md:632` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Command {
    name: String,
    retries: u8,
}

struct User {
    name: String,
}

fn parse(input: &str) -> Command {
    Command {
        name: input.trim().to_owned(),
        retries: 0,
    }
}

fn enrich(command: &mut Command) {
    command.retries = 3;
}

fn enriched(mut command: Command) -> Command {
    command.retries = 3;
    command
}

fn user_name(user: &User) -> &str {
    &user.name
}

fn main() {
    let source = String::from(" build ");
    let mut command = parse(&source);
    enrich(&mut command);
    let command = enriched(command);

    let user = User {
        name: String::from("Ada"),
    };

    assert_eq!(source, " build ");
    assert_eq!(command, Command {
        name: String::from("build"),
        retries: 3,
    });
    assert_eq!(user_name(&user), "Ada");
}
```

### `ch05-b035` — 5.17 Cómo leer un diagnóstico del borrow checker

Source: `05.Borrowing.md:708` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let mut values = vec![String::from("uno")];
    let first = &values[0];

    values.push(String::from("dos"));
    // error[E0502]: the vector is still shared-borrowed

    println!("{first}");
}
```

### `ch05-b036` — 5.17 Cómo leer un diagnóstico del borrow checker

Source: `05.Borrowing.md:722` · mode: `run`

```rust
fn main() {
    let mut values = vec![String::from("uno")];
    let first_index = 0;

    values.push(String::from("dos"));
    let first = &values[first_index];

    assert_eq!(first, "uno");
}
```

## 06.Lifetimes

### `ch06-b001` — 6.1 Tres conceptos que no deben confundirse

Source: `06.Lifetimes.md:13` · mode: `reference`

```text
vida del dato       → tramo de ejecución en que el valor existe
región del préstamo → tramo en que una referencia puede utilizarse
anotación 'a        → relación genérica exigida entre referencias
```

### `ch06-b002` — 6.2 Una salida prestada debe proceder de algún lugar válido

Source: `06.Lifetimes.md:31` · mode: `run`

```rust
fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust seguro");
    let word = first_word(&sentence);

    assert_eq!(word, "Rust");
    assert_eq!(sentence, "Rust seguro");
}
```

### `ch06-b003` — 6.2 Una salida prestada debe proceder de algún lugar válido

Source: `06.Lifetimes.md:47` · mode: `reference`

```text
fn first_word<'a>(input: &'a str) -> &'a str
```

### `ch06-b004` — 6.3 Dos entradas hacen visible la relación

Source: `06.Lifetimes.md:57` · mode: `run`

```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let left = String::from("ferris");
    let right = String::from("rust");

    assert_eq!(longest(&left, &right), "ferris");
}
```

### `ch06-b005` — 6.4 El contrato limita el resultado aunque conozcamos una rama

Source: `06.Lifetimes.md:88` · mode: `compile_fail`

```rust,compile_fail
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() { first } else { second }
}

fn main() {
    let long = String::from("una cadena larga");
    let result;

    {
        let short = String::from("breve");
        result = longest(&long, &short);
    }

    println!("{result}");
    // error[E0597]: short does not live long enough
}
```

### `ch06-b006` — 6.5 Lifetimes independientes expresan contratos más precisos

Source: `06.Lifetimes.md:115` · mode: `run`

```rust
fn choose_first<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
    first
}

fn main() {
    let persistent = String::from("permanezco");
    let result;

    {
        let temporary = String::from("temporal");
        result = choose_first(&persistent, &temporary);
    }

    assert_eq!(result, "permanezco");
}
```

### `ch06-b007` — 6.6 Una anotación no repara una referencia colgante

Source: `06.Lifetimes.md:147` · mode: `compile_fail`

```rust,compile_fail
fn make<'a>() -> &'a str {
    let text = String::from("hola");
    &text
    // error[E0515]: cannot return a reference to local data
}

fn main() {}
```

### `ch06-b008` — 6.6 Una anotación no repara una referencia colgante

Source: `06.Lifetimes.md:161` · mode: `run`

```rust
fn make() -> String {
    String::from("hola")
}

fn main() {
    let text = make();
    assert_eq!(text, "hola");
}
```

### `ch06-b009` — 6.7 Reglas de elisión

Source: `06.Lifetimes.md:184` · mode: `run`

```rust
struct User {
    name: String,
}

impl User {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let user = User {
        name: String::from("Ada"),
    };

    assert_eq!(user.name(), "Ada");
}
```

### `ch06-b010` — 6.7 Reglas de elisión

Source: `06.Lifetimes.md:210` · mode: `compile_fail`

```rust,compile_fail
fn pick(first: &str, second: &str) -> &str {
    if first.is_empty() { second } else { first }
    // error[E0106]: missing lifetime specifier
}

fn main() {}
```

### `ch06-b011` — 6.8 Structs que almacenan vistas

Source: `06.Lifetimes.md:223` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct UserView<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Ada");
    let view = UserView { name: &name };

    assert_eq!(view, UserView { name: "Ada" });
}
```

### `ch06-b012` — 6.8 Structs que almacenan vistas

Source: `06.Lifetimes.md:243` · mode: `compile_fail`

```rust,compile_fail
struct UserView<'a> {
    name: &'a str,
}

fn local_view<'a>() -> UserView<'a> {
    let name = String::from("Ada");
    UserView { name: &name }
    // error[E0515]: cannot return a value referencing local data
}

fn main() {}
```

### `ch06-b013` — 6.9 Vista temporal frente a modelo owned

Source: `06.Lifetimes.md:263` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct HeaderView<'a> {
    name: &'a str,
    value: &'a str,
}

fn parse_header(line: &str) -> Option<HeaderView<'_>> {
    let (name, value) = line.split_once(':')?;
    Some(HeaderView {
        name: name.trim(),
        value: value.trim(),
    })
}

fn main() {
    let line = String::from("Content-Type: text/plain");
    let header = parse_header(&line).unwrap();

    assert_eq!(header.name, "Content-Type");
    assert_eq!(header.value, "text/plain");
}
```

### `ch06-b014` — 6.9 Vista temporal frente a modelo owned

Source: `06.Lifetimes.md:289` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Header {
    name: String,
    value: String,
}

impl Header {
    fn from_view(view: HeaderView<'_>) -> Self {
        Self {
            name: view.name.to_owned(),
            value: view.value.to_owned(),
        }
    }
}

#[derive(Debug)]
struct HeaderView<'a> {
    name: &'a str,
    value: &'a str,
}

fn main() {
    let view = HeaderView {
        name: "Accept",
        value: "application/json",
    };
    let header = Header::from_view(view);

    assert_eq!(header.name, "Accept");
}
```

### `ch06-b015` — 6.10 `impl` y métodos sobre tipos prestados

Source: `06.Lifetimes.md:328` · mode: `run`

```rust
struct UserView<'a> {
    name: &'a str,
}

impl<'a> UserView<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn choose_label<'b>(&self, fallback: &'b str) -> &'a str {
        if self.name.is_empty() {
            "anonymous"
        } else {
            let _ = fallback;
            self.name
        }
    }
}

fn main() {
    let name = String::from("Ada");
    let view = UserView { name: &name };

    assert_eq!(view.name(), "Ada");
    assert_eq!(view.choose_label("fallback"), "Ada");
}
```

### `ch06-b016` — 6.11 Vistas internas y mutación

Source: `06.Lifetimes.md:365` · mode: `run`

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let first = first_word(&text);

    assert_eq!(first, "hello");
    text.clear();

    assert!(text.is_empty());
}
```

### `ch06-b017` — 6.11 Vistas internas y mutación

Source: `06.Lifetimes.md:385` · mode: `compile_fail`

```rust,compile_fail
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let first = first_word(&text);

    text.clear();
    // error[E0502]: mutable access overlaps the shared view

    println!("{first}");
}
```

### `ch06-b018` — 6.11 Vistas internas y mutación

Source: `06.Lifetimes.md:403` · mode: `run`

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let first = first_word(&text).to_owned();

    text.clear();
    assert_eq!(first, "hello");
}
```

### `ch06-b019` — 6.12 Qué significa `'static`

Source: `06.Lifetimes.md:421` · mode: `run`

```rust
fn banner() -> &'static str {
    "Rust a fondo"
}

fn main() {
    assert_eq!(banner(), "Rust a fondo");
}
```

### `ch06-b020` — 6.12 Qué significa `'static`

Source: `06.Lifetimes.md:433` · mode: `run`

```rust
fn parse(input: &str) -> usize {
    input.len()
}

fn main() {
    let dynamic = String::from("entrada");
    assert_eq!(parse(&dynamic), 7);
}
```

### `ch06-b021` — 6.12 Qué significa `'static`

Source: `06.Lifetimes.md:446` · mode: `run`

```rust
fn require_static<T: 'static>(_value: T) {}

fn main() {
    let owned = String::from("propio");
    require_static(owned);
}
```

### `ch06-b022` — 6.13 Bounds entre lifetimes y tipos

Source: `06.Lifetimes.md:461` · mode: `reference`

```text
'long: 'short → 'long es válido al menos durante 'short
T: 'a         → los préstamos contenidos en T permiten usar T durante 'a
```

### `ch06-b023` — 6.13 Bounds entre lifetimes y tipos

Source: `06.Lifetimes.md:468` · mode: `run`

```rust
fn shorten<'long: 'short, 'short>(value: &'long str) -> &'short str {
    value
}

fn main() {
    let text = String::from("válido");
    let view = shorten(&text);
    assert_eq!(view, "válido");
}
```

### `ch06-b024` — 6.14 HRTB: aceptar un préstamo fresco en cada llamada

Source: `06.Lifetimes.md:486` · mode: `run`

```rust
fn apply_to_texts<F>(function: F)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let first = String::from("uno");
    assert_eq!(function(&first), 3);

    let second = String::from("cuatro");
    assert_eq!(function(&second), 6);
}

fn main() {
    apply_to_texts(str::len);
}
```

### `ch06-b025` — 6.15 Iteradores que producen vistas

Source: `06.Lifetimes.md:509` · mode: `run`

```rust
fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn main() {
    let input = String::from(" uno \n\n dos ");
    let lines: Vec<_> = non_empty_lines(&input).collect();

    assert_eq!(lines, ["uno", "dos"]);
}
```

### `ch06-b026` — 6.15 Iteradores que producen vistas

Source: `06.Lifetimes.md:527` · mode: `run`

```rust
fn normalized_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn main() {
    let lines = {
        let input = String::from(" UNO \n DOS ");
        normalized_lines(&input)
    };

    assert_eq!(lines, ["uno", "dos"]);
}
```

### `ch06-b027` — 6.16 Diseño de dominio: construir owned y exponer vistas

Source: `06.Lifetimes.md:553` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
}

impl Email {
    fn parse(value: String) -> Result<Self, EmailError> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }
        if !value.contains('@') {
            return Err(EmailError::MissingAt);
        }
        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_inner(self) -> String {
        self.0
    }
}

fn main() {
    let email = Email::parse(String::from("ada@example.test")).unwrap();
    assert_eq!(email.as_str(), "ada@example.test");
    assert_eq!(email.into_inner(), "ada@example.test");
}
```

### `ch06-b028` — 6.17 Lifetimes y async: adelanto

Source: `06.Lifetimes.md:602` · mode: `run`

```rust
async fn echo(text: &str) -> &str {
    text
}

fn main() {
    let text = String::from("hola");
    let future = echo(&text);

    drop(future);
    assert_eq!(text, "hola");
}
```

### `ch06-b029` — 6.17 Lifetimes y async: adelanto

Source: `06.Lifetimes.md:622` · mode: `compile_fail`

```rust,compile_fail
async fn echo(text: &str) -> &str {
    text
}

fn require_static<T: 'static>(_value: T) {}

fn main() {
    let text = String::from("hola");
    let future = echo(&text);

    require_static(future);
    // error[E0597]: text would need to be borrowed for 'static
}
```

## 07.Structs-y-modelado-de-datos

### `ch07-b001` — 7.1 Del grupo de valores al concepto

Source: `07.Structs-y-modelado-de-datos.md:13` · mode: `run`

```rust
fn tuple_area(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    assert_eq!(tuple_area((3, 4)), 12);

    let rectangle = Rectangle {
        width: 3,
        height: 4,
    };
    assert_eq!(rectangle.area(), 12);
}
```

### `ch07-b002` — 7.2 Tres formas de struct

Source: `07.Structs-y-modelado-de-datos.md:49` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    active: bool,
}

#[derive(Debug, PartialEq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Production;

fn main() {
    let user = User {
        id: UserId(42),
        active: true,
    };
    let environment = Production;

    assert_eq!(user.id, UserId(42));
    assert!(user.active);
    assert_eq!(format!("{environment:?}"), "Production");
}
```

### `ch07-b003` — 7.3 Construcción y field init shorthand

Source: `07.Structs-y-modelado-de-datos.md:85` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    email: String,
    username: String,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}

fn main() {
    let user = build_user(
        String::from("ada@example.test"),
        String::from("ada"),
    );

    assert_eq!(user.username, "ada");
    assert!(user.active);
}
```

### `ch07-b004` — 7.4 Mutabilidad del binding y privacidad del tipo

Source: `07.Structs-y-modelado-de-datos.md:120` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    email: String,
    active: bool,
}

fn main() {
    let mut user = User {
        email: String::from("ada@example.test"),
        active: true,
    };

    user.active = false;
    user.email.push_str(".invalid");

    assert!(!user.active);
}
```

### `ch07-b005` — 7.5 Struct update syntax mueve lo que no es `Copy`

Source: `07.Structs-y-modelado-de-datos.md:148` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let first = User {
        email: String::from("ada@example.test"),
        username: String::from("ada"),
        active: true,
    };

    let second = User {
        email: String::from("grace@example.test"),
        ..first
    };

    assert_eq!(second.username, "ada");
    assert_eq!(first.email, "ada@example.test");
    assert!(first.active);
}
```

### `ch07-b006` — 7.5 Struct update syntax mueve lo que no es `Copy`

Source: `07.Structs-y-modelado-de-datos.md:178` · mode: `compile_fail`

```rust,compile_fail
struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let first = User {
        email: String::from("ada@example.test"),
        username: String::from("ada"),
        active: true,
    };
    let _second = User {
        email: String::from("grace@example.test"),
        ..first
    };

    println!("{}", first.username);
    // error[E0382]: username was moved
}
```

### `ch07-b007` — 7.6 Datos owned y vistas prestadas

Source: `07.Structs-y-modelado-de-datos.md:207` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    email: String,
}

#[derive(Debug, PartialEq)]
struct UserView<'a> {
    email: &'a str,
}

fn view(user: &User) -> UserView<'_> {
    UserView { email: &user.email }
}

fn main() {
    let user = User {
        email: String::from("ada@example.test"),
    };

    assert_eq!(view(&user).email, "ada@example.test");
}
```

### `ch07-b008` — 7.7 Bloques `impl` y receptores

Source: `07.Structs-y-modelado-de-datos.md:237` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn into_dimensions(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn main() {
    let mut rectangle = Rectangle {
        width: 3,
        height: 4,
    };

    assert_eq!(rectangle.area(), 12);
    rectangle.scale(2);
    assert_eq!(rectangle.into_dimensions(), (6, 8));
}
```

### `ch07-b009` — 7.8 Associated functions y constructores

Source: `07.Structs-y-modelado-de-datos.md:283` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

fn main() {
    assert_eq!(
        Rectangle::square(5),
        Rectangle {
            width: 5,
            height: 5,
        }
    );
}
```

### `ch07-b010` — 7.9 Constructores inteligentes protegen invariantes

Source: `07.Structs-y-modelado-de-datos.md:317` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq)]
pub enum RectangleError {
    ZeroWidth,
    ZeroHeight,
}

impl Rectangle {
    pub fn try_new(width: u32, height: u32) -> Result<Self, RectangleError> {
        if width == 0 {
            return Err(RectangleError::ZeroWidth);
        }
        if height == 0 {
            return Err(RectangleError::ZeroHeight);
        }
        Ok(Self { width, height })
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    assert_eq!(Rectangle::try_new(0, 4), Err(RectangleError::ZeroWidth));
    assert_eq!(Rectangle::try_new(3, 4).unwrap().area(), 12);
}
```

### `ch07-b011` — 7.11 Newtypes contra la obsesión por primitivos

Source: `07.Structs-y-modelado-de-datos.md:376` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq, Eq)]
struct Email(String);

#[derive(Debug, PartialEq, Eq)]
struct Username(String);

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: UserId,
    email: Email,
    username: Username,
}

fn main() {
    let user = User {
        id: UserId(7),
        email: Email(String::from("ada@example.test")),
        username: Username(String::from("ada")),
    };

    assert_eq!(user.id, UserId(7));
}
```

### `ch07-b012` — 7.13 Getters y nombres que anticipan ownership

Source: `07.Structs-y-modelado-de-datos.md:427` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Email(String);

struct User {
    email: Email,
    active: bool,
}

impl User {
    fn email(&self) -> &Email {
        &self.email
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn replace_email(&mut self, email: Email) -> Email {
        std::mem::replace(&mut self.email, email)
    }

    fn into_email(self) -> Email {
        self.email
    }
}

fn main() {
    let mut user = User {
        email: Email(String::from("old@example.test")),
        active: true,
    };

    assert_eq!(user.email().0, "old@example.test");
    assert!(user.is_active());
    let old = user.replace_email(Email(String::from("new@example.test")));
    assert_eq!(old.0, "old@example.test");
    assert_eq!(user.into_email().0, "new@example.test");
}
```

### `ch07-b013` — 7.13 Getters y nombres que anticipan ownership

Source: `07.Structs-y-modelado-de-datos.md:470` · mode: `reference`

```text
field()       → observa una vista o valor Copy
field_mut()   → presta acceso mutable directo
as_*()        → vista barata
to_*()        → crea una representación nueva
into_*()      → consume y transfiere
```

### `ch07-b014` — 7.14 Derives con criterio semántico

Source: `07.Structs-y-modelado-de-datos.md:484` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Email(String);

fn main() {
    let id = UserId(7);
    let copied = id;
    assert_eq!(id, copied);

    let email = Email(String::from("ada@example.test"));
    let cloned = email.clone();
    assert_eq!(email, cloned);
}
```

### `ch07-b015` — 7.14 Derives con criterio semántico

Source: `07.Structs-y-modelado-de-datos.md:512` · mode: `compile_fail`

```rust,compile_fail
#[derive(Clone, Copy)]
struct Email(String);
// error[E0204]: String does not implement Copy

fn main() {}
```

### `ch07-b016` — 7.15 Tipos secretos y `Debug` redactado

Source: `07.Structs-y-modelado-de-datos.md:526` · mode: `run`

```rust
use std::fmt;

#[derive(PartialEq, Eq)]
struct PasswordHash(String);

#[derive(Debug, PartialEq, Eq)]
struct EmptyHash;

impl PasswordHash {
    fn parse(value: String) -> Result<Self, EmptyHash> {
        if value.is_empty() {
            Err(EmptyHash)
        } else {
            Ok(Self(value))
        }
    }

    fn matches_for_demo(&self, candidate: &str) -> bool {
        self.0 == candidate
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PasswordHash([REDACTED])")
    }
}

fn main() {
    let hash = PasswordHash::parse(String::from("secret-hash")).unwrap();
    assert!(hash.matches_for_demo("secret-hash"));
    assert_eq!(format!("{hash:?}"), "PasswordHash([REDACTED])");
}
```

### `ch07-b017` — 7.16 Encapsular estado derivado

Source: `07.Structs-y-modelado-de-datos.md:568` · mode: `run`

```rust
#[derive(Default)]
struct Measurements {
    values: Vec<i32>,
}

impl Measurements {
    fn add(&mut self, value: i32) {
        self.values.push(value);
    }

    fn remove_last(&mut self) -> Option<i32> {
        self.values.pop()
    }

    fn average(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }

        let total: i64 = self.values.iter().map(|&value| i64::from(value)).sum();
        Some(total as f64 / self.values.len() as f64)
    }
}

fn main() {
    let mut values = Measurements::default();
    assert_eq!(values.average(), None);

    values.add(10);
    values.add(20);
    assert_eq!(values.average(), Some(15.0));

    values.remove_last();
    assert_eq!(values.average(), Some(10.0));
}
```

### `ch07-b018` — 7.17 Métodos frente a funciones libres

Source: `07.Structs-y-modelado-de-datos.md:612` · mode: `reference`

```text
email.as_str()
user.deactivate()
cart.add_item(item)
rectangle.area()
```

### `ch07-b019` — 7.18 Builder: estado incompleto separado del resultado

Source: `07.Structs-y-modelado-de-datos.md:627` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Default)]
struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    workers: Option<usize>,
}

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingHost,
    MissingPort,
    ZeroWorkers,
}

impl ServerConfigBuilder {
    fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    fn port(mut self, value: u16) -> Self {
        self.port = Some(value);
        self
    }

    fn workers(mut self, value: usize) -> Self {
        self.workers = Some(value);
        self
    }

    fn build(self) -> Result<ServerConfig, ConfigError> {
        let host = self.host.ok_or(ConfigError::MissingHost)?;
        let port = self.port.ok_or(ConfigError::MissingPort)?;
        let workers = self.workers.unwrap_or(4);
        if workers == 0 {
            return Err(ConfigError::ZeroWorkers);
        }
        Ok(ServerConfig {
            host,
            port,
            workers,
        })
    }
}

fn main() {
    let config = ServerConfigBuilder::default()
        .host("127.0.0.1")
        .port(8080)
        .workers(8)
        .build()
        .unwrap();

    assert_eq!(config.workers, 8);
    assert_eq!(
        ServerConfigBuilder::default().port(8080).build(),
        Err(ConfigError::MissingHost)
    );
}
```

### `ch07-b020` — 7.19 Typestate con structs distintos

Source: `07.Structs-y-modelado-de-datos.md:702` · mode: `run`

```rust
struct DraftPost {
    content: String,
}

struct PublishedPost {
    content: String,
}

impl DraftPost {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn publish(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut draft = DraftPost::new();
    draft.add_text("contenido");
    let published = draft.publish();

    assert_eq!(published.content(), "contenido");
}
```

## 08.Enums-y-tipos-algebraicos

### `ch08-b001` — 8.1 Producto: todas las partes

Source: `08.Enums-y-tipos-algebraicos.md:13` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    email: Email,
}

#[derive(Debug, PartialEq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Email(String);

fn main() {
    let user = User {
        id: UserId(7),
        email: Email(String::from("ada@example.test")),
    };

    assert_eq!(user.id, UserId(7));
}
```

### `ch08-b002` — 8.2 Suma: una alternativa

Source: `08.Enums-y-tipos-algebraicos.md:44` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum PaymentMethod {
    Cash,
    Card,
    BankTransfer,
}

fn main() {
    let method = PaymentMethod::Card;
    assert_eq!(method, PaymentMethod::Card);
}
```

### `ch08-b003` — 8.3 Estados cerrados en vez de strings

Source: `08.Enums-y-tipos-algebraicos.md:68` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

#[derive(Debug, PartialEq)]
struct Order {
    status: OrderStatus,
}

fn main() {
    let order = Order {
        status: OrderStatus::Submitted,
    };
    assert_eq!(order.status, OrderStatus::Submitted);
}
```

### `ch08-b004` — 8.4 Variantes con los datos que necesitan

Source: `08.Enums-y-tipos-algebraicos.md:98` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Message {
    Quit,
    Text(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: -2 },
        Message::ChangeColor(20, 40, 60),
    ];

    assert!(matches!(messages[1], Message::Move { x: 3, y: -2 }));
}
```

### `ch08-b005` — 8.5 Variantes ricas como mini-structs

Source: `08.Enums-y-tipos-algebraicos.md:124` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Payment {
    Cash {
        received_cents: u32,
    },
    Card {
        last4: String,
        authorization_code: String,
    },
    BankTransfer {
        reference: String,
    },
}

fn describe(payment: &Payment) -> String {
    match payment {
        Payment::Cash { received_cents } => {
            format!("cash: {received_cents} cents")
        }
        Payment::Card {
            last4,
            authorization_code,
        } => format!("card {last4}, auth {authorization_code}"),
        Payment::BankTransfer { reference } => {
            format!("transfer {reference}")
        }
    }
}

fn main() {
    let payment = Payment::Card {
        last4: String::from("4242"),
        authorization_code: String::from("AUTH-7"),
    };

    assert_eq!(describe(&payment), "card 4242, auth AUTH-7");
}
```

### `ch08-b006` — 8.6 `match` exhaustivo convierte cambios en trabajo visible

Source: `08.Enums-y-tipos-algebraicos.md:170` · mode: `run`

```rust
#[derive(Clone, Copy)]
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

fn action(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "edit",
        OrderStatus::Submitted => "review",
        OrderStatus::Paid => "prepare",
        OrderStatus::Cancelled => "stop",
    }
}

fn main() {
    assert_eq!(action(OrderStatus::Paid), "prepare");
}
```

### `ch08-b007` — 8.6 `match` exhaustivo convierte cambios en trabajo visible

Source: `08.Enums-y-tipos-algebraicos.md:197` · mode: `compile_fail`

```rust,compile_fail
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

fn action(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "edit",
        OrderStatus::Submitted => "review",
        OrderStatus::Paid => "prepare",
    }
    // error[E0004]: Cancelled is not covered
}

fn main() {}
```

### `ch08-b008` — 8.7 Matching por valor, referencia o referencia mutable

Source: `08.Enums-y-tipos-algebraicos.md:223` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn emphasize(message: &mut Message) -> Option<usize> {
    match message {
        Message::Text(text) => {
            text.push('!');
            Some(text.len())
        }
        Message::Quit => None,
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));

    if let Message::Text(text) = &message {
        assert_eq!(text, "hola");
    }

    assert_eq!(emphasize(&mut message), Some(5));
    assert_eq!(message, Message::Text(String::from("hola!")));
}
```

### `ch08-b009` — 8.8 `Option<T>` representa presencia o ausencia

Source: `08.Enums-y-tipos-algebraicos.md:258` · mode: `reference`

```text
enum Option<T> {
    Some(T),
    None,
}
```

### `ch08-b010` — 8.8 `Option<T>` representa presencia o ausencia

Source: `08.Enums-y-tipos-algebraicos.md:267` · mode: `run`

```rust
fn parse_positive(input: &str) -> Option<u32> {
    let value = input.parse::<u32>().ok()?;
    (value > 0).then_some(value)
}

fn main() {
    assert_eq!(parse_positive("7"), Some(7));
    assert_eq!(parse_positive("0"), None);
    assert_eq!(parse_positive("abc"), None);
}
```

### `ch08-b011` — 8.9 `Option<&T>` devuelve una vista opcional

Source: `08.Enums-y-tipos-algebraicos.md:286` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    name: String,
}

fn find_user(users: &[User], id: UserId) -> Option<&User> {
    users.iter().find(|user| user.id == id)
}

fn main() {
    let users = vec![User {
        id: UserId(7),
        name: String::from("Ada"),
    }];

    assert_eq!(find_user(&users, UserId(7)).map(|user| user.name.as_str()), Some("Ada"));
    assert_eq!(find_user(&users, UserId(9)), None);
}
```

### `ch08-b012` — 8.10 Combinadores de `Option`

Source: `08.Enums-y-tipos-algebraicos.md:317` · mode: `run`

```rust
fn normalized_port(input: Option<&str>) -> Option<u16> {
    input
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .and_then(|text| text.parse::<u16>().ok())
        .filter(|port| *port != 0)
}

fn main() {
    assert_eq!(normalized_port(Some(" 8080 ")), Some(8080));
    assert_eq!(normalized_port(Some("0")), None);
    assert_eq!(normalized_port(None), None);
}
```

### `ch08-b013` — 8.11 `Result<T, E>` conserva éxito o causa de fallo

Source: `08.Enums-y-tipos-algebraicos.md:345` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(DivisionError::DivisionByZero));
}
```

### `ch08-b014` — 8.12 Elegir entre `Option`, `Result` y ambos

Source: `08.Enums-y-tipos-algebraicos.md:379` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User(&'static str);

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

fn find_user(id: u64) -> Result<Option<User>, RepositoryError> {
    match id {
        7 => Ok(Some(User("Ada"))),
        0 => Err(RepositoryError::Unavailable),
        _ => Ok(None),
    }
}

fn main() {
    assert_eq!(find_user(7), Ok(Some(User("Ada"))));
    assert_eq!(find_user(9), Ok(None));
    assert_eq!(find_user(0), Err(RepositoryError::Unavailable));
}
```

### `ch08-b015` — 8.13 Errores de dominio con datos

Source: `08.Enums-y-tipos-algebraicos.md:409` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
enum UsernameError {
    Empty,
    TooShort { minimum: usize, actual: usize },
    TooLong { maximum: usize, actual: usize },
    InvalidCharacter { index: usize, character: char },
}

fn validate_username(value: &str) -> Result<(), UsernameError> {
    if value.is_empty() {
        return Err(UsernameError::Empty);
    }
    let length = value.chars().count();
    if length < 3 {
        return Err(UsernameError::TooShort {
            minimum: 3,
            actual: length,
        });
    }
    if let Some((index, character)) = value
        .char_indices()
        .find(|(_, character)| !character.is_alphanumeric())
    {
        return Err(UsernameError::InvalidCharacter { index, character });
    }
    Ok(())
}

fn main() {
    assert_eq!(
        validate_username("a!"),
        Err(UsernameError::TooShort { minimum: 3, actual: 2 })
    );
    assert_eq!(validate_username("ada"), Ok(()));
}
```

### `ch08-b016` — 8.14 Métodos en enums

Source: `08.Enums-y-tipos-algebraicos.md:453` · mode: `run`

```rust
#[derive(Clone, Copy)]
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

impl OrderStatus {
    fn can_cancel(self) -> bool {
        matches!(self, Self::Draft | Self::Submitted)
    }

    fn is_terminal(self) -> bool {
        matches!(self, Self::Paid | Self::Cancelled)
    }
}

fn main() {
    assert!(OrderStatus::Draft.can_cancel());
    assert!(!OrderStatus::Paid.can_cancel());
    assert!(OrderStatus::Cancelled.is_terminal());
}
```

### `ch08-b017` — 8.15 Transiciones runtime entre variantes

Source: `08.Enums-y-tipos-algebraicos.md:485` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq)]
enum OrderError {
    CannotPayFrom(OrderStatus),
}

struct Order {
    status: OrderStatus,
}

impl Order {
    fn pay(&mut self) -> Result<(), OrderError> {
        match self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Paid;
                Ok(())
            }
            status => Err(OrderError::CannotPayFrom(status)),
        }
    }
}

fn main() {
    let mut order = Order {
        status: OrderStatus::Pending,
    };
    assert_eq!(order.pay(), Ok(()));
    assert_eq!(order.pay(), Err(OrderError::CannotPayFrom(OrderStatus::Paid)));
}
```

### `ch08-b018` — 8.16 Componer enums evita monstruos planos

Source: `08.Enums-y-tipos-algebraicos.md:530` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum PaymentStatus {
    Pending,
    Completed(CompletedPayment),
    Failed(PaymentFailure),
}

#[derive(Debug, PartialEq)]
struct CompletedPayment {
    transaction_id: String,
    amount_cents: u32,
}

#[derive(Debug, PartialEq)]
enum PaymentFailure {
    InsufficientFunds,
    CardExpired,
    FraudSuspected,
    Provider { code: String },
}

fn main() {
    let status = PaymentStatus::Failed(PaymentFailure::Provider {
        code: String::from("P-42"),
    });

    assert!(matches!(
        status,
        PaymentStatus::Failed(PaymentFailure::Provider { ref code }) if code == "P-42"
    ));
}
```

### `ch08-b019` — 8.17 Wildcards y evolución

Source: `08.Enums-y-tipos-algebraicos.md:570` · mode: `run`

```rust
#[derive(Clone, Copy)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn is_waiting(status: OrderStatus) -> bool {
    match status {
        OrderStatus::Pending => true,
        _ => false,
    }
}

fn main() {
    assert!(is_waiting(OrderStatus::Pending));
    assert!(!is_waiting(OrderStatus::Shipped));
}
```

### `ch08-b020` — 8.18 `if let`, `let else` y `matches!`

Source: `08.Enums-y-tipos-algebraicos.md:598` · mode: `run`

```rust
fn require_name(value: Option<String>) -> Result<String, &'static str> {
    let Some(name) = value else {
        return Err("nombre ausente");
    };

    if let Some(first) = name.chars().next() {
        assert!(first.is_alphabetic());
    }

    Ok(name)
}

fn main() {
    assert_eq!(require_name(Some(String::from("Ada"))), Ok(String::from("Ada")));
    assert_eq!(require_name(None), Err("nombre ausente"));
    assert!(matches!(Some(3), Some(value) if value > 0));
}
```

## 09.Pattern-matching-profundo

### `ch09-b001` — 9.1 Anatomía de un patrón

Source: `09.Pattern-matching-profundo.md:13` · mode: `reference`

```text
literales · bindings · _ · tuplas · structs · enums
slices · rangos · or-patterns · guards · bindings @
```

### `ch09-b002` — 9.1 Anatomía de un patrón

Source: `09.Pattern-matching-profundo.md:20` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Event {
    Connected { user_id: u64 },
    Disconnected { user_id: u64, reason: String },
}

fn user_id(event: &Event) -> u64 {
    match event {
        Event::Connected { user_id }
        | Event::Disconnected { user_id, .. } => *user_id,
    }
}

fn main() {
    let event = Event::Disconnected {
        user_id: 7,
        reason: String::from("timeout"),
    };

    assert_eq!(user_id(&event), 7);
}
```

### `ch09-b003` — 9.2 Patrones fuera de `match`

Source: `09.Pattern-matching-profundo.md:50` · mode: `run`

```rust
fn sum_pair((left, right): (i32, i32)) -> i32 {
    left + right
}

fn main() {
    let (x, y) = (10, 20);
    let add = |(left, right): (i32, i32)| left + right;
    let values = ["a", "b"];
    let indexed: Vec<_> = values.iter().enumerate().collect();

    assert_eq!((x, y), (10, 20));
    assert_eq!(sum_pair((2, 3)), 5);
    assert_eq!(add((4, 5)), 9);
    assert_eq!(indexed[1], (1, &"b"));
}
```

### `ch09-b004` — 9.3 Patrones refutables e irrefutables

Source: `09.Pattern-matching-profundo.md:76` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let maybe_value = Some(10);
    let Some(value) = maybe_value;
    // error[E0005]: refutable pattern in local binding

    println!("{value}");
}
```

### `ch09-b005` — 9.3 Patrones refutables e irrefutables

Source: `09.Pattern-matching-profundo.md:88` · mode: `run`

```rust
fn require_value(value: Option<i32>) -> Result<i32, &'static str> {
    let Some(value) = value else {
        return Err("valor ausente");
    };
    Ok(value)
}

fn main() {
    assert_eq!(require_value(Some(10)), Ok(10));
    assert_eq!(require_value(None), Err("valor ausente"));
}
```

### `ch09-b006` — 9.4 Orden de brazos y alcanzabilidad

Source: `09.Pattern-matching-profundo.md:108` · mode: `run`

```rust
fn classify(number: i32) -> &'static str {
    match number {
        0 => "zero",
        1..=9 => "one digit",
        _ => "other",
    }
}

fn main() {
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(7), "one digit");
}
```

### `ch09-b007` — 9.4 Orden de brazos y alcanzabilidad

Source: `09.Pattern-matching-profundo.md:127` · mode: `compile_fail`

```rust,compile_fail
#![deny(unreachable_patterns)]

fn classify(number: i32) -> &'static str {
    match number {
        _ => "anything",
        0 => "zero",
        // error: unreachable pattern
    }
}

fn main() {
    let _ = classify(0);
}
```

### `ch09-b008` — 9.5 Un identificador captura; no compara

Source: `09.Pattern-matching-profundo.md:149` · mode: `run`

```rust
fn main() {
    let expected = 10;
    let value = 20;

    let captured = match value {
        expected => expected,
    };

    assert_eq!(captured, 20);
    assert_eq!(expected, 10);
}
```

### `ch09-b009` — 9.5 Un identificador captura; no compara

Source: `09.Pattern-matching-profundo.md:165` · mode: `run`

```rust
fn equals_expected(value: i32, expected: i32) -> bool {
    match value {
        candidate if candidate == expected => true,
        _ => false,
    }
}

const SPECIAL: i32 = 42;

fn main() {
    assert!(equals_expected(10, 10));
    assert!(matches!(42, SPECIAL));
}
```

### `ch09-b010` — 9.6 Literales y rangos

Source: `09.Pattern-matching-profundo.md:187` · mode: `run`

```rust
fn age_group(age: u8) -> &'static str {
    match age {
        0..=12 => "child",
        13..=17 => "teenager",
        18..=64 => "adult",
        65..=u8::MAX => "senior",
    }
}

fn character_group(character: char) -> &'static str {
    match character {
        'a'..='z' => "lowercase ASCII",
        'A'..='Z' => "uppercase ASCII",
        '0'..='9' => "digit ASCII",
        _ => "other",
    }
}

fn main() {
    assert_eq!(age_group(17), "teenager");
    assert_eq!(character_group('ñ'), "other");
}
```

### `ch09-b011` — 9.7 Or-patterns y bindings compatibles

Source: `09.Pattern-matching-profundo.md:218` · mode: `run`

```rust
#[derive(Debug)]
enum Event {
    Created { id: u64 },
    Updated { id: u64 },
    Deleted { id: u64 },
}

fn changed_id(event: Event) -> u64 {
    match event {
        Event::Created { id }
        | Event::Updated { id }
        | Event::Deleted { id } => id,
    }
}

fn main() {
    assert_eq!(changed_id(Event::Updated { id: 7 }), 7);
}
```

### `ch09-b012` — 9.7 Or-patterns y bindings compatibles

Source: `09.Pattern-matching-profundo.md:243` · mode: `compile_fail`

```rust,compile_fail
enum Event {
    Created { id: u64 },
    Idle,
}

fn id(event: Event) -> u64 {
    match event {
        Event::Created { id } | Event::Idle => id,
        // error[E0408]: id is not bound in all patterns
    }
}

fn main() {}
```

### `ch09-b013` — 9.8 `_` frente a `_name`

Source: `09.Pattern-matching-profundo.md:263` · mode: `run`

```rust
fn main() {
    let maybe_text = Some(String::from("hola"));

    if let Some(_) = maybe_text {
        println!("hay texto");
    }

    assert_eq!(maybe_text.as_deref(), Some("hola"));
}
```

### `ch09-b014` — 9.8 `_` frente a `_name`

Source: `09.Pattern-matching-profundo.md:279` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let maybe_text = Some(String::from("hola"));

    if let Some(_text) = maybe_text {
        println!("hay texto");
    }

    println!("{maybe_text:?}");
    // error[E0382]: value was partially moved
}
```

### `ch09-b015` — 9.9 `..` ignora el resto

Source: `09.Pattern-matching-profundo.md:298` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

fn main() {
    let user = User {
        id: 7,
        name: String::from("Ada"),
        active: true,
    };
    let User { id, active, .. } = &user;

    let tuple = (1, 2, 3, 4);
    let (first, .., last) = tuple;

    assert_eq!((*id, *active), (7, true));
    assert_eq!((first, last), (1, 4));
    assert_eq!(user.name, "Ada");
}
```

### `ch09-b016` — 9.10 Desestructuración de structs

Source: `09.Pattern-matching-profundo.md:329` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn describe(point: Point) -> String {
    match point {
        Point { x: 0, y } => format!("y axis at {y}"),
        Point { x, y: 0 } => format!("x axis at {x}"),
        Point { x, y } => format!("{x}, {y}"),
    }
}

fn main() {
    let point = Point { x: 3, y: 4 };
    let Point {
        x: horizontal,
        y: vertical,
    } = point;

    assert_eq!((horizontal, vertical), (3, 4));
    assert_eq!(describe(Point { x: 0, y: 5 }), "y axis at 5");
}
```

### `ch09-b017` — 9.11 Desestructuración de enums

Source: `09.Pattern-matching-profundo.md:362` · mode: `run`

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Point,
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}

fn main() {
    let shape = Shape::Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(area(&shape), 12.0);
}
```

### `ch09-b018` — 9.12 El scrutinee determina ownership

Source: `09.Pattern-matching-profundo.md:393` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn inspect(message: &Message) -> Option<&str> {
    match message {
        Message::Text(text) => Some(text),
        Message::Quit => None,
    }
}

fn modify(message: &mut Message) {
    if let Message::Text(text) = message {
        text.push('!');
    }
}

fn consume(message: Message) -> Option<String> {
    match message {
        Message::Text(text) => Some(text),
        Message::Quit => None,
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));
    assert_eq!(inspect(&message), Some("hola"));

    modify(&mut message);
    assert_eq!(consume(message), Some(String::from("hola!")));
}
```

### `ch09-b019` — 9.13 `ref`, `ref mut` y patrones sobre referencias

Source: `09.Pattern-matching-profundo.md:435` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn main() {
    let mut message = Message::Text(String::from("hola"));

    match message {
        Message::Text(ref text) => assert_eq!(text, "hola"),
        Message::Quit => {}
    }

    match message {
        Message::Text(ref mut text) => text.push('!'),
        Message::Quit => {}
    }

    assert_eq!(message, Message::Text(String::from("hola!")));
}
```

### `ch09-b020` — 9.13 `ref`, `ref mut` y patrones sobre referencias

Source: `09.Pattern-matching-profundo.md:463` · mode: `run`

```rust
fn main() {
    let number = 10;
    let reference = &number;

    let &copied = reference;
    assert_eq!(copied, 10);
}
```

### `ch09-b021` — 9.14 Patrones anidados

Source: `09.Pattern-matching-profundo.md:479` · mode: `run`

```rust
#[derive(Debug)]
enum UserStatus {
    Active,
    Suspended { reason: String },
}

#[derive(Debug)]
struct User {
    id: u64,
    status: UserStatus,
}

fn suspension(user: &User) -> Option<(u64, &str)> {
    match user {
        User {
            id,
            status: UserStatus::Suspended { reason },
        } => Some((*id, reason)),
        _ => None,
    }
}

fn main() {
    let user = User {
        id: 7,
        status: UserStatus::Suspended {
            reason: String::from("review"),
        },
    };
    assert_eq!(suspension(&user), Some((7, "review")));
}
```

### `ch09-b022` — 9.15 Guards añaden condiciones no estructurales

Source: `09.Pattern-matching-profundo.md:519` · mode: `run`

```rust
#[derive(Debug)]
enum ServiceError {
    Http { status: u16, path: String },
    Timeout,
}

#[derive(Debug, PartialEq)]
enum Severity {
    Severe,
    Normal,
}

fn severity(error: &ServiceError) -> Severity {
    match error {
        ServiceError::Http { status, .. } if *status >= 500 => Severity::Severe,
        ServiceError::Http { .. } | ServiceError::Timeout => Severity::Normal,
    }
}

fn main() {
    let error = ServiceError::Http {
        status: 503,
        path: String::from("/users"),
    };
    assert_eq!(severity(&error), Severity::Severe);
}
```

### `ch09-b023` — 9.16 Bindings `@`: comprobar y conservar

Source: `09.Pattern-matching-profundo.md:554` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Position {
    Inside { x: i32, y: i32 },
    Outside { x: i32, y: i32 },
}

fn classify(x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ 0..=100, y @ 0..=100) => Position::Inside { x, y },
        (x, y) => Position::Outside { x, y },
    }
}

fn main() {
    assert_eq!(classify(20, 40), Position::Inside { x: 20, y: 40 });
    assert_eq!(classify(-1, 40), Position::Outside { x: -1, y: 40 });
}
```

### `ch09-b024` — 9.17 Slice patterns expresan aridad y resto

Source: `09.Pattern-matching-profundo.md:580` · mode: `run`

```rust
fn describe(values: &[i32]) -> String {
    match values {
        [] => String::from("empty"),
        [one] => format!("one: {one}"),
        [first, second] => format!("two: {first}, {second}"),
        [first, middle @ .., last] => {
            format!("many: {first}, {} middle, {last}", middle.len())
        }
    }
}

fn main() {
    assert_eq!(describe(&[]), "empty");
    assert_eq!(describe(&[1]), "one: 1");
    assert_eq!(describe(&[1, 2, 3, 4]), "many: 1, 2 middle, 4");
}
```

### `ch09-b025` — 9.17 Slice patterns expresan aridad y resto

Source: `09.Pattern-matching-profundo.md:603` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Command<'a> {
    Help,
    Create { name: &'a str },
    Delete { id: &'a str },
}

fn parse<'a>(args: &'a [&'a str]) -> Result<Command<'a>, &'static str> {
    match args {
        ["help"] => Ok(Command::Help),
        ["user", "create", name] => Ok(Command::Create { name }),
        ["user", "delete", id] => Ok(Command::Delete { id }),
        _ => Err("uso inválido"),
    }
}

fn main() {
    assert_eq!(
        parse(&["user", "create", "ada"]),
        Ok(Command::Create { name: "ada" })
    );
}
```

### `ch09-b026` — 9.18 Elegir la construcción adecuada

Source: `09.Pattern-matching-profundo.md:640` · mode: `run`

```rust
fn drain(mut stack: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    while let Some(value) = stack.pop() {
        output.push(value);
    }
    output
}

fn positive(value: Option<i32>) -> Result<i32, &'static str> {
    let Some(value) = value else {
        return Err("ausente");
    };

    if let 1..=i32::MAX = value {
        Ok(value)
    } else {
        Err("no positivo")
    }
}

fn main() {
    assert_eq!(drain(vec![1, 2, 3]), [3, 2, 1]);
    assert_eq!(positive(Some(4)), Ok(4));
    assert!(matches!(positive(Some(0)), Err(_)));
}
```

### `ch09-b027` — 9.19 `?` frente a `match`

Source: `09.Pattern-matching-profundo.md:674` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber,
    Zero,
}

fn parse_non_zero(input: &str) -> Result<u32, ParseError> {
    let value = input
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidNumber)?;

    match value {
        0 => Err(ParseError::Zero),
        value => Ok(value),
    }
}

fn main() {
    assert_eq!(parse_non_zero("7"), Ok(7));
    assert_eq!(parse_non_zero("x"), Err(ParseError::InvalidNumber));
    assert_eq!(parse_non_zero("0"), Err(ParseError::Zero));
}
```

### `ch09-b028` — 9.20 Caso práctico: comandos de dominio

Source: `09.Pattern-matching-profundo.md:705` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
struct Username(String);

#[derive(Debug, PartialEq)]
enum UserCommand {
    Register {
        email: Email,
        username: Username,
    },
    Suspend {
        user_id: u64,
        reason: String,
    },
    Delete {
        user_id: u64,
    },
}

fn audit(command: &UserCommand) -> String {
    match command {
        UserCommand::Register { email, username } => {
            format!("register {} as {}", email.0, username.0)
        }
        UserCommand::Suspend { user_id, reason } => {
            format!("suspend {user_id}: {reason}")
        }
        UserCommand::Delete { user_id } => format!("delete {user_id}"),
    }
}

fn main() {
    let command = UserCommand::Suspend {
        user_id: 7,
        reason: String::from("review"),
    };

    assert_eq!(audit(&command), "suspend 7: review");
}
```

## 10.Errores-como-parte-del-dominio

### `ch10-b001` — 10.2 El árbol de decisión mínimo

Source: `10.Errores-como-parte-del-dominio.md:33` · mode: `reference`

```text
T              siempre produce un valor válido
Option<T>      produce un valor o ausencia normal
Result<T, E>   produce un valor o un fallo explicado
panic          denuncia una suposición interna rota
```

### `ch10-b002` — 10.3 `Option<T>` representa ausencia, no fracaso

Source: `10.Errores-como-parte-del-dominio.md:46` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    id: u64,
    name: String,
}

fn find_name(users: &[User], id: u64) -> Option<&str> {
    users
        .iter()
        .find(|user| user.id == id)
        .map(|user| user.name.as_str())
}

fn main() {
    let users = vec![User {
        id: 7,
        name: String::from("Ada"),
    }];

    assert_eq!(find_name(&users, 7), Some("Ada"));
    assert_eq!(find_name(&users, 8), None);
}
```

### `ch10-b003` — 10.4 Transformar `Option` sin perder ownership

Source: `10.Errores-como-parte-del-dominio.md:77` · mode: `run`

```rust
fn normalized_tag(input: Option<&str>) -> Option<String> {
    input
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(str::to_lowercase)
}

fn label_length(label: &Option<String>) -> Option<usize> {
    label.as_deref().map(str::len)
}

fn main() {
    assert_eq!(normalized_tag(Some("  Rust ")), Some(String::from("rust")));
    assert_eq!(normalized_tag(Some("   ")), None);

    let label = Some(String::from("owned"));
    assert_eq!(label_length(&label), Some(5));
    assert_eq!(label.as_deref(), Some("owned"));
}
```

### `ch10-b004` — 10.5 `Result<T, E>` hace visible el camino de fallo

Source: `10.Errores-como-parte-del-dominio.md:111` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(DivisionError::DivisionByZero));
}
```

### `ch10-b005` — 10.6 Errores concretos frente a `String`

Source: `10.Errores-como-parte-del-dominio.md:137` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum PortError {
    Empty,
    NotANumber,
    Reserved { port: u16 },
}

fn parse_port(input: &str) -> Result<u16, PortError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(PortError::Empty);
    }

    let port = input.parse::<u16>().map_err(|_| PortError::NotANumber)?;
    if port < 1024 {
        return Err(PortError::Reserved { port });
    }

    Ok(port)
}

fn main() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("80"), Err(PortError::Reserved { port: 80 }));
}
```

### `ch10-b006` — 10.7 Un value object fallible concentra la invariante

Source: `10.Errores-como-parte-del-dominio.md:171` · mode: `run`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    MissingAt,
    TooLong { maximum: usize, actual: usize },
}

impl Email {
    pub fn parse(input: &str) -> Result<Self, EmailError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(EmailError::Empty);
        }
        if !input.contains('@') {
            return Err(EmailError::MissingAt);
        }

        const MAXIMUM: usize = 254;
        if input.len() > MAXIMUM {
            return Err(EmailError::TooLong {
                maximum: MAXIMUM,
                actual: input.len(),
            });
        }

        Ok(Self(input.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let email = Email::parse("  ada@example.com ").unwrap();
    assert_eq!(email.as_str(), "ada@example.com");
    assert_eq!(Email::parse("ada"), Err(EmailError::MissingAt));
}
```

### `ch10-b007` — 10.8 `?` propaga; no decide el modelo

Source: `10.Errores-como-parte-del-dominio.md:223` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
enum EmailError {
    MissingAt,
}

impl Email {
    fn parse(input: &str) -> Result<Self, EmailError> {
        input
            .contains('@')
            .then(|| Self(input.to_owned()))
            .ok_or(EmailError::MissingAt)
    }
}

#[derive(Debug, PartialEq)]
struct User {
    email: Email,
}

#[derive(Debug, PartialEq)]
enum CreateUserError {
    InvalidEmail(EmailError),
}

fn create_user(input: &str) -> Result<User, CreateUserError> {
    let email = Email::parse(input).map_err(CreateUserError::InvalidEmail)?;
    Ok(User { email })
}

fn main() {
    assert!(create_user("ada@example.com").is_ok());
    assert_eq!(
        create_user("ada"),
        Err(CreateUserError::InvalidEmail(EmailError::MissingAt))
    );
}
```

### `ch10-b008` — 10.9 `From` permite conversiones naturales con `?`

Source: `10.Errores-como-parte-del-dominio.md:271` · mode: `run`

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    InvalidNumber(ParseIntError),
    ZeroPort,
}

impl From<ParseIntError> for ConfigError {
    fn from(source: ParseIntError) -> Self {
        Self::InvalidNumber(source)
    }
}

fn parse_port(input: &str) -> Result<u16, ConfigError> {
    let port = input.parse::<u16>()?;
    if port == 0 {
        return Err(ConfigError::ZeroPort);
    }
    Ok(port)
}

fn main() {
    assert_eq!(parse_port("8080").unwrap(), 8080);
    assert!(matches!(parse_port("abc"), Err(ConfigError::InvalidNumber(_))));
}
```

### `ch10-b009` — 10.10 `map_err` traduce en el punto exacto

Source: `10.Errores-como-parte-del-dominio.md:306` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    id: u64,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

#[derive(Debug, PartialEq)]
enum LoadUserError {
    Repository { id: u64, source: RepositoryError },
}

fn load_user(
    id: u64,
    repository_result: Result<User, RepositoryError>,
) -> Result<User, LoadUserError> {
    repository_result.map_err(|source| LoadUserError::Repository { id, source })
}

fn main() {
    assert_eq!(
        load_user(7, Err(RepositoryError::Unavailable)),
        Err(LoadUserError::Repository {
            id: 7,
            source: RepositoryError::Unavailable,
        })
    );
}
```

### `ch10-b010` — 10.12 `TryFrom` expresa conversiones fallibles

Source: `10.Errores-como-parte-del-dominio.md:359` · mode: `run`

```rust
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Quantity(u32);

#[derive(Debug, PartialEq, Eq)]
enum QuantityError {
    Zero,
    AboveMaximum { maximum: u32, actual: u32 },
}

impl TryFrom<u32> for Quantity {
    type Error = QuantityError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const MAXIMUM: u32 = 100;
        match value {
            0 => Err(QuantityError::Zero),
            value if value > MAXIMUM => Err(QuantityError::AboveMaximum {
                maximum: MAXIMUM,
                actual: value,
            }),
            value => Ok(Self(value)),
        }
    }
}

fn main() {
    assert_eq!(Quantity::try_from(3), Ok(Quantity(3)));
    assert_eq!(Quantity::try_from(0), Err(QuantityError::Zero));
}
```

### `ch10-b011` — 10.13 `Result<(), E>` modela una acción fallible

Source: `10.Errores-como-parte-del-dominio.md:399` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Shipped,
    Cancelled,
}

#[derive(Debug, PartialEq)]
enum CancelError {
    AlreadyShipped,
    AlreadyCancelled,
}

fn cancel(status: &mut Status) -> Result<(), CancelError> {
    match status {
        Status::Pending => {
            *status = Status::Cancelled;
            Ok(())
        }
        Status::Shipped => Err(CancelError::AlreadyShipped),
        Status::Cancelled => Err(CancelError::AlreadyCancelled),
    }
}

fn main() {
    let mut status = Status::Pending;
    assert_eq!(cancel(&mut status), Ok(()));
    assert_eq!(cancel(&mut status), Err(CancelError::AlreadyCancelled));
}
```

### `ch10-b012` — 10.14 `Result<Option<T>, E>` tiene tres resultados

Source: `10.Errores-como-parte-del-dominio.md:437` · mode: `run`

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

fn repository_find(
    users: &[User],
    id: u64,
) -> Result<Option<User>, RepositoryError> {
    if id == 0 {
        return Err(RepositoryError::Unavailable);
    }
    Ok(users.iter().find(|user| user.id == id).cloned())
}

#[derive(Debug, PartialEq)]
enum GetUserError {
    NotFound { id: u64 },
    Repository(RepositoryError),
}

fn get_user(users: &[User], id: u64) -> Result<User, GetUserError> {
    repository_find(users, id)
        .map_err(GetUserError::Repository)?
        .ok_or(GetUserError::NotFound { id })
}

fn main() {
    let users = [User { id: 7 }];
    assert_eq!(get_user(&users, 7), Ok(User { id: 7 }));
    assert_eq!(get_user(&users, 8), Err(GetUserError::NotFound { id: 8 }));
    assert_eq!(
        get_user(&users, 0),
        Err(GetUserError::Repository(RepositoryError::Unavailable))
    );
}
```

### `ch10-b013` — 10.15 `unwrap` y `expect` son afirmaciones

Source: `10.Errores-como-parte-del-dominio.md:487` · mode: `run`

```rust
fn main() {
    let port = "8080"
        .parse::<u16>()
        .expect("el literal 8080 está controlado por el programa");

    assert_eq!(port, 8080);
}
```

### `ch10-b014` — 10.15 `unwrap` y `expect` son afirmaciones

Source: `10.Errores-como-parte-del-dominio.md:499` · mode: `compile_only`

```rust,no_run
fn parse_external_port(input: &str) -> u16 {
    input.parse::<u16>().unwrap() // panic ante input recuperable
}

fn main() {
    let _ = parse_external_port("dato externo");
}
```

### `ch10-b015` — 10.16 Panic y assertions denuncian bugs

Source: `10.Errores-como-parte-del-dominio.md:515` · mode: `run`

```rust
#[derive(Debug)]
struct Percentage(u8);

impl Percentage {
    fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    fn complement(&self) -> u8 {
        debug_assert!(self.0 <= 100, "Percentage conserva su invariante");
        100 - self.0
    }
}

fn main() {
    let progress = Percentage::new(35).unwrap();
    assert_eq!(progress.complement(), 65);
    assert!(Percentage::new(101).is_none());
}
```

### `ch10-b016` — 10.17 `Debug`, `Display` y el trait `Error`

Source: `10.Errores-como-parte-del-dominio.md:547` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum UsernameError {
    Empty,
    TooShort { minimum: usize, actual: usize },
}

impl fmt::Display for UsernameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "el nombre no puede estar vacío"),
            Self::TooShort { minimum, actual } => write!(
                formatter,
                "el nombre necesita al menos {minimum} caracteres y tiene {actual}"
            ),
        }
    }
}

impl Error for UsernameError {}

fn main() {
    let error = UsernameError::TooShort {
        minimum: 3,
        actual: 1,
    };
    assert_eq!(error.to_string(), "el nombre necesita al menos 3 caracteres y tiene 1");
}
```

### `ch10-b017` — 10.18 `source()` conserva la cadena causal

Source: `10.Errores-como-parte-del-dominio.md:586` · mode: `run`

```rust
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct ConfigError {
    field: &'static str,
    source: ParseIntError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "configuración inválida en {}", self.field)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_workers(input: &str) -> Result<u16, ConfigError> {
    input.parse::<u16>().map_err(|source| ConfigError {
        field: "workers",
        source,
    })
}

fn main() {
    let error = parse_workers("many").unwrap_err();
    assert_eq!(error.to_string(), "configuración inválida en workers");
    assert!(error.source().is_some());
}
```

### `ch10-b018` — 10.20 Traducir en la frontera sin filtrar detalles

Source: `10.Errores-como-parte-del-dominio.md:644` · mode: `run`

```rust
#[derive(Debug)]
enum RegisterUserError {
    InvalidEmail,
    DuplicateEmail,
    RepositoryUnavailable,
}

fn into_http(error: RegisterUserError) -> (u16, &'static str) {
    match error {
        RegisterUserError::InvalidEmail => (400, "email inválido"),
        RegisterUserError::DuplicateEmail => (409, "email ya registrado"),
        RegisterUserError::RepositoryUnavailable => {
            (503, "servicio temporalmente no disponible")
        }
    }
}

fn main() {
    assert_eq!(
        into_http(RegisterUserError::DuplicateEmail),
        (409, "email ya registrado")
    );
}
```

### `ch10-b019` — 10.22 Alias concretos y errores borrados

Source: `10.Errores-como-parte-del-dominio.md:690` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum ImportError {
    Empty,
}

type ImportResult<T> = Result<T, ImportError>;

fn import_line(input: &str) -> ImportResult<&str> {
    (!input.trim().is_empty())
        .then_some(input.trim())
        .ok_or(ImportError::Empty)
}

fn main() {
    assert_eq!(import_line(" Rust "), Ok("Rust"));
}
```

### `ch10-b020` — 10.22 Alias concretos y errores borrados

Source: `10.Errores-como-parte-del-dominio.md:711` · mode: `run`

```rust
use std::error::Error;

fn read_answer(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(input.parse::<u32>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(read_answer("42")?, 42);
    Ok(())
}
```

### `ch10-b021` — 10.23 `main` también puede devolver `Result`

Source: `10.Errores-como-parte-del-dominio.md:730` · mode: `run`

```rust
use std::error::Error;

fn load_limit(input: &str) -> Result<u32, Box<dyn Error>> {
    let limit = input.parse::<u32>()?;
    Ok(limit)
}

fn main() -> Result<(), Box<dyn Error>> {
    let limit = load_limit("25")?;
    assert_eq!(limit, 25);
    Ok(())
}
```

### `ch10-b022` — 10.24 Combinadores de `Result` y legibilidad

Source: `10.Errores-como-parte-del-dominio.md:751` · mode: `reference`

```text
map       transforma Ok(T) en Ok(U)
map_err   transforma Err(E) en Err(F)
and_then  encadena T -> Result<U, E>
or_else   intenta recuperarse con E -> Result<T, F>
```

### `ch10-b023` — 10.24 Combinadores de `Result` y legibilidad

Source: `10.Errores-como-parte-del-dominio.md:760` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum QuantityError {
    InvalidNumber,
    Zero,
}

fn parse_quantity(input: &str) -> Result<u32, QuantityError> {
    input
        .parse::<u32>()
        .map_err(|_| QuantityError::InvalidNumber)
        .and_then(|value| {
            if value == 0 {
                Err(QuantityError::Zero)
            } else {
                Ok(value)
            }
        })
}

fn main() {
    assert_eq!(parse_quantity("3"), Ok(3));
    assert_eq!(parse_quantity("0"), Err(QuantityError::Zero));
}
```

### `ch10-b024` — 10.25 Construcción perezosa del error

Source: `10.Errores-como-parte-del-dominio.md:792` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct MissingUser {
    detail: String,
}

fn require_user(user: Option<&str>, id: u64) -> Result<&str, MissingUser> {
    user.ok_or_else(|| MissingUser {
        detail: format!("usuario {id} no encontrado"),
    })
}

fn main() {
    assert_eq!(require_user(Some("Ada"), 7), Ok("Ada"));
    assert_eq!(
        require_user(None, 7),
        Err(MissingUser {
            detail: String::from("usuario 7 no encontrado"),
        })
    );
}
```

### `ch10-b025` — 10.26 Probar el contrato, no solo el mensaje

Source: `10.Errores-como-parte-del-dominio.md:821` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum SignupError {
    TooYoung { minimum: u8, actual: u8 },
}

fn signup(age: u8) -> Result<(), SignupError> {
    const MINIMUM: u8 = 18;
    if age < MINIMUM {
        Err(SignupError::TooYoung {
            minimum: MINIMUM,
            actual: age,
        })
    } else {
        Ok(())
    }
}

fn main() {
    assert_eq!(
        signup(16),
        Err(SignupError::TooYoung {
            minimum: 18,
            actual: 16,
        })
    );
    assert!(matches!(signup(17), Err(SignupError::TooYoung { .. })));
}
```

### `ch10-b026` — 10.27 Eliminar un error mediante typestate

Source: `10.Errores-como-parte-del-dominio.md:857` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct PaidOrder {
    id: u64,
}

#[derive(Debug, PartialEq)]
struct ShippedOrder {
    id: u64,
}

impl PaidOrder {
    fn ship(self) -> ShippedOrder {
        ShippedOrder { id: self.id }
    }
}

fn main() {
    let paid = PaidOrder { id: 7 };
    let shipped = paid.ship();
    assert_eq!(shipped, ShippedOrder { id: 7 });
}
```

## 10A.Anexo-errores-enum-vs-dyn-trait

### `ch10a-b001` — A.1 Dos contratos distintos

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:13` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
    TooLong { maximum: usize, actual: usize },
}

fn validate_email(input: &str) -> Result<(), EmailError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(EmailError::Empty);
    }
    if !input.contains('@') {
        return Err(EmailError::MissingAt);
    }
    if input.len() > 254 {
        return Err(EmailError::TooLong {
            maximum: 254,
            actual: input.len(),
        });
    }
    Ok(())
}

fn main() {
    assert_eq!(validate_email("ada"), Err(EmailError::MissingAt));
}
```

### `ch10a-b002` — A.1 Dos contratos distintos

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:45` · mode: `reference`

```text
enum EmailError        conjunto cerrado; el caller conoce los casos
Box<dyn DomainError>   conjunto abierto; el caller conoce capacidades
```

### `ch10a-b003` — A.2 Un conjunto abierto completo

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:56` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrorCategory {
    Validation,
    Conflict,
    Infrastructure,
}

trait DomainError: Error + Send + Sync + 'static {
    fn code(&self) -> &'static str;
    fn category(&self) -> ErrorCategory;
}

#[derive(Debug)]
struct EmptyEmail;

impl fmt::Display for EmptyEmail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "el email no puede estar vacío")
    }
}

impl Error for EmptyEmail {}

impl DomainError for EmptyEmail {
    fn code(&self) -> &'static str {
        "email.empty"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Validation
    }
}

fn validate_email(input: &str) -> Result<(), Box<dyn DomainError>> {
    if input.trim().is_empty() {
        Err(Box::new(EmptyEmail))
    } else {
        Ok(())
    }
}

fn main() {
    let error = validate_email(" ").unwrap_err();
    assert_eq!(error.code(), "email.empty");
    assert_eq!(error.category(), ErrorCategory::Validation);
}
```

### `ch10a-b004` — A.3 Qué significa realmente `dyn Trait`

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:116` · mode: `reference`

```text
puntero a datos + puntero a vtable
```

### `ch10a-b005` — A.4 Exhaustividad como herramienta de evolución

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:126` · mode: `run`

```rust
#[derive(Debug)]
enum RegisterError {
    InvalidEmail,
    DuplicateEmail,
    RepositoryUnavailable,
}

fn status(error: RegisterError) -> u16 {
    match error {
        RegisterError::InvalidEmail => 400,
        RegisterError::DuplicateEmail => 409,
        RegisterError::RepositoryUnavailable => 503,
    }
}

fn main() {
    assert_eq!(status(RegisterError::DuplicateEmail), 409);
}
```

### `ch10a-b006` — A.5 Tests y datos por variante

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:155` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum QuantityError {
    Zero,
    AboveMaximum { maximum: u32, actual: u32 },
}

fn quantity(value: u32) -> Result<u32, QuantityError> {
    match value {
        0 => Err(QuantityError::Zero),
        1..=100 => Ok(value),
        actual => Err(QuantityError::AboveMaximum {
            maximum: 100,
            actual,
        }),
    }
}

fn main() {
    assert_eq!(
        quantity(120),
        Err(QuantityError::AboveMaximum {
            maximum: 100,
            actual: 120,
        })
    );
}
```

### `ch10a-b007` — A.6 Downcasting es una salida de emergencia

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:192` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct EmptyEmail;

impl fmt::Display for EmptyEmail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "email vacío")
    }
}

impl Error for EmptyEmail {}

fn main() {
    let error: Box<dyn Error> = Box::new(EmptyEmail);
    assert!(error.downcast_ref::<EmptyEmail>().is_some());
}
```

### `ch10a-b008` — A.7 Dyn compatibility limita el diseño del trait

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:221` · mode: `compile_fail`

```rust,compile_fail
trait DomainError {
    fn attach<T>(&self, context: T);
}

fn main() {
    let _error: Option<Box<dyn DomainError>> = None;
    // error[E0038]: DomainError is not dyn compatible
}
```

### `ch10a-b009` — A.8 `Send`, `Sync` y `'static` no son decoración

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:238` · mode: `run`

```rust
use std::error::Error;

type AnyError = Box<dyn Error + Send + Sync + 'static>;

fn assert_transportable<T: Send + Sync + 'static>() {}

fn main() {
    assert_transportable::<AnyError>();
}
```

### `ch10a-b010` — A.9 Genéricos conservan el tipo sin boxing

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:260` · mode: `run`

```rust
fn retry_once<T, E, F>(mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    operation().or_else(|_| operation())
}

fn main() {
    let mut calls = 0;
    let result: Result<&str, &str> = retry_once(|| {
        calls += 1;
        if calls == 2 { Ok("listo") } else { Err("temporal") }
    });

    assert_eq!(result, Ok("listo"));
}
```

### `ch10a-b011` — A.10 No hace falta un `AppError` global

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:285` · mode: `reference`

```text
domain/email/error.rs          EmailError
domain/order/error.rs          CancelOrderError
application/register/error.rs RegisterUserError
adapters/database/error.rs     RepositoryError
```

### `ch10a-b012` — A.10 No hace falta un `AppError` global

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:294` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum EmailError {
    MissingAt,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

#[derive(Debug, PartialEq)]
enum RegisterUserError {
    InvalidEmail(EmailError),
    DuplicateEmail,
    Repository(RepositoryError),
}

impl From<EmailError> for RegisterUserError {
    fn from(source: EmailError) -> Self {
        Self::InvalidEmail(source)
    }
}

fn main() {
    let error = RegisterUserError::from(EmailError::MissingAt);
    assert_eq!(error, RegisterUserError::InvalidEmail(EmailError::MissingAt));
}
```

### `ch10a-b013` — A.11 Structs concretos dentro de un enum wrapper

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:330` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct EmailTooLong {
    maximum: usize,
    actual: usize,
}

impl EmailTooLong {
    fn excess(&self) -> usize {
        self.actual.saturating_sub(self.maximum)
    }
}

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
    TooLong(EmailTooLong),
}

fn main() {
    let detail = EmailTooLong {
        maximum: 254,
        actual: 260,
    };
    assert_eq!(detail.excess(), 6);
    assert!(matches!(EmailError::TooLong(detail), EmailError::TooLong(_)));
}
```

### `ch10a-b014` — A.12 Enfoque híbrido: concreto dentro, común fuera

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:366` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

trait CodedError: Error {
    fn code(&self) -> &'static str;
}

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
}

impl fmt::Display for EmailError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "email vacío"),
            Self::MissingAt => write!(formatter, "falta @"),
        }
    }
}

impl Error for EmailError {}

impl CodedError for EmailError {
    fn code(&self) -> &'static str {
        match self {
            Self::Empty => "email.empty",
            Self::MissingAt => "email.missing_at",
        }
    }
}

fn public_code(error: &dyn CodedError) -> &'static str {
    error.code()
}

fn main() {
    let error = EmailError::MissingAt;
    assert_eq!(public_code(&error), "email.missing_at");
    assert_eq!(error, EmailError::MissingAt);
}
```

### `ch10a-b015` — A.13 Borrar el tipo en la frontera

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:417` · mode: `run`

```rust
use std::error::Error;

fn load_limit(input: &str) -> Result<u32, Box<dyn Error>> {
    let limit = input.parse::<u32>()?;
    Ok(limit)
}

fn run() -> Result<(), Box<dyn Error>> {
    assert_eq!(load_limit("25")?, 25);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
```

### `ch10a-b016` — A.14 `thiserror` reduce boilerplate, no cambia el contrato

Source: `10A.Anexo-errores-enum-vs-dyn-trait.md:441` · mode: `run`

```rust
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
enum EmailError {
    #[error("el email no puede estar vacío")]
    Empty,
    #[error("el email debe contener @")]
    MissingAt,
    #[error("email demasiado largo: máximo {maximum}, actual {actual}")]
    TooLong { maximum: usize, actual: usize },
}

fn main() {
    let error = EmailError::TooLong {
        maximum: 254,
        actual: 260,
    };
    assert!(error.to_string().contains("260"));
}
```

## 11.Closures

### `ch11-b001` — 11.1 Sintaxis y valor de retorno

Source: `11.Closures.md:13` · mode: `run`

```rust
fn main() {
    let no_arguments = || 42;
    let one_argument = |value| value + 1;
    let two_arguments = |left, right| left + right;
    let block = |value: i32| -> i32 {
        let doubled = value * 2;
        doubled + 1
    };

    assert_eq!(no_arguments(), 42);
    assert_eq!(one_argument(1), 2);
    assert_eq!(two_arguments(2, 3), 5);
    assert_eq!(block(4), 9);
}
```

### `ch11-b002` — 11.2 Cada closure tiene un tipo único

Source: `11.Closures.md:36` · mode: `run`

```rust
fn apply<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(value)
}

fn main() {
    let increment = |value| value + 1;
    let double = |value| value * 2;

    assert_eq!(apply(10, increment), 11);
    assert_eq!(apply(10, double), 20);
}
```

### `ch11-b003` — 11.3 La inferencia fija una closure concreta

Source: `11.Closures.md:61` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let identity = |value| value;
    let number = identity(10);
    let text = identity(String::from("hola"));
    // error[E0308]: la primera llamada fijó el tipo como i32

    println!("{number} {text}");
}
```

### `ch11-b004` — 11.3 La inferencia fija una closure concreta

Source: `11.Closures.md:74` · mode: `run`

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    assert_eq!(identity(10), 10);
    assert_eq!(identity(String::from("hola")), "hola");
}
```

### `ch11-b005` — 11.4 Captura mínima del entorno

Source: `11.Closures.md:89` · mode: `run`

```rust
fn main() {
    let factor = 10;
    let multiply = |value| value * factor;

    assert_eq!(multiply(3), 30);
    assert_eq!(factor, 10);
}
```

### `ch11-b006` — 11.5 Lectura: captura compartida y `Fn`

Source: `11.Closures.md:105` · mode: `run`

```rust
fn call_twice<F>(operation: F) -> (usize, usize)
where
    F: Fn() -> usize,
{
    (operation(), operation())
}

fn main() {
    let name = String::from("Ada");
    let length = || name.len();

    assert_eq!(call_twice(length), (3, 3));
    assert_eq!(name, "Ada");
}
```

### `ch11-b007` — 11.6 Mutación: captura exclusiva y `FnMut`

Source: `11.Closures.md:128` · mode: `run`

```rust
fn repeat<F>(times: usize, mut operation: F)
where
    F: FnMut(usize),
{
    for index in 0..times {
        operation(index);
    }
}

fn main() {
    let mut total = 0;
    repeat(4, |index| total += index);
    assert_eq!(total, 6);
}
```

### `ch11-b008` — 11.6 Mutación: captura exclusiva y `FnMut`

Source: `11.Closures.md:149` · mode: `run`

```rust
fn main() {
    let mut count = 0;
    {
        let mut increment = || count += 1;
        increment();
        increment();
    }
    assert_eq!(count, 2);
}
```

### `ch11-b009` — 11.7 Consumo: mover fuera y `FnOnce`

Source: `11.Closures.md:165` · mode: `run`

```rust
fn call_once<F, T>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    operation()
}

fn main() {
    let text = String::from("owned");
    let consume = move || text;

    assert_eq!(call_once(consume), "owned");
}
```

### `ch11-b010` — 11.7 Consumo: mover fuera y `FnOnce`

Source: `11.Closures.md:185` · mode: `compile_fail`

```rust,compile_fail
fn main() {
    let text = String::from("owned");
    let consume = move || text;

    let first = consume();
    let second = consume();
    // error[E0382]: use of moved value: consume

    println!("{first} {second}");
}
```

### `ch11-b011` — 11.8 `move` decide la captura, no el número de llamadas

Source: `11.Closures.md:202` · mode: `run`

```rust
fn call_twice<F>(operation: F)
where
    F: Fn(),
{
    operation();
    operation();
}

fn main() {
    let text = String::from("hola");
    let print_length = move || println!("{}", text.len());

    call_twice(print_length);
}
```

### `ch11-b012` — 11.10 El receptor de la API determina el bound

Source: `11.Closures.md:241` · mode: `run`

```rust
fn once<F>(operation: F)
where
    F: FnOnce(),
{
    operation();
}

fn many<F>(mut operation: F)
where
    F: FnMut(),
{
    operation();
    operation();
}

fn shared<F>(operation: &F)
where
    F: Fn(),
{
    operation();
}

fn main() {
    once(|| println!("una"));

    let mut calls = 0;
    many(|| calls += 1);
    assert_eq!(calls, 2);

    let label = String::from("compartida");
    let show = || println!("{label}");
    shared(&show);
    shared(&show);
}
```

### `ch11-b013` — 11.11 Fallback perezoso y `FnOnce`

Source: `11.Closures.md:284` · mode: `run`

```rust
fn unwrap_or_else<T, F>(value: Option<T>, fallback: F) -> T
where
    F: FnOnce() -> T,
{
    match value {
        Some(value) => value,
        None => fallback(),
    }
}

fn main() {
    let fallback = String::from("anonymous");
    let name = unwrap_or_else(None, || fallback);
    assert_eq!(name, "anonymous");
}
```

### `ch11-b014` — 11.12 Callbacks repetidos y estado

Source: `11.Closures.md:308` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rectangles = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut calls = 0;

    rectangles.sort_by_key(|rectangle| {
        calls += 1;
        rectangle.width
    });

    assert_eq!(rectangles[0].width, 3);
    assert!(calls >= rectangles.len());
}
```

### `ch11-b015` — 11.13 Closures `move` y threads

Source: `11.Closures.md:339` · mode: `run`

```rust
use std::thread;

fn main() {
    let values = vec![1, 2, 3];
    let handle = thread::spawn(move || values.into_iter().sum::<i32>());

    assert_eq!(handle.join().unwrap(), 6);
}
```

### `ch11-b016` — 11.14 Función nombrada, function item y puntero a función

Source: `11.Closures.md:356` · mode: `run`

```rust
fn normalize(input: &str) -> String {
    input.trim().to_lowercase()
}

fn apply<F>(input: &str, operation: F) -> String
where
    F: Fn(&str) -> String,
{
    operation(input)
}

fn main() {
    assert_eq!(apply(" Rust ", normalize), "rust");

    let pointer: fn(&str) -> String = normalize;
    assert_eq!(pointer(" BOOK "), "book");
}
```

### `ch11-b017` — 11.15 Pasar closures con el bound mínimo

Source: `11.Closures.md:382` · mode: `run`

```rust
fn transform<T, U, F>(value: T, operation: F) -> U
where
    F: FnOnce(T) -> U,
{
    operation(value)
}

fn main() {
    let suffix = String::from("!");
    let result = transform(String::from("hola"), |mut text| {
        text.push_str(&suffix);
        text
    });
    assert_eq!(result, "hola!");
}
```

### `ch11-b018` — 11.16 Devolver una closure con `impl Fn`

Source: `11.Closures.md:406` · mode: `run`

```rust
fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |value| value + amount
}

fn main() {
    let add_ten = make_adder(10);
    assert_eq!(add_ten(5), 15);
    assert_eq!(add_ten(7), 17);
}
```

### `ch11-b019` — 11.17 Elegir entre closures en runtime

Source: `11.Closures.md:424` · mode: `run`

```rust
fn make_operation(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|value| value * 2),
        _ => Box::new(|value| value + 1),
    }
}

fn main() {
    assert_eq!(make_operation("double")(4), 8);
    assert_eq!(make_operation("increment")(4), 5);
}
```

### `ch11-b020` — 11.18 Closures almacenadas en structs

Source: `11.Closures.md:444` · mode: `run`

```rust
struct Validator<F> {
    predicate: F,
}

impl<F> Validator<F>
where
    F: Fn(&str) -> bool,
{
    fn is_valid(&self, input: &str) -> bool {
        (self.predicate)(input)
    }
}

fn main() {
    let minimum = 3;
    let validator = Validator {
        predicate: |input: &str| input.len() >= minimum,
    };
    assert!(validator.is_valid("rust"));
    assert!(!validator.is_valid("rs"));
}
```

### `ch11-b021` — 11.19 Lifetimes de closures prestadas

Source: `11.Closures.md:474` · mode: `run`

```rust
fn main() {
    let prefix = String::from("ru");
    let starts_with_prefix = |candidate: &str| candidate.starts_with(&prefix);

    assert!(starts_with_prefix("rust"));
    assert_eq!(prefix, "ru");
}
```

### `ch11-b022` — 11.20 Fábricas: poseer la configuración

Source: `11.Closures.md:492` · mode: `compile_fail`

```rust,compile_fail
fn bad_filter() -> impl Fn(&str) -> bool {
    let prefix = String::from("ru");
    |candidate| candidate.starts_with(&prefix)
    // error[E0373]: la closure puede sobrevivir a prefix
}

fn main() {}
```

### `ch11-b023` — 11.20 Fábricas: poseer la configuración

Source: `11.Closures.md:504` · mode: `run`

```rust
fn prefix_filter(prefix: impl Into<String>) -> impl Fn(&str) -> bool {
    let prefix = prefix.into();
    move |candidate| candidate.starts_with(&prefix)
}

fn main() {
    let is_rust = prefix_filter("ru");
    assert!(is_rust("rust"));
    assert!(!is_rust("book"));
}
```

### `ch11-b024` — 11.21 Closures e iteradores

Source: `11.Closures.md:523` · mode: `run`

```rust
fn normalized_names(names: &[String]) -> Vec<String> {
    names
        .iter()
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.is_empty())
        .collect()
}

fn main() {
    let names = vec![String::from(" Ada "), String::from("  ")];
    assert_eq!(normalized_names(&names), [String::from("ada")]);
    assert_eq!(names[0], " Ada ");
}
```

### `ch11-b025` — 11.22 Salidas prestadas y owned

Source: `11.Closures.md:545` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    name: String,
    active: bool,
}

fn active_names(users: &[User]) -> Vec<&str> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.as_str())
        .collect()
}

fn active_names_owned(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.clone())
        .collect()
}

fn main() {
    let users = [User {
        name: String::from("Ada"),
        active: true,
    }];
    assert_eq!(active_names(&users), ["Ada"]);
    assert_eq!(active_names_owned(&users), [String::from("Ada")]);
}
```

### `ch11-b026` — 11.23 Devolver iteradores que contienen closures

Source: `11.Closures.md:584` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    name: String,
}

fn names_with_prefix<'a>(
    users: &'a [User],
    prefix: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
    users
        .iter()
        .map(|user| user.name.as_str())
        .filter(move |name| name.starts_with(prefix))
}

fn main() {
    let users = [
        User { name: String::from("Ada") },
        User { name: String::from("Grace") },
    ];
    assert_eq!(names_with_prefix(&users, "A").collect::<Vec<_>>(), ["Ada"]);
}
```

### `ch11-b027` — 11.24 Caso práctico: reintentos

Source: `11.Closures.md:615` · mode: `compile_only`

```rust,no_run
use std::num::NonZeroUsize;

fn retry<T, E, F>(attempts: NonZeroUsize, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 1..=attempts.get() {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error) if attempt == attempts.get() => return Err(error),
            Err(_) => {}
        }
    }
    unreachable!("NonZeroUsize garantiza al menos un intento")
}

fn main() {
    let mut calls = 0;
    let result = retry(NonZeroUsize::new(3).unwrap(), || {
        calls += 1;
        (calls == 3).then_some("listo").ok_or("temporal")
    });

    assert_eq!(result, Ok("listo"));
    assert_eq!(calls, 3);
}
```

### `ch11-b028` — 11.25 Closure frente a trait de dominio

Source: `11.Closures.md:650` · mode: `run`

```rust
#[derive(Debug, Clone, Copy)]
struct Cents(u64);

#[derive(Debug)]
struct PaymentError;

trait PaymentProcessor {
    fn charge(&self, amount: Cents) -> Result<u64, PaymentError>;
    fn refund(&self, payment_id: u64) -> Result<(), PaymentError>;
}

fn apply_discount<F>(amount: Cents, discount: F) -> Cents
where
    F: FnOnce(Cents) -> Cents,
{
    discount(amount)
}

fn main() {
    let reduced = apply_discount(Cents(1_000), |amount| Cents(amount.0 - 100));
    assert_eq!(reduced.0, 900);
}
```

### `ch11-b029` — 11.26 Async closures: adelanto

Source: `11.Closures.md:681` · mode: `run`

```rust
fn main() {
    let offset = 1_u32;
    let operation = async move |value: u32| value + offset;
    let _future = operation(41);
}
```

## 12.Iteradores

### `ch12-b001` — 12.1 `Iterator` es una máquina de `next`

Source: `12.Iteradores.md:13` · mode: `run`

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

### `ch12-b002` — 12.1 `Iterator` es una máquina de `next`

Source: `12.Iteradores.md:28` · mode: `run`

```rust
fn main() {
    let numbers = [10, 20, 30];
    let mut iterator = numbers.iter();

    assert_eq!(iterator.next(), Some(&10));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&30));
    assert_eq!(iterator.next(), None);
}
```

### `ch12-b003` — 12.2 Elegir la entrada define ownership

Source: `12.Iteradores.md:46` · mode: `run`

```rust
fn main() {
    let mut names = vec![String::from("Ada"), String::from("Grace")];

    let observed: Vec<&str> = names.iter().map(String::as_str).collect();
    assert_eq!(observed, ["Ada", "Grace"]);

    names.iter_mut().for_each(|name| name.make_ascii_uppercase());
    assert_eq!(names, ["ADA", "GRACE"]);

    let lengths: Vec<usize> = names.into_iter().map(|name| name.len()).collect();
    assert_eq!(lengths, [3, 5]);
}
```

### `ch12-b004` — 12.2 Elegir la entrada define ownership

Source: `12.Iteradores.md:61` · mode: `reference`

```text
iter()      Item = &T       observa; la colección sobrevive
iter_mut()  Item = &mut T   modifica con exclusividad
into_iter() Item = T        consume o transfiere los elementos
```

### `ch12-b005` — 12.3 `for` usa `IntoIterator`

Source: `12.Iteradores.md:73` · mode: `run`

```rust
fn main() {
    let mut names = vec![String::from("Ada"), String::from("Grace")];

    for name in &names {
        assert!(!name.is_empty());
    }

    for name in &mut names {
        name.push('!');
    }

    let owned: Vec<String> = names.into_iter().collect();
    assert_eq!(owned, [String::from("Ada!"), String::from("Grace!")]);
}
```

### `ch12-b006` — 12.4 Adaptadores lazy y consumidores

Source: `12.Iteradores.md:96` · mode: `run`

```rust
use std::cell::Cell;

fn main() {
    let calls = Cell::new(0);
    let pipeline = [1, 2, 3].into_iter().map(|value| {
        calls.set(calls.get() + 1);
        value * 10
    });

    assert_eq!(calls.get(), 0);
    let result: Vec<_> = pipeline.collect();
    assert_eq!(calls.get(), 3);
    assert_eq!(result, [10, 20, 30]);
}
```

### `ch12-b007` — 12.5 Seguir el tipo de `Item`

Source: `12.Iteradores.md:121` · mode: `reference`

```text
values.iter()                         Item = &i32
      .copied()                       Item = i32
      .filter(|value| value % 2 == 0) Item = i32
      .map(|value| value * 2)         Item = i32
      .collect::<Vec<_>>()            Vec<i32>
```

### `ch12-b008` — 12.6 `map`, `filter` y `copied`

Source: `12.Iteradores.md:135` · mode: `run`

```rust
fn doubled_evens(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .map(|value| value * 2)
        .collect()
}

fn main() {
    assert_eq!(doubled_evens(&[1, 2, 3, 4]), [4, 8]);
}
```

### `ch12-b009` — 12.7 `filter_map`: descartar debe ser política explícita

Source: `12.Iteradores.md:160` · mode: `run`

```rust
fn parse_valid_numbers(inputs: &[&str]) -> Vec<u32> {
    inputs
        .iter()
        .filter_map(|input| input.parse::<u32>().ok())
        .collect()
}

fn main() {
    assert_eq!(parse_valid_numbers(&["10", "x", "20"]), [10, 20]);
}
```

### `ch12-b010` — 12.8 `flat_map` y `flatten`: uno a muchos

Source: `12.Iteradores.md:179` · mode: `run`

```rust
fn main() {
    let lines = ["hello world", "rust language"];
    let words: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect();

    assert_eq!(words, ["hello", "world", "rust", "language"]);
}
```

### `ch12-b011` — 12.8 `flat_map` y `flatten`: uno a muchos

Source: `12.Iteradores.md:193` · mode: `run`

```rust
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(nested.into_iter().flatten().collect::<Vec<_>>(), [1, 2, 3, 4]);

    let optional = [Some(1), None, Some(3)];
    assert_eq!(optional.into_iter().flatten().collect::<Vec<_>>(), [1, 3]);
}
```

### `ch12-b012` — 12.9 Adaptadores estructurales

Source: `12.Iteradores.md:209` · mode: `run`

```rust
fn main() {
    let numbered: Vec<_> = ["Ada", "Grace"]
        .into_iter()
        .enumerate()
        .collect();
    assert_eq!(numbered, [(0, "Ada"), (1, "Grace")]);

    let paired: Vec<_> = ["Ada", "Grace"]
        .into_iter()
        .zip([36, 85])
        .collect();
    assert_eq!(paired, [("Ada", 36), ("Grace", 85)]);

    let window: Vec<_> = (0..10).skip(2).take(3).collect();
    assert_eq!(window, [2, 3, 4]);

    let chained: Vec<_> = [1, 2].into_iter().chain([3, 4]).collect();
    assert_eq!(chained, [1, 2, 3, 4]);
}
```

### `ch12-b013` — 12.10 Consumidores con cortocircuito

Source: `12.Iteradores.md:237` · mode: `run`

```rust
fn first_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn main() {
    let values = [1, 3, 4, 8];
    assert_eq!(first_even(&values), Some(4));
    assert!(values.iter().any(|value| *value > 5));
    assert!(values.iter().all(|value| *value > 0));
}
```

### `ch12-b014` — 12.11 `collect` está dirigido por el tipo destino

Source: `12.Iteradores.md:256` · mode: `run`

```rust
use std::collections::HashMap;

fn main() {
    let vector: Vec<_> = (1..=3).map(|value| value * 2).collect();
    let text: String = ['R', 'u', 's', 't'].into_iter().collect();
    let index: HashMap<_, _> = [(7_u64, "Ada"), (8, "Grace")]
        .into_iter()
        .collect();

    assert_eq!(vector, [2, 4, 6]);
    assert_eq!(text, "Rust");
    assert_eq!(index.get(&7), Some(&"Ada"));
}
```

### `ch12-b015` — 12.12 `collect` sobre `Result` falla rápido

Source: `12.Iteradores.md:278` · mode: `run`

```rust
use std::num::ParseIntError;

fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .split_whitespace()
        .map(str::parse::<i32>)
        .collect()
}

fn main() {
    assert_eq!(parse_numbers("10 20 -3"), Ok(vec![10, 20, -3]));
    assert!(parse_numbers("10 x 20").is_err());
}
```

### `ch12-b016` — 12.13 Agregar con `sum`, `fold` y `reduce`

Source: `12.Iteradores.md:300` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Stats {
    count: usize,
    sum: i64,
}

fn stats(values: &[i64]) -> Stats {
    values.iter().copied().fold(
        Stats { count: 0, sum: 0 },
        |stats, value| Stats {
            count: stats.count + 1,
            sum: stats.sum + value,
        },
    )
}

fn main() {
    assert_eq!(stats(&[2, 3, 5]), Stats { count: 3, sum: 10 });
    assert_eq!([2, 3, 5].into_iter().sum::<i32>(), 10);
    assert_eq!([2, 3, 5].into_iter().reduce(i32::max), Some(5));
    assert_eq!([].into_iter().reduce(i32::max), None);
}
```

### `ch12-b017` — 12.14 `try_fold` combina acumulación y fallo

Source: `12.Iteradores.md:331` · mode: `run`

```rust
fn checked_sum(values: impl IntoIterator<Item = i64>) -> Option<i64> {
    values.into_iter().try_fold(0_i64, i64::checked_add)
}

fn main() {
    assert_eq!(checked_sum([1, 2, 3]), Some(6));
    assert_eq!(checked_sum([i64::MAX, 1]), None);
}
```

### `ch12-b018` — 12.14 `try_fold` combina acumulación y fallo

Source: `12.Iteradores.md:344` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
enum SumError {
    InvalidNumber { index: usize },
    Overflow { index: usize },
}

fn parse_and_sum(inputs: &[&str]) -> Result<u32, SumError> {
    inputs.iter().enumerate().try_fold(0_u32, |total, (index, input)| {
        let value = input
            .parse::<u32>()
            .map_err(|_| SumError::InvalidNumber { index })?;
        total
            .checked_add(value)
            .ok_or(SumError::Overflow { index })
    })
}

fn main() {
    assert_eq!(parse_and_sum(&["10", "20"]), Ok(30));
    assert_eq!(
        parse_and_sum(&["10", "x"]),
        Err(SumError::InvalidNumber { index: 1 })
    );
}
```

### `ch12-b019` — 12.15 Efectos: `inspect`, `for_each` o `for`

Source: `12.Iteradores.md:377` · mode: `run`

```rust
fn main() {
    let mut seen = Vec::new();
    let result: Vec<_> = [1, 2, 3, 4]
        .into_iter()
        .inspect(|value| seen.push(*value))
        .filter(|value| value % 2 == 0)
        .collect();

    assert_eq!(seen, [1, 2, 3, 4]);
    assert_eq!(result, [2, 4]);
}
```

### `ch12-b020` — 12.16 `copied` frente a `cloned`: copiar tarde

Source: `12.Iteradores.md:397` · mode: `run`

```rust
fn selected_owned(names: &[String]) -> Vec<String> {
    names
        .iter()
        .filter(|name| name.starts_with('A'))
        .cloned()
        .collect()
}

fn main() {
    let names = vec![String::from("Ada"), String::from("Grace")];
    assert_eq!(selected_owned(&names), [String::from("Ada")]);
    assert_eq!(names.len(), 2);
}
```

### `ch12-b021` — 12.17 Iteradores que prestan de una entrada

Source: `12.Iteradores.md:419` · mode: `run`

```rust
fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn main() {
    let text = String::from("\n Rust \n\n ownership ");
    assert_eq!(non_empty_lines(&text).collect::<Vec<_>>(), ["Rust", "ownership"]);
}
```

### `ch12-b022` — 12.18 Iteradores owned devueltos

Source: `12.Iteradores.md:439` · mode: `run`

```rust
fn default_names() -> impl Iterator<Item = String> {
    vec![String::from("Ada"), String::from("Grace")].into_iter()
}

fn main() {
    let names: Vec<_> = default_names().collect();
    assert_eq!(names, [String::from("Ada"), String::from("Grace")]);
}
```

### `ch12-b023` — 12.19 No se pueden prestar datos locales

Source: `12.Iteradores.md:456` · mode: `compile_fail`

```rust,compile_fail
fn words() -> impl Iterator<Item = &'static str> {
    let text = String::from("hello rust");
    text.split_whitespace()
    // error[E0515]: devuelve un valor que referencia text
}

fn main() {}
```

### `ch12-b024` — 12.19 No se pueden prestar datos locales

Source: `12.Iteradores.md:468` · mode: `run`

```rust
fn words(input: &str) -> impl Iterator<Item = &str> {
    input.split_whitespace()
}

fn owned_words(input: String) -> impl Iterator<Item = String> {
    input
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .into_iter()
}

fn main() {
    assert_eq!(words("hello rust").collect::<Vec<_>>(), ["hello", "rust"]);
    assert_eq!(
        owned_words(String::from("hello rust")).collect::<Vec<_>>(),
        [String::from("hello"), String::from("rust")]
    );
}
```

### `ch12-b025` — 12.20 Aceptar `IntoIterator`

Source: `12.Iteradores.md:496` · mode: `run`

```rust
fn total<I>(items: I) -> i64
where
    I: IntoIterator<Item = i64>,
{
    items.into_iter().sum()
}

fn main() {
    assert_eq!(total([1, 2, 3]), 6);
    assert_eq!(total(vec![4, 5]), 9);
    assert_eq!(total((1..=4).map(i64::from)), 10);
}
```

### `ch12-b026` — 12.21 Devolver `impl Iterator`

Source: `12.Iteradores.md:517` · mode: `run`

```rust
fn even_numbers_up_to(limit: u32) -> impl Iterator<Item = u32> {
    (0..=limit).filter(|value| value % 2 == 0)
}

fn main() {
    assert_eq!(even_numbers_up_to(7).collect::<Vec<_>>(), [0, 2, 4, 6]);
}
```

### `ch12-b027` — 12.22 Implementar un iterador propio

Source: `12.Iteradores.md:533` · mode: `run`

```rust
use std::iter::FusedIterator;

#[derive(Debug)]
struct Countdown {
    next: u32,
}

impl Countdown {
    fn new(start: u32) -> Self {
        Self { next: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            return None;
        }
        let current = self.next;
        self.next -= 1;
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Countdown {}
impl FusedIterator for Countdown {}

fn main() {
    let mut countdown = Countdown::new(3);
    assert_eq!(countdown.len(), 3);
    assert_eq!(countdown.by_ref().collect::<Vec<_>>(), [3, 2, 1]);
    assert_eq!(countdown.next(), None);
}
```

### `ch12-b028` — 12.24 Caso práctico: consultas prestadas y owned

Source: `12.Iteradores.md:586` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    active: bool,
}

fn active_names(users: &[User]) -> Vec<&str> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.as_str())
        .collect()
}

fn active_names_owned(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.clone())
        .collect()
}

fn main() {
    let users = [
        User { name: String::from("Ada"), active: true },
        User { name: String::from("Grace"), active: false },
    ];
    assert_eq!(active_names(&users), ["Ada"]);
    assert_eq!(active_names_owned(&users), [String::from("Ada")]);
}
```

### `ch12-b029` — 12.25 Validación masiva con contexto

Source: `12.Iteradores.md:625` · mode: `run`

```rust
use std::num::ParseIntError;

#[derive(Debug)]
struct ParseAtIndexError {
    index: usize,
    source: ParseIntError,
}

fn parse_with_indices(inputs: &[&str]) -> Result<Vec<u32>, ParseAtIndexError> {
    inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            input
                .parse::<u32>()
                .map_err(|source| ParseAtIndexError { index, source })
        })
        .collect()
}

fn main() {
    let error = parse_with_indices(&["10", "bad", "20"]).unwrap_err();
    assert_eq!(error.index, 1);
    assert!(!error.source.to_string().is_empty());
}
```

### `ch12-b030` — 12.27 Cuándo un `for` comunica mejor

Source: `12.Iteradores.md:672` · mode: `run`

```rust
fn partition_parse(inputs: &[&str]) -> (Vec<u32>, Vec<usize>) {
    let mut values = Vec::new();
    let mut invalid_indices = Vec::new();

    for (index, input) in inputs.iter().enumerate() {
        match input.parse::<u32>() {
            Ok(value) => values.push(value),
            Err(_) => invalid_indices.push(index),
        }
    }

    (values, invalid_indices)
}

fn main() {
    assert_eq!(partition_parse(&["10", "x", "20"]), (vec![10, 20], vec![1]));
}
```

## 13.Composición-declarativa

### `ch13-b001` — 13.1 Declarativo describe intención

Source: `13.Composición-declarativa.md:13` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    email: String,
    active: bool,
}

fn active_emails(users: &[User]) -> Vec<&str> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.email.as_str())
        .collect()
}

fn main() {
    let users = [
        User { email: String::from("ada@example.com"), active: true },
        User { email: String::from("grace@example.com"), active: false },
    ];
    assert_eq!(active_emails(&users), ["ada@example.com"]);
}
```

### `ch13-b002` — 13.2 Una etapa, una intención

Source: `13.Composición-declarativa.md:43` · mode: `reference`

```text
fuente → selección/validación → transformación → agregación/frontera
```

### `ch13-b003` — 13.2 Una etapa, una intención

Source: `13.Composición-declarativa.md:49` · mode: `run`

```rust
#[derive(Debug)]
enum Status {
    Draft,
    Ready,
}

#[derive(Debug)]
struct Order {
    status: Status,
    total_cents: u64,
}

fn is_billable(order: &Order) -> bool {
    matches!(order.status, Status::Ready) && order.total_cents > 0
}

fn billable_total(orders: &[Order]) -> u64 {
    orders
        .iter()
        .filter(|order| is_billable(order))
        .map(|order| order.total_cents)
        .sum()
}

fn main() {
    let orders = [
        Order { status: Status::Ready, total_cents: 500 },
        Order { status: Status::Draft, total_cents: 900 },
    ];
    assert_eq!(billable_total(&orders), 500);
}
```

### `ch13-b004` — 13.3 Imperativo y declarativo pueden ser equivalentes

Source: `13.Composición-declarativa.md:89` · mode: `run`

```rust
#[derive(Debug)]
struct OrderLine {
    quantity: u32,
    unit_cents: u64,
}

fn total_imperative(lines: &[OrderLine]) -> u64 {
    let mut total = 0;
    for line in lines {
        if line.quantity > 0 {
            total += u64::from(line.quantity) * line.unit_cents;
        }
    }
    total
}

fn total_declarative(lines: &[OrderLine]) -> u64 {
    lines
        .iter()
        .filter(|line| line.quantity > 0)
        .map(|line| u64::from(line.quantity) * line.unit_cents)
        .sum()
}

fn main() {
    let lines = [
        OrderLine { quantity: 2, unit_cents: 150 },
        OrderLine { quantity: 0, unit_cents: 999 },
    ];
    assert_eq!(total_imperative(&lines), 300);
    assert_eq!(total_declarative(&lines), 300);
}
```

### `ch13-b005` — 13.4 `map` conserva la forma exterior

Source: `13.Composición-declarativa.md:130` · mode: `run`

```rust
fn main() {
    let optional = Some("Rust").map(str::len);
    let fallible: Result<usize, &str> = Ok("Rust").map(str::len);
    let many: Vec<usize> = ["Rust", "book"]
        .into_iter()
        .map(str::len)
        .collect();

    assert_eq!(optional, Some(4));
    assert_eq!(fallible, Ok(4));
    assert_eq!(many, [4, 4]);
}
```

### `ch13-b006` — 13.4 `map` conserva la forma exterior

Source: `13.Composición-declarativa.md:145` · mode: `reference`

```text
Option<T>      --map--> Option<U>
Result<T, E>   --map--> Result<U, E>
Iterator<T>    --map--> Iterator<U>
```

### `ch13-b007` — 13.5 `and_then` evita contextos anidados

Source: `13.Composición-declarativa.md:157` · mode: `run`

```rust
fn parse_non_zero(input: &str) -> Option<u16> {
    input.parse::<u16>().ok().filter(|port| *port != 0)
}

fn main() {
    let raw = Some("8080");
    let nested = raw.map(parse_non_zero);
    let flat = raw.and_then(parse_non_zero);

    assert_eq!(nested, Some(Some(8080)));
    assert_eq!(flat, Some(8080));
}
```

### `ch13-b008` — 13.6 Puentes entre `Option` y `Result`

Source: `13.Composición-declarativa.md:178` · mode: `run`

```rust
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingPort,
    InvalidPort,
}

fn required_port(value: Option<&str>) -> Result<u16, ConfigError> {
    value
        .ok_or(ConfigError::MissingPort)?
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort)
}

fn optional_port(value: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    value.map(str::parse::<u16>).transpose()
}

fn main() {
    assert_eq!(required_port(Some("8080")), Ok(8080));
    assert_eq!(required_port(None), Err(ConfigError::MissingPort));
    assert_eq!(optional_port(None), Ok(None));
    assert_eq!(optional_port(Some("8080")), Ok(Some(8080)));
}
```

### `ch13-b009` — 13.7 Separar efecto externo y transformación pura

Source: `13.Composición-declarativa.md:212` · mode: `run`

```rust
use std::num::ParseIntError;

fn parse_optional_port(raw: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    raw.map(str::parse::<u16>).transpose()
}

fn main() {
    // La frontera real haría: std::env::var("PORT").ok()
    let raw = Some(String::from("8080"));
    let parsed = parse_optional_port(raw.as_deref());
    assert_eq!(parsed, Ok(Some(8080)));
}
```

### `ch13-b010` — 13.8 `collect` sobre `Result`: política fail-fast

Source: `13.Composición-declarativa.md:231` · mode: `run`

```rust
use std::num::ParseIntError;

fn parse_all(inputs: &[&str]) -> Result<Vec<u32>, ParseIntError> {
    inputs
        .iter()
        .map(|input| input.parse::<u32>())
        .collect()
}

fn main() {
    assert_eq!(parse_all(&["10", "20"]), Ok(vec![10, 20]));
    assert!(parse_all(&["10", "bad", "20"]).is_err());
}
```

### `ch13-b011` — 13.9 Acumular todos los rechazos es otra API

Source: `13.Composición-declarativa.md:253` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Record {
    id: u64,
    name: String,
}

#[derive(Debug, PartialEq)]
enum ImportError {
    MissingField { line: usize },
    InvalidId { line: usize },
}

#[derive(Debug, PartialEq)]
struct ImportReport {
    accepted: Vec<Record>,
    rejected: Vec<ImportError>,
}

fn parse_line(line_number: usize, line: &str) -> Result<Record, ImportError> {
    let (raw_id, name) = line
        .split_once(',')
        .ok_or(ImportError::MissingField { line: line_number })?;
    let id = raw_id
        .trim()
        .parse()
        .map_err(|_| ImportError::InvalidId { line: line_number })?;
    Ok(Record { id, name: name.trim().to_owned() })
}

fn import_all(input: &str) -> ImportReport {
    let mut report = ImportReport { accepted: Vec::new(), rejected: Vec::new() };

    for (index, line) in input.lines().enumerate() {
        match parse_line(index + 1, line) {
            Ok(record) => report.accepted.push(record),
            Err(error) => report.rejected.push(error),
        }
    }
    report
}

fn main() {
    let report = import_all("1,Ada\nbad,Grace\n3,Linus");
    assert_eq!(report.accepted.len(), 2);
    assert_eq!(report.rejected, [ImportError::InvalidId { line: 2 }]);
}
```

### `ch13-b012` — 13.11 `partition`, `unzip` y colecciones dirigidas por tipo

Source: `13.Composición-declarativa.md:322` · mode: `run`

```rust
use std::collections::HashMap;

fn main() {
    let (even, odd): (Vec<u32>, Vec<u32>) =
        (1..=5).partition(|number| number % 2 == 0);
    assert_eq!(even, [2, 4]);
    assert_eq!(odd, [1, 3, 5]);

    let (ids, names): (Vec<u64>, Vec<&str>) =
        [(7, "Ada"), (8, "Grace")].into_iter().unzip();
    assert_eq!(ids, [7, 8]);
    assert_eq!(names, ["Ada", "Grace"]);

    let index: HashMap<u64, &str> = [(7, "Ada"), (8, "Grace")]
        .into_iter()
        .collect();
    assert_eq!(index.get(&8), Some(&"Grace"));
}
```

### `ch13-b013` — 13.12 Materializar una sola vez

Source: `13.Composición-declarativa.md:349` · mode: `run`

```rust
fn total_selected(values: &[u64]) -> u64 {
    values
        .iter()
        .copied()
        .map(|value| value * 2)
        .filter(|value| value % 3 == 0)
        .sum()
}

fn main() {
    assert_eq!(total_selected(&[1, 2, 3, 4, 6]), 18);
}
```

### `ch13-b014` — 13.13 Filtrar antes de copiar

Source: `13.Composición-declarativa.md:370` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    name: String,
}

fn short_names_owned(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.name.len() < 8)
        .map(|user| user.name.clone())
        .collect()
}

fn main() {
    let users = [
        User { name: String::from("Ada") },
        User { name: String::from("Alexandria") },
    ];
    assert_eq!(short_names_owned(&users), [String::from("Ada")]);
}
```

### `ch13-b015` — 13.14 Fronteras prestadas y owned

Source: `13.Composición-declarativa.md:397` · mode: `run`

```rust
fn normalized(input: &[String]) -> impl Iterator<Item = &str> {
    input
        .iter()
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn normalized_owned(input: &[String]) -> Vec<String> {
    normalized(input).map(str::to_owned).collect()
}

fn main() {
    let input = vec![String::from(" Rust "), String::from("  ")];
    assert_eq!(normalized(&input).collect::<Vec<_>>(), ["Rust"]);
    assert_eq!(normalized_owned(&input), [String::from("Rust")]);
}
```

### `ch13-b016` — 13.15 La máquina devuelta también puede prestar

Source: `13.Composición-declarativa.md:425` · mode: `compile_fail`

```rust,compile_fail
fn normalized_words() -> impl Iterator<Item = String> {
    let data = vec![String::from(" hola "), String::from(" mundo ")];
    data.iter().map(|value| value.trim().to_owned())
    // error[E0515]: el iterador contiene un préstamo de data
}

fn main() {}
```

### `ch13-b017` — 13.15 La máquina devuelta también puede prestar

Source: `13.Composición-declarativa.md:437` · mode: `run`

```rust
fn normalized_words() -> impl Iterator<Item = String> {
    vec![String::from(" hola "), String::from(" mundo ")]
        .into_iter()
        .map(|value| value.trim().to_owned())
}

fn main() {
    assert_eq!(
        normalized_words().collect::<Vec<_>>(),
        [String::from("hola"), String::from("mundo")]
    );
}
```

### `ch13-b018` — 13.16 APIs genéricas sobre secuencias

Source: `13.Composición-declarativa.md:456` · mode: `run`

```rust
fn total<I>(prices: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    prices.into_iter().sum()
}

fn main() {
    assert_eq!(total([100, 200]), 300);
    assert_eq!(total(vec![300, 400]), 700);
    assert_eq!(total((1..=3).map(|value| value * 10)), 60);
}
```

### `ch13-b019` — 13.17 Extension traits como vocabulario local

Source: `13.Composición-declarativa.md:477` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Order {
    total_cents: u64,
}

trait OrderIteratorExt: Iterator<Item = Order> + Sized {
    fn billable(self) -> impl Iterator<Item = Order> {
        self.filter(|order| order.total_cents > 0)
    }
}

impl<I> OrderIteratorExt for I where I: Iterator<Item = Order> {}

fn main() {
    let total: u64 = vec![
        Order { total_cents: 0 },
        Order { total_cents: 500 },
    ]
    .into_iter()
    .billable()
    .map(|order| order.total_cents)
    .sum();

    assert_eq!(total, 500);
}
```

### `ch13-b020` — 13.19 Cuando un `for` es la forma declarativa más clara

Source: `13.Composición-declarativa.md:521` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct ReviewReport {
    accepted: Vec<u64>,
    rejected: Vec<u64>,
    total_cents: u64,
}

fn review(requests: &[(u64, u64)]) -> ReviewReport {
    let mut report = ReviewReport {
        accepted: Vec::new(),
        rejected: Vec::new(),
        total_cents: 0,
    };

    for &(id, amount) in requests {
        if amount == 0 {
            report.rejected.push(id);
            continue;
        }
        report.accepted.push(id);
        report.total_cents += amount;
    }
    report
}

fn main() {
    assert_eq!(
        review(&[(1, 500), (2, 0)]),
        ReviewReport { accepted: vec![1], rejected: vec![2], total_cents: 500 }
    );
}
```

## 14.Programación-orientada-a-tipos

### `ch14-b001` — 14.1 Un tipo puede ser una frontera ejecutable

Source: `14.Programación-orientada-a-tipos.md:13` · mode: `run`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    InvalidShape,
}

impl Email {
    pub fn parse(raw: impl Into<String>) -> Result<Self, EmailError> {
        let normalized = raw.into().trim().to_ascii_lowercase();
        if normalized.is_empty() {
            return Err(EmailError::Empty);
        }
        let valid = normalized
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
        valid.then_some(Self(normalized)).ok_or(EmailError::InvalidShape)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn welcome_recipient(email: &Email) -> String {
    format!("Bienvenida para {}", email.as_str())
}

fn main() {
    let email = Email::parse(" ADA@Example.com ").unwrap();
    assert_eq!(email.as_str(), "ada@example.com");
    assert_eq!(welcome_recipient(&email), "Bienvenida para ada@example.com");
    assert_eq!(Email::parse("sin-arroba"), Err(EmailError::InvalidShape));
}
```

### `ch14-b002` — 14.2 Un alias nombra; un newtype distingue

Source: `14.Programación-orientada-a-tipos.md:58` · mode: `run`

```rust
type UserIdAlias = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(u64);

fn user_path(id: UserId) -> String {
    format!("/users/{}", id.0)
}

fn main() {
    let alias: UserIdAlias = 7_u64;
    let raw: u64 = alias; // Es exactamente el mismo tipo.
    assert_eq!(raw, 7);
    assert_eq!(user_path(UserId(7)), "/users/7");
    let _order = OrderId(7);
}
```

### `ch14-b003` — 14.2 Un alias nombra; un newtype distingue

Source: `14.Programación-orientada-a-tipos.md:84` · mode: `compile_fail`

```rust,compile_fail
struct UserId(u64);
struct OrderId(u64);

fn cancel_order(_id: OrderId) {}

fn main() {
    let user = UserId(9);
    cancel_order(user);
    // error[E0308]: se esperaba OrderId, se encontró UserId
}
```

### `ch14-b004` — 14.3 La privacidad sostiene la invariante

Source: `14.Programación-orientada-a-tipos.md:103` · mode: `run`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, PartialEq, Eq)]
pub enum UsernameError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl Username {
    pub const MAX_LEN: usize = 20;

    pub fn parse(raw: &str) -> Result<Self, UsernameError> {
        let value = raw.trim();
        if value.is_empty() {
            return Err(UsernameError::Empty);
        }
        if value.chars().count() > Self::MAX_LEN {
            return Err(UsernameError::TooLong {
                max: Self::MAX_LEN,
                actual: value.chars().count(),
            });
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

fn main() {
    let name = Username::parse("  Ferris  ").unwrap();
    assert_eq!(name.as_str(), "Ferris");
    assert_eq!(name.into_string(), "Ferris");
}
```

### `ch14-b005` — 14.4 «Coste cero» no significa «ABI garantizada»

Source: `14.Programación-orientada-a-tipos.md:152` · mode: `run`

```rust
use std::mem::{align_of, size_of};

#[repr(transparent)]
struct FileDescriptor(i32);

fn main() {
    assert_eq!(size_of::<FileDescriptor>(), size_of::<i32>());
    assert_eq!(align_of::<FileDescriptor>(), align_of::<i32>());
}
```

### `ch14-b006` — 14.5 Parsear, validar y normalizar son decisiones distintas

Source: `14.Programación-orientada-a-tipos.md:175` · mode: `run`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountryCode([u8; 2]);

#[derive(Debug, PartialEq, Eq)]
pub enum CountryCodeError {
    WrongLength,
    NotAsciiAlphabetic,
}

impl CountryCode {
    pub fn parse(raw: &str) -> Result<Self, CountryCodeError> {
        let bytes = raw.as_bytes();
        if bytes.len() != 2 {
            return Err(CountryCodeError::WrongLength);
        }
        if !bytes.iter().all(u8::is_ascii_alphabetic) {
            return Err(CountryCodeError::NotAsciiAlphabetic);
        }
        Ok(Self([
            bytes[0].to_ascii_uppercase(),
            bytes[1].to_ascii_uppercase(),
        ]))
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("invariante: dos letras ASCII")
    }
}

fn main() {
    let lower = CountryCode::parse("es").unwrap();
    let upper = CountryCode::parse("ES").unwrap();
    assert_eq!(lower, upper);
    assert_eq!(lower.as_str(), "ES");
}
```

### `ch14-b007` — 14.6 Conversiones que dicen la verdad

Source: `14.Programación-orientada-a-tipos.md:219` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

#[derive(Debug, PartialEq, Eq)]
enum PortError {
    Zero,
    InvalidNumber,
}

impl TryFrom<&str> for Port {
    type Error = PortError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let value = raw.parse::<u16>().map_err(|_| PortError::InvalidNumber)?;
        (value != 0).then_some(Self(value)).ok_or(PortError::Zero)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}

fn main() {
    let port = Port::try_from("8080").unwrap();
    assert_eq!(u16::from(port), 8080);
    assert_eq!(Port::try_from("0"), Err(PortError::Zero));
}
```

### `ch14-b008` — 14.8 El comportamiento debe respetar unidades y overflow

Source: `14.Programación-orientada-a-tipos.md:272` · mode: `run`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
enum AmountError {
    Overflow,
    Insufficient { available: u64, requested: u64 },
}

impl Cents {
    fn checked_add(self, other: Self) -> Result<Self, AmountError> {
        self.0.checked_add(other.0).map(Self).ok_or(AmountError::Overflow)
    }

    fn checked_sub(self, other: Self) -> Result<Self, AmountError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(AmountError::Insufficient {
                available: self.0,
                requested: other.0,
            })
    }
}

fn main() {
    assert_eq!(Cents(500).checked_add(Cents(250)), Ok(Cents(750)));
    assert_eq!(
        Cents(300).checked_sub(Cents(500)),
        Err(AmountError::Insufficient { available: 300, requested: 500 })
    );
}
```

### `ch14-b009` — 14.9 Un enum reemplaza combinaciones inválidas de flags

Source: `14.Programación-orientada-a-tipos.md:313` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
enum WriteMode {
    CreateNew,
    Replace,
    Append,
}

fn describe(mode: WriteMode) -> &'static str {
    match mode {
        WriteMode::CreateNew => "falla si ya existe",
        WriteMode::Replace => "sustituye el contenido",
        WriteMode::Append => "conserva y añade",
    }
}

fn main() {
    assert_eq!(describe(WriteMode::CreateNew), "falla si ya existe");
    assert_eq!(describe(WriteMode::Replace), "sustituye el contenido");
    assert_eq!(describe(WriteMode::Append), "conserva y añade");
}
```

### `ch14-b010` — 14.10 Reutiliza invariantes de la biblioteca estándar

Source: `14.Programación-orientada-a-tipos.md:342` · mode: `run`

```rust
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RetryLimit(NonZeroU32);

impl RetryLimit {
    fn new(value: u32) -> Option<Self> {
        NonZeroU32::new(value).map(Self)
    }

    fn get(self) -> u32 {
        self.0.get()
    }
}

fn main() {
    assert_eq!(RetryLimit::new(3).map(RetryLimit::get), Some(3));
    assert_eq!(RetryLimit::new(0), None);
}
```

### `ch14-b011` — 14.11 Estados como tipos distintos

Source: `14.Programación-orientada-a-tipos.md:370` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Draft {
    body: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Published {
    body: String,
    revision: u64,
}

impl Draft {
    fn publish(self) -> Published {
        Published { body: self.body, revision: 1 }
    }
}

impl Published {
    fn body(&self) -> &str {
        &self.body
    }
}

fn main() {
    let published = Draft { body: String::from("Rust") }.publish();
    assert_eq!(published.body(), "Rust");
    assert_eq!(published.revision, 1);
}
```

### `ch14-b012` — 14.12 Typestate genérico: representación común, API distinta

Source: `14.Programación-orientada-a-tipos.md:407` · mode: `run`

```rust
use std::marker::PhantomData;

#[derive(Debug)]
struct Disconnected;

#[derive(Debug)]
struct Connected;

#[derive(Debug)]
struct Client<State> {
    endpoint: String,
    state: PhantomData<State>,
}

impl Client<Disconnected> {
    fn new(endpoint: impl Into<String>) -> Self {
        Self { endpoint: endpoint.into(), state: PhantomData }
    }

    fn connect(self) -> Client<Connected> {
        Client { endpoint: self.endpoint, state: PhantomData }
    }
}

impl Client<Connected> {
    fn send(&self, payload: &[u8]) -> usize {
        payload.len()
    }
}

impl<State> Client<State> {
    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

fn main() {
    let client = Client::<Disconnected>::new("local");
    assert_eq!(client.endpoint(), "local");
    let client = client.connect();
    assert_eq!(client.send(b"ping"), 4);
}
```

### `ch14-b013` — 14.13 La máquina de estados queda en las firmas

Source: `14.Programación-orientada-a-tipos.md:462` · mode: `compile_fail`

```rust,compile_fail
use std::marker::PhantomData;

struct Disconnected;
struct Connected;
struct Client<State>(PhantomData<State>);

impl Client<Disconnected> {
    fn new() -> Self { Self(PhantomData) }
}

impl Client<Connected> {
    fn send(&self) {}
}

fn main() {
    Client::<Disconnected>::new().send();
    // no existe send para Client<Disconnected>
}
```

### `ch14-b014` — 14.15 Una transición fallible debe decidir quién conserva el valor

Source: `14.Programación-orientada-a-tipos.md:493` · mode: `run`

```rust
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
struct Draft;

#[derive(Debug, PartialEq, Eq)]
struct Verified;

#[derive(Debug, PartialEq, Eq)]
struct Registration<State> {
    email: String,
    state: PhantomData<State>,
}

#[derive(Debug, PartialEq, Eq)]
enum VerifyError {
    InvalidToken,
}

impl Registration<Draft> {
    fn new(email: impl Into<String>) -> Self {
        Self { email: email.into(), state: PhantomData }
    }

    fn verify(
        self,
        token: &str,
    ) -> Result<Registration<Verified>, (Self, VerifyError)> {
        if token != "known-token" {
            return Err((self, VerifyError::InvalidToken));
        }
        Ok(Registration { email: self.email, state: PhantomData })
    }
}

fn main() {
    let draft = Registration::<Draft>::new("ada@example.com");
    let (draft, error) = draft.verify("wrong").unwrap_err();
    assert_eq!(error, VerifyError::InvalidToken);
    let verified = draft.verify("known-token").unwrap();
    assert_eq!(verified.email, "ada@example.com");
}
```

### `ch14-b015` — 14.17 Caso completo: factura con typestate

Source: `14.Programación-orientada-a-tipos.md:550` · mode: `run`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
struct DraftInvoice {
    lines: Vec<Cents>,
}

#[derive(Debug, PartialEq, Eq)]
struct IssuedInvoice {
    number: u64,
    lines: Vec<Cents>,
    total: Cents,
}

#[derive(Debug, PartialEq, Eq)]
enum IssueError {
    Empty,
    Overflow,
}

impl DraftInvoice {
    fn issue(self, number: u64) -> Result<IssuedInvoice, (Self, IssueError)> {
        if self.lines.is_empty() {
            return Err((self, IssueError::Empty));
        }
        let total = self.lines.iter().try_fold(0_u64, |sum, line| {
            sum.checked_add(line.0).ok_or(IssueError::Overflow)
        });
        match total {
            Ok(total) => Ok(IssuedInvoice {
                number,
                lines: self.lines,
                total: Cents(total),
            }),
            Err(error) => Err((self, error)),
        }
    }
}

fn main() {
    let draft = DraftInvoice { lines: vec![Cents(200), Cents(300)] };
    let issued = draft.issue(41).unwrap();
    assert_eq!(issued.number, 41);
    assert_eq!(issued.lines.len(), 2);
    assert_eq!(issued.total, Cents(500));
}
```

### `ch14-b016` — 14.18 El mismo caso con enum

Source: `14.Programación-orientada-a-tipos.md:604` · mode: `compile_only`

```rust,no_run
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
enum Invoice {
    Draft { lines: Vec<Cents> },
    Issued { number: u64, lines: Vec<Cents>, total: Cents },
}

#[derive(Debug, PartialEq, Eq)]
enum IssueError {
    Empty,
    AlreadyIssued,
}

impl Invoice {
    fn issue(&mut self, number: u64) -> Result<(), IssueError> {
        match self {
            Self::Draft { lines } if lines.is_empty() => Err(IssueError::Empty),
            Self::Draft { lines } => {
                let total = Cents(lines.iter().map(|line| line.0).sum());
                let lines = std::mem::take(lines);
                *self = Self::Issued { number, lines, total };
                Ok(())
            }
            Self::Issued { .. } => Err(IssueError::AlreadyIssued),
        }
    }
}

fn main() {
    let mut invoice = Invoice::Draft { lines: vec![Cents(500)] };
    assert_eq!(invoice.issue(7), Ok(()));
    assert_eq!(invoice.issue(8), Err(IssueError::AlreadyIssued));
    match invoice {
        Invoice::Issued { number, lines, total } => {
            assert_eq!((number, lines.len(), total), (7, 1, Cents(500)));
        }
        Invoice::Draft { .. } => panic!("la transición debía completarse"),
    }
}
```

### `ch14-b017` — 14.21 DTOs permisivos, dominio estricto

Source: `14.Programación-orientada-a-tipos.md:673` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Email(String);

impl TryFrom<String> for Email {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.contains('@').then_some(Self(value)).ok_or(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Age(u8);

impl TryFrom<u8> for Age {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        (value >= 18).then_some(Self(value)).ok_or(())
    }
}

struct RawSignup {
    email: String,
    age: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Signup {
    email: Email,
    age: Age,
}

#[derive(Debug, PartialEq, Eq)]
enum SignupError {
    Email,
    Age,
}

impl TryFrom<RawSignup> for Signup {
    type Error = SignupError;

    fn try_from(raw: RawSignup) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::try_from(raw.email).map_err(|_| SignupError::Email)?,
            age: Age::try_from(raw.age).map_err(|_| SignupError::Age)?,
        })
    }
}

fn main() {
    let signup = Signup::try_from(RawSignup {
        email: String::from("ada@example.com"),
        age: 36,
    });
    assert!(signup.is_ok());
    assert_eq!(
        Signup::try_from(RawSignup { email: String::from("bad"), age: 36 }),
        Err(SignupError::Email)
    );
}
```

## 15.Traits-bounds-coherence-y-orphan-rules

### `ch15-b001` — 15.1 Trait e implementación inherente resuelven problemas distintos

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:13` · mode: `run`

```rust
trait Summary {
    fn title(&self) -> &str;

    fn summary(&self) -> String {
        self.title().to_owned()
    }
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn title(&self) -> &str {
        &self.title
    }
}

fn render(item: &impl Summary) -> String {
    format!("Resumen: {}", item.summary())
}

fn main() {
    let article = Article { title: String::from("Ownership") };
    assert_eq!(render(&article), "Resumen: Ownership");
}
```

### `ch15-b002` — 15.3 Superficie mínima: requerido frente a derivable

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:59` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Email(String);

#[derive(Debug, PartialEq, Eq)]
struct User {
    email: Email,
}

trait Notifier {
    fn send_raw(&mut self, to: &Email, body: &str);

    fn welcome(&mut self, user: &User) {
        self.send_raw(&user.email, "Bienvenido");
    }

    fn password_reset(&mut self, user: &User, token: &str) {
        self.send_raw(&user.email, &format!("Token: {token}"));
    }
}

#[derive(Default)]
struct RecordingNotifier {
    sent: Vec<String>,
}

impl Notifier for RecordingNotifier {
    fn send_raw(&mut self, to: &Email, body: &str) {
        self.sent.push(format!("{}: {body}", to.0));
    }
}

fn main() {
    let user = User { email: Email(String::from("ada@example.com")) };
    let mut notifier = RecordingNotifier::default();
    notifier.welcome(&user);
    notifier.password_reset(&user, "abc");
    assert_eq!(notifier.sent.len(), 2);
}
```

### `ch15-b003` — 15.4 El receiver declara el tipo de efecto

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:111` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Batch {
    values: Vec<u32>,
}

trait BatchOps {
    fn len(&self) -> usize;
    fn push(&mut self, value: u32);
    fn finish(self) -> Vec<u32>;
}

impl BatchOps for Batch {
    fn len(&self) -> usize { self.values.len() }
    fn push(&mut self, value: u32) { self.values.push(value); }
    fn finish(self) -> Vec<u32> { self.values }
}

fn main() {
    let mut batch = Batch { values: vec![1] };
    assert_eq!(batch.len(), 1);
    batch.push(2);
    assert_eq!(batch.finish(), [1, 2]);
}
```

### `ch15-b004` — 15.5 Un bound concede exactamente unas operaciones

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:141` · mode: `run`

```rust
use std::fmt::Display;

fn surround<T>(value: &T) -> String
where
    T: Display,
{
    format!("<{value}>")
}

fn main() {
    assert_eq!(surround(&42), "<42>");
    assert_eq!(surround(&"Rust"), "<Rust>");
}
```

### `ch15-b005` — 15.6 `where` permite hablar de items asociados

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:163` · mode: `run`

```rust
use std::fmt::Display;

fn render_all<I>(items: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: Display,
{
    items.into_iter().map(|item| item.to_string()).collect()
}

fn main() {
    assert_eq!(render_all([10, 20]), ["10", "20"]);
    assert_eq!(render_all(vec!["a", "b"]), ["a", "b"]);
}
```

### `ch15-b006` — 15.7 Coloca el bound donde se usa

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:186` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.left >= self.right { &self.left } else { &self.right }
    }
}

fn main() {
    let numbers = Pair::new(10, 30);
    assert_eq!(numbers.larger(), &30);

    struct Token;
    let tokens = Pair::new(Token, Token);
    let _ = tokens; // El tipo existe; simplemente no ofrece larger.
}
```

### `ch15-b007` — 15.8 Supertraits: un contrato compuesto

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:221` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

trait DomainError: Error + Send + Sync + 'static {}

impl<T> DomainError for T
where
    T: Error + Send + Sync + 'static,
{}

#[derive(Debug)]
struct MissingUser(u64);

impl fmt::Display for MissingUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no existe el usuario {}", self.0)
    }
}

impl Error for MissingUser {}

fn erase(error: impl DomainError) -> Box<dyn DomainError> {
    Box::new(error)
}

fn main() {
    assert_eq!(erase(MissingUser(7)).to_string(), "no existe el usuario 7");
}
```

### `ch15-b008` — 15.9 Un blanket impl compromete evolución

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:260` · mode: `compile_fail`

```rust,compile_fail
use std::fmt::Display;

trait Label {
    fn label(&self) -> String;
}

impl<T: Display> Label for T {
    fn label(&self) -> String { self.to_string() }
}

impl Label for u32 {
    fn label(&self) -> String { format!("number:{self}") }
}

fn main() {}
```

### `ch15-b009` — 15.10 Coherence garantiza un solo significado

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:284` · mode: `compile_fail`

```rust,compile_fail
trait Category {
    fn category(&self) -> &'static str;
}

impl<T> Category for T {
    fn category(&self) -> &'static str { "generic" }
}

impl Category for String {
    fn category(&self) -> &'static str { "text" }
}

fn main() {}
```

### `ch15-b010` — 15.13 El newtype es la salida explícita

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:332` · mode: `run`

```rust
use std::fmt;

struct Lines(Vec<String>);

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("\n"))
    }
}

impl From<Vec<String>> for Lines {
    fn from(lines: Vec<String>) -> Self {
        Self(lines)
    }
}

fn main() {
    let lines = Lines::from(vec![String::from("uno"), String::from("dos")]);
    assert_eq!(lines.to_string(), "uno\ndos");
}
```

### `ch15-b011` — 15.14 Traits de extensión para añadir sintaxis local

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:361` · mode: `run`

```rust
trait StrExt {
    fn non_blank(&self) -> Option<&str>;
}

impl StrExt for str {
    fn non_blank(&self) -> Option<&str> {
        let trimmed = self.trim();
        (!trimmed.is_empty()).then_some(trimmed)
    }
}

fn main() {
    assert_eq!("  Rust ".non_blank(), Some("Rust"));
    assert_eq!("   ".non_blank(), None);
}
```

### `ch15-b012` — 15.15 `derive` genera un impl con bounds

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:387` · mode: `compile_fail`

```rust,compile_fail
use std::rc::Rc;

#[derive(Clone)]
struct Shared<T> {
    inner: Rc<T>,
}

struct Connection;

fn main() {
    let shared = Shared { inner: Rc::new(Connection) };
    let _copy = shared.clone();
    // Connection no implementa Clone, aunque Rc<Connection> sí.
}
```

### `ch15-b013` — 15.15 `derive` genera un impl con bounds

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:406` · mode: `run`

```rust
use std::rc::Rc;

struct Shared<T> {
    inner: Rc<T>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self { inner: Rc::clone(&self.inner) }
    }
}

struct Connection;

fn main() {
    let shared = Shared { inner: Rc::new(Connection) };
    let copy = shared.clone();
    assert!(Rc::ptr_eq(&shared.inner, &copy.inner));
}
```

### `ch15-b014` — 15.17 `Self: Sized` separa métodos estáticos de métodos dyn

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:440` · mode: `run`

```rust
trait Job {
    fn name(&self) -> &str;

    fn finish(self) -> String
    where
        Self: Sized,
    {
        format!("finalizado: {}", self.name())
    }
}

struct ImportJob(String);

impl Job for ImportJob {
    fn name(&self) -> &str {
        &self.0
    }
}

fn inspect(job: &dyn Job) -> &str {
    job.name()
}

fn main() {
    let job = ImportJob(String::from("usuarios"));
    assert_eq!(inspect(&job), "usuarios");
    assert_eq!(job.finish(), "finalizado: usuarios");
}
```

### `ch15-b015` — 15.19 Dobles de prueba sin frameworks

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:483` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

trait IdGenerator {
    fn next_id(&mut self) -> u64;
}

struct Sequential(u64);

impl IdGenerator for Sequential {
    fn next_id(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

fn register(ids: &mut impl IdGenerator, name: &str) -> User {
    User { id: ids.next_id(), name: name.to_owned() }
}

fn main() {
    let mut ids = Sequential(40);
    assert_eq!(register(&mut ids, "Ada").id, 41);
    assert_eq!(register(&mut ids, "Grace").id, 42);
}
```

### `ch15-b016` — 15.20 Pruebas de contrato reutilizables

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:520` · mode: `run`

```rust
use std::collections::HashMap;

trait Repository: Default {
    fn save(&mut self, key: &str, value: &str);
    fn find(&self, key: &str) -> Option<&str>;
    fn delete(&mut self, key: &str) -> bool;
}

#[derive(Default)]
struct MemoryRepository(HashMap<String, String>);

impl Repository for MemoryRepository {
    fn save(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_owned(), value.to_owned());
    }

    fn find(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.0.remove(key).is_some()
    }
}

fn assert_repository_contract<R: Repository>() {
    let mut repository = R::default();
    repository.save("language", "Rust");
    assert_eq!(repository.find("language"), Some("Rust"));
    assert!(repository.delete("language"));
    assert!(!repository.delete("language"));
}

fn main() {
    assert_repository_contract::<MemoryRepository>();
}
```

### `ch15-b017` — 15.22 Sellar un trait controla quién puede implementarlo

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:575` · mode: `run`

```rust
mod tokens {
    mod private {
        pub trait Sealed {}
    }

    pub struct Word(pub String);

    impl private::Sealed for Word {}

    pub trait Token: private::Sealed {
        fn text(&self) -> &str;
    }

    impl Token for Word {
        fn text(&self) -> &str {
            &self.0
        }
    }
}

use tokens::{Token, Word};

fn main() {
    assert_eq!(Word(String::from("rust")).text(), "rust");
}
```

### `ch15-b018` — 15.24 Resolución de métodos y sintaxis cualificada

Source: `15.Traits-bounds-coherence-y-orphan-rules.md:622` · mode: `run`

```rust
struct Report;

impl Report {
    fn label(&self) -> &'static str { "inherente" }
}

trait Label {
    fn label(&self) -> &'static str;
}

impl Label for Report {
    fn label(&self) -> &'static str { "trait" }
}

fn main() {
    let report = Report;
    assert_eq!(report.label(), "inherente");
    assert_eq!(Label::label(&report), "trait");
    assert_eq!(<Report as Label>::label(&report), "trait");
}
```

## 16.Genéricos-y-monomorfización

### `ch16-b001` — 16.1 El caller elige `T`

Source: `16.Genéricos-y-monomorfización.md:11` · mode: `run`

```rust
fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}

#[derive(Debug, PartialEq, Eq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

fn main() {
    assert_eq!(first(&[10, 20]), Some(&10));
    assert_eq!(first(&[String::from("Rust")]).map(String::as_str), Some("Rust"));
    assert_eq!(Pair::new('a', 'b').left, 'a');
}
```

### `ch16-b002` — 16.2 Sin bounds, el cuerpo debe ser universal

Source: `16.Genéricos-y-monomorfización.md:43` · mode: `compile_fail`

```rust,compile_fail
fn largest<T>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

fn main() {}
```

### `ch16-b003` — 16.2 Sin bounds, el cuerpo debe ser universal

Source: `16.Genéricos-y-monomorfización.md:59` · mode: `run`

```rust
fn largest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

fn main() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(largest::<i32>(&[]), None);
}
```

### `ch16-b004` — 16.3 Una referencia puede eliminar `Clone`

Source: `16.Genéricos-y-monomorfización.md:82` · mode: `run`

```rust
fn max_ref<'a, T: Ord>(left: &'a T, right: &'a T) -> &'a T {
    if left >= right { left } else { right }
}

fn max_owned<T>(left: &T, right: &T) -> T
where
    T: Ord + Clone,
{
    max_ref(left, right).clone()
}

fn main() {
    let left = String::from("Ada");
    let right = String::from("Grace");
    assert_eq!(max_ref(&left, &right), "Grace");
    assert_eq!(max_owned(&left, &right), "Grace");
}
```

### `ch16-b005` — 16.4 Varios parámetros describen relaciones distintas

Source: `16.Genéricos-y-monomorfización.md:108` · mode: `run`

```rust
fn convert_all<I, T, U>(input: I) -> Vec<U>
where
    I: IntoIterator<Item = T>,
    U: From<T>,
{
    input.into_iter().map(U::from).collect()
}

fn main() {
    let numbers = convert_all::<_, u16, u64>([10_u16, 20]);
    assert_eq!(numbers, [10_u64, 20]);
}
```

### `ch16-b006` — 16.5 `impl` general, condicional y concreto

Source: `16.Genéricos-y-monomorfización.md:129` · mode: `run`

```rust
#[derive(Debug, PartialEq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.left >= self.right { &self.left } else { &self.right }
    }
}

impl Pair<f64> {
    fn distance_from_origin(&self) -> f64 {
        self.left.hypot(self.right)
    }
}

fn main() {
    let point = Pair::new(3.0_f64, 4.0);
    assert_eq!(point.larger(), &4.0);
    assert_eq!(point.distance_from_origin(), 5.0);

    struct Token;
    let _tokens = Pair::new(Token, Token); // Existe, pero no tiene larger.
}
```

### `ch16-b007` — 16.6 Bounds sobre el tipo frente a bounds sobre la operación

Source: `16.Genéricos-y-monomorfización.md:168` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Envelope<T> {
    value: T,
}

impl<T> Envelope<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn map<U>(self, operation: impl FnOnce(T) -> U) -> Envelope<U> {
        Envelope { value: operation(self.value) }
    }
}

impl<T: Clone> Envelope<T> {
    fn duplicate(&self) -> (T, T) {
        (self.value.clone(), self.value.clone())
    }
}

fn main() {
    let length = Envelope::new(String::from("Rust")).map(|text| text.len());
    assert_eq!(length, Envelope { value: 4 });
    assert_eq!(Envelope::new(7).duplicate(), (7, 7));
}
```

### `ch16-b008` — 16.7 Contenedores genéricos de dominio

Source: `16.Genéricos-y-monomorfización.md:203` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Versioned<T> {
    version: u64,
    value: T,
}

impl<T> Versioned<T> {
    fn map<U>(self, operation: impl FnOnce(T) -> U) -> Versioned<U> {
        Versioned {
            version: self.version,
            value: operation(self.value),
        }
    }
}

fn main() {
    let entity = Versioned { version: 7, value: String::from("Rust") };
    assert_eq!(entity.map(|value| value.len()), Versioned { version: 7, value: 4 });
}
```

### `ch16-b009` — 16.8 Const generics: el valor forma parte del tipo

Source: `16.Genéricos-y-monomorfización.md:229` · mode: `run`

```rust
#[derive(Debug, PartialEq, Eq)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    cells: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new(cells: [[T; COLS]; ROWS]) -> Self {
        Self { cells }
    }

    fn dimensions(&self) -> (usize, usize) {
        (ROWS, COLS)
    }
}

fn main() {
    let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(matrix.dimensions(), (2, 3));
    assert_eq!(matrix.cells[1][2], 6);
}
```

### `ch16-b010` — 16.8 Const generics: el valor forma parte del tipo

Source: `16.Genéricos-y-monomorfización.md:256` · mode: `compile_fail`

```rust,compile_fail
struct Samples<const N: usize>([f32; N]);

fn compare<const N: usize>(_left: &Samples<N>, _right: &Samples<N>) {}

fn main() {
    let short = Samples([0.0; 8]);
    let long = Samples([0.0; 16]);
    compare(&short, &long);
    // se esperaba Samples<8>, se encontró Samples<16>
}
```

### `ch16-b011` — 16.10 Monomorfización: una definición, instancias concretas

Source: `16.Genéricos-y-monomorfización.md:281` · mode: `run`

```rust
fn twice<T>(value: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    value + value
}

fn main() {
    assert_eq!(twice(21_i32), 42);       // instancia para i32
    assert_eq!(twice(1.5_f64), 3.0);    // instancia para f64
}
```

### `ch16-b012` — 16.12 Reducir bloat con una envoltura genérica pequeña

Source: `16.Genéricos-y-monomorfización.md:315` · mode: `run`

```rust
use std::path::Path;

fn file_name(path: impl AsRef<Path>) -> Option<String> {
    file_name_core(path.as_ref())
}

fn file_name_core(path: &Path) -> Option<String> {
    path.file_name()?.to_str().map(str::to_owned)
}

fn main() {
    assert_eq!(file_name("reports/book.md"), Some(String::from("book.md")));
    assert_eq!(file_name(Path::new("notes.txt")), Some(String::from("notes.txt")));
}
```

### `ch16-b013` — 16.13 `Sized` implícito y `?Sized`

Source: `16.Genéricos-y-monomorfización.md:338` · mode: `run`

```rust
use std::fmt::Display;

fn describe<T>(value: &T) -> String
where
    T: Display + ?Sized,
{
    value.to_string()
}

fn main() {
    let number = 42;
    let erased: &dyn Display = &number;
    assert_eq!(describe(&number), "42");
    assert_eq!(describe(erased), "42");
}
```

### `ch16-b014` — 16.14 Inferencia, anotaciones y turbofish

Source: `16.Genéricos-y-monomorfización.md:360` · mode: `run`

```rust
fn parse_pair<T>(left: &str, right: &str) -> Result<(T, T), T::Err>
where
    T: std::str::FromStr,
{
    Ok((left.parse()?, right.parse()?))
}

fn main() {
    let ports = parse_pair::<u16>("80", "443").unwrap();
    let coordinates: (i64, i64) = parse_pair("-3", "7").unwrap();
    assert_eq!(ports, (80, 443));
    assert_eq!(coordinates, (-3, 7));
}
```

## 17.Associated-types-y-GATs

### `ch17-b001` — 17.1 Los associated items pertenecen a una implementación

Source: `17.Associated-types-y-GATs.md:13` · mode: `run`

```rust
trait Codec {
    type Output;
    type Error;
    const NAME: &'static str;

    fn decode(&self, input: &[u8]) -> Result<Self::Output, Self::Error>;
}

struct Utf8;

impl Codec for Utf8 {
    type Output = String;
    type Error = std::str::Utf8Error;
    const NAME: &'static str = "utf-8";

    fn decode(&self, input: &[u8]) -> Result<String, Self::Error> {
        std::str::from_utf8(input).map(str::to_owned)
    }
}

fn main() {
    assert_eq!(Utf8::NAME, "utf-8");
    assert_eq!(Utf8.decode(b"Rust").unwrap(), "Rust");
}
```

### `ch17-b002` — 17.2 Un repositorio fija entidad y error

Source: `17.Associated-types-y-GATs.md:44` · mode: `run`

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

trait Repository {
    type Entity;
    type Error;

    fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error>;
}

struct InMemoryUsers {
    users: Vec<User>,
}

impl Repository for InMemoryUsers {
    type Entity = User;
    type Error = std::convert::Infallible;

    fn find(&self, id: u64) -> Result<Option<User>, Self::Error> {
        Ok(self.users.iter().find(|user| user.id == id).cloned())
    }
}

fn main() {
    let repository = InMemoryUsers {
        users: vec![User { id: 7, name: String::from("Ada") }],
    };
    assert_eq!(repository.find(7).unwrap().unwrap().name, "Ada");
    assert_eq!(repository.find(9).unwrap(), None);
}
```

### `ch17-b003` — 17.3 Proyecciones: hablar del tipo elegido

Source: `17.Associated-types-y-GATs.md:86` · mode: `run`

```rust
trait Repository {
    type Entity;
    type Error;
    fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
enum RequireError<E> {
    Missing { id: u64 },
    Backend(E),
}

fn require<R>(repository: &R, id: u64) -> Result<R::Entity, RequireError<R::Error>>
where
    R: Repository,
{
    repository
        .find(id)
        .map_err(RequireError::Backend)?
        .ok_or(RequireError::Missing { id })
}
```

### `ch17-b004` — 17.4 Parámetro genérico: el caller puede elegir varias veces

Source: `17.Associated-types-y-GATs.md:114` · mode: `run`

```rust
trait ParseAs<Output> {
    fn parse_as(&self, input: &str) -> Option<Output>;
}

struct Flexible;

impl ParseAs<u32> for Flexible {
    fn parse_as(&self, input: &str) -> Option<u32> {
        input.parse().ok()
    }
}

impl ParseAs<f64> for Flexible {
    fn parse_as(&self, input: &str) -> Option<f64> {
        input.parse().ok()
    }
}

fn main() {
    let parser = Flexible;
    let integer: Option<u32> = ParseAs::parse_as(&parser, "42");
    let decimal: Option<f64> = ParseAs::parse_as(&parser, "4.2");
    assert_eq!(integer, Some(42));
    assert_eq!(decimal, Some(4.2));
}
```

### `ch17-b005` — 17.4 Parámetro genérico: el caller puede elegir varias veces

Source: `17.Associated-types-y-GATs.md:146` · mode: `compile_fail`

```rust,compile_fail
trait Parse {
    type Output;
    fn parse(&self, input: &str) -> Option<Self::Output>;
}

struct Flexible;

impl Parse for Flexible {
    type Output = u32;
    fn parse(&self, input: &str) -> Option<u32> { input.parse().ok() }
}

impl Parse for Flexible {
    type Output = f64;
    fn parse(&self, input: &str) -> Option<f64> { input.parse().ok() }
}

fn main() {}
```

### `ch17-b006` — 17.5 `Iterator::Item` es la asociación cotidiana

Source: `17.Associated-types-y-GATs.md:173` · mode: `run`

```rust
trait MinimalIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Countdown(u8);

impl MinimalIterator for Countdown {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let current = self.0;
        (current > 0).then(|| {
            self.0 -= 1;
            current
        })
    }
}

fn main() {
    let mut countdown = Countdown(2);
    assert_eq!(countdown.next(), Some(2));
    assert_eq!(countdown.next(), Some(1));
    assert_eq!(countdown.next(), None);
}
```

### `ch17-b007` — 17.6 Bounds e igualdades sobre proyecciones

Source: `17.Associated-types-y-GATs.md:205` · mode: `run`

```rust
use std::fmt::Display;

fn render_items<I>(items: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: Display,
{
    items.into_iter().map(|item| item.to_string()).collect()
}

fn sum_exact<I>(items: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    items.into_iter().sum()
}

fn main() {
    assert_eq!(render_items([10, 20]), ["10", "20"]);
    assert_eq!(sum_exact([10, 20]), 30);
}
```

### `ch17-b008` — 17.7 Sintaxis completamente cualificada

Source: `17.Associated-types-y-GATs.md:235` · mode: `run`

```rust
trait Left {
    type Value;
    fn value(&self) -> Self::Value;
}

trait Right {
    type Value;
    fn value(&self) -> Self::Value;
}

struct Both;

impl Left for Both {
    type Value = u32;
    fn value(&self) -> u32 { 10 }
}

impl Right for Both {
    type Value = &'static str;
    fn value(&self) -> &'static str { "ten" }
}

fn main() {
    let both = Both;
    let number: <Both as Left>::Value = <Both as Left>::value(&both);
    let text: <Both as Right>::Value = <Both as Right>::value(&both);
    assert_eq!(number, 10);
    assert_eq!(text, "ten");
}
```

### `ch17-b009` — 17.8 Un GAT es una familia asociada

Source: `17.Associated-types-y-GATs.md:273` · mode: `run`

```rust
trait ViewStore {
    type View<'a>
    where
        Self: 'a;

    fn view(&self, index: usize) -> Option<Self::View<'_>>;
}

struct Names(Vec<String>);

impl ViewStore for Names {
    type View<'a> = &'a str where Self: 'a;

    fn view(&self, index: usize) -> Option<&str> {
        self.0.get(index).map(String::as_str)
    }
}

fn main() {
    let names = Names(vec![String::from("Ada"), String::from("Grace")]);
    assert_eq!(names.view(1), Some("Grace"));
}
```

### `ch17-b010` — 17.9 Por qué aparece `where Self: 'a`

Source: `17.Associated-types-y-GATs.md:304` · mode: `run`

```rust
trait Lend {
    type Item<'a>
    where
        Self: 'a;

    fn lend<'a>(&'a self) -> Self::Item<'a>;
}

struct Text(String);

impl Lend for Text {
    type Item<'a> = &'a str where Self: 'a;

    fn lend<'a>(&'a self) -> &'a str {
        &self.0
    }
}

fn main() {
    assert_eq!(Text(String::from("Rust")).lend(), "Rust");
}
```

### `ch17-b011` — 17.10 Lending iterator: el item depende de cada préstamo

Source: `17.Associated-types-y-GATs.md:334` · mode: `run`

```rust
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

### `ch17-b012` — 17.11 Ventanas mutables y solapadas, pero no simultáneas

Source: `17.Associated-types-y-GATs.md:350` · mode: `run`

```rust
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowsMut<'slice, T> {
    slice: &'slice mut [T],
    size: usize,
    position: usize,
}

impl<'slice, T> WindowsMut<'slice, T> {
    fn new(slice: &'slice mut [T], size: usize) -> Self {
        Self { slice, size, position: 0 }
    }
}

impl<T> LendingIterator for WindowsMut<'_, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<&'a mut [T]> {
        let start = self.position;
        let end = start.checked_add(self.size)?;
        if self.size == 0 || end > self.slice.len() {
            return None;
        }
        self.position += 1;
        Some(&mut self.slice[start..end])
    }
}

fn main() {
    let mut data = [1, 2, 3, 4];
    let mut windows = WindowsMut::new(&mut data, 2);

    windows.next().unwrap()[1] = 20; // ventana [0..2]
    windows.next().unwrap()[1] = 30; // ventana [1..3]
    assert_eq!(data, [1, 20, 30, 4]);
}
```

### `ch17-b013` — 17.12 El compilador impide dos items coexistentes

Source: `17.Associated-types-y-GATs.md:400` · mode: `compile_fail`

```rust,compile_fail
trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct OneAtATime<'a>(&'a mut [u8]);

impl LendingIterator for OneAtATime<'_> {
    type Item<'a> = &'a mut u8 where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<&'a mut u8> {
        self.0.first_mut()
    }
}

fn main() {
    let mut data = [1];
    let mut lender = OneAtATime(&mut data);
    let first = lender.next().unwrap();
    let second = lender.next().unwrap();
    *first += *second;
    // error[E0499]: lender sigue prestado por first
}
```

### `ch17-b014` — 17.14 GATs con parámetros de tipo y const

Source: `17.Associated-types-y-GATs.md:443` · mode: `run`

```rust
trait ContainerFamily {
    type Container<T, const N: usize>;
}

struct Arrays;

impl ContainerFamily for Arrays {
    type Container<T, const N: usize> = [T; N];
}

fn main() {
    let values: <Arrays as ContainerFamily>::Container<u16, 3> = [10, 20, 30];
    assert_eq!(values, [10, 20, 30]);
}
```

### `ch17-b015` — 17.15 Associated type con bounds propios

Source: `17.Associated-types-y-GATs.md:464` · mode: `run`

```rust
use std::fmt::Display;

trait Source {
    type Item: Display;
    fn load(&self) -> Self::Item;
}

struct PortSource;

impl Source for PortSource {
    type Item = u16;
    fn load(&self) -> u16 { 8080 }
}

fn render_source(source: &impl Source) -> String {
    source.load().to_string()
}

fn main() {
    assert_eq!(render_source(&PortSource), "8080");
}
```

### `ch17-b016` — 17.16 Combinar entrada genérica y salida asociada

Source: `17.Associated-types-y-GATs.md:492` · mode: `run`

```rust
trait Convert<Input> {
    type Output;
    fn convert(&self, input: Input) -> Self::Output;
}

struct Length;

impl Convert<String> for Length {
    type Output = usize;
    fn convert(&self, input: String) -> usize { input.len() }
}

impl Convert<&str> for Length {
    type Output = usize;
    fn convert(&self, input: &str) -> usize { input.len() }
}

fn main() {
    assert_eq!(Length.convert(String::from("Rust")), 4);
    assert_eq!(Length.convert("GAT"), 3);
}
```

### `ch17-b017` — 17.17 GATs y dyn compatibility

Source: `17.Associated-types-y-GATs.md:524` · mode: `compile_fail`

```rust,compile_fail
trait ViewStore {
    type View<'a> where Self: 'a;
    fn view(&self) -> Self::View<'_>;
}

fn erase(_store: &dyn ViewStore) {}

fn main() {}
```

## 18.Impl-Trait-RPIT-y-RPITIT

### `ch18-b001` — 18.1 La posición decide quién elige

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:11` · mode: `run`

```rust
use std::fmt::Display;

fn label(value: impl Display) -> String {
    format!("valor={value}")
}

fn ids() -> impl Iterator<Item = u64> {
    10..13
}

fn main() {
    assert_eq!(label(42), "valor=42");
    assert_eq!(label("Rust"), "valor=Rust");
    assert_eq!(ids().collect::<Vec<_>>(), [10, 11, 12]);
}
```

### `ch18-b002` — 18.2 APIT es un parámetro genérico anónimo

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:39` · mode: `run`

```rust
use std::fmt::Display;

fn render_pair(left: impl Display, right: impl Display) -> String {
    format!("{left} | {right}")
}

fn equal<T: PartialEq>(left: T, right: T) -> bool {
    left == right
}

fn main() {
    assert_eq!(render_pair(7, "días"), "7 | días");
    assert!(equal(String::from("Rust"), String::from("Rust")));
}
```

### `ch18-b003` — 18.3 RPIT oculta exactamente un tipo concreto

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:66` · mode: `compile_fail`

```rust,compile_fail
fn numbers(reverse: bool) -> impl Iterator<Item = u32> {
    if reverse {
        (0..3).rev() // Rev<Range<u32>>
    } else {
        0..3         // Range<u32>
    }
}

fn main() {
    let _ = numbers(true);
}
```

### `ch18-b004` — 18.4 Cada RPIT tiene identidad opaca propia

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:88` · mode: `compile_fail`

```rust,compile_fail
fn first() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn second() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn require_same<T>(_left: T, _right: T) {}

fn main() {
    require_same(first(), second());
}
```

### `ch18-b005` — 18.5 Iteradores y closures sin nombre escribible

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:110` · mode: `run`

```rust
fn positive<'a>(values: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    values.iter().copied().filter(|value| *value > 0)
}

fn main() {
    let values = [-2, 4, 0, 7];
    assert_eq!(positive(&values).collect::<Vec<_>>(), [4, 7]);
}
```

### `ch18-b006` — 18.5 Iteradores y closures sin nombre escribible

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:125` · mode: `run`

```rust
fn multiplier(factor: i64) -> impl Fn(i64) -> i64 {
    move |value| value * factor
}

fn main() {
    let double = multiplier(2);
    let triple = multiplier(3);
    assert_eq!(double(21), 42);
    assert_eq!(triple(14), 42);
}
```

### `ch18-b007` — 18.6 Tres reparaciones para ramas distintas

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:144` · mode: `run`

```rust
#[derive(Clone, Debug)]
enum EitherIter<A, B> {
    Forward(A),
    Reverse(B),
}

impl<T, A, B> Iterator for EitherIter<A, B>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Self::Forward(iterator) => iterator.next(),
            Self::Reverse(iterator) => iterator.next(),
        }
    }
}

fn numbers_static(reverse: bool) -> impl Iterator<Item = u32> {
    if reverse {
        EitherIter::Reverse((0..3).rev())
    } else {
        EitherIter::Forward(0..3)
    }
}

fn numbers_dynamic(reverse: bool) -> Box<dyn Iterator<Item = u32>> {
    if reverse {
        Box::new((0..3).rev())
    } else {
        Box::new(0..3)
    }
}

fn main() {
    assert_eq!(numbers_static(true).collect::<Vec<_>>(), [2, 1, 0]);
    assert_eq!(numbers_dynamic(false).collect::<Vec<_>>(), [0, 1, 2]);
}
```

### `ch18-b008` — 18.6 Tres reparaciones para ramas distintas

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:194` · mode: `run`

```rust
fn scaled(factor: i32) -> impl Iterator<Item = i32> {
    (0..3).map(move |value| value * factor)
}

fn main() {
    assert_eq!(scaled(2).collect::<Vec<_>>(), [0, 2, 4]);
    assert_eq!(scaled(-1).collect::<Vec<_>>(), [0, -1, -2]);
}
```

### `ch18-b009` — 18.8 Captura automática en Rust 2024

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:224` · mode: `run`

```rust
use std::fmt::Debug;

fn length<'a>(text: &'a str) -> impl Copy + Debug + PartialEq<usize> + use<> {
    text.len()
}

fn main() {
    let answer;
    {
        let text = String::from("Rust");
        answer = length(&text);
    }
    assert_eq!(answer, 4);
}
```

### `ch18-b010` — 18.8 Captura automática en Rust 2024

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:245` · mode: `run`

```rust
fn tag<'a, T, const N: usize>(_value: &'a T) -> impl Copy + use<T, N> {
    N
}

fn main() {
    let value = String::from("dato");
    let _tag = tag::<_, 8>(&value);
}
```

### `ch18-b011` — 18.10 RPITIT: el tipo opaco pertenece al impl

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:272` · mode: `run`

```rust
trait Catalog {
    fn names(&self) -> impl Iterator<Item = &str>;
}

struct VecCatalog(Vec<String>);

impl Catalog for VecCatalog {
    fn names(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

fn joined<C: Catalog>(catalog: &C) -> String {
    catalog.names().collect::<Vec<_>>().join(", ")
}

fn main() {
    let catalog = VecCatalog(vec![String::from("Rust"), String::from("Cargo")]);
    assert_eq!(joined(&catalog), "Rust, Cargo");
}
```

### `ch18-b012` — 18.11 El caller conoce bounds, no métodos accidentales

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:301` · mode: `run`

```rust
fn countdown() -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    0_u8..4
}

fn main() {
    let values = countdown();
    assert_eq!(values.len(), 4);
    assert_eq!(countdown().rev().collect::<Vec<_>>(), [3, 2, 1, 0]);
}
```

### `ch18-b013` — 18.11 El caller conoce bounds, no métodos accidentales

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:317` · mode: `compile_fail`

```rust,compile_fail
fn values() -> impl Iterator<Item = u8> {
    0_u8..4
}

fn main() {
    let _ = values().len();
}
```

### `ch18-b014` — 18.12 La excepción de `Send` y otros auto traits

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:333` · mode: `run`

```rust
fn jobs() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn assert_send<T: Send>(_value: T) {}

fn main() {
    assert_send(jobs());
}
```

### `ch18-b015` — 18.12 La excepción de `Send` y otros auto traits

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:349` · mode: `run`

```rust
trait Jobs {
    fn pending(&self) -> impl Iterator<Item = u64> + Send;
}

struct Queue(Vec<u64>);

impl Jobs for Queue {
    fn pending(&self) -> impl Iterator<Item = u64> + Send {
        self.0.clone().into_iter()
    }
}

fn require_send<T: Send>(_value: T) {}

fn main() {
    let queue = Queue(vec![10, 20]);
    require_send(queue.pending());
}
```

### `ch18-b016` — 18.13 RPITIT no es dyn compatible

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:378` · mode: `compile_fail`

```rust,compile_fail
trait Catalog {
    fn names(&self) -> impl Iterator<Item = &str>;
}

fn print_dynamic(_catalog: &dyn Catalog) {}

fn main() {}
```

### `ch18-b017` — 18.13 RPITIT no es dyn compatible

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:390` · mode: `run`

```rust
trait Catalog {
    fn names(&self) -> Box<dyn Iterator<Item = &str> + '_>;
}

struct Words(Vec<String>);

impl Catalog for Words {
    fn names(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(self.0.iter().map(String::as_str))
    }
}

fn count(catalog: &dyn Catalog) -> usize {
    catalog.names().count()
}

fn main() {
    let words = Words(vec![String::from("uno"), String::from("dos")]);
    assert_eq!(count(&words), 2);
}
```

### `ch18-b018` — 18.14 `async fn` también produce un tipo opaco

Source: `18.Impl-Trait-RPIT-y-RPITIT.md:419` · mode: `run`

```rust
use std::future::Future;

async fn compute(input: u32) -> u32 {
    input * 2
}

fn compute_desugared(input: u32) -> impl Future<Output = u32> {
    async move { input * 2 }
}

fn accepts_future(_future: impl Future<Output = u32>) {}

fn main() {
    accepts_future(compute(21));
    accepts_future(compute_desugared(21));
}
```

## 19.Trait-objects-y-dispatch-dinámico

### `ch19-b001` — 19.1 Borrar el tipo permite heterogeneidad

Source: `19.Trait-objects-y-dispatch-dinámico.md:11` · mode: `run`

```rust
trait Renderer {
    fn render(&self) -> String;
}

struct Text(String);
struct Number(i64);

impl Renderer for Text {
    fn render(&self) -> String {
        self.0.clone()
    }
}

impl Renderer for Number {
    fn render(&self) -> String {
        self.0.to_string()
    }
}

fn render_all(items: &[Box<dyn Renderer>]) -> Vec<String> {
    items.iter().map(|item| item.render()).collect()
}

fn main() {
    let items: Vec<Box<dyn Renderer>> = vec![
        Box::new(Text(String::from("Rust"))),
        Box::new(Number(2024)),
    ];
    assert_eq!(render_all(&items), ["Rust", "2024"]);
}
```

### `ch19-b002` — 19.4 La coerción construye el trait object

Source: `19.Trait-objects-y-dispatch-dinámico.md:74` · mode: `run`

```rust
trait Renderer {
    fn render(&self) -> String;
}

struct Label(&'static str);

impl Renderer for Label {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn borrowed(renderer: &dyn Renderer) -> String {
    renderer.render()
}

fn owned(renderer: Box<dyn Renderer>) -> String {
    renderer.render()
}

fn main() {
    let label = Label("prestado");
    assert_eq!(borrowed(&label), "prestado");

    let boxed: Box<dyn Renderer> = Box::new(Label("owned"));
    assert_eq!(owned(boxed), "owned");
}
```

### `ch19-b003` — 19.5 Caso completo: pipeline elegido en runtime

Source: `19.Trait-objects-y-dispatch-dinámico.md:108` · mode: `run`

```rust
#[derive(Clone, Copy)]
struct Event {
    kind: &'static str,
    payload: &'static str,
}

trait Sink {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str>;
}

struct AllEvents;

impl Sink for AllEvents {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str> {
        Ok(Some(format!("all:{}", event.payload)))
    }
}

struct AuditOnly;

impl Sink for AuditOnly {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str> {
        Ok((event.kind == "audit").then(|| format!("audit:{}", event.payload)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DispatchReport {
    records: Vec<String>,
    failures: usize,
}

struct Pipeline {
    sinks: Vec<Box<dyn Sink>>,
}

impl Pipeline {
    fn from_config(audit_enabled: bool) -> Self {
        let mut sinks: Vec<Box<dyn Sink>> = vec![Box::new(AllEvents)];
        if audit_enabled {
            sinks.push(Box::new(AuditOnly));
        }
        Self { sinks }
    }

    fn dispatch(&mut self, event: Event) -> DispatchReport {
        let mut report = DispatchReport { records: Vec::new(), failures: 0 };
        for sink in &mut self.sinks {
            match sink.write(event) {
                Ok(Some(record)) => report.records.push(record),
                Ok(None) => {}
                Err(_) => report.failures += 1,
            }
        }
        report
    }
}

fn main() {
    let event = Event { kind: "audit", payload: "login" };
    let mut basic = Pipeline::from_config(false);
    let mut audited = Pipeline::from_config(true);

    assert_eq!(basic.dispatch(event).records, ["all:login"]);
    assert_eq!(audited.dispatch(event).records, ["all:login", "audit:login"]);
}
```

### `ch19-b004` — 19.6 Qué significa dyn compatibility

Source: `19.Trait-objects-y-dispatch-dinámico.md:192` · mode: `run`

```rust
trait Command {
    fn execute(&self) -> &'static str;

    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

struct UnitCommand;

impl Command for UnitCommand {
    fn execute(&self) -> &'static str {
        "ok"
    }

    fn duplicate(&self) -> Self {
        Self
    }
}

fn main() {
    let command: &dyn Command = &UnitCommand;
    assert_eq!(command.execute(), "ok");

    let concrete = UnitCommand;
    let _copy = concrete.duplicate();
}
```

### `ch19-b005` — 19.7 Por qué un método genérico no cabe en la vtable

Source: `19.Trait-objects-y-dispatch-dinámico.md:228` · mode: `compile_fail`

```rust,compile_fail
use std::fmt::Display;

trait Encoder {
    fn encode<T: Display>(&self, value: T) -> String;
}

fn use_dynamic(_encoder: &dyn Encoder) {}

fn main() {}
```

### `ch19-b006` — 19.7 Por qué un método genérico no cabe en la vtable

Source: `19.Trait-objects-y-dispatch-dinámico.md:242` · mode: `run`

```rust
trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

trait Store {
    fn save_bytes(&mut self, key: &str, bytes: &[u8]);
}

#[derive(Default)]
struct MemoryStore {
    last: Option<(String, Vec<u8>)>,
}

impl Store for MemoryStore {
    fn save_bytes(&mut self, key: &str, bytes: &[u8]) {
        self.last = Some((key.to_owned(), bytes.to_vec()));
    }
}

fn save<S, T>(store: &mut S, key: &str, value: &T)
where
    S: Store + ?Sized,
    T: Encode,
{
    store.save_bytes(key, &value.encode());
}

fn main() {
    let mut memory = MemoryStore::default();
    let erased: &mut dyn Store = &mut memory;
    save(erased, "answer", &42_u32);
    assert_eq!(memory.last, Some((String::from("answer"), b"42".to_vec())));
}
```

### `ch19-b007` — 19.8 Associated types: hay que fijar la proyección

Source: `19.Trait-objects-y-dispatch-dinámico.md:290` · mode: `run`

```rust
fn sum(iterator: &mut dyn Iterator<Item = i32>) -> i32 {
    iterator.sum()
}

fn main() {
    let mut values = vec![2, 3, 5].into_iter();
    assert_eq!(sum(&mut values), 10);
}
```

### `ch19-b008` — 19.8 Associated types: hay que fijar la proyección

Source: `19.Trait-objects-y-dispatch-dinámico.md:305` · mode: `compile_fail`

```rust,compile_fail
fn consume(_iterator: &mut dyn Iterator) {}

fn main() {}
```

### `ch19-b009` — 19.9 El lifetime del objeto pertenece a sus datos

Source: `19.Trait-objects-y-dispatch-dinámico.md:319` · mode: `compile_fail`

```rust,compile_fail
trait Renderer {
    fn render(&self) -> String;
}

struct TextView<'a>(&'a str);

impl Renderer for TextView<'_> {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn boxed_view(text: &str) -> Box<dyn Renderer> {
    Box::new(TextView(text))
}

fn main() {}
```

### `ch19-b010` — 19.9 El lifetime del objeto pertenece a sus datos

Source: `19.Trait-objects-y-dispatch-dinámico.md:341` · mode: `run`

```rust
trait Renderer {
    fn render(&self) -> String;
}

struct TextView<'a>(&'a str);

impl Renderer for TextView<'_> {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn boxed_view<'a>(text: &'a str) -> Box<dyn Renderer + 'a> {
    Box::new(TextView(text))
}

fn main() {
    let text = String::from("vista");
    let renderer = boxed_view(&text);
    assert_eq!(renderer.render(), "vista");
}
```

### `ch19-b011` — 19.10 Auto traits también forman parte del objeto

Source: `19.Trait-objects-y-dispatch-dinámico.md:371` · mode: `run`

```rust
trait Task {
    fn name(&self) -> &str;
}

struct Job(&'static str);

impl Task for Job {
    fn name(&self) -> &str {
        self.0
    }
}

fn assert_thread_safe<T: Send + Sync>(_value: &T) {}

fn main() {
    let task: Box<dyn Task + Send + Sync> = Box::new(Job("backup"));
    assert_thread_safe(&task);
    assert_eq!(task.name(), "backup");
}
```

### `ch19-b012` — 19.11 Servicio genérico frente a servicio dinámico

Source: `19.Trait-objects-y-dispatch-dinámico.md:397` · mode: `run`

```rust
trait Repository {
    fn find_name(&self, id: u64) -> Option<String>;
}

struct MemoryRepository;

impl Repository for MemoryRepository {
    fn find_name(&self, id: u64) -> Option<String> {
        (id == 1).then(|| String::from("Ada"))
    }
}

struct StaticService<R> {
    repository: R,
}

impl<R: Repository> StaticService<R> {
    fn name(&self, id: u64) -> Option<String> {
        self.repository.find_name(id)
    }
}

struct DynamicService {
    repository: Box<dyn Repository>,
}

impl DynamicService {
    fn name(&self, id: u64) -> Option<String> {
        self.repository.find_name(id)
    }
}

fn main() {
    let static_service = StaticService { repository: MemoryRepository };
    let dynamic_service = DynamicService { repository: Box::new(MemoryRepository) };
    assert_eq!(static_service.name(1).as_deref(), Some("Ada"));
    assert_eq!(dynamic_service.name(1).as_deref(), Some("Ada"));
}
```

### `ch19-b013` — 19.14 Downcasting suele revelar un contrato incompleto

Source: `19.Trait-objects-y-dispatch-dinámico.md:469` · mode: `run`

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Text(String);
struct Count(usize);

impl Describe for Text {
    fn describe(&self) -> String {
        format!("texto de {} bytes", self.0.len())
    }
}

impl Describe for Count {
    fn describe(&self) -> String {
        format!("contador={}", self.0)
    }
}

fn main() {
    let values: Vec<Box<dyn Describe>> = vec![
        Box::new(Text(String::from("Rust"))),
        Box::new(Count(4)),
    ];
    let descriptions = values.iter().map(|value| value.describe()).collect::<Vec<_>>();
    assert_eq!(descriptions, ["texto de 4 bytes", "contador=4"]);
}
```

### `ch19-b014` — 19.15 Decoradores: composición sin conocer el tipo interno

Source: `19.Trait-objects-y-dispatch-dinámico.md:505` · mode: `run`

```rust
trait Sink {
    fn send(&mut self, kind: &str, value: String);
}

#[derive(Default)]
struct MemorySink {
    values: Vec<String>,
}

impl Sink for MemorySink {
    fn send(&mut self, _kind: &str, value: String) {
        self.values.push(value);
    }
}

struct FilteredSink<S, P> {
    inner: S,
    accepts: P,
}

impl<S, P> Sink for FilteredSink<S, P>
where
    S: Sink,
    P: Fn(&str) -> bool,
{
    fn send(&mut self, kind: &str, value: String) {
        if (self.accepts)(kind) {
            self.inner.send(kind, value);
        }
    }
}

fn main() {
    let memory = MemorySink::default();
    let mut sink = FilteredSink { inner: memory, accepts: |kind: &str| kind == "audit" };
    sink.send("debug", String::from("ignorado"));
    sink.send("audit", String::from("guardado"));
    assert_eq!(sink.inner.values, ["guardado"]);
}
```

## 20.Subtyping-variance-y-HRTB

### `ch20-b001` — 20.1 El subtyping de Rust es deliberadamente estrecho

Source: `20.Subtyping-variance-y-HRTB.md:13` · mode: `run`

```rust
fn shorten<'short>(value: &'static str) -> &'short str {
    value
}

fn pair_with_local<'a>(long: &'static str, local: &'a str) -> (&'a str, &'a str) {
    (long, local)
}

fn main() {
    let local = String::from("corto");
    let (left, right) = pair_with_local("largo", &local);
    assert_eq!((left, right), ("largo", "corto"));
    assert_eq!(shorten("estático"), "estático");
}
```

### `ch20-b002` — 20.1 El subtyping de Rust es deliberadamente estrecho

Source: `20.Subtyping-variance-y-HRTB.md:34` · mode: `compile_fail`

```rust,compile_fail
fn extend<'short>(value: &'short str) -> &'static str {
    value
}

fn main() {}
```

### `ch20-b003` — 20.4 Por qué `&mut T` es invariante en `T`

Source: `20.Subtyping-variance-y-HRTB.md:71` · mode: `run`

```rust
fn assign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let first = String::from("primero");
    let second = String::from("segundo");
    let mut slot: &str = &first;
    assign(&mut slot, &second);
    assert_eq!(slot, "segundo");
}
```

### `ch20-b004` — 20.4 Por qué `&mut T` es invariante en `T`

Source: `20.Subtyping-variance-y-HRTB.md:89` · mode: `compile_fail`

```rust,compile_fail
fn assign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let mut slot: &'static str = "válido siempre";
    {
        let temporary = String::from("temporal");
        assign(&mut slot, &temporary);
    }
    println!("{slot}");
}
```

### `ch20-b005` — 20.5 Interior mutability exige la misma cautela

Source: `20.Subtyping-variance-y-HRTB.md:110` · mode: `run`

```rust
use std::cell::Cell;

fn replace<'a>(slot: &Cell<&'a str>, value: &'a str) {
    slot.set(value);
}

fn main() {
    let first = String::from("A");
    let second = String::from("B");
    let slot = Cell::new(first.as_str());
    replace(&slot, &second);
    assert_eq!(slot.get(), "B");
}
```

### `ch20-b006` — 20.6 Tus structs heredan la variance de sus campos

Source: `20.Subtyping-variance-y-HRTB.md:130` · mode: `run`

```rust
use std::cell::Cell;

struct Viewer<'a> {
    view: &'a str,
}

struct Editor<'a> {
    view: &'a str,
    selected: Cell<&'a str>,
}

fn shorten_viewer<'short>(viewer: Viewer<'static>) -> Viewer<'short> {
    viewer
}

fn main() {
    let viewer = Viewer { view: "documento" };
    let shorter = shorten_viewer(viewer);
    assert_eq!(shorter.view, "documento");

    let editor = Editor { view: "documento", selected: Cell::new("doc") };
    assert_eq!(editor.selected.get(), "doc");
    assert_eq!(editor.view, "documento");
}
```

### `ch20-b007` — 20.6 Tus structs heredan la variance de sus campos

Source: `20.Subtyping-variance-y-HRTB.md:161` · mode: `compile_fail`

```rust,compile_fail
use std::cell::Cell;

struct Editor<'a> {
    selected: Cell<&'a str>,
}

fn shorten_editor<'short>(editor: Editor<'static>) -> Editor<'short> {
    editor
}

fn main() {}
```

### `ch20-b008` — 20.7 Las entradas de función son contravariantes

Source: `20.Subtyping-variance-y-HRTB.md:181` · mode: `run`

```rust
fn accepts_any(text: &str) -> usize {
    text.len()
}

fn main() {
    let only_receives_static: fn(&'static str) -> usize = accepts_any;
    assert_eq!(only_receives_static("Rust"), 4);
}
```

### `ch20-b009` — 20.7 Las entradas de función son contravariantes

Source: `20.Subtyping-variance-y-HRTB.md:196` · mode: `compile_fail`

```rust,compile_fail
fn only_static(text: &'static str) -> usize {
    text.len()
}

fn main() {
    let accepts_any: fn(&str) -> usize = only_static;
    let local = String::from("local");
    assert_eq!(accepts_any(&local), 5);
}
```

### `ch20-b010` — 20.8 HRTB significa «para todo lifetime»

Source: `20.Subtyping-variance-y-HRTB.md:214` · mode: `run`

```rust
fn with_local<F>(callback: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let local = String::from("interno");
    callback(&local)
}

fn length(value: &str) -> usize {
    value.len()
}

fn main() {
    assert_eq!(with_local(length), 7);
    assert_eq!(with_local(|value: &str| value.chars().count()), 7);
}
```

### `ch20-b011` — 20.9 La sintaxis corta suele esconder el `for<'a>`

Source: `20.Subtyping-variance-y-HRTB.md:243` · mode: `run`

```rust
fn identity(value: &str) -> &str {
    value
}

fn apply<F>(callback: F, value: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    callback(value)
}

fn main() {
    let local = String::from("Rust");
    assert_eq!(apply(identity, &local), "Rust");
}
```

### `ch20-b012` — 20.10 Cuando un callback no es suficientemente general

Source: `20.Subtyping-variance-y-HRTB.md:269` · mode: `compile_fail`

```rust,compile_fail
fn require_identity<F>(_callback: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
}

fn main() {
    let fallback = String::from("capturado");
    let ignores_input = |_input: &str| fallback.as_str();
    require_identity(ignores_input);
}
```

### `ch20-b013` — 20.10 Cuando un callback no es suficientemente general

Source: `20.Subtyping-variance-y-HRTB.md:285` · mode: `run`

```rust
fn require_identity<F>(callback: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("local");
    callback(&local).to_owned()
}

fn identity(value: &str) -> &str {
    value
}

fn main() {
    assert_eq!(require_identity(identity), "local");
}
```

### `ch20-b014` — 20.11 `PhantomData` declara una relación lógica

Source: `20.Subtyping-variance-y-HRTB.md:309` · mode: `run`

```rust
use std::marker::PhantomData;
use std::mem::size_of;

struct Owns<T> {
    id: usize,
    marker: PhantomData<T>,
}

struct Consumes<T> {
    id: usize,
    marker: PhantomData<fn(T)>,
}

fn main() {
    let owns = Owns::<String> { id: 1, marker: PhantomData };
    let consumes = Consumes::<String> { id: 2, marker: PhantomData };
    assert_eq!((owns.id, consumes.id), (1, 2));
    assert_eq!(size_of::<Owns<String>>(), size_of::<usize>());
    assert_eq!(size_of::<Consumes<String>>(), size_of::<usize>());
}
```

## 21.Box-Rc-y-Arc

### `ch21-b001` — 21.2 `Box<T>`

Source: `21.Box-Rc-y-Arc.md:28` · mode: `run`

```rust
let boxed = Box::new(String::from("rust"));
assert_eq!(boxed.len(), 4);
```

### `ch21-b002` — 21.2 `Box<T>`

Source: `21.Box-Rc-y-Arc.md:37` · mode: `run`

```rust
let big: Box<[u8; 4096]> = Box::new([0; 4096]);
assert_eq!(std::mem::size_of_val(&big), std::mem::size_of::<usize>());
```

### `ch21-b003` — 21.2 `Box<T>`

Source: `21.Box-Rc-y-Arc.md:46` · mode: `run`

```rust
struct DetailedReport {
    lines: [u64; 512],
}

enum Message {
    Ping,
    Close,
    Report(Box<DetailedReport>), // sin Box, cada Message mediría ~4 KB
}
```

### `ch21-b004` — 21.3 Tipos recursivos

Source: `21.Box-Rc-y-Arc.md:64` · mode: `compile_fail`

```rust,compile_fail
enum List<T> {
    Nil,
    Cons(T, List<T>),
    // error[E0072]: recursive type `List` has infinite size
}
```

### `ch21-b005` — 21.3 Tipos recursivos

Source: `21.Box-Rc-y-Arc.md:74` · mode: `run`

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

fn sum(list: &List<i32>) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(value, rest) => value + sum(rest),
    }
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
assert_eq!(sum(&list), 3);
```

### `ch21-b006` — 21.4 `Box<dyn Trait>`

Source: `21.Box-Rc-y-Arc.md:97` · mode: `run`

```rust
trait Job { fn run(&self); }

fn queue(job: impl Job + 'static) -> Box<dyn Job> {
    Box::new(job)
}
```

### `ch21-b007` — 21.5 `Rc<T>`

Source: `21.Box-Rc-y-Arc.md:111` · mode: `run`

```rust
use std::rc::Rc;

let config = Rc::new(String::from("local"));
let another_owner = Rc::clone(&config);
assert_eq!(Rc::strong_count(&config), 2);
```

### `ch21-b008` — 21.6 Recuperar unicidad y copy-on-write

Source: `21.Box-Rc-y-Arc.md:125` · mode: `run`

```rust
use std::rc::Rc;

let mut current = Rc::new(vec![1, 2]);
assert!(Rc::get_mut(&mut current).is_some());

let snapshot = Rc::clone(&current);
assert!(Rc::get_mut(&mut current).is_none());

Rc::make_mut(&mut current).push(3); // clona Vec porque existe snapshot
assert_eq!(&*snapshot, &[1, 2]);
assert_eq!(&*current, &[1, 2, 3]);
```

### `ch21-b009` — 21.7 `Weak<T>` y ciclos

Source: `21.Box-Rc-y-Arc.md:145` · mode: `contextual`

```rust,ignore
let weak = Rc::downgrade(&config);
drop(config);
drop(another_owner);
assert!(weak.upgrade().is_none());
```

### `ch21-b010` — 21.7 `Weak<T>` y ciclos

Source: `21.Box-Rc-y-Arc.md:156` · mode: `run`

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

let root = Rc::new(Node {
    name: String::from("root"),
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(Vec::new()),
});

let leaf = Rc::new(Node {
    name: String::from("leaf"),
    parent: RefCell::new(Rc::downgrade(&root)),
    children: RefCell::new(Vec::new()),
});

root.children.borrow_mut().push(Rc::clone(&leaf));

let parent_name = leaf.parent.borrow().upgrade().map(|node| node.name.clone());
assert_eq!(parent_name.as_deref(), Some("root"));
```

### `ch21-b011` — 21.10 `Arc<T>`

Source: `21.Box-Rc-y-Arc.md:212` · mode: `run`

```rust
use std::sync::Arc;
use std::thread;

struct Config {
    retries: u32,
    endpoint: String,
}

let config = Arc::new(Config {
    retries: 3,
    endpoint: String::from("https://api.example.com"),
});

let handles: Vec<_> = (0..4)
    .map(|worker| {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            format!("worker {worker} usa {} con {} reintentos",
                config.endpoint, config.retries)
        })
    })
    .collect();

for handle in handles {
    println!("{}", handle.join().unwrap());
}
```

### `ch21-b012` — 21.12 `Deref` y coerciones

Source: `21.Box-Rc-y-Arc.md:265` · mode: `run`

```rust
use std::sync::Arc;

let shared: Arc<String> = Arc::new(String::from("rust"));
assert_eq!(shared.len(), 4);   // String::len, a través del Arc

let view: &str = &shared;      // Arc<String> -> &String -> &str, dos pasos de coerción
assert_eq!(view, "rust");
```

### `ch21-b013` — 21.13 Elegir por el grafo de ownership

Source: `21.Box-Rc-y-Arc.md:287` · mode: `contextual`

```rust,ignore
struct Order {
    customer_id: CustomerId, // no Rc<Customer>
    lines: Vec<OrderLine>,
}
```

## 22.Cell-y-RefCell

### `ch22-b001` — 22.3 `Cell<T>`

Source: `22.Cell-y-RefCell.md:31` · mode: `run`

```rust
use std::cell::Cell;

struct Metrics {
    hits: Cell<u64>,
}

impl Metrics {
    fn record(&self) {
        self.hits.set(self.hits.get() + 1);
    }
}
```

### `ch22-b002` — 22.4 Por qué `Cell` es segura

Source: `22.Cell-y-RefCell.md:53` · mode: `run`

```rust
use std::cell::Cell;

let message = Cell::new(String::from("pendiente"));

let previous = message.replace(String::from("enviado"));
assert_eq!(previous, "pendiente");

let last = message.take(); // deja String::default() dentro
assert_eq!(last, "enviado");
assert_eq!(message.take(), "");
```

### `ch22-b003` — 22.5 `RefCell<T>`

Source: `22.Cell-y-RefCell.md:72` · mode: `run`

```rust
use std::cell::RefCell;

let names = RefCell::new(vec![String::from("Ada")]);
names.borrow_mut().push(String::from("Linus"));
assert_eq!(names.borrow().len(), 2);
```

### `ch22-b004` — 22.5 `RefCell<T>`

Source: `22.Cell-y-RefCell.md:82` · mode: `should_panic`

```rust,should_panic
use std::cell::RefCell;

let names = RefCell::new(vec![String::from("Ada")]);

let reader = names.borrow();
let writer = names.borrow_mut(); // panic: already borrowed: BorrowMutError
println!("{}", reader.len());
```

### `ch22-b005` — 22.7 El scope de los guards

Source: `22.Cell-y-RefCell.md:115` · mode: `contextual`

```rust,ignore
let first_borrow = names.borrow();
println!("{}", first_borrow.len());
drop(first_borrow);

names.borrow_mut().clear();
```

### `ch22-b006` — 22.8 Guards escondidos: `match` e `if let`

Source: `22.Cell-y-RefCell.md:129` · mode: `should_panic`

```rust,should_panic
use std::cell::RefCell;
use std::collections::HashMap;

let cache = RefCell::new(HashMap::<String, u64>::new());

let value = match cache.borrow().get("clave") {
    Some(value) => *value,
    None => {
        // El guard de `borrow()` sigue vivo durante todo el match:
        cache.borrow_mut().insert(String::from("clave"), 42); // panic
        42
    }
};
```

### `ch22-b007` — 22.8 Guards escondidos: `match` e `if let`

Source: `22.Cell-y-RefCell.md:147` · mode: `run`

```rust
use std::cell::RefCell;
use std::collections::HashMap;

let cache = RefCell::new(HashMap::<String, u64>::new());

let value = if let Some(value) = cache.borrow().get("clave") {
    *value
} else {
    // Edición 2024: el guard ya se liberó al entrar aquí.
    cache.borrow_mut().insert(String::from("clave"), 42);
    42
};

assert_eq!(value, 42);
```

### `ch22-b008` — 22.8 Guards escondidos: `match` e `if let`

Source: `22.Cell-y-RefCell.md:168` · mode: `contextual`

```rust,ignore
let cached = cache.borrow().get("clave").copied(); // el guard muere aquí

let value = match cached {
    Some(value) => value,
    None => {
        cache.borrow_mut().insert(String::from("clave"), 42);
        42
    }
};
```

### `ch22-b009` — 22.9 Evitar panics con `try_borrow`

Source: `22.Cell-y-RefCell.md:184` · mode: `contextual`

```rust,ignore
match names.try_borrow_mut() {
    Ok(mut values) => values.push(String::from("Grace")),
    Err(_) => eprintln!("estado ocupado"),
}
```

### `ch22-b010` — 22.10 `Rc<RefCell<T>>`

Source: `22.Cell-y-RefCell.md:197` · mode: `run`

```rust
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct EventLog {
    events: Rc<RefCell<Vec<String>>>,
}

impl EventLog {
    fn new() -> Self {
        Self { events: Rc::new(RefCell::new(Vec::new())) }
    }

    fn record(&self, event: impl Into<String>) {
        self.events.borrow_mut().push(event.into());
    }

    fn snapshot(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}
```

### `ch22-b011` — 22.10 `Rc<RefCell<T>>`

Source: `22.Cell-y-RefCell.md:222` · mode: `contextual`

```rust,ignore
let log = EventLog::new();

let for_button = log.clone();
let on_click = move || for_button.record("click");

on_click();
log.record("shutdown");

assert_eq!(log.snapshot(), vec!["click".to_string(), "shutdown".to_string()]);
```

### `ch22-b012` — 22.11 Reentrancia: liberar antes de llamar hacia fuera

Source: `22.Cell-y-RefCell.md:242` · mode: `run`

```rust
use std::cell::RefCell;

struct Dispatcher {
    events: RefCell<Vec<&'static str>>,
}

impl Dispatcher {
    fn emit(&self, callback: impl FnOnce(&Self)) {
        {
            let mut events = self.events.borrow_mut();
            events.push("emit");
        } // RefMut destruido antes de la callback

        callback(self);
    }

    fn count(&self) -> usize {
        self.events.borrow().len()
    }
}

let dispatcher = Dispatcher { events: RefCell::new(Vec::new()) };
dispatcher.emit(|same| assert_eq!(same.count(), 1));
```

### `ch22-b013` — 22.12 Caches y lazy initialization

Source: `22.Cell-y-RefCell.md:274` · mode: `run`

```rust
use std::cell::OnceCell;

struct Settings {
    raw_port: String,
    parsed: OnceCell<u16>,
}

impl Settings {
    fn port(&self) -> u16 {
        *self.parsed.get_or_init(|| self.raw_port.trim().parse().unwrap_or(8080))
    }
}
```

### `ch22-b014` — 22.12 Caches y lazy initialization

Source: `22.Cell-y-RefCell.md:289` · mode: `contextual`

```rust,ignore
let settings = Settings {
    raw_port: String::from(" 9090 "),
    parsed: OnceCell::new(),
};

assert_eq!(settings.port(), 9090); // parsea la primera vez
assert_eq!(settings.port(), 9090); // reutiliza sin parsear
```

## 23.Mutex-RwLock-y-estado-compartido

### `ch23-b001` — 23.1 La exclusión forma parte del tipo

Source: `23.Mutex-RwLock-y-estado-compartido.md:9` · mode: `run`

```rust
use std::sync::Mutex;

let balance = Mutex::new(100_u64);
{
    let mut value = balance.lock().unwrap();
    *value += 50;
}
```

### `ch23-b002` — 23.3 Compartir entre threads

Source: `23.Mutex-RwLock-y-estado-compartido.md:42` · mode: `run`

```rust
use std::{sync::{Arc, Mutex}, thread};

let counter = Arc::new(Mutex::new(0_u64));
let handles: Vec<_> = (0..4)
    .map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || *counter.lock().unwrap() += 1)
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}

assert_eq!(*counter.lock().unwrap(), 4);
```

### `ch23-b003` — 23.4 Región crítica pequeña

Source: `23.Mutex-RwLock-y-estado-compartido.md:66` · mode: `contextual`

```rust,ignore
let snapshot = {
    let state = shared.lock().unwrap();
    state.clone_for_report()
};

write_report(snapshot)?;
```

### `ch23-b004` — 23.5 La API expone transiciones, no guards

Source: `23.Mutex-RwLock-y-estado-compartido.md:81` · mode: `run`

```rust
use std::sync::Mutex;

#[derive(Default)]
struct Counts {
    total: u32,
    reserved: u32,
}

struct Inventory {
    counts: Mutex<Counts>,
}

impl Inventory {
    fn reserve(&self, amount: u32) -> bool {
        let mut counts = self.counts.lock().unwrap();
        let Some(after) = counts.reserved.checked_add(amount) else {
            return false;
        };
        if after > counts.total {
            return false;
        }
        counts.reserved = after;
        true
    }

    fn snapshot(&self) -> (u32, u32) {
        let counts = self.counts.lock().unwrap();
        (counts.total, counts.reserved)
    }
}

let inventory = Inventory {
    counts: Mutex::new(Counts { total: 10, reserved: 0 }),
};
assert!(inventory.reserve(4));
assert_eq!(inventory.snapshot(), (10, 4));
```

### `ch23-b005` — 23.6 Poisoning

Source: `23.Mutex-RwLock-y-estado-compartido.md:126` · mode: `contextual`

```rust,ignore
let mut value = match balance.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
        // Decisión consciente: sabemos verificar/restaurar la invariante.
        poisoned.into_inner()
    }
};
*value = 0;
```

### `ch23-b006` — 23.7 `RwLock<T>`

Source: `23.Mutex-RwLock-y-estado-compartido.md:143` · mode: `contextual`

```rust,ignore
use std::sync::RwLock;

let config = RwLock::new(Config::default());
let port = config.read().unwrap().port;
config.write().unwrap().reload()?;
```

### `ch23-b007` — 23.8 Deadlocks

Source: `23.Mutex-RwLock-y-estado-compartido.md:159` · mode: `reference`

```text
thread A: lock users -> lock orders
thread B: lock orders -> lock users
```

### `ch23-b008` — 23.8 Deadlocks

Source: `23.Mutex-RwLock-y-estado-compartido.md:174` · mode: `run`

```rust
use std::sync::Mutex;

struct Account {
    id: u64,
    balance: Mutex<u64>,
}

#[derive(Debug, PartialEq, Eq)]
enum TransferError {
    SameAccount,
    Insufficient,
}

fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(), TransferError> {
    if from.id == to.id {
        // Sin esta guarda, la doble adquisición del mismo mutex
        // bloquearía para siempre: el Mutex de std no es reentrante.
        return Err(TransferError::SameAccount);
    }

    // Regla global: bloquear siempre primero la cuenta de id menor.
    let (first, second) = if from.id < to.id { (from, to) } else { (to, from) };
    let mut first_guard = first.balance.lock().unwrap();
    let mut second_guard = second.balance.lock().unwrap();

    let (from_balance, to_balance) = if from.id < to.id {
        (&mut *first_guard, &mut *second_guard)
    } else {
        (&mut *second_guard, &mut *first_guard)
    };

    if *from_balance < amount {
        return Err(TransferError::Insufficient);
    }
    *from_balance -= amount;
    *to_balance += amount;
    Ok(())
}
```

### `ch23-b009` — 23.9 Esperar una condición: `Condvar`

Source: `23.Mutex-RwLock-y-estado-compartido.md:221` · mode: `compile_only`

```rust,no_run
use std::sync::{Condvar, Mutex};

struct Queue {
    items: Mutex<Vec<u64>>,
    ready: Condvar,
}

impl Queue {
    fn push(&self, item: u64) {
        self.items.lock().unwrap().push(item);
        self.ready.notify_one();
    }

    fn pop_blocking(&self) -> u64 {
        let mut items = self.items.lock().unwrap();
        loop {
            if let Some(item) = items.pop() {
                return item;
            }
            items = self.ready.wait(items).unwrap();
        }
    }
}
```

### `ch23-b010` — 23.10 Granularidad e invariantes

Source: `23.Mutex-RwLock-y-estado-compartido.md:258` · mode: `contextual`

```rust,ignore
// Dos locks: la invariante "reserved <= total" puede observarse rota.
struct Inventory {
    total: Mutex<u32>,
    reserved: Mutex<u32>,
}

// Un lock: la invariante cambia atómicamente o no cambia.
struct InventoryAtomic {
    counts: Mutex<Counts>,
}

struct Counts {
    total: u32,
    reserved: u32,
}
```

### `ch23-b011` — 23.11 Estado compartido frente a mensajes

Source: `23.Mutex-RwLock-y-estado-compartido.md:280` · mode: `reference`

```text
productor A ──┐
productor B ──┼── Sender<Command> ──> [thread dueño del State] ──> eventos/respuestas
productor C ──┘
```

## 24.Pin-Unpin-y-tipos-autorreferenciales

### `ch24-b001` — 24.1 Mover no es mutar

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:11` · mode: `contextual`

```rust,ignore
async fn example() {
    let data = [0_u8; 64];
    let view = &data[..4]; // referencia a datos que viven EN el future
    pause().await;
    println!("{view:?}");
}
```

### `ch24-b002` — 24.1 Mover no es mutar

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:22` · mode: `reference`

```text
ExampleFuture (estado tras el primer poll)
├── data: [u8; 64]          bytes propios del future
└── view: puntero ────────> ...hacia data, DENTRO del mismo future
```

### `ch24-b003` — 24.2 Qué garantiza `Pin<P>`

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:36` · mode: `contextual`

```rust,ignore
let pinned: std::pin::Pin<Box<MyFuture>> = Box::pin(make_future());
```

### `ch24-b004` — 24.2 Qué garantiza `Pin<P>`

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:42` · mode: `compile_fail`

```rust,compile_fail
use std::pin::pin;

let future = pin!(async { 1 });
let inner = future.get_mut();
// error[E0277]: `{async block}` cannot be unpinned
```

### `ch24-b005` — 24.3 Se mueve el puntero, no el pointee

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:60` · mode: `run`

```rust
use std::marker::PhantomPinned;

struct Stable {
    name: String,
    _pin: PhantomPinned,
}

let pinned = Box::pin(Stable {
    name: String::from("rust"),
    _pin: PhantomPinned,
});
let before = pinned.as_ref().get_ref() as *const Stable;

let moved_handle = pinned; // se mueve Pin<Box<_>>, no Stable
let after = moved_handle.as_ref().get_ref() as *const Stable;

assert_eq!(before, after);
assert_eq!(moved_handle.as_ref().get_ref().name.as_str(), "rust");
```

### `ch24-b006` — 24.4 `Unpin`

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:87` · mode: `run`

```rust
use std::pin::Pin;

let mut number = 42_u64;
let pinned: Pin<&mut u64> = Pin::new(&mut number);
let normal: &mut u64 = pinned.get_mut(); // u64: Unpin, sin restricción real
*normal += 1;
assert_eq!(number, 43);
```

### `ch24-b007` — 24.4 `Unpin`

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:101` · mode: `run`

```rust
use std::marker::PhantomPinned;

struct AddressSensitive {
    data: String,
    _pin: PhantomPinned,
}
```

### `ch24-b008` — 24.5 Construcción pinneada

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:116` · mode: `run`

```rust
use std::pin::pin;

let future = async { 42 };
let mut pinned = pin!(future);
```

### `ch24-b009` — 24.7 Proyección de campos

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:144` · mode: `run`

```rust
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Counted<F> {
    future: F,     // estructuralmente pinneado: es la máquina async
    polls: u32,    // no pinneado: un u32 puede moverse sin riesgo
}

impl<F> Counted<F> {
    fn project(self: Pin<&mut Self>) -> (Pin<&mut F>, &mut u32) {
        // SAFETY: `future` nunca se mueve fuera de `self` ni se
        // reemplaza: solo se re-pinnea. Entregar `polls` como &mut
        // no permite mover `future`, y `Counted` no implementa
        // `Unpin` a la carta ni un Drop que mueva campos.
        unsafe {
            let this = self.get_unchecked_mut();
            (Pin::new_unchecked(&mut this.future), &mut this.polls)
        }
    }
}

impl<F: Future> Future for Counted<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let (future, polls) = self.project();
        *polls += 1;
        future.poll(cx)
    }
}
```

### `ch24-b010` — 24.9 Relación con `Future::poll`

Source: `24.Pin-Unpin-y-tipos-autorreferenciales.md:192` · mode: `run`

```rust
trait Future {
    type Output;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output>;
}
```

## 25.Módulos-paths-y-privacidad

### `ch25-b001` — 25.2 Crate roots

Source: `25.Módulos-paths-y-privacidad.md:25` · mode: `illustrative`

```rust,ignore
// src/lib.rs
pub mod domain;
mod internal;
```

### `ch25-b002` — 25.3 Archivos y módulos

Source: `25.Módulos-paths-y-privacidad.md:35` · mode: `reference`

```text
src/
├── lib.rs
├── domain.rs
└── domain/
    ├── order.rs
    └── user.rs
```

### `ch25-b003` — 25.3 Archivos y módulos

Source: `25.Módulos-paths-y-privacidad.md:46` · mode: `illustrative`

```rust,ignore
pub mod order;
pub mod user;
```

### `ch25-b004` — 25.4 Paths

Source: `25.Módulos-paths-y-privacidad.md:62` · mode: `run`

```rust
// src/lib.rs
pub mod domain {
    pub mod order {
        pub struct Order {
            pub id: u64,
        }

        pub fn validate(order: &Order) -> bool {
            super::shared_rule(order.id) // sube al módulo `domain`
        }
    }

    fn shared_rule(id: u64) -> bool {
        id != 0
    }
}

pub fn report(order: &domain::order::Order) -> String {
    format!("pedido {}", order.id)
}
```

### `ch25-b005` — 25.5 `pub` es visibilidad, no alcanzabilidad automática

Source: `25.Módulos-paths-y-privacidad.md:91` · mode: `run`

```rust
mod internal {
    pub struct Engine;
}

pub use internal::Engine; // crea la ruta pública estable crate::Engine
```

### `ch25-b006` — 25.6 `use` no cambia privacidad

Source: `25.Módulos-paths-y-privacidad.md:107` · mode: `contextual`

```rust,ignore
use crate::domain::order::Order;
```

### `ch25-b007` — 25.6 `use` no cambia privacidad

Source: `25.Módulos-paths-y-privacidad.md:115` · mode: `contextual`

```rust,ignore
use std::collections::HashMap;            // tipos: importados directamente
use std::fmt::{self, Display, Formatter}; // `self` trae también el módulo
use crate::domain::order as orders;       // renombrar rutas largas o en conflicto

fn describe(map: &HashMap<String, u64>) -> fmt::Result {
    // funciones ajenas suelen llamarse cualificadas: fmt::format, cmp::min...
    Ok(())
}
```

### `ch25-b008` — 25.7 Visibilidad graduada

Source: `25.Módulos-paths-y-privacidad.md:130` · mode: `contextual`

```rust,ignore
pub struct Api;
pub(crate) struct CrateInternal;
pub(super) fn parent_only() {}
pub(in crate::domain) fn domain_only() {}
```

### `ch25-b009` — 25.7 Visibilidad graduada

Source: `25.Módulos-paths-y-privacidad.md:139` · mode: `compile_fail`

```rust,compile_fail
mod auth {
    fn hash_password(input: &str) -> String {
        format!("hash:{input}")
    }
}

fn main() {
    auth::hash_password("secreto");
    // error[E0603]: function `hash_password` is private
}
```

### `ch25-b010` — 25.8 Facade mediante reexports

Source: `25.Módulos-paths-y-privacidad.md:158` · mode: `illustrative`

```rust,ignore
mod order;
mod user;

pub use order::{Order, OrderError};
pub use user::{User, UserId};
```

### `ch25-b011` — 25.9 Privacidad y construcción

Source: `25.Módulos-paths-y-privacidad.md:172` · mode: `run`

```rust
pub struct Percentage(u8);

impl Percentage {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
}
```

### `ch25-b012` — 25.9 Privacidad y construcción

Source: `25.Módulos-paths-y-privacidad.md:184` · mode: `compile_fail`

```rust,compile_fail
let p = other_crate::Percentage(150);
// error[E0603]: cannot initialize a tuple struct which contains private fields
```

### `ch25-b013` — 25.9 Privacidad y construcción

Source: `25.Módulos-paths-y-privacidad.md:193` · mode: `run`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_out_of_range() {
        assert!(Percentage::new(101).is_none());
        assert!(Percentage::new(100).is_some());
    }
}
```

### `ch25-b014` — 25.11 API pública accidental

Source: `25.Módulos-paths-y-privacidad.md:216` · mode: `contextual`

```rust,ignore
// Aunque `Config` sea tuyo, esta firma promete `toml` para siempre:
pub fn parse(input: &str) -> Result<Config, toml::de::Error>
```

### `ch25-b015` — 25.12 Diseñar espacio para evolucionar

Source: `25.Módulos-paths-y-privacidad.md:227` · mode: `run`

```rust
#[non_exhaustive]
pub enum LoadError {
    NotFound,
    InvalidFormat,
}
```

## 26.Crates-workspaces-y-capas

### `ch26-b001` — 26.3 Workspace

Source: `26.Crates-workspaces-y-capas.md:36` · mode: `parse`

```toml
[workspace]
resolver = "3"
members = [
    "crates/domain",
    "crates/application",
    "crates/adapters",
    "apps/server",
]
```

### `ch26-b002` — 26.4 Dependencias heredadas

Source: `26.Crates-workspaces-y-capas.md:53` · mode: `parse`

```toml
[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

### `ch26-b003` — 26.4 Dependencias heredadas

Source: `26.Crates-workspaces-y-capas.md:64` · mode: `parse`

```toml
[package]
edition.workspace = true

[dependencies]
serde.workspace = true
```

### `ch26-b004` — 26.5 Dirección de dependencias

Source: `26.Crates-workspaces-y-capas.md:78` · mode: `reference`

```text
server/desktop ──> adapters ──> application ──> domain
                         └────────────────────> domain
```

### `ch26-b005` — 26.5 Dirección de dependencias

Source: `26.Crates-workspaces-y-capas.md:91` · mode: `toml_composite`

```toml
# crates/application/Cargo.toml
[dependencies]
domain = { path = "../domain" }

# crates/adapters/Cargo.toml
[dependencies]
domain = { path = "../domain" }
application = { path = "../application" }
sqlx = "0.8"   # la infraestructura pesada vive aquí, no en el núcleo
```

### `ch26-b006` — 26.6 Dónde vive el trait

Source: `26.Crates-workspaces-y-capas.md:109` · mode: `contextual`

```rust,ignore
// crates/application/src/lib.rs
use domain::{Order, OrderId};

#[derive(Debug)]
pub struct RepositoryError(pub String);

pub trait OrderRepository {
    fn find(&self, id: OrderId) -> Result<Option<Order>, RepositoryError>;
    fn save(&self, order: &Order) -> Result<(), RepositoryError>;
}

pub struct PlaceOrder<R> {
    repository: R,
}

impl<R: OrderRepository> PlaceOrder<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, order: Order) -> Result<(), RepositoryError> {
        // reglas del caso de uso...
        self.repository.save(&order)
    }
}
```

### `ch26-b007` — 26.7 Library crate más binarios finos

Source: `26.Crates-workspaces-y-capas.md:145` · mode: `contextual`

```rust,ignore
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let app = build_application(config)?;
    app.run()
}
```

### `ch26-b008` — 26.8 Tipos de frontera

Source: `26.Crates-workspaces-y-capas.md:159` · mode: `reference`

```text
JSON/SQL/IPC DTO <-> comando o respuesta de aplicación <-> tipos de dominio
```

### `ch26-b009` — 26.8 Tipos de frontera

Source: `26.Crates-workspaces-y-capas.md:165` · mode: `contextual`

```rust,ignore
// crates/adapters/src/http.rs
pub struct CreateOrderRequest {
    pub customer_id: u64,
    pub lines: Vec<LineRequest>,
}

impl TryFrom<CreateOrderRequest> for application::PlaceOrderCommand {
    type Error = ValidationError;

    fn try_from(request: CreateOrderRequest) -> Result<Self, Self::Error> {
        // aquí se valida, se convierten unidades y se rechaza lo inválido
        /* ... */
    }
}
```

### `ch26-b010` — 26.11 Tests en un workspace

Source: `26.Crates-workspaces-y-capas.md:209` · mode: `parse`

```toml
# crates/application/Cargo.toml
[dev-dependencies]
test-support = { path = "../test-support" }
```

## 27.Cargo-features-targets-y-profiles

### `ch27-b001` — 27.1 El manifest es parte del diseño

Source: `27.Cargo-features-targets-y-profiles.md:9` · mode: `parse`

```toml
[package]
name = "catalog"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
```

### `ch27-b002` — 27.2 Dependencias y SemVer

Source: `27.Cargo-features-targets-y-profiles.md:21` · mode: `parse`

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
```

### `ch27-b003` — 27.2 Dependencias y SemVer

Source: `27.Cargo-features-targets-y-profiles.md:28` · mode: `reference`

```text
"1"     == "^1"     -> >=1.0.0, <2.0.0
"1.4"   == "^1.4"   -> >=1.4.0, <2.0.0
"0.3.2" == "^0.3.2" -> >=0.3.2, <0.4.0
"0.0.3" == "^0.0.3" -> >=0.0.3, <0.0.4
"=1.4.2"            -> exactamente 1.4.2
```

### `ch27-b004` — 27.3 Dependencias por contexto

Source: `27.Cargo-features-targets-y-profiles.md:50` · mode: `parse`

```toml
[dependencies]
tracing = "0.1"

[dev-dependencies]
proptest = "1"

[build-dependencies]
cc = "1"
```

### `ch27-b005` — 27.3 Dependencias por contexto

Source: `27.Cargo-features-targets-y-profiles.md:63` · mode: `run`

```rust
// build.rs
fn main() {
    // Solo reejecutar si cambia el schema, no en cada build:
    println!("cargo::rerun-if-changed=schema/catalog.proto");
    // Exponer un valor calculado al código:
    println!("cargo::rustc-env=CATALOG_SCHEMA_VERSION=3");
}
```

### `ch27-b006` — 27.3 Dependencias por contexto

Source: `27.Cargo-features-targets-y-profiles.md:73` · mode: `illustrative`

```rust,ignore
// En el código de la crate:
const SCHEMA_VERSION: &str = env!("CATALOG_SCHEMA_VERSION");
```

### `ch27-b007` — 27.4 Features son aditivas

Source: `27.Cargo-features-targets-y-profiles.md:82` · mode: `parse`

```toml
[features]
default = []
json = ["dep:serde", "dep:serde_json"]

[dependencies]
serde = { version = "1", optional = true }
serde_json = { version = "1", optional = true }
```

### `ch27-b008` — 27.4 Features son aditivas

Source: `27.Cargo-features-targets-y-profiles.md:94` · mode: `parse`

```toml
# MAL DISEÑO: dos features que eligen backend
[features]
backend-postgres = []
backend-sqlite = []
```

### `ch27-b009` — 27.4 Features son aditivas

Source: `27.Cargo-features-targets-y-profiles.md:105` · mode: `reference`

```text
cargo check --no-default-features
cargo check --no-default-features --features json
cargo check --all-features
```

### `ch27-b010` — 27.5 Features por capacidad

Source: `27.Cargo-features-targets-y-profiles.md:117` · mode: `run`

```rust
#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "json")]
impl Order {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Order siempre serializa")
    }
}
```

### `ch27-b011` — 27.6 Targets

Source: `27.Cargo-features-targets-y-profiles.md:144` · mode: `parse`

```toml
[[bin]]
name = "debug-inspector"
required-features = ["inspector"]
```

### `ch27-b012` — 27.7 Profiles

Source: `27.Cargo-features-targets-y-profiles.md:152` · mode: `parse`

```toml
[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

### `ch27-b013` — 27.7 Profiles

Source: `27.Cargo-features-targets-y-profiles.md:169` · mode: `parse`

```toml
[profile.dev.package."*"]
opt-level = 2
```

### `ch27-b014` — 27.8 Inspeccionar la resolución

Source: `27.Cargo-features-targets-y-profiles.md:184` · mode: `reference`

```text
cargo metadata
cargo tree
cargo tree -e features
cargo update -p crate_name
cargo build --all-targets
cargo test --workspace
```

## 28.Testing-doctests-y-property-testing

### `ch28-b001` — 28.1 Probar contratos

Source: `28.Testing-doctests-y-property-testing.md:9` · mode: `run`

```rust
#[test]
fn percentage_rejects_values_above_one_hundred() {
    assert!(Percentage::new(101).is_none());
}
```

### `ch28-b002` — 28.2 Unit tests

Source: `28.Testing-doctests-y-property-testing.md:22` · mode: `run`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_surrounding_space() {
        assert_eq!(Name::parse("  Ada  ").unwrap().as_str(), "Ada");
    }
}
```

### `ch28-b003` — 28.3 Integration tests

Source: `28.Testing-doctests-y-property-testing.md:40` · mode: `contextual`

```rust,ignore
use catalog::Percentage;

#[test]
fn public_constructor_is_usable() {
    assert_eq!(Percentage::new(25).unwrap().get(), 25);
}
```

### `ch28-b004` — 28.3 Integration tests

Source: `28.Testing-doctests-y-property-testing.md:51` · mode: `reference`

```text
tests/
├── api.rs          # cada archivo .rs es una crate de test
├── workflows.rs
└── common/
    └── mod.rs      # compartido: NO se ejecuta como test propio
```

### `ch28-b005` — 28.3 Integration tests

Source: `28.Testing-doctests-y-property-testing.md:59` · mode: `illustrative`

```rust,ignore
// tests/api.rs
mod common;

#[test]
fn creates_order_through_public_api() {
    let app = common::test_app();
    /* ... */
}
```

### `ch28-b006` — 28.4 Tests que devuelven `Result`

Source: `28.Testing-doctests-y-property-testing.md:74` · mode: `run`

```rust
#[test]
fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original = Record::sample();
    let decoded = decode(&encode(&original)?)?;
    assert_eq!(decoded, original);
    Ok(())
}
```

### `ch28-b007` — 28.4 Tests que devuelven `Result`

Source: `28.Testing-doctests-y-property-testing.md:86` · mode: `run`

```rust
#[test]
fn rejects_missing_at() {
    let error = Email::parse("sin-arroba").unwrap_err();
    assert_eq!(error, EmailError::MissingAt);
}
```

### `ch28-b008` — 28.4 Tests que devuelven `Result`

Source: `28.Testing-doctests-y-property-testing.md:96` · mode: `run`

```rust
#[test]
fn io_failure_is_preserved_as_source() {
    let error = load_config("/ruta/inexistente").unwrap_err();
    assert!(matches!(error, LoadConfigError::Io(_)));
}
```

### `ch28-b009` — 28.5 Doctests

Source: `28.Testing-doctests-y-property-testing.md:110` · mode: `run`

```rust
/// Suma dos cantidades comprobando overflow.
///
/// ```
/// # use catalog::checked_total;
/// assert_eq!(checked_total([2, 3]), Some(5));
/// ```
pub fn checked_total(values: impl IntoIterator<Item = u64>) -> Option<u64> {
    values.into_iter().try_fold(0, u64::checked_add)
}
```

### `ch28-b010` — 28.6 Fakes antes que mocks universales

Source: `28.Testing-doctests-y-property-testing.md:130` · mode: `contextual`

```rust,ignore
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum RepoError {
    Unavailable,
}

trait UserRepository {
    fn add(&mut self, user: User) -> Result<(), RepoError>;
    fn find(&self, id: u64) -> Option<&User>;
}

#[derive(Default)]
struct InMemoryUsers {
    users: HashMap<u64, User>,
}

impl UserRepository for InMemoryUsers {
    fn add(&mut self, user: User) -> Result<(), RepoError> {
        self.users.insert(user.id, user);
        Ok(())
    }

    fn find(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }
}
```

### `ch28-b011` — 28.6 Fakes antes que mocks universales

Source: `28.Testing-doctests-y-property-testing.md:160` · mode: `run`

```rust
#[test]
fn registration_stores_the_user() {
    let mut repo = InMemoryUsers::default();
    register(&mut repo, User { id: 7, name: String::from("Ada") }).unwrap();
    assert_eq!(repo.find(7).map(|u| u.name.as_str()), Some("Ada"));
}
```

### `ch28-b012` — 28.6 Fakes antes que mocks universales

Source: `28.Testing-doctests-y-property-testing.md:171` · mode: `contextual`

```rust,ignore
struct UnavailableUsers;

impl UserRepository for UnavailableUsers {
    fn add(&mut self, _user: User) -> Result<(), RepoError> {
        Err(RepoError::Unavailable)
    }

    fn find(&self, _id: u64) -> Option<&User> {
        None
    }
}

#[test]
fn registration_reports_backend_failure() {
    let mut repo = UnavailableUsers;
    let error = register(&mut repo, User { id: 7, name: String::from("Ada") }).unwrap_err();
    assert!(matches!(error, RegisterError::Repository(RepoError::Unavailable)));
}
```

### `ch28-b013` — 28.7 Determinismo

Source: `28.Testing-doctests-y-property-testing.md:198` · mode: `run`

```rust
trait Clock {
    fn now(&self) -> u64; // epoch millis, o un tipo Timestamp propio
}

struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}
```

### `ch28-b014` — 28.8 Property testing

Source: `28.Testing-doctests-y-property-testing.md:220` · mode: `run`

```rust
use proptest::prelude::*;

proptest::proptest! {
    #[test]
    fn reversing_twice_restores_input(values in proptest::collection::vec(any::<i32>(), 0..100)) {
        let reversed_twice: Vec<_> = values.iter().rev().rev().copied().collect();
        prop_assert_eq!(reversed_twice, values);
    }
}
```

### `ch28-b015` — 28.8 Property testing

Source: `28.Testing-doctests-y-property-testing.md:234` · mode: `run`

```rust
use proptest::prelude::*;

proptest::proptest! {
    #[test]
    fn parse_never_panics(input in any::<String>()) {
        let _ = CountryCode::parse(&input);
    }
}
```

### `ch28-b016` — 28.9 La línea de comandos

Source: `28.Testing-doctests-y-property-testing.md:253` · mode: `reference`

```text
cargo test                      # todos los targets de test
cargo test parse                # filtra por nombre parcial
cargo test -- --nocapture       # muestra stdout/stderr de tests que pasan
cargo test -- --test-threads=1  # serializa (diagnóstico de interferencias)
cargo test --doc                # solo doctests
cargo test -p application       # un package del workspace
```

## 29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs

### `ch29-b001` — 29.1 Documentación como contrato

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:9` · mode: `run`

```rust
//! Tipos para construir pedidos válidos.

/// Identificador opaco de pedido.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(u64);
```

### `ch29-b002` — 29.2 Estructura de un item

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:32` · mode: `contextual`

```rust,ignore
/// Reserva stock para todas las líneas del pedido.
///
/// La reserva es atómica: o se reservan todas las líneas o ninguna.
///
/// # Examples
///
/// ```
/// # use catalog::{Inventory, Order};
/// let mut inventory = Inventory::with_stock("SKU-1", 10);
/// let order = Order::single("SKU-1", 3);
/// inventory.reserve(&order)?;
/// assert_eq!(inventory.available("SKU-1"), 7);
/// # Ok::<(), catalog::ReserveError>(())
/// ```
///
/// # Errors
///
/// Devuelve [`ReserveError::Insufficient`] si alguna línea supera el stock
/// disponible. En ese caso ninguna línea queda reservada.
pub fn reserve(&mut self, order: &Order) -> Result<(), ReserveError> {
    /* ... */
}
```

### `ch29-b003` — 29.3 Enlaces intra-doc

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:61` · mode: `illustrative`

```rust,ignore
/// Construye un [`Order`] asociado a [`CustomerId`].
///
/// Véase también [`Order::cancel`].
```

### `ch29-b004` — 29.4 Doctests que enseñan

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:73` · mode: `reference`

```text
cargo doc --no-deps --open
cargo test --doc
```

### `ch29-b005` — 29.5 `#[must_use]`

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:82` · mode: `run`

```rust
#[must_use = "el plan no se ejecuta hasta llamar a run"]
pub struct Plan { /* ... */ }
```

### `ch29-b006` — 29.5 `#[must_use]`

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:89` · mode: `contextual`

```rust,ignore
impl ClientBuilder {
    #[must_use]
    pub fn with_retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }
}
```

### `ch29-b007` — 29.6 Evolución de enums y structs

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:105` · mode: `run`

```rust
#[non_exhaustive]
pub enum ProtocolError {
    Timeout,
    Rejected,
}
```

### `ch29-b008` — 29.6 Evolución de enums y structs

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:115` · mode: `contextual`

```rust,ignore
match error {
    ProtocolError::Timeout => retry(),
    ProtocolError::Rejected => abort(),
    _ => report_unknown(&error), // exigido por non_exhaustive
}
```

### `ch29-b009` — 29.7 Clippy

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:131` · mode: `reference`

```text
cargo clippy --all-targets --all-features -- -D warnings
```

### `ch29-b010` — 29.7 Clippy

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:144` · mode: `parse`

```toml
[lints.rust]
missing_docs = "warn"

[lints.clippy]
unwrap_used = "warn"
dbg_macro = "warn"
```

### `ch29-b011` — 29.7 Clippy

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:157` · mode: `run`

```rust
#[allow(clippy::too_many_arguments, reason = "la firma refleja el protocolo externo")]
fn from_wire(/* ... */) { }
```

### `ch29-b012` — 29.8 rustfmt

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:166` · mode: `reference`

```text
cargo fmt --all -- --check
```

### `ch29-b013` — 29.9 Naming idiomático

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:183` · mode: `contextual`

```rust,ignore
impl Snapshot {
    pub fn as_bytes(&self) -> &[u8] { /* vista, O(1), sin asignar */ }
    pub fn to_vec(&self) -> Vec<u8> { /* copia: asigna y duplica */ }
    pub fn into_bytes(self) -> Vec<u8> { /* consume: entrega el buffer sin copiar */ }
    pub fn is_empty(&self) -> bool { /* predicado barato */ }
    pub fn try_merge(&self, other: &Snapshot) -> Result<Snapshot, MergeError> { /* fallible */ }
}
```

### `ch29-b014` — 29.12 Deprecación

Source: `29.Rustdoc-Clippy-rustfmt-y-diseño-de-APIs.md:229` · mode: `run`

```rust
#[deprecated(since = "1.4.0", note = "usa `Order::try_new`")]
pub fn new_unchecked(/* ... */) { }
```

## 30.Threads-channels-y-estado-compartido

### `ch30-b001` — 30.2 Crear un thread

Source: `30.Threads-channels-y-estado-compartido.md:13` · mode: `run`

```rust
use std::thread;

let handle = thread::spawn(|| {
    (1..=100).sum::<u64>()
});

let total = handle.join().expect("el worker hizo panic");
assert_eq!(total, 5050);
```

### `ch30-b002` — 30.3 Capturas con `move`

Source: `30.Threads-channels-y-estado-compartido.md:36` · mode: `compile_fail`

```rust,compile_fail
use std::thread;

let names = vec![String::from("Ada"), String::from("Grace")];

let handle = thread::spawn(|| names.len());
// error[E0373]: closure may outlive the current function,
// but it borrows `names`, which is owned by the current function
```

### `ch30-b003` — 30.3 Capturas con `move`

Source: `30.Threads-channels-y-estado-compartido.md:48` · mode: `contextual`

```rust,ignore
let names = vec![String::from("Ada"), String::from("Grace")];

let handle = thread::spawn(move || names.len());
assert_eq!(handle.join().unwrap(), 2);
```

### `ch30-b004` — 30.4 Threads con scope

Source: `30.Threads-channels-y-estado-compartido.md:61` · mode: `contextual`

```rust,ignore
let values = [10, 20, 30, 40];

thread::scope(|scope| {
    let left = scope.spawn(|| values[..2].iter().sum::<i32>());
    let right = scope.spawn(|| values[2..].iter().sum::<i32>());

    assert_eq!(left.join().unwrap() + right.join().unwrap(), 100);
});
```

### `ch30-b005` — 30.5 Message passing

Source: `30.Threads-channels-y-estado-compartido.md:80` · mode: `contextual`

```rust,ignore
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
let worker = thread::spawn(move || {
    tx.send(String::from("terminado")).unwrap();
});

let message = rx.recv().unwrap();
worker.join().unwrap();
assert_eq!(message, "terminado");
```

### `ch30-b006` — 30.6 Varios productores y cierre

Source: `30.Threads-channels-y-estado-compartido.md:99` · mode: `contextual`

```rust,ignore
let tx2 = tx.clone();
```

### `ch30-b007` — 30.6 Varios productores y cierre

Source: `30.Threads-channels-y-estado-compartido.md:109` · mode: `contextual`

```rust,ignore
for command in rx {
    apply(command)?;
}
```

### `ch30-b008` — 30.7 Channels acotados

Source: `30.Threads-channels-y-estado-compartido.md:119` · mode: `run`

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

let (tx, rx) = mpsc::sync_channel::<u32>(2);

let producer = thread::spawn(move || {
    for value in 0..5 {
        // Con 2 elementos en vuelo, este send BLOQUEA hasta que
        // el consumidor retire uno: el productor no puede adelantarse.
        tx.send(value).unwrap();
    }
});

for value in rx {
    thread::sleep(Duration::from_millis(10)); // simula consumidor lento
    println!("procesando {value}");
}

producer.join().unwrap();
```

### `ch30-b009` — 30.8 Estado compartido

Source: `30.Threads-channels-y-estado-compartido.md:152` · mode: `reference`

```text
estado compartido: varios acceden al mismo valor
message passing:  el valor o comando cambia de dueño
```

### `ch30-b010` — 30.9 Un worker que posee el estado

Source: `30.Threads-channels-y-estado-compartido.md:163` · mode: `run`

```rust
use std::sync::mpsc::{self, Sender};
use std::thread;

enum Command {
    Add(u64),
    Total(Sender<u64>), // el canal de respuesta viaja dentro del comando
}

fn spawn_counter() -> (Sender<Command>, thread::JoinHandle<u64>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut total = 0; // estado SIN locks: un solo dueño
        for command in rx {
            match command {
                Command::Add(value) => total += value,
                Command::Total(reply) => {
                    // Si el solicitante ya no espera, ignorar es la política:
                    let _ = reply.send(total);
                }
            }
        }
        total // el for terminó: no quedan senders vivos
    });

    (tx, handle)
}
```

### `ch30-b011` — 30.9 Un worker que posee el estado

Source: `30.Threads-channels-y-estado-compartido.md:193` · mode: `contextual`

```rust,ignore
let (commands, worker) = spawn_counter();

commands.send(Command::Add(3)).unwrap();
commands.send(Command::Add(4)).unwrap();

let (reply_tx, reply_rx) = mpsc::channel();
commands.send(Command::Total(reply_tx)).unwrap();
assert_eq!(reply_rx.recv().unwrap(), 7);

drop(commands);                        // shutdown: cae el último sender
assert_eq!(worker.join().unwrap(), 7); // join recoge el estado final
```

### `ch30-b012` — 30.10 Diseñar un worker

Source: `30.Threads-channels-y-estado-compartido.md:232` · mode: `contextual`

```rust,ignore
enum Command {
    Store(Record),
    Flush,
    Shutdown,
}
```

### `ch30-b013` — 30.11 Granularidad

Source: `30.Threads-channels-y-estado-compartido.md:253` · mode: `run`

```rust
let workers = std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(4);
```

## 31.Send-y-Sync

### `ch31-b001` — 31.2 Auto traits

Source: `31.Send-y-Sync.md:19` · mode: `run`

```rust
struct Report {
    title: String,
    values: Vec<u64>,
}

fn assert_send_sync<T: Send + Sync>() {}

assert_send_sync::<Report>();
```

### `ch31-b002` — 31.2 Auto traits

Source: `31.Send-y-Sync.md:34` · mode: `compile_fail`

```rust,compile_fail
use std::rc::Rc;

struct CachedReport {
    title: String,
    render_cache: Rc<String>, // este campo elimina Send y Sync
}

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<CachedReport>();
    // error[E0277]: `Rc<String>` cannot be sent between threads safely
    // note: required because it appears within the type `CachedReport`
}
```

### `ch31-b003` — 31.4 `Arc<T>` no basta siempre

Source: `31.Send-y-Sync.md:61` · mode: `compile_fail`

```rust,compile_fail
use std::{cell::RefCell, sync::Arc};

let state = Arc::new(RefCell::new(0));
std::thread::spawn(move || *state.borrow_mut() += 1);
```

### `ch31-b004` — 31.4 `Arc<T>` no basta siempre

Source: `31.Send-y-Sync.md:70` · mode: `run`

```rust
use std::sync::{Arc, Mutex};

let state = Arc::new(Mutex::new(0));
let worker_state = Arc::clone(&state);

std::thread::spawn(move || *worker_state.lock().unwrap() += 1)
    .join()
    .unwrap();

assert_eq!(*state.lock().unwrap(), 1);
```

### `ch31-b005` — 31.5 Closures y futures

Source: `31.Send-y-Sync.md:100` · mode: `run`

```rust
fn require_send<T: Send>(_: T) {}

let text = String::from("owned");
let future = async move { text.len() };
require_send(future);
```

### `ch31-b006` — 31.5 Closures y futures

Source: `31.Send-y-Sync.md:110` · mode: `compile_fail`

```rust,compile_fail
use std::rc::Rc;

fn require_send<T: Send>(_: T) {}

let shared = Rc::new(5);
let future = async move {
    let value = *shared; // `shared` sigue capturado por el future
    pause().await;
    value
};
require_send(future);
// error: future cannot be sent between threads safely
```

### `ch31-b007` — 31.5 Closures y futures

Source: `31.Send-y-Sync.md:127` · mode: `contextual`

```rust,ignore
let shared = Rc::new(5);
let value = *shared; // se extrae aquí: el future captura un i32

let future = async move {
    pause().await;
    value
};
require_send(future);
```

### `ch31-b008` — 31.6 `'static` es otra dimensión

Source: `31.Send-y-Sync.md:146` · mode: `reference`

```text
thread::spawn  exige F: Send + 'static   (el thread puede sobrevivir al caller)
scope.spawn    exige F: Send + 'scope    (el join está garantizado antes del fin)
```

### `ch31-b009` — 31.7 Bounds en APIs

Source: `31.Send-y-Sync.md:157` · mode: `run`

```rust
fn run_in_worker<F, T>(job: F) -> std::thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    std::thread::spawn(job)
}
```

### `ch31-b010` — 31.8 Implementación manual es unsafe

Source: `31.Send-y-Sync.md:171` · mode: `contextual`

```rust,ignore
unsafe impl Send for MyHandle {}
unsafe impl Sync for MyHandle {}
```

### `ch31-b011` — 31.8 Implementación manual es unsafe

Source: `31.Send-y-Sync.md:178` · mode: `run`

```rust
struct DeviceBuffer {
    data: *mut u8, // los punteros crudos anulan la derivación automática
    len: usize,
}

impl DeviceBuffer {
    fn write(&mut self, offset: usize, byte: u8) {
        assert!(offset < self.len);
        // SAFETY: poseemos en exclusiva [data, data+len) y &mut self
        // garantiza acceso único durante la escritura.
        unsafe { self.data.add(offset).write(byte) }
    }
}

// SAFETY: DeviceBuffer posee en exclusiva su región de memoria; ningún
// alias externo sobrevive a la construcción, toda mutación exige &mut self
// y la liberación ocurre una sola vez en Drop. Moverlo a otro thread
// transfiere la autoridad completa, sin estado afín a un thread concreto.
unsafe impl Send for DeviceBuffer {}
```

### `ch31-b012` — 31.9 Cómo leer el diagnóstico

Source: `31.Send-y-Sync.md:210` · mode: `reference`

```text
error: future cannot be sent between threads safely
note: future is not `Send` as this value is used across an await
    |
    |     let guard = state.lock().unwrap();
    |         ----- has type `MutexGuard<'_, u64>` which is not `Send`
    |     pause().await;
    |     ^^^^^^^^^^^^^ await occurs here, with `guard` maybe used later
```

## 32.Atomics-y-orden-de-memoria

### `ch32-b001` — 32.1 Atomicidad no es una invariante completa

Source: `32.Atomics-y-orden-de-memoria.md:11` · mode: `run`

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..100_000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(COUNTER.load(Ordering::Relaxed), 400_000);
}
```

### `ch32-b002` — 32.1 Atomicidad no es una invariante completa

Source: `32.Atomics-y-orden-de-memoria.md:38` · mode: `contextual`

```rust,ignore
// Transición NO atómica aunque cada operación lo sea:
let current = counter.load(Ordering::Relaxed);
counter.store(current + 1, Ordering::Relaxed);
```

### `ch32-b003` — 32.1 Atomicidad no es una invariante completa

Source: `32.Atomics-y-orden-de-memoria.md:44` · mode: `reference`

```text
thread A: load  -> 41
thread B: load  -> 41
thread A: store 42
thread B: store 42     // un incremento se perdió
```

### `ch32-b004` — 32.2 Data race frente a race condition

Source: `32.Atomics-y-orden-de-memoria.md:61` · mode: `run`

```rust
use std::sync::atomic::{AtomicU64, Ordering};

fn withdraw_broken(balance: &AtomicU64, amount: u64) -> bool {
    if balance.load(Ordering::Acquire) >= amount {
        // Otro thread puede pasar esta misma comprobación aquí,
        // antes de que restemos.
        balance.fetch_sub(amount, Ordering::AcqRel);
        true
    } else {
        false
    }
}
```

### `ch32-b005` — 32.3 Operaciones read-modify-write

Source: `32.Atomics-y-orden-de-memoria.md:82` · mode: `contextual`

```rust,ignore
fn decrement_if_positive(value: &std::sync::atomic::AtomicUsize) -> bool {
    value
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_sub(1)
        })
        .is_ok()
}
```

### `ch32-b006` — 32.3 Operaciones read-modify-write

Source: `32.Atomics-y-orden-de-memoria.md:94` · mode: `contextual`

```rust,ignore
fn withdraw(balance: &AtomicU64, amount: u64) -> bool {
    balance
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_sub(amount)
        })
        .is_ok()
}
```

### `ch32-b007` — 32.4 `Relaxed`

Source: `32.Atomics-y-orden-de-memoria.md:114` · mode: `run`

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

let shutdown = Arc::new(AtomicBool::new(false));
let flag = Arc::clone(&shutdown);

let worker = thread::spawn(move || {
    let mut processed = 0_u64;
    while !flag.load(Ordering::Relaxed) {
        processed += 1; // una unidad de trabajo
    }
    processed
});

shutdown.store(true, Ordering::Relaxed);
let total = worker.join().unwrap();
println!("procesadas {total} unidades");
```

### `ch32-b008` — 32.5 `Release` y `Acquire`

Source: `32.Atomics-y-orden-de-memoria.md:145` · mode: `run`

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DATA: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);

// Productor
DATA.store(42, Ordering::Relaxed);
READY.store(true, Ordering::Release);
```

### `ch32-b009` — 32.5 `Release` y `Acquire`

Source: `32.Atomics-y-orden-de-memoria.md:158` · mode: `contextual`

```rust,ignore
// Consumidor
while !READY.load(Ordering::Acquire) {
    std::hint::spin_loop();
}
assert_eq!(DATA.load(Ordering::Relaxed), 42); // garantizado tras observar READY
```

### `ch32-b010` — 32.8 ABA, overflow y wraparound

Source: `32.Atomics-y-orden-de-memoria.md:204` · mode: `reference`

```text
thread A: lee top = nodo X, se prepara para CAS(X -> X.next)
thread B: pop X, pop Y, push X          (top vuelve a ser X...
                                         ...pero X.next ya no es Y)
thread A: CAS(top, X -> X.next)          éxito aparente;
                                         top apunta ahora a un nodo liberado
```

## 33.Modelo-mental-async

### `ch33-b001` — 33.1 Async trata espera, no velocidad automática

Source: `33.Modelo-mental-async.md:11` · mode: `contextual`

```rust,ignore
async fn load_profile(id: UserId) -> Result<Profile, LoadError> {
    let user = load_user(id).await?;
    let permissions = load_permissions(id).await?;
    Ok(Profile { user, permissions })
}
```

### `ch33-b002` — 33.1 Async trata espera, no velocidad automática

Source: `33.Modelo-mental-async.md:21` · mode: `illustrative`

```rust,ignore
// Capítulo 38: ambos futures avanzan de forma alternada en la misma task.
let (user, permissions) = join!(load_user(id), load_permissions(id));
```

### `ch33-b003` — 33.3 `async fn` es lazy; una task lanzada ya tiene vida propia

Source: `33.Modelo-mental-async.md:52` · mode: `contextual`

```rust,ignore
let request = load_user(id); // todavía no completa el I/O
let user = request.await?;
```

### `ch33-b004` — 33.3 `async fn` es lazy; una task lanzada ya tiene vida propia

Source: `33.Modelo-mental-async.md:59` · mode: `run`

```rust
fn main() {
    let future = async {
        println!("cuerpo ejecutado");
        42
    };

    // Sin executor que lo sondee, el cuerpo nunca corre:
    drop(future);
    println!("fin del programa");
}
```

### `ch33-b005` — 33.4 Executor

Source: `33.Modelo-mental-async.md:82` · mode: `reference`

```text
ready queue -> poll task -> Ready(result)
                         -> Pending + registrar Waker
evento -> wake -> ready queue
```

### `ch33-b006` — 33.6 Cooperación

Source: `33.Modelo-mental-async.md:108` · mode: `contextual`

```rust,ignore
async fn bad() {
    loop {
        perform_cpu_step(); // nunca cede
    }
}
```

### `ch33-b007` — 33.6 Cooperación

Source: `33.Modelo-mental-async.md:118` · mode: `illustrative`

```rust,ignore
// El worker async queda libre mientras el pool bloqueante calcula:
let digest = tokio::task::spawn_blocking(move || hash_large_file(path)).await?;
```

### `ch33-b008` — 33.7 Blocking dentro de async

Source: `33.Modelo-mental-async.md:129` · mode: `compile_only`

```rust,no_run
async fn bad_read() -> std::io::Result<String> {
    std::fs::read_to_string("large.txt")
}
```

### `ch33-b009` — 33.8 Concurrencia no es spawning

Source: `33.Modelo-mental-async.md:143` · mode: `illustrative`

```rust,ignore
// Misma task: concurrencia sin nueva unidad de fallo ni bounds extra.
let (a, b) = join!(load_user(id), load_permissions(id));

// Task nueva: unidad independiente, con handle, errores propios,
// requisitos Send + 'static y responsabilidad de supervisión.
let handle = spawn(refresh_cache(id));
```

### `ch33-b010` — 33.10 Structured concurrency

Source: `33.Modelo-mental-async.md:176` · mode: `reference`

```text
request scope
├── load user
├── load permissions
└── timeout/cancel -> ambos trabajos resueltos
```

### `ch33-b011` — 33.11 Cuándo elegir async

Source: `33.Modelo-mental-async.md:189` · mode: `reference`

```text
¿muchas esperas concurrentes (red, timers)?      -> async
¿pocos flujos, llamadas bloqueantes, simplicidad? -> threads
¿cálculo divisible que satura cores?              -> pool paralelo (p. ej. rayon)
¿mezcla?                                          -> fronteras: channels entre modelos
```

## 34.Future-Poll-Context-y-Waker

### `ch34-b001` — 34.1 El contrato mínimo

Source: `34.Future-Poll-Context-y-Waker.md:9` · mode: `run`

```rust
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

trait FutureShape {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### `ch34-b002` — 34.5 Un future inmediato y un executor de juguete

Source: `34.Future-Poll-Context-y-Waker.md:69` · mode: `run`

```rust
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Immediate<T>(Option<T>);

impl<T: Unpin> Future for Immediate<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("future sondeado tras completar"))
    }
}
```

### `ch34-b003` — 34.5 Un future inmediato y un executor de juguete

Source: `34.Future-Poll-Context-y-Waker.md:91` · mode: `compile_only`

```rust,no_run
use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn main() {
    let value = block_on(async { 40 + 2 });
    assert_eq!(value, 42);
}
```

### `ch34-b004` — 34.6 Futures compuestos

Source: `34.Future-Poll-Context-y-Waker.md:122` · mode: `reference`

```text
Map<F, Closure>
├── future F pinneado
├── closure todavía disponible
└── estado: esperando | completado
```

### `ch34-b005` — 34.8 Despertares perdidos

Source: `34.Future-Poll-Context-y-Waker.md:149` · mode: `reference`

```text
consumidor: comprueba la cola      -> vacía
productor:  encola el dato
productor:  mira si hay waker      -> aún no hay -> no despierta a nadie
consumidor: registra su waker
consumidor: devuelve Pending       -> el evento ya pasó y nadie lo despertará
```

## 35.Async-fn-bloques-y-máquinas-de-estado

### `ch35-b001` — 35.1 Una función que construye un future

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:7` · mode: `contextual`

```rust,ignore
async fn fetch_name(id: UserId) -> Result<String, FetchError> {
    let response = request(id).await?;
    response
        .name()
        .map(str::to_owned)
        .ok_or(FetchError::MissingName)
}
```

### `ch35-b002` — 35.2 Bloques async

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:32` · mode: `contextual`

```rust,ignore
let future = async {
    let left = load_left().await?;
    let right = load_right().await?;
    Ok::<_, LoadError>((left, right))
};
```

### `ch35-b003` — 35.3 Captura y `async move`

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:48` · mode: `contextual`

```rust,ignore
let request_id = String::from("req-42");
let future = async move {
    log_request(&request_id).await;
};
```

### `ch35-b004` — 35.3 Captura y `async move`

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:59` · mode: `compile_fail`

```rust,compile_fail
fn make_logger(prefix: String) -> impl std::future::Future<Output = ()> {
    async {
        println!("{prefix}: listo");
    }
    // error[E0373]: async block may outlive the current function,
    // but it borrows `prefix`, which is owned by the current function
}
```

### `ch35-b005` — 35.3 Captura y `async move`

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:71` · mode: `run`

```rust
fn make_logger(prefix: String) -> impl std::future::Future<Output = ()> {
    async move {
        println!("{prefix}: listo");
    }
}
```

### `ch35-b006` — 35.4 La máquina de estado conceptual

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:85` · mode: `reference`

```text
LoadAccess
├── Created { id }
├── WaitingUser { user_future }
├── WaitingPermissions { user, permissions_future }
└── Done
```

### `ch35-b007` — 35.5 Medir tamaño sin inventar garantías

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:103` · mode: `run`

```rust
use std::mem::size_of_val;

fn main() {
    let tiny = async {};

    let with_buffer = async {
        let buffer = [7_u8; 1024];
        std::future::ready(()).await;
        std::hint::black_box(buffer)[0]
    };

    println!("async vacío: {} bytes", size_of_val(&tiny));
    println!("buffer vivo tras await: {} bytes", size_of_val(&with_buffer));
}
```

### `ch35-b008` — 35.6 Locales que cruzan `.await`

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:128` · mode: `contextual`

```rust,ignore
async fn process() {
    let large = vec![0_u8; 1_000_000];
    wait_for_signal().await;
    consume(large);
}
```

### `ch35-b009` — 35.6 Locales que cruzan `.await`

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:138` · mode: `contextual`

```rust,ignore
async fn smaller() {
    let checksum = {
        let large = build_buffer();
        let checksum = calculate_checksum(&large);
        consume(large);
        checksum
    };

    wait_for_signal().await;
    publish(checksum);
}
```

### `ch35-b010` — 35.7 `?`, `return` y la frontera de control

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:166` · mode: `contextual`

```rust,ignore
fn make_job() -> impl std::future::Future<Output = Result<u64, Error>> {
    async {
        let value = load().await?;
        Ok(value + 1)
    }
}
```

### `ch35-b011` — 35.9 Orden y concurrencia

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:192` · mode: `contextual`

```rust,ignore
let user_future = load_user(id);
let permissions_future = load_permissions(id);
let (user, permissions) = join(user_future, permissions_future).await;
```

### `ch35-b012` — 35.10 Recursión async e indirección

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:206` · mode: `compile_fail`

```rust,compile_fail
async fn countdown(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        countdown(n - 1).await + 1
        // error[E0733]: recursion in an async fn requires boxing
    }
}
```

### `ch35-b013` — 35.10 Recursión async e indirección

Source: `35.Async-fn-bloques-y-máquinas-de-estado.md:219` · mode: `run`

```rust
async fn countdown(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        Box::pin(countdown(n - 1)).await + 1
    }
}
```

## 36.Ownership-y-lifetimes-en-futures

### `ch36-b001` — 36.2 Un future puede prestar

Source: `36.Ownership-y-lifetimes-en-futures.md:19` · mode: `contextual`

```rust,ignore
async fn length(text: &str) -> usize {
    cooperative_pause().await;
    text.len()
}
```

### `ch36-b002` — 36.2 Un future puede prestar

Source: `36.Ownership-y-lifetimes-en-futures.md:34` · mode: `contextual`

```rust,ignore
let text = String::from("rust");
let result = length(&text).await;
drop(text);
assert_eq!(result, 4);
```

### `ch36-b003` — 36.3 Hacer owned el mínimo necesario

Source: `36.Ownership-y-lifetimes-en-futures.md:49` · mode: `contextual`

```rust,ignore
fn owned_length(text: &str) -> impl std::future::Future<Output = usize> + Send + 'static {
    let owned = text.to_owned();
    async move {
        cooperative_pause().await;
        owned.len()
    }
}
```

### `ch36-b004` — 36.4 El contrato real de `spawn`

Source: `36.Ownership-y-lifetimes-en-futures.md:69` · mode: `reference`

```text
Future + Send + 'static
Future::Output + Send + 'static
```

### `ch36-b005` — 36.6 Préstamos que cruzan `.await`

Source: `36.Ownership-y-lifetimes-en-futures.md:92` · mode: `contextual`

```rust,ignore
async fn append_after_wait(buffer: &mut String) {
    let view = buffer.as_str();
    log(view).await;
    buffer.push('!');
}
```

### `ch36-b006` — 36.8 Guards: tipo de mutex y protocolo

Source: `36.Ownership-y-lifetimes-en-futures.md:126` · mode: `contextual`

```rust,ignore
async fn update(shared: &std::sync::Mutex<State>) {
    let mut guard = shared.lock().unwrap();
    guard.prepare();
    send_notification().await;
    guard.finish();
}
```

### `ch36-b007` — 36.8 Guards: tipo de mutex y protocolo

Source: `36.Ownership-y-lifetimes-en-futures.md:139` · mode: `contextual`

```rust,ignore
let ticket = {
    let mut state = shared.lock().unwrap();
    state.prepare_notification()
};

send_notification(&ticket.payload).await;

let mut state = shared.lock().unwrap();
state.finish_if_revision_matches(ticket.revision);
```

### `ch36-b008` — 36.10 Salidas prestadas

Source: `36.Ownership-y-lifetimes-en-futures.md:169` · mode: `contextual`

```rust,ignore
async fn first_line(input: &str) -> Option<&str> {
    cooperative_pause().await;
    input.lines().next()
}
```

## 37.Pinning-de-futures

### `ch37-b001` — 37.4 `.await` se ocupa normalmente

Source: `37.Pinning-de-futures.md:49` · mode: `contextual`

```rust,ignore
let value = operation().await?;
```

### `ch37-b002` — 37.6 Pinning local con `pin!`

Source: `37.Pinning-de-futures.md:69` · mode: `contextual`

```rust,ignore
use std::pin::pin;

let future = operation();
let mut future = pin!(future);

// Una API de bajo nivel puede recibir future.as_mut().
```

### `ch37-b003` — 37.7 Pinning owned con `Box::pin`

Source: `37.Pinning-de-futures.md:86` · mode: `contextual`

```rust,ignore
let future = Box::pin(operation());
```

### `ch37-b004` — 37.8 Futures heterogéneos y lifetimes honestos

Source: `37.Pinning-de-futures.md:100` · mode: `run`

```rust
use std::{future::Future, pin::Pin};

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

fn job(input: &str) -> BoxFuture<'_, usize> {
    Box::pin(async move { input.len() })
}
```

### `ch37-b005` — 37.10 Proyección y pinning estructural

Source: `37.Pinning-de-futures.md:127` · mode: `reference`

```text
Parent
├── child: F       [estructuralmente pinneado]
├── output: Option [movible si la invariante lo permite]
└── state          [movible si la invariante lo permite]
```

## 38.Join-select-timeouts-y-cancelación

### `ch38-b001` — 38.2 Secuencial frente a concurrente

Source: `38.Join-select-timeouts-y-cancelación.md:21` · mode: `contextual`

```rust,ignore
let user = load_user(id).await?;
let orders = load_orders(id).await?;
```

### `ch38-b002` — 38.2 Secuencial frente a concurrente

Source: `38.Join-select-timeouts-y-cancelación.md:30` · mode: `contextual`

```rust,ignore
let (user, orders) = tokio::join!(load_user(id), load_orders(id));
let user = user?;
let orders = orders?;
```

### `ch38-b003` — 38.3 `try_join`: fallo temprano con cancelación

Source: `38.Join-select-timeouts-y-cancelación.md:44` · mode: `contextual`

```rust,ignore
let (user, orders) = tokio::try_join!(
    load_user(id),
    load_orders(id),
)?;
```

### `ch38-b004` — 38.4 Lifecycle preciso de `select!`

Source: `38.Join-select-timeouts-y-cancelación.md:59` · mode: `contextual`

```rust,ignore
tokio::select! {
    result = receive_command() => handle(result)?,
    _ = shutdown_signal() => begin_shutdown(),
}
```

### `ch38-b005` — 38.5 `select!` dentro de un loop

Source: `38.Join-select-timeouts-y-cancelación.md:83` · mode: `contextual`

```rust,ignore
loop {
    tokio::select! {
        maybe = rx.recv() => match maybe {
            Some(command) => process(command).await?,
            None => break,
        },
        _ = shutdown.changed() => break,
    }
}
```

### `ch38-b006` — 38.8 Timeout y deadline

Source: `38.Join-select-timeouts-y-cancelación.md:138` · mode: `contextual`

```rust,ignore
let result = tokio::time::timeout(
    std::time::Duration::from_secs(2),
    load_user(id),
)
.await;
```

### `ch38-b007` — 38.9 Cancelación cooperativa y punto de compromiso

Source: `38.Join-select-timeouts-y-cancelación.md:158` · mode: `reference`

```text
Running -> ShutdownRequested -> Draining -> Stopped
```

### `ch38-b008` — 38.10 Tasks, handles y abort

Source: `38.Join-select-timeouts-y-cancelación.md:174` · mode: `contextual`

```rust,ignore
let handle = tokio::spawn(async move { run_worker().await });
let output = handle.await?;
```

## 39.Streams-channels-y-backpressure

### `ch39-b001` — 39.1 Una secuencia asíncrona

Source: `39.Streams-channels-y-backpressure.md:9` · mode: `run`

```rust
trait Stream {
    type Item;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>>;
}
```

### `ch39-b002` — 39.2 Iterador, stream y fuente externa

Source: `39.Streams-channels-y-backpressure.md:34` · mode: `contextual`

```rust,ignore
while let Some(event) = stream.next().await {
    handle(event).await?;
}
```

### `ch39-b003` — 39.5 `mpsc` acotado: qué garantiza

Source: `39.Streams-channels-y-backpressure.md:76` · mode: `contextual`

```rust,ignore
let (tx, mut rx) = tokio::sync::mpsc::channel::<Job>(64);

tx.send(job).await?;
while let Some(job) = rx.recv().await {
    process(job).await;
}
```

### `ch39-b004` — 39.5 `mpsc` acotado: qué garantiza

Source: `39.Streams-channels-y-backpressure.md:89` · mode: `contextual`

```rust,ignore
let permit = tx.reserve().await?;
let job = build_expensive_job();
permit.send(job);
```

### `ch39-b005` — 39.6 Concurrencia limitada sobre streams

Source: `39.Streams-channels-y-backpressure.md:101` · mode: `reference`

```text
stream<Item>
  -> map(Item -> Future<Output>)
  -> buffer_unordered(N)
  -> stream<Output en orden de finalización>
```

### `ch39-b006` — 39.8 Request-response sin borrar errores

Source: `39.Streams-channels-y-backpressure.md:130` · mode: `contextual`

```rust,ignore
struct Lookup {
    id: UserId,
    reply: tokio::sync::oneshot::Sender<Result<User, LookupError>>,
}
```

### `ch39-b007` — 39.9 Broadcast, lag y replay

Source: `39.Streams-channels-y-backpressure.md:149` · mode: `contextual`

```rust,ignore
match receiver.recv().await {
    Ok(event) => apply(event),
    Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
        request_snapshot(missed).await?;
    }
    Err(tokio::sync::broadcast::error::RecvError::Closed) => return Ok(()),
}
```

### `ch39-b008` — 39.10 Cierre y permisos pendientes

Source: `39.Streams-channels-y-backpressure.md:165` · mode: `reference`

```text
Running -> AdmissionClosed -> Draining(buffer + permits) -> WorkersJoined -> Stopped
```

## 40.Async-closures-y-AsyncFn

### `ch40-b001` — 40.1 Superficie estable desde Rust 1.85

Source: `40.Async-closures-y-AsyncFn.md:9` · mode: `contextual`

```rust,ignore
let fetch = async |id: UserId| -> Result<User, LoadError> {
    load_user(id).await
};

let user = fetch(id).await?;
```

### `ch40-b002` — 40.3 `async ||` frente a `|| async {}`

Source: `40.Async-closures-y-AsyncFn.md:36` · mode: `contextual`

```rust,ignore
let old_style = || async {
    do_work().await
};

let first_class = async || {
    do_work().await
};
```

### `ch40-b003` — 40.3 `async ||` frente a `|| async {}`

Source: `40.Async-closures-y-AsyncFn.md:48` · mode: `contextual`

```rust,ignore
let mut values = Vec::new();
let mut push_later = async || {
    let value = load_value().await;
    values.push(value);
};

push_later().await;
```

### `ch40-b004` — 40.4 La familia de traits y sus receptores

Source: `40.Async-closures-y-AsyncFn.md:64` · mode: `run`

```rust
async fn shared<F>(callback: &F) -> usize
where
    F: AsyncFn() -> usize,
{
    callback().await
}

async fn mutable<F>(callback: &mut F) -> usize
where
    F: AsyncFnMut() -> usize,
{
    callback().await
}

async fn once<F>(callback: F) -> usize
where
    F: AsyncFnOnce() -> usize,
{
    callback().await
}
```

### `ch40-b005` — 40.5 `move` decide entrada, no cardinalidad

Source: `40.Async-closures-y-AsyncFn.md:101` · mode: `run`

```rust
fn examples() {
    let client = String::from("client");
    let reusable = async move || client.len();
    // `client` pertenece a la closure, pero cada llamada solo lo presta.

    let token = String::from("single-use");
    let consume_once = async move || token;
    // El output mueve `token` fuera: solo `AsyncFnOnce`.

    let _ = (reusable, consume_once);
}
```

### `ch40-b006` — 40.6 El préstamo dura hasta `Ready` o drop

Source: `40.Async-closures-y-AsyncFn.md:125` · mode: `compile_fail`

```rust,compile_fail
async fn invalid_overlap<F>(mut callback: F)
where
    F: AsyncFnMut(),
{
    let first = callback();
    let second = callback(); // segundo préstamo mutable
    first.await;
    second.await;
}
```

### `ch40-b007` — 40.7 Callbacks higher-ranked y lending de argumentos

Source: `40.Async-closures-y-AsyncFn.md:145` · mode: `run`

```rust
async fn visit_all<F>(items: &[String], visitor: F) -> Vec<usize>
where
    F: for<'a> AsyncFn(&'a str) -> usize,
{
    let mut outputs = Vec::with_capacity(items.len());
    for item in items {
        outputs.push(visitor(item).await);
    }
    outputs
}
```

### `ch40-b008` — 40.10 Retry: cardinalidad y efectos

Source: `40.Async-closures-y-AsyncFn.md:192` · mode: `run`

```rust
async fn retry<F, T, E>(mut operation: F, max_attempts: usize) -> Result<T, E>
where
    F: AsyncFnMut() -> Result<T, E>,
{
    assert!(max_attempts > 0);
    let mut last_error = None;
    for _ in 0..max_attempts {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.expect("hubo al menos un intento"))
}
```

### `ch40-b009` — 40.11 Borrado de tipo y dispatch dinámico

Source: `40.Async-closures-y-AsyncFn.md:215` · mode: `run`

```rust
use std::{future::Future, pin::Pin};

trait DynCallback {
    fn call<'a>(
        &'a mut self,
        input: &'a str,
    ) -> Pin<Box<dyn Future<Output = usize> + 'a>>;
}
```

## 41.Async-fn-en-traits

### `ch41-b001` — 41.1 Sintaxis nativa y alcance

Source: `41.Async-fn-en-traits.md:9` · mode: `contextual`

```rust,ignore
trait UserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError>;
}

impl UserRepository for InMemoryUsers {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError> {
        Ok(self.users.get(&id).cloned())
    }
}
```

### `ch41-b002` — 41.2 Desugaring mediante RPITIT

Source: `41.Async-fn-en-traits.md:29` · mode: `contextual`

```rust,ignore
trait UserRepository {
    fn find(
        &self,
        id: UserId,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepoError>>;
}
```

### `ch41-b003` — 41.4 `Send` pertenece al future devuelto

Source: `41.Async-fn-en-traits.md:60` · mode: `contextual`

```rust,ignore
trait SendUserRepository: Send + Sync {
    fn find(
        &self,
        id: UserId,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepoError>> + Send;
}
```

### `ch41-b004` — 41.5 Semver y variantes local/Send

Source: `41.Async-fn-en-traits.md:81` · mode: `contextual`

```rust,ignore
#[trait_variant::make(UserRepository: Send)]
pub trait LocalUserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError>;
}
```

### `ch41-b005` — 41.6 Lo que el caller sabe —y lo que no

Source: `41.Async-fn-en-traits.md:98` · mode: `contextual`

```rust,ignore
trait GatUserRepository {
    type Find<'a>: std::future::Future<Output = Option<User>> + 'a
    where
        Self: 'a;

    fn find(&self, id: u64) -> Self::Find<'_>;
}
```

### `ch41-b006` — 41.7 Un spawn genérico necesita toda la cadena

Source: `41.Async-fn-en-traits.md:112` · mode: `contextual`

```rust,ignore
fn spawn_find<R>(
    repository: std::sync::Arc<R>,
    id: u64,
) -> tokio::task::JoinHandle<Option<User>>
where
    R: SendUserRepository + 'static,
{
    tokio::spawn(async move { repository.find(id).await })
}
```

### `ch41-b007` — 41.8 Por qué el trait nativo no admite `dyn`

Source: `41.Async-fn-en-traits.md:132` · mode: `compile_fail`

```rust,compile_fail
trait Repository {
    async fn find(&self, id: u64) -> Option<u64>;
}

fn choose_at_runtime(repository: &dyn Repository) {
    let _ = repository;
}
```

### `ch41-b008` — 41.9 Interfaz dyn-compatible boxed

Source: `41.Async-fn-en-traits.md:148` · mode: `contextual`

```rust,ignore
use std::{future::Future, pin::Pin};

trait DynUserRepository: Send + Sync {
    fn find<'a>(
        &'a self,
        id: UserId,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<User>, RepoError>>
                + Send
                + 'a,
        >,
    >;
}
```

### `ch41-b009` — 41.12 Dominio, errores y cancelación

Source: `41.Async-fn-en-traits.md:190` · mode: `reference`

```text
entidad / value object -> invariantes puras y síncronas
caso de uso            -> coordina puertos; puede ser async
adaptador              -> runtime, red, base de datos
```

## 42.Rust-2024-y-captura-de-lifetimes

### `ch42-b001` — 42.1 El tipo visible y el tipo oculto

Source: `42.Rust-2024-y-captura-de-lifetimes.md:9` · mode: `run`

```rust
fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace()
}

fn main() {
    let text = String::from("uno dos");
    assert_eq!(words(&text).collect::<Vec<_>>(), ["uno", "dos"]);
}
```

### `ch42-b002` — 42.3 Captura automática: la matriz completa

Source: `42.Rust-2024-y-captura-de-lifetimes.md:50` · mode: `run`

```rust
fn length(text: &str) -> impl Copy {
    text.len()
}
```

### `ch42-b003` — 42.4 Overcapture observable

Source: `42.Rust-2024-y-captura-de-lifetimes.md:64` · mode: `compile_fail`

```rust,compile_fail
fn length(text: &str) -> impl Copy + PartialEq<usize> {
    text.len()
}

fn main() {
    let text = String::from("rust");
    let result = length(&text);
    drop(text); // `result` todavía puede capturar el préstamo en Rust 2024
    assert!(result == 4);
}
```

### `ch42-b004` — 42.5 `use<...>` como contrato preciso

Source: `42.Rust-2024-y-captura-de-lifetimes.md:85` · mode: `run`

```rust
fn length(text: &str) -> impl Copy + PartialEq<usize> + use<> {
    text.len()
}

fn main() {
    let text = String::from("rust");
    let result = length(&text);
    drop(text);
    assert!(result == 4);
}
```

### `ch42-b005` — 42.5 `use<...>` como contrato preciso

Source: `42.Rust-2024-y-captura-de-lifetimes.md:100` · mode: `run`

```rust
fn attach<'a, T>(key: &'a str, value: T) -> impl Sized + use<'a, T> {
    (key, value)
}
```

### `ch42-b006` — 42.6 Restricciones que no deben adivinarse

Source: `42.Rust-2024-y-captura-de-lifetimes.md:123` · mode: `run`

```rust
use std::fmt::Debug;

fn transform<'a, T>(_: &'a str, value: T) -> impl Debug + use<T>
where
    T: Debug,
{
    value
}
```

### `ch42-b007` — 42.7 Frontera de versión: 1.82, 1.85 y 1.87

Source: `42.Rust-2024-y-captura-de-lifetimes.md:148` · mode: `illustrative`

```rust,ignore
trait CurrentApi {
    fn metadata<'a>(&'a self) -> impl Sized + use<Self>;
}
```

### `ch42-b008` — 42.8 Captura no es `outlives`

Source: `42.Rust-2024-y-captura-de-lifetimes.md:165` · mode: `run`

```rust
fn old_pair<'a, T: 'a>(anchor: &'a (), value: T) -> impl Sized + 'a {
    (anchor, value)
}
```

### `ch42-b009` — 42.8 Captura no es `outlives`

Source: `42.Rust-2024-y-captura-de-lifetimes.md:173` · mode: `run`

```rust
fn pair<'a, T>(anchor: &'a (), value: T) -> impl Sized + use<'a, T> {
    (anchor, value)
}
```

### `ch42-b010` — 42.9 Async: captura real y captura conservadora

Source: `42.Rust-2024-y-captura-de-lifetimes.md:185` · mode: `run`

```rust
async fn borrowed_length(text: &str) -> usize {
    tokio::task::yield_now().await;
    text.len()
}
```

### `ch42-b011` — 42.9 Async: captura real y captura conservadora

Source: `42.Rust-2024-y-captura-de-lifetimes.md:194` · mode: `run`

```rust
use std::future::Future;

fn independent_length(text: &str) -> impl Future<Output = usize> + use<> {
    let length = text.len();
    async move { length }
}
```

### `ch42-b012` — 42.11 Migración de edition con evidencia

Source: `42.Rust-2024-y-captura-de-lifetimes.md:223` · mode: `reference`

```console
cargo fix --edition --workspace --all-targets --all-features
# actualizar cada Cargo.toml a edition = "2024"
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## 43.Diseño-runtime-agnostic-y-Tokio

### `ch43-b001` — 43.4 Dominio síncrono, aplicación async

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:44` · mode: `run`

```rust
use std::future::Future;

#[derive(Clone)]
struct Account {
    balance: u64,
}

#[derive(Debug)]
enum AccountError {
    Overflow,
}

impl Account {
    fn deposit(&mut self, amount: u64) -> Result<u64, AccountError> {
        self.balance = self.balance.checked_add(amount).ok_or(AccountError::Overflow)?;
        Ok(self.balance)
    }
}

trait AccountRepository {
    type Error;

    fn load(&self) -> impl Future<Output = Result<Account, Self::Error>> + Send;
    fn save(&self, account: Account) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

async fn deposit<R>(repository: &R, amount: u64) -> Result<u64, &'static str>
where
    R: AccountRepository + Sync,
{
    let mut account = repository.load().await.map_err(|_| "load")?;
    let balance = account.deposit(amount).map_err(|_| "domain")?;
    repository.save(account).await.map_err(|_| "save")?;
    Ok(balance)
}
```

### `ch43-b002` — 43.5 Features mínimas y verificables

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:99` · mode: `parse`

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal", "sync", "time"] }

[dev-dependencies]
tokio = { version = "1", features = ["macros", "rt", "test-util", "time"] }
```

### `ch43-b003` — 43.5 Features mínimas y verificables

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:111` · mode: `run`

```rust
#[tokio::main]
async fn main() {
    tokio::task::yield_now().await;
}
```

### `ch43-b004` — 43.6 `Runtime`, `Handle` y contextos

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:124` · mode: `run`

```rust
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

fn spawn_cleanup(handle: &Handle) -> JoinHandle<()> {
    handle.spawn(async {
        tokio::task::yield_now().await;
    })
}
```

### `ch43-b005` — 43.8 Frontera bloqueante y CPU

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:161` · mode: `run`

```rust
async fn count_words(document: String) -> Result<usize, tokio::task::JoinError> {
    tokio::task::spawn_blocking(move || document.split_whitespace().count()).await
}
```

### `ch43-b006` — 43.9 Dos relojes y tests sin sleeps reales

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:183` · mode: `run`

```rust
use std::time::Duration;
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()
        .unwrap();

    runtime.block_on(async {
        let started = tokio::time::Instant::now();
        tokio::time::sleep(Duration::from_secs(60)).await;
        assert_eq!(started.elapsed(), Duration::from_secs(60));
    });
}
```

### `ch43-b007` — 43.11 Shutdown como protocolo de estados

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:220` · mode: `reference`

```text
RUNNING -> QUIESCING -> DRAINING -> FORCING -> STOPPED
             |             |            |
      cerrar admisión   deadline    abort + join
```

### `ch43-b008` — 43.13 Observabilidad que sigue a la task

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:258` · mode: `run`

```rust
use tracing::Instrument;

fn spawn_job(job_id: u64) -> tokio::task::JoinHandle<()> {
    let span = tracing::info_span!("job", job_id);
    tokio::spawn(
        async move {
            tokio::task::yield_now().await;
        }
        .instrument(span),
    )
}
```

### `ch43-b009` — 43.14 Axum como adaptador HTTP

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:278` · mode: `run`

```rust
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
struct AppState {
    users: Arc<BTreeMap<u64, String>>,
}

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>, StatusCode> {
    let name = state.users.get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(UserResponse { id, name }))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/users/{id}", get(get_user))
        .with_state(state)
}
```

### `ch43-b010` — 43.15 Tauri 2 como adaptador IPC

Source: `43.Diseño-runtime-agnostic-y-Tokio.md:320` · mode: `reference`

```text
webview -> comando IPC -> DTO de entrada -> caso de uso -> dominio/adaptadores
webview <- Result/DTO  <- traducción estable <-----------+
```

## 44.Safe-Rust-y-responsabilidad-unsafe

### `ch44-b001` — 44.4 `unsafe` no amplía el conjunto válido

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:52` · mode: `compile_only`

```rust,no_run
fn main() {
    let pointer: *const i32 = std::ptr::null();

    // Compila, pero ejecutarlo intentaría crear un acceso inválido: UB.
    let _value = unsafe { *pointer };
}
```

### `ch44-b002` — 44.5 Anatomía de un contrato `# Safety`

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:82` · mode: `run`

```rust
/// Reads one copyable value from a raw pointer.
///
/// # Safety
///
/// `pointer` must be non-null, properly aligned, initialized for `T` and
/// valid for a read during this call. No conflicting access may occur.
unsafe fn read_one<T: Copy>(pointer: *const T) -> T {
    // SAFETY: the caller provides exactly the preconditions required by `read`.
    unsafe { pointer.read() }
}

fn main() {
    let value = 42;
    // SAFETY: the pointer comes from this live, aligned, initialized `i32`.
    assert_eq!(unsafe { read_one(&raw const value) }, 42);
}
```

### `ch44-b003` — 44.6 Caso de estudio: `slice::from_raw_parts`

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:117` · mode: `compile_only`

```rust,no_run
/// Creates a shared slice tied to a lifetime chosen by the caller.
///
/// # Safety
///
/// `pointer` and `length` must satisfy every `slice::from_raw_parts`
/// precondition, and the memory must remain valid and immutable for `'a`.
unsafe fn view<'a, T>(pointer: *const T, length: usize) -> &'a [T] {
    // SAFETY: delegated verbatim to this function's caller contract.
    unsafe { std::slice::from_raw_parts(pointer, length) }
}

fn main() {}
```

### `ch44-b004` — 44.7 Comprobar null no basta para una API safe

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:145` · mode: `run`

```rust
unsafe fn raw_sum(pointer: *const i32, length: usize) -> i32 {
    // SAFETY: the caller supplies one readable, initialized region.
    let values = unsafe { std::slice::from_raw_parts(pointer, length) };
    values.iter().sum()
}

fn safe_sum(values: &[i32]) -> i32 {
    // SAFETY: pointer and length come from this same live slice.
    unsafe { raw_sum(values.as_ptr(), values.len()) }
}

fn main() {
    assert_eq!(safe_sum(&[10, 20, 12]), 42);
    assert_eq!(safe_sum(&[]), 0);
}
```

### `ch44-b005` — 44.9 `unsafe_op_in_unsafe_fn`

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:185` · mode: `run`

```rust
#![deny(unsafe_op_in_unsafe_fn)]

/// Advances a pointer inside its allocation.
///
/// # Safety
///
/// `pointer.add(count)` must remain in-bounds or one-past the same allocation.
unsafe fn advance<T>(pointer: *const T, count: usize) -> *const T {
    // SAFETY: guaranteed by the caller contract above.
    unsafe { pointer.add(count) }
}

fn main() {}
```

### `ch44-b006` — 44.10 Unsafe traits: obligación de implementer

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:207` · mode: `run`

```rust
/// Exposes one contiguous region readable for the lifetime of `&self`.
///
/// # Safety
///
/// The pointer-length pair must be non-null, initialized, contained in one
/// live allocation and free from conflicting mutation while `self` is borrowed.
unsafe trait ContiguousBytes {
    fn raw_parts(&self) -> (*const u8, usize);
}

// SAFETY: an array owns exactly N initialized contiguous bytes.
unsafe impl<const N: usize> ContiguousBytes for [u8; N] {
    fn raw_parts(&self) -> (*const u8, usize) {
        (self.as_ptr(), self.len())
    }
}
```

### `ch44-b007` — 44.11 Extern blocks y atributos unsafe en Rust 2024

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:234` · mode: `run`

```rust
unsafe extern "C" {
    // Safe only if the linked symbol really accepts every i32 value this way.
    pub safe fn abs(input: i32) -> i32;

    // The caller must provide a valid NUL-terminated string pointer.
    pub unsafe fn strlen(pointer: *const std::ffi::c_char) -> usize;
}

fn main() {}
```

### `ch44-b008` — 44.11 Extern blocks y atributos unsafe en Rust 2024

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:248` · mode: `run`

```rust
// SAFETY: this artifact defines this symbol exactly once with the published C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn rust_course_library_version() -> u32 {
    1
}

fn main() {
    assert_eq!(rust_course_library_version(), 1);
}
```

### `ch44-b009` — 44.12 Soundness se juzga desde el caller safe

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:266` · mode: `compile_only`

```rust,no_run
fn forged<'a>() -> &'a i32 {
    let local = 42;
    // El cast no liga `'a` a `local`: la referencia escapará de su storage.
    unsafe { &*(&raw const local) }
}

fn main() {
    let _dangling = forged();
}
```

### `ch44-b010` — 44.13 Panic safety, `Drop` y estados temporales

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:286` · mode: `run`

```rust
fn replace_after_successful_build<T, F>(slot: &mut T, build: F)
where
    F: FnOnce(&T) -> T,
{
    let replacement = build(slot); // si hace panic, `slot` sigue intacto
    *slot = replacement;
}
```

### `ch44-b011` — 44.16 Herramientas: detectores, no demostraciones

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:344` · mode: `reference`

```console
rustup +nightly component add miri
cargo +nightly miri test
cargo test --release --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

### `ch44-b012` — 44.17 Primero busca la alternativa safe

Source: `44.Safe-Rust-y-responsabilidad-unsafe.md:369` · mode: `run`

```rust
fn checked_get<T>(values: &[T], index: usize) -> Option<&T> {
    if index < values.len() {
        // SAFETY: this branch proves the exact precondition of `get_unchecked`.
        Some(unsafe { values.get_unchecked(index) })
    } else {
        None
    }
}

fn main() {
    assert_eq!(checked_get(&[10, 20], 1), Some(&20));
    assert_eq!(checked_get(&[10, 20], 2), None);
}
```

## 45.Punteros-crudos-aliasing-y-provenance

### `ch45-b001` — 45.2 Qué concede un puntero crudo

Source: `45.Punteros-crudos-aliasing-y-provenance.md:21` · mode: `run`

```rust
let mut value = 7_i32;
let read: *const i32 = &raw const value;
let write: *mut i32 = &raw mut value;

// SAFETY: ambos punteros proceden del mismo `value`, que sigue vivo y
// alineado; las operaciones están secuenciadas y no hay referencias activas.
unsafe {
    assert_eq!(read.read(), 7);
    write.write(8);
}

assert_eq!(value, 8);
```

### `ch45-b002` — 45.4 `&raw` evita una referencia intermedia

Source: `45.Punteros-crudos-aliasing-y-provenance.md:59` · mode: `run`

```rust
#[repr(C, packed)]
struct Header {
    kind: u8,
    sequence: u32,
}

let header = Header {
    kind: 3,
    sequence: 0x1020_3040,
};
let pointer = &raw const header.sequence;

// SAFETY: el puntero señala el campo inicializado; `read_unaligned` no exige
// la alineación natural de `u32` que una referencia sí exigiría.
let sequence = unsafe { pointer.read_unaligned() };
assert_eq!(sequence, 0x1020_3040);
```

### `ch45-b003` — 45.5 Convertir a referencia afirma mucho más

Source: `45.Punteros-crudos-aliasing-y-provenance.md:86` · mode: `compile_only`

```rust,no_run
unsafe fn forged_ref<'a, T>(pointer: *const T) -> &'a T {
    // SAFETY: esta línea solo sería correcta si el contrato externo demostrase
    // todas las premisas de referencia, incluido el `'a` elegido.
    unsafe { &*pointer }
}
```

### `ch45-b004` — 45.5 Convertir a referencia afirma mucho más

Source: `45.Punteros-crudos-aliasing-y-provenance.md:98` · mode: `run`

```rust
fn second<T>(owner: &[T]) -> Option<&T> {
    let pointer = owner.as_ptr();
    (owner.len() > 1).then(|| {
        // SAFETY: el índice está dentro de `owner` y la referencia devuelta
        // hereda exactamente su lifetime compartido.
        unsafe { &*pointer.add(1) }
    })
}

assert_eq!(second(&[10, 20, 30]), Some(&20));
```

### `ch45-b005` — 45.6 Qué garantiza —y qué no— `NonNull<T>`

Source: `45.Punteros-crudos-aliasing-y-provenance.md:126` · mode: `run`

```rust
use std::mem::align_of;
use std::ptr::NonNull;

let pointer = NonNull::<u64>::dangling();
assert_ne!(pointer.as_ptr().addr(), 0);
assert_eq!(pointer.as_ptr().addr() % align_of::<u64>(), 0);

// No se dereferencia: non-null + alineado no implica memoria accesible.
```

### `ch45-b006` — 45.8 Referencias compartidas y `UnsafeCell<T>`

Source: `45.Punteros-crudos-aliasing-y-provenance.md:160` · mode: `compile_only`

```rust,no_run
#![allow(invalid_reference_casting)]

let value = 1_i32;
let shared = &value;
let raw = shared as *const i32 as *mut i32;

// UB: la escritura contradice la referencia compartida activa.
unsafe { raw.write(2) };
println!("{shared}");
```

### `ch45-b007` — 45.8 Referencias compartidas y `UnsafeCell<T>`

Source: `45.Punteros-crudos-aliasing-y-provenance.md:179` · mode: `run`

```rust
use std::cell::UnsafeCell;

struct LocalCounter(UnsafeCell<u64>);

impl LocalCounter {
    fn increment(&self) -> u64 {
        let pointer = self.0.get();
        // SAFETY: UnsafeCell permite la mutación compartida; el tipo no es Sync
        // y el método no llama código reentrante mientras modifica el valor.
        unsafe {
            *pointer += 1;
            *pointer
        }
    }
}

let counter = LocalCounter(UnsafeCell::new(41));
assert_eq!(counter.increment(), 42);
```

### `ch45-b008` — 45.9 Exclusividad, derivación y reborrowing

Source: `45.Punteros-crudos-aliasing-y-provenance.md:206` · mode: `run`

```rust
use std::ptr;

fn swap_disjoint<T>(values: &mut [T], left: usize, right: usize) -> bool {
    if left >= values.len() || right >= values.len() || left == right {
        return false;
    }

    let base = values.as_mut_ptr();
    // SAFETY: ambos índices pertenecen a la misma slice y son distintos; el
    // préstamo `&mut [T]` impide accesos externos durante la operación.
    unsafe { ptr::swap(base.add(left), base.add(right)) };
    true
}

let mut values = ["a", "b", "c"];
assert!(swap_disjoint(&mut values, 0, 2));
assert_eq!(values, ["c", "b", "a"]);
```

### `ch45-b009` — 45.10 Aritmética: dirección alcanzada y camino permitido

Source: `45.Punteros-crudos-aliasing-y-provenance.md:237` · mode: `run`

```rust
let values = [10_u32, 20, 30];
let begin = values.as_ptr();

// SAFETY: dos elementos están dentro de la misma array viva.
let third = unsafe { begin.add(2) };
// SAFETY: `third` señala un `u32` inicializado y compartidamente legible.
assert_eq!(unsafe { third.read() }, 30);

// SAFETY: one-past puede calcularse, pero no leerse.
let end = unsafe { begin.add(values.len()) };
assert_eq!(end, unsafe { third.add(1) });
```

### `ch45-b010` — 45.13 Tagged pointers sin perder provenance

Source: `45.Punteros-crudos-aliasing-y-provenance.md:281` · mode: `run`

```rust
#[repr(align(2))]
struct Word(u16);

let value = Word(42);
let base = &value as *const Word;
let tagged = base.map_addr(|address| address | 1);

assert_eq!(tagged.addr() & 1, 1);
let restored = tagged.map_addr(|address| address & !1);

// SAFETY: se recuperó exactamente la dirección de `value`; `map_addr`
// conservó su provenance y `value` sigue prestado compartidamente.
assert_eq!(unsafe { (*restored).0 }, 42);
```

### `ch45-b011` — 45.16 Copias, solapamiento y ownership lógico

Source: `45.Punteros-crudos-aliasing-y-provenance.md:330` · mode: `run`

```rust
use std::ops::Range;
use std::ptr;

fn copy_within_raw<T: Copy>(
    values: &mut [T],
    source: Range<usize>,
    destination: usize,
) -> bool {
    if source.start > source.end || source.end > values.len() {
        return false;
    }
    let count = source.end - source.start;
    let Some(destination_end) = destination.checked_add(count) else {
        return false;
    };
    if destination_end > values.len() {
        return false;
    }

    let base = values.as_mut_ptr();
    // SAFETY: ambos rangos están dentro de la slice; `copy` admite overlap y
    // `T: Copy` permite usar tanto las copias de origen como las de destino.
    unsafe { ptr::copy(base.add(source.start), base.add(destination), count) };
    true
}

let mut values = [1, 2, 3, 4, 5];
assert!(copy_within_raw(&mut values, 0..4, 1));
assert_eq!(values, [1, 1, 2, 3, 4]);
```

### `ch45-b012` — 45.17 Miri: detector potente, no especificación final

Source: `45.Punteros-crudos-aliasing-y-provenance.md:370` · mode: `reference`

```console
rustup +nightly component add miri
cargo +nightly miri test
```

## 46.Layout-alignment-padding-y-repr

### `ch46-b001` — 46.2 Tamaño, alineación y offsets

Source: `46.Layout-alignment-padding-y-repr.md:26` · mode: `run`

```rust
use std::mem::{align_of, size_of};

assert_eq!(size_of::<u32>(), 4);
assert!(align_of::<u32>().is_power_of_two());
assert_eq!(size_of::<u32>() % align_of::<u32>(), 0);
```

### `ch46-b002` — 46.4 Medir no estabiliza

Source: `46.Layout-alignment-padding-y-repr.md:52` · mode: `run`

```rust
use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Header {
    kind: u8,
    payload_length: u32,
    version: u16,
}

assert_eq!(offset_of!(Header, kind), 0);
assert!(offset_of!(Header, kind) < offset_of!(Header, payload_length));
assert!(offset_of!(Header, payload_length) < offset_of!(Header, version));
assert_eq!(size_of::<Header>() % align_of::<Header>(), 0);
```

### `ch46-b003` — 46.5 Cómo se calcula un struct `repr(C)`

Source: `46.Layout-alignment-padding-y-repr.md:83` · mode: `run`

```rust
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

### `ch46-b004` — 46.8 `repr(transparent)`: misma representación, invariantes distintas

Source: `46.Layout-alignment-padding-y-repr.md:128` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Percentage(u8);

impl Percentage {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

assert_eq!(std::mem::size_of::<Percentage>(), std::mem::size_of::<u8>());
assert_eq!(Percentage::new(75).map(Percentage::get), Some(75));
assert_eq!(Percentage::new(101), None);
```

### `ch46-b005` — 46.9 Enums sin payload y entradas externas

Source: `46.Layout-alignment-padding-y-repr.md:156` · mode: `run`

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum RecordTag {
    User = 1,
    Order = 2,
}

impl TryFrom<u8> for RecordTag {
    type Error = u8;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            1 => Ok(Self::User),
            2 => Ok(Self::Order),
            other => Err(other),
        }
    }
}

assert_eq!(RecordTag::try_from(2), Ok(RecordTag::Order));
assert_eq!(RecordTag::try_from(99), Err(99));
```

### `ch46-b006` — 46.10 Enums con payload son tagged unions

Source: `46.Layout-alignment-padding-y-repr.md:193` · mode: `run`

```rust
#[repr(C, u8)]
enum Event {
    Started { job_id: u32 },
    Finished { job_id: u32, code: u16 },
    Stopped,
}

let event = Event::Finished {
    job_id: 7,
    code: 0,
};
assert!(std::mem::size_of_val(&event) >= std::mem::size_of::<u32>());
```

### `ch46-b007` — 46.11 Niches: depender solo de garantías publicadas

Source: `46.Layout-alignment-padding-y-repr.md:218` · mode: `run`

```rust
use std::mem::{align_of, size_of};
use std::ptr::NonNull;

assert_eq!(
    size_of::<Option<NonNull<u32>>>(),
    size_of::<NonNull<u32>>(),
);
assert_eq!(
    align_of::<Option<NonNull<u32>>>(),
    align_of::<NonNull<u32>>(),
);
```

### `ch46-b008` — 46.12 `repr(packed)` reduce alineación, no obligaciones

Source: `46.Layout-alignment-padding-y-repr.md:240` · mode: `run`

```rust
#[repr(C, packed)]
struct Packet {
    tag: u8,
    payload_length: u32,
}

let packet = Packet {
    tag: 1,
    payload_length: 42,
};
let pointer = &raw const packet.payload_length;

// SAFETY: el campo está inicializado dentro de `packet`; el raw borrow evita
// crear `&u32` y `read_unaligned` admite la alineación reducida.
let length = unsafe { pointer.read_unaligned() };
assert_eq!(length, 42);
```

### `ch46-b009` — 46.13 `repr(align(N))` eleva el requisito

Source: `46.Layout-alignment-padding-y-repr.md:265` · mode: `run`

```rust
#[repr(align(64))]
struct CacheLine<T>(T);

assert_eq!(std::mem::align_of::<CacheLine<u8>>(), 64);
assert_eq!(std::mem::size_of::<CacheLine<u8>>(), 64);

let lines = [CacheLine(10_u8), CacheLine(20_u8)];
let distance = (&raw const lines[1]).addr() - (&raw const lines[0]).addr();
assert_eq!(distance, 64);
```

### `ch46-b010` — 46.14 ZSTs, DSTs y punteros anchos

Source: `46.Layout-alignment-padding-y-repr.md:283` · mode: `run`

```rust
use std::mem::{align_of, size_of};

assert_eq!(size_of::<()>(), 0);
assert_eq!(align_of::<()>(), 1);
assert_eq!(size_of::<[(); 1_000]>(), 0);
```

### `ch46-b011` — 46.15 Un formato binario se codifica explícitamente

Source: `46.Layout-alignment-padding-y-repr.md:299` · mode: `run`

```rust
const MAGIC: [u8; 2] = *b"DR";

fn encode_header(version: u16, tag: u8, length: u32) -> [u8; 9] {
    let mut bytes = [0; 9];
    bytes[0..2].copy_from_slice(&MAGIC);
    bytes[2..4].copy_from_slice(&version.to_le_bytes());
    bytes[4] = tag;
    bytes[5..9].copy_from_slice(&length.to_le_bytes());
    bytes
}

assert_eq!(
    encode_header(3, 2, 65_537),
    [b'D', b'R', 3, 0, 2, 1, 0, 1, 0],
);
```

### `ch46-b012` — 46.16 Assertions de layout y matriz de targets

Source: `46.Layout-alignment-padding-y-repr.md:323` · mode: `run`

```rust
use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Pair {
    left: u32,
    right: u32,
}

const _: () = assert!(offset_of!(Pair, left) == 0);
const _: () = assert!(offset_of!(Pair, right) == size_of::<u32>());
const _: () = assert!(size_of::<Pair>() % align_of::<Pair>() == 0);
```

## 47.FFI-y-fronteras-con-C

### `ch47-b001` — 47.2 `unsafe extern` en Rust 2024

Source: `47.FFI-y-fronteras-con-C.md:25` · mode: `compile_only`

```rust,no_run
use std::ffi::c_char;

unsafe extern "C" {
    pub unsafe fn strlen(text: *const c_char) -> usize;
}
```

### `ch47-b002` — 47.3 Exportar símbolos también es unsafe

Source: `47.FFI-y-fronteras-con-C.md:46` · mode: `run`

```rust
// SAFETY: la biblioteca publica una única definición de este símbolo y su
// header declara exactamente `uint32_t course_abi_version(void)`.
#[unsafe(no_mangle)]
pub extern "C" fn course_abi_version() -> u32 {
    1
}

assert_eq!(course_abi_version(), 1);
```

### `ch47-b003` — 47.4 Cuatro capas, una sola zona raw

Source: `47.FFI-y-fronteras-con-C.md:63` · mode: `reference`

```text
bindings raw: símbolos, ABI y constantes
        ↓
wrapper de ownership: handles, Drop, strings, errores
        ↓
API safe: préstamos, Result, tipos de dominio
        ↓
aplicación: sin raw pointers ni códigos C
```

### `ch47-b004` — 47.6 C strings: bytes primero, UTF-8 después

Source: `47.FFI-y-fronteras-con-C.md:105` · mode: `run`

```rust
use std::ffi::{CStr, c_char};

unsafe fn raw_strlen(pointer: *const c_char) -> usize {
    // SAFETY: esta función privada delega las premisas a su caller.
    unsafe { CStr::from_ptr(pointer) }.to_bytes().len()
}

fn c_length(text: &CStr) -> usize {
    // SAFETY: `text` aporta región viva, non-null y terminada en NUL; la
    // función simulada no conserva el puntero tras retornar.
    unsafe { raw_strlen(text.as_ptr()) }
}

assert_eq!(c_length(c"Rust"), 4);
```

### `ch47-b005` — 47.8 Puntero + longitud + parámetro de salida

Source: `47.FFI-y-fronteras-con-C.md:145` · mode: `run`

```rust
use std::ffi::c_int;

const OK: c_int = 0;
const NULL_OUTPUT: c_int = -1;
const NULL_INPUT: c_int = -2;
const OVERFLOW: c_int = -3;

/// # Safety
///
/// `output` debe ser escribible como `u64`. Si `len > 0`, `values` describe
/// `len` elementos inicializados dentro de una allocation viva y disjunta.
unsafe extern "C" fn sum_u32(
    values: *const u32,
    len: usize,
    output: *mut u64,
) -> c_int {
    if output.is_null() {
        return NULL_OUTPUT;
    }
    let values = if len == 0 {
        &[]
    } else {
        if values.is_null() {
            return NULL_INPUT;
        }
        // SAFETY: el contrato público aporta rango, init, vida y aliasing.
        unsafe { std::slice::from_raw_parts(values, len) }
    };
    let Some(sum) = values
        .iter()
        .try_fold(0_u64, |sum, value| sum.checked_add(u64::from(*value)))
    else {
        return OVERFLOW;
    };
    // SAFETY: `output` es escribible y el input ya se consumió.
    unsafe { output.write(sum) };
    OK
}

let values = [10_u32, 20, 12];
let mut output = 0_u64;
// SAFETY: input y output son regiones vivas, alineadas y disjuntas.
assert_eq!(unsafe { sum_u32(values.as_ptr(), 3, &mut output) }, OK);
assert_eq!(output, 42);
```

### `ch47-b006` — 47.10 Handle opaco RAII

Source: `47.FFI-y-fronteras-con-C.md:215` · mode: `reference`

```text
buffer_create(...) -> BufferHandle*   // null + código al fallar
buffer_destroy(BufferHandle*)         // exactamente una vez
buffer_data(const BufferHandle*, ...)
```

### `ch47-b007` — 47.12 Códigos, null, `errno` y valores desconocidos

Source: `47.FFI-y-fronteras-con-C.md:244` · mode: `run`

```rust
use std::ffi::c_int;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Ready,
    Busy,
    Unknown(c_int),
}

fn decode_status(code: c_int) -> Status {
    match code {
        0 => Status::Ready,
        1 => Status::Busy,
        other => Status::Unknown(other),
    }
}

assert_eq!(decode_status(91), Status::Unknown(91));
```

### `ch47-b008` — 47.13 Callbacks: function pointer + contexto

Source: `47.FFI-y-fronteras-con-C.md:273` · mode: `run`

```rust
use std::ffi::{c_int, c_void};

#[repr(C)]
struct Event {
    kind: c_int,
}

type Callback = unsafe extern "C" fn(
    user_data: *mut c_void,
    event: *const Event,
) -> c_int;

let _type_check: Option<Callback> = None;
```

### `ch47-b009` — 47.16 Panic, unwind y el payload capturado

Source: `47.FFI-y-fronteras-con-C.md:336` · mode: `compile_only`

```rust,no_run
use std::ffi::c_int;
use std::panic::{UnwindSafe, catch_unwind};

const PANIC: c_int = -128;

fn ffi_firewall<F>(operation: F) -> c_int
where
    F: FnOnce() -> c_int + UnwindSafe,
{
    match catch_unwind(operation) {
        Ok(code) => code,
        Err(payload) => {
            // Evita que un payload hostil vuelva a hacer panic en Drop.
            std::mem::forget(payload);
            PANIC
        }
    }
}

assert_eq!(ffi_firewall(|| 7), 7);
assert_eq!(ffi_firewall(|| panic!("boom")), PANIC);
```

### `ch47-b010` — 47.18 Versionar una ABI C

Source: `47.FFI-y-fronteras-con-C.md:397` · mode: `compile_only`

```rust,no_run
use std::ffi::c_void;

#[repr(C)]
struct ApiV1 {
    abi_version: u32,
    struct_size: usize,
    context: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

let api = ApiV1 {
    abi_version: 1,
    struct_size: std::mem::size_of::<ApiV1>(),
    context: std::ptr::null_mut(),
    destroy: None,
};
assert_eq!(api.abi_version, 1);
```

## 48.Lectura-guiada-del-Rustonomicon

### `ch48-b001` — 48.4 Caso guía: el estado completo de un buffer raw

Source: `48.Lectura-guiada-del-Rustonomicon.md:64` · mode: `reference`

```text
S = (pointer, capacity, initialized, exposed_length, layout, owner)
```

### `ch48-b002` — 48.5 Inicialización parcial como máquina de estados

Source: `48.Lectura-guiada-del-Rustonomicon.md:91` · mode: `reference`

```text
1. producir T                 ← puede devolver error o hacer panic
2. escribir T en el slot k   ← el slot pasa a ser válido
3. incrementar initialized   ← el guard ya puede destruirlo
```

### `ch48-b003` — 48.6 `MaybeUninit<T>` representa «todavía no es T»

Source: `48.Lectura-guiada-del-Rustonomicon.md:105` · mode: `run`

```rust
use std::mem::MaybeUninit;

let mut slot = MaybeUninit::<String>::uninit();
let initialized: &mut String = slot.write(String::from("ready"));
initialized.push('!');

// SAFETY: `write` produjo un String válido y no se ha movido ni destruido.
let value = unsafe { slot.assume_init() };
assert_eq!(value, "ready!");
```

### `ch48-b004` — 48.11 Variance se demuestra con coerciones pequeñas

Source: `48.Lectura-guiada-del-Rustonomicon.md:196` · mode: `run`

```rust
use std::marker::PhantomData;

struct Reader<'a, T>(PhantomData<&'a T>);

fn shorten<'long: 'short, 'short, T>(reader: Reader<'long, T>) -> Reader<'short, T> {
    reader
}

let long = Reader::<'static, String>(PhantomData);
let _short = shorten(long);
```

### `ch48-b005` — 48.11 Variance se demuestra con coerciones pequeñas

Source: `48.Lectura-guiada-del-Rustonomicon.md:211` · mode: `compile_fail`

```rust,compile_fail
use std::marker::PhantomData;

struct Writer<'a, T>(PhantomData<&'a mut T>);

fn invent_static<'short>(writer: Writer<'_, &'short str>) -> Writer<'_, &'static str> {
    writer
}

fn main() {}
```

### `ch48-b006` — 48.12 `Send` y `Sync`: prueba de operaciones, no de campos

Source: `48.Lectura-guiada-del-Rustonomicon.md:229` · mode: `run`

```rust
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RawOwner<T> {
    pointer: NonNull<T>,
    owns: PhantomData<T>,
}

// SAFETY: la prueba completa debe justificar ownership único, accesos y Drop.
unsafe impl<T: Send> Send for RawOwner<T> {}
// SAFETY: desde &RawOwner<T> solo debe poder obtenerse &T.
unsafe impl<T: Sync> Sync for RawOwner<T> {}

fn main() {}
```

### `ch48-b007` — 48.14 `transmute` como pregunta de cinco partes

Source: `48.Lectura-guiada-del-Rustonomicon.md:293` · mode: `run`

```rust
let bytes = [0x78, 0x56, 0x34, 0x12];
let value = u32::from_le_bytes(bytes);
assert_eq!(value, 0x1234_5678);
assert_eq!("Rust".as_bytes(), b"Rust");
```

### `ch48-b008` — 48.18 Laboratorio reproducible

Source: `48.Lectura-guiada-del-Rustonomicon.md:351` · mode: `reference`

```console
cargo test --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.85.0 test -p course-solutions --lib unsafe_low_level::c48::tests --all-features
cargo +nightly miri test -p course-solutions --lib unsafe_low_level::c48::tests
```

## 49.Macro-rules-higiene-y-fragmentos

### `ch49-b001` — 49.2 Matcher y transcriber

Source: `49.Macro-rules-higiene-y-fragmentos.md:34` · mode: `run`

```rust
macro_rules! vec_of_strings {
    ($($value:expr),* $(,)?) => {{
        let mut output = Vec::new();
        $(output.push($value.to_string());)*
        output
    }};
}

let names = vec_of_strings!["Ada", String::from("Grace"),];
assert_eq!(names, ["Ada", "Grace"]);
```

### `ch49-b002` — 49.4 Primera coincidencia, sin backtracking posterior

Source: `49.Macro-rules-higiene-y-fragmentos.md:61` · mode: `compile_fail`

```rust,compile_fail
macro_rules! choose {
    ($value:expr) => { missing_function($value) };
    ($($tokens:tt)*) => { 0 };
}

fn main() {
    let _ = choose!(1); // coincide con el primer brazo; el segundo no rescata el error
}
```

### `ch49-b003` — 49.6 Follow sets: compatibilidad con sintaxis futura

Source: `49.Macro-rules-higiene-y-fragmentos.md:118` · mode: `compile_fail`

```rust,compile_fail
macro_rules! invalid_follow {
    ($value:expr [ $index:expr ]) => { $value };
}

fn main() {}
```

### `ch49-b004` — 49.7 Repeticiones y cardinalidad

Source: `49.Macro-rules-higiene-y-fragmentos.md:140` · mode: `run`

```rust
macro_rules! make_newtypes {
    ($( $name:ident($inner:ty) ),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            struct $name($inner);
        )+
    };
}

make_newtypes!(UserId(u64), OrderId(u64),);
assert_eq!(UserId(7).0, 7);
assert_eq!(OrderId(9).0, 9);
```

### `ch49-b005` — 49.8 La expansión debe tener límites estructurales

Source: `49.Macro-rules-higiene-y-fragmentos.md:161` · mode: `run`

```rust
macro_rules! measured {
    ($body:expr) => {{
        let start = std::time::Instant::now();
        let output = $body;
        (output, start.elapsed())
    }};
}

let (answer, elapsed) = measured!(40 + 2);
assert_eq!(answer, 42);
assert!(elapsed <= std::time::Duration::from_secs(1));
```

### `ch49-b006` — 49.9 Evaluar una vez también es API

Source: `49.Macro-rules-higiene-y-fragmentos.md:181` · mode: `run`

```rust
use std::cell::Cell;

macro_rules! sum_once {
    ($( $value:expr ),+ $(,)?) => {{
        let mut total = 0;
        $(
            let evaluated_once = $value;
            total += evaluated_once;
        )+
        total
    }};
}

let calls = Cell::new(0);
let next = || {
    calls.set(calls.get() + 1);
    10
};
assert_eq!(sum_once!(next(), next()), 20);
assert_eq!(calls.get(), 2);
```

### `ch49-b007` — 49.10 Higiene mixta

Source: `49.Macro-rules-higiene-y-fragmentos.md:215` · mode: `run`

```rust
macro_rules! doubled {
    ($value:expr) => {{
        let temporary = $value;
        temporary * 2
    }};
}

let temporary = 99;
assert_eq!(doubled!(21), 42);
assert_eq!(temporary, 99);
```

### `ch49-b008` — 49.11 `$crate`, exportación y privacidad

Source: `49.Macro-rules-higiene-y-fragmentos.md:234` · mode: `run`

```rust
#[doc(hidden)]
pub fn __normalize(input: &str) -> String {
    input.trim().to_owned()
}

#[macro_export]
macro_rules! normalized {
    ($input:expr) => {
        $crate::__normalize($input)
    };
}

fn main() {
    assert_eq!(normalized!(" Rust "), "Rust");
}
```

### `ch49-b009` — 49.13 Forwarding: los fragmentos son opacos

Source: `49.Macro-rules-higiene-y-fragmentos.md:274` · mode: `run`

```rust
macro_rules! classify {
    (3) => { "literal tres" };
    ($value:expr) => { "expresión opaca" };
}

macro_rules! forward {
    ($value:expr) => { classify!($value) };
}

assert_eq!(classify!(3), "literal tres");
assert_eq!(forward!(3), "expresión opaca");
```

### `ch49-b010` — 49.14 TT munchers y ambigüedad local

Source: `49.Macro-rules-higiene-y-fragmentos.md:294` · mode: `run`

```rust
macro_rules! count_tokens {
    () => { 0usize };
    ($_head:tt $($tail:tt)*) => { 1usize + count_tokens!($($tail)*) };
}

assert_eq!(count_tokens!(alpha + beta), 3);
```

### `ch49-b011` — 49.15 Diagnósticos diseñados

Source: `49.Macro-rules-higiene-y-fragmentos.md:316` · mode: `compile_fail`

```rust,compile_fail
const fn port_from_literal(value: u16) -> u16 {
    value
}

macro_rules! checked_port {
    ($value:literal) => { port_from_literal($value) };
    ($($other:tt)*) => {
        compile_error!("checked_port! espera un único literal entero entre 0 y 65535")
    };
}

fn main() {
    let base = 8000;
    let _ = checked_port!(base + 80);
}
```

### `ch49-b012` — 49.16 Editions y `expr_2021`

Source: `49.Macro-rules-higiene-y-fragmentos.md:344` · mode: `run`

```rust
macro_rules! expression_kind {
    (const $value:expr) => { "const block" };
    (_) => { "placeholder" };
    ($value:expr_2021) => { "expresión heredada" };
}

assert_eq!(expression_kind!(1 + 2), "expresión heredada");
assert_eq!(expression_kind!(const { 1 + 2 }), "const block");
assert_eq!(expression_kind!(_), "placeholder");
```

## 50.Procedural-macros-y-derives

### `ch50-b001` — 50.3 Arquitectura host/target sin ciclo

Source: `50.Procedural-macros-y-derives.md:40` · mode: `reference`

```text
macro_lab/      crate proc-macro: parsea y genera tokens
macro_api/      trait Entity + reexport de las tres macros
macro_fixture/  consumidor externo; renombra macro_api como domain_api
solutions/      ejercicios y asserts de contrato
```

### `ch50-b002` — 50.4 La crate `proc-macro`

Source: `50.Procedural-macros-y-derives.md:53` · mode: `parse`

```toml
[package]
name = "course-macro-lab"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
publish = false

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1"
quote = "1"
syn = { version = "2", features = ["full"] }
```

### `ch50-b003` — 50.6 Pipeline parsear → validar → modelar → emitir

Source: `50.Procedural-macros-y-derives.md:100` · mode: `illustrative`

```rust,ignore
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_entity(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
```

### `ch50-b004` — 50.7 Un derive añade items

Source: `50.Procedural-macros-y-derives.md:120` · mode: `illustrative`

```rust,ignore
#[derive(course_macro_api::Entity)]
#[entity(id = "user_id")]
struct User {
    user_id: u64,
    name: String,
}
```

### `ch50-b005` — 50.8 Preservar generics no es copiar una cadena

Source: `50.Procedural-macros-y-derives.md:137` · mode: `run`

```rust
trait Marker {}

struct Wrapper<'a, T, const N: usize>
where
    T: 'a,
{
    values: &'a [T; N],
}

impl<'a, T, const N: usize> Marker for Wrapper<'a, T, N>
where
    T: 'a,
{}

let values = [String::from("Rust")];
let wrapper = Wrapper { values: &values };
assert_eq!(wrapper.values.len(), 1);
```

### `ch50-b006` — 50.8 Preservar generics no es copiar una cadena

Source: `50.Procedural-macros-y-derives.md:159` · mode: `reference`

```text
impl <impl_generics> Trait for Name <type_generics> <where_clause>
```

### `ch50-b007` — 50.10 Helper attributes como gramática cerrada

Source: `50.Procedural-macros-y-derives.md:193` · mode: `illustrative`

```rust,ignore
attribute.parse_nested_meta(|meta| {
    if meta.path.is_ident("id") {
        options.id = Some(meta.value()?.parse()?);
        Ok(())
    } else if meta.path.is_ident("crate_path") {
        options.crate_path = Some(meta.value()?.parse()?);
        Ok(())
    } else {
        Err(meta.error("se esperaba `id` o `crate_path`"))
    }
})?;
```

### `ch50-b008` — 50.11 Diagnósticos y spans son parte del contrato

Source: `50.Procedural-macros-y-derives.md:215` · mode: `reference`

```text
error: el campo `missing` no existe en esta struct
 --> src/lib.rs:4:15
  |
4 | #[entity(id = "missing")]
  |               ^^^^^^^^^
```

### `ch50-b009` — 50.12 Higiene procedural y paths runtime

Source: `50.Procedural-macros-y-derives.md:239` · mode: `parse`

```toml
[dependencies]
domain-api = { package = "course-macro-api", path = "../macro_api" }
```

### `ch50-b010` — 50.13 Attribute macros: reemplazo total

Source: `50.Procedural-macros-y-derives.md:256` · mode: `illustrative`

```rust,ignore
#[proc_macro_attribute]
pub fn preserve_item(attribute: TokenStream, item: TokenStream) -> TokenStream {
    // validar attribute, parsear item y devolver el item completo preservado
}
```

### `ch50-b011` — 50.14 Function-like macros: parser antes que DSL ilimitada

Source: `50.Procedural-macros-y-derives.md:278` · mode: `illustrative`

```rust,ignore
const FIELDS: &[&str] = course_macro_api::field_names!(id, payload,);
```

### `ch50-b012` — 50.20 Laboratorio verificable del libro

Source: `50.Procedural-macros-y-derives.md:372` · mode: `reference`

```console
cargo test -p course-macro-fixture --all-targets
cargo test -p course-macro-api --doc
cargo test -p course-solutions --lib organization::c50::tests --all-features
cargo +1.85.0 test -p course-macro-api --doc
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

## 51.Editions-migraciones-y-evolución

### `ch51-b001` — 51.2 Tres ejes que no deben colapsarse

Source: `51.Editions-migraciones-y-evolución.md:30` · mode: `parse`

```toml
[package]
name = "domain-ledger"
version = "1.4.0"
edition = "2024"
rust-version = "1.85"
```

### `ch51-b002` — 51.3 La frontera de compatibilidad de una edition

Source: `51.Editions-migraciones-y-evolución.md:60` · mode: `run`

```rust
fn r#gen() -> &'static str {
    "nombre legado"
}

assert_eq!(r#gen(), "nombre legado");
```

### `ch51-b003` — 51.6 Puerta 1: baseline reproducible

Source: `51.Editions-migraciones-y-evolución.md:98` · mode: `reference`

```text
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo fmt --all --check
```

### `ch51-b004` — 51.7 Puerta 2: lints y `cargo fix --edition`

Source: `51.Editions-migraciones-y-evolución.md:113` · mode: `reference`

```text
cargo fix --edition --workspace --all-features --all-targets
```

### `ch51-b005` — 51.10 Temporales en `if let`

Source: `51.Editions-migraciones-y-evolución.md:161` · mode: `run`

```rust
use std::cell::RefCell;

let values = RefCell::new(Vec::<u8>::new());
if let Some(value) = values.borrow_mut().pop() {
    assert!(value > 0);
} else {
    // En 2024 ya se destruyó el RefMut temporal de la condición.
    values.borrow_mut().push(1);
}

assert_eq!(values.into_inner(), vec![1]);
```

### `ch51-b006` — 51.11 Temporales de una tail expression

Source: `51.Editions-migraciones-y-evolución.md:181` · mode: `run`

```rust
use std::cell::RefCell;

fn stored_values() -> usize {
    let values = RefCell::new(vec![10, 20, 30]);
    values.borrow().len()
}

assert_eq!(stored_values(), 3);
```

### `ch51-b007` — 51.12 Match ergonomics más explícitas

Source: `51.Editions-migraciones-y-evolución.md:198` · mode: `run`

```rust
let names = [String::from("Ada"), String::from("Grace")];
let &[ref first, ref second] = &names;

assert_eq!((first.as_str(), second.as_str()), ("Ada", "Grace"));
```

### `ch51-b008` — 51.13 Captura de parámetros en RPIT

Source: `51.Editions-migraciones-y-evolución.md:213` · mode: `run`

```rust
fn copied<'a>(values: &'a [u32]) -> impl Iterator<Item = u32> + use<'a> {
    values.iter().copied()
}

assert_eq!(copied(&[2, 3, 5]).sum::<u32>(), 10);
```

### `ch51-b009` — 51.14 `expr` y `expr_2021` en macros

Source: `51.Editions-migraciones-y-evolución.md:227` · mode: `run`

```rust
macro_rules! classify_2024 {
    ($value:expr) => { "general" };
    (const $value:expr) => { "const" };
    (_) => { "placeholder" };
}

macro_rules! preserve_2021 {
    ($value:expr_2021) => { "general" };
    (const $value:expr) => { "const" };
    (_) => { "placeholder" };
}

assert_eq!(classify_2024!(const { 4 }), "general");
assert_eq!(classify_2024!(_), "general");
assert_eq!(preserve_2021!(const { 4 }), "const");
assert_eq!(preserve_2021!(_), "placeholder");
```

### `ch51-b010` — 51.15 `unsafe` explícito exige una nueva auditoría

Source: `51.Editions-migraciones-y-evolución.md:258` · mode: `run`

```rust
#![deny(unsafe_op_in_unsafe_fn)]

unsafe fn read_copy<T: Copy>(pointer: *const T) -> T {
    // SAFETY: el caller garantiza que pointer está alineado, es legible y
    // apunta a un T inicializado durante toda esta lectura.
    unsafe { pointer.read() }
}

let value = 21_u32;
// SAFETY: &value produce un puntero válido, alineado e inicializado.
assert_eq!(unsafe { read_copy(&value) }, 21);
```

### `ch51-b011` — 51.16 Cargo y `resolver = "3"`

Source: `51.Editions-migraciones-y-evolución.md:280` · mode: `parse`

```toml
[workspace]
members = ["domain", "cli"]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.85"
```

### `ch51-b012` — 51.16 Cargo y `resolver = "3"`

Source: `51.Editions-migraciones-y-evolución.md:292` · mode: `parse`

```toml
[package]
name = "domain"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
```

### `ch51-b013` — 51.18 `rust-toolchain.toml`: reproducibilidad, no MSRV

Source: `51.Editions-migraciones-y-evolución.md:321` · mode: `parse`

```toml
[toolchain]
channel = "1.85.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

## 52.Rustc-MIR-LLVM-y-toolchain

### `ch52-b001` — 52.8 Moves, drops y unwind observables

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:92` · mode: `run`

```rust
struct Ticket {
    label: String,
}

#[inline(never)]
fn inspect_ticket(ticket: Ticket) -> usize {
    ticket.label.len()
}

assert_eq!(inspect_ticket(Ticket { label: String::from("MIR") }), 3);
```

### `ch52-b002` — 52.10 Queries e invalidación incremental

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:123` · mode: `reference`

```text
source(api) → HIR(api) → typeck(api) → metadata(crate)
source(body privado) → MIR(body) → codegen(unit)
```

### `ch52-b003` — 52.11 Monomorfización: de genérico a instancias

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:136` · mode: `run`

```rust
use std::ops::Add;

#[inline(never)]
fn twice<T>(value: T) -> T
where
    T: Copy + Add<Output = T>,
{
    value + value
}

assert_eq!(twice(21_u64), 42);
assert_eq!(twice(1.5_f64), 3.0);
```

### `ch52-b004` — 52.15 Emisiones estables de `rustc`

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:196` · mode: `reference`

```text
rustc --edition=2024 --crate-type=lib --emit=mir,llvm-ir,asm code/compiler_lab/pipeline.rs
rustc --edition=2024 -C opt-level=3 --crate-type=lib --emit=asm code/compiler_lab/pipeline.rs
```

### `ch52-b005` — 52.18 Timings: buscar camino crítico, no la crate más grande

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:243` · mode: `reference`

```text
cargo clean
cargo build --workspace --timings
cargo build --workspace --timings
```

### `ch52-b006` — 52.20 Profiles: hipótesis que se miden

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:280` · mode: `parse`

```toml
[profile.release-observable]
inherits = "release"
debug = "line-tables-only"
lto = "thin"
codegen-units = 1
incremental = false
```

### `ch52-b007` — 52.21 Inspeccionar el build que Cargo realmente pidió

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:297` · mode: `reference`

```text
cargo metadata --format-version 1
cargo tree --edges all --duplicates
cargo build -vv
cargo rustc --lib -- --emit=mir
```

### `ch52-b008` — 52.25 Cross-compilation completa

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:347` · mode: `reference`

```text
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
```

### `ch52-b009` — 52.25 Cross-compilation completa

Source: `52.Rustc-MIR-LLVM-y-toolchain.md:364` · mode: `parse`

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
runner = "qemu-aarch64"
```

## 53.Diseño-de-librerías-idiomáticas

### `ch53-b001` — 53.1 Diseñar una promesa, no una colección de módulos

Source: `53.Diseño-de-librerías-idiomáticas.md:9` · mode: `illustrative`

```rust,ignore
use atlas_client::{Client, UserId};

let id = UserId::parse("usr_42")?;
let user = client.user(id).await?;
println!("{}", user.display_name());
```

### `ch53-b002` — 53.4 Fachada estable, interior privado

Source: `53.Diseño-de-librerías-idiomáticas.md:62` · mode: `run`

```rust
mod client {
    #[derive(Debug)]
    pub struct Client;
}
mod error {
    #[derive(Debug)]
    pub struct LoadError;
}
mod model {
    #[derive(Debug)]
    pub struct User;
    #[derive(Debug)]
    pub struct UserId;
}

pub use client::Client;
pub use error::LoadError;
pub use model::{User, UserId};

fn main() {
    let _ = (Client, LoadError, User, UserId);
}
```

### `ch53-b003` — 53.6 Invariantes en constructores y newtypes

Source: `53.Diseño-de-librerías-idiomáticas.md:108` · mode: `run`

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UserId(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserIdError {
    Empty,
    InvalidPrefix,
}

impl UserId {
    pub fn parse(input: impl Into<String>) -> Result<Self, UserIdError> {
        let input = input.into();
        if input.is_empty() {
            return Err(UserIdError::Empty);
        }
        if !input.starts_with("usr_") {
            return Err(UserIdError::InvalidPrefix);
        }
        Ok(Self(input))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let id = UserId::parse("usr_42").unwrap();
    assert_eq!(id.as_str(), "usr_42");
    assert_eq!(UserId::parse(""), Err(UserIdError::Empty));
}
```

### `ch53-b004` — 53.7 Ownership y coste visibles

Source: `53.Diseño-de-librerías-idiomáticas.md:146` · mode: `illustrative`

```rust,ignore
impl Document {
    pub fn title(&self) -> &str;
    pub fn rename(&mut self, title: Title);
    pub fn into_bytes(self) -> Vec<u8>;
}
```

### `ch53-b005` — 53.8 Bounds mínimos en el lugar mínimo

Source: `53.Diseño-de-librerías-idiomáticas.md:171` · mode: `run`

```rust
use std::fmt::Display;

pub fn labels<T: Display>(items: &[T]) -> Vec<String> {
    items.iter().map(ToString::to_string).collect()
}

struct DisplayOnly(u8);

impl Display for DisplayOnly {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "item-{}", self.0)
    }
}

fn main() {
    let values = [DisplayOnly(7)];
    assert_eq!(labels(&values), ["item-7"]);
}
```

### `ch53-b006` — 53.10 Errores para que el caller decida

Source: `53.Diseño-de-librerías-idiomáticas.md:208` · mode: `run`

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum LoadError {
    NotFound { key: String },
    InvalidFormat { line: usize },
    Backend { source: Box<dyn Error + Send + Sync> },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { key } => write!(f, "no existe {key}"),
            Self::InvalidFormat { line } => write!(f, "formato inválido en línea {line}"),
            Self::Backend { .. } => f.write_str("falló el backend"),
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

fn main() {
    let error = LoadError::NotFound { key: "port".into() };
    assert!(matches!(error, LoadError::NotFound { .. }));
}
```

### `ch53-b007` — 53.15 Features aditivas y unificación

Source: `53.Diseño-de-librerías-idiomáticas.md:296` · mode: `parse`

```toml
[features]
default = []
serde = ["dep:serde"]
tokio = ["dep:tokio"]

[dependencies]
serde = { version = "1", optional = true }
tokio = { version = "1", optional = true }
```

### `ch53-b008` — 53.17 `no_std`: separar `core`, `alloc` y `std`

Source: `53.Diseño-de-librerías-idiomáticas.md:333` · mode: `illustrative`

```rust,ignore
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
```

### `ch53-b009` — 53.23 MSRV, edition y dependencias públicas

Source: `53.Diseño-de-librerías-idiomáticas.md:413` · mode: `parse`

```toml
[package]
edition = "2024"
rust-version = "1.85"
```

### `ch53-b010` — 53.25 Empaquetar antes de publicar

Source: `53.Diseño-de-librerías-idiomáticas.md:445` · mode: `reference`

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo package --list
cargo publish --dry-run
```

## 54.Refactorización-hacia-código-excelente

### `ch54-b001` — 54.6 Caso inicial: válido, pero con contratos mezclados

Source: `54.Refactorización-hacia-código-excelente.md:72` · mode: `run`

```rust
fn create_user(
    email: String,
    admin: bool,
    users: &mut Vec<(String, bool)>,
    events: &mut Vec<String>,
) -> Result<(), String> {
    if !email.contains('@') {
        return Err("bad email".to_owned());
    }
    if users.iter().any(|(existing, _)| existing == &email) {
        return Err("duplicate".to_owned());
    }
    users.push((email.clone(), admin));
    events.push(format!("created:{email}"));
    Ok(())
}

fn main() {
    let mut users = Vec::new();
    let mut events = Vec::new();
    create_user("ada@example.test".into(), true, &mut users, &mut events).unwrap();
    assert_eq!(users, [("ada@example.test".to_owned(), true)]);
    assert_eq!(events, ["created:ada@example.test"]);
}
```

### `ch54-b002` — 54.7 Primer corte: nombres e invariantes

Source: `54.Refactorización-hacia-código-excelente.md:103` · mode: `run`

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
struct Email(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Role {
    Member,
    Administrator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    email: Email,
    role: Role,
}

impl Email {
    fn parse(value: String) -> Result<Self, String> {
        value
            .contains('@')
            .then_some(Self(value))
            .ok_or_else(|| "invalid email".to_owned())
    }
}

fn main() {
    let user = User {
        email: Email::parse("ada@example.test".into()).unwrap(),
        role: Role::Administrator,
    };
    assert_eq!(user.role, Role::Administrator);
}
```

### `ch54-b003` — 54.8 Segundo corte: errores que conservan causalidad

Source: `54.Refactorización-hacia-código-excelente.md:143` · mode: `illustrative`

```rust,ignore
#[derive(Debug)]
enum CreateUserError {
    InvalidEmail(EmailError),
    DuplicateEmail,
    Storage(StorageError),
}
```

### `ch54-b004` — 54.9 Separar decisión de efecto

Source: `54.Refactorización-hacia-código-excelente.md:167` · mode: `illustrative`

```rust,ignore
fn prepare_user(email: String, role: Role) -> Result<User, EmailError> {
    Ok(User {
        email: Email::parse(email)?,
        role,
    })
}

fn register(
    store: &mut impl UserStore,
    events: &mut impl EventSink,
    user: User,
) -> Result<(), RegisterError> {
    store.insert_unique(user.clone())?;
    events.user_registered(&user)?;
    Ok(())
}
```

### `ch54-b005` — 54.10 Estados y transiciones explícitas

Source: `54.Refactorización-hacia-código-excelente.md:194` · mode: `run`

```rust
#[derive(Debug)]
struct Draft {
    title: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Published {
    title: String,
    revision: u64,
}

impl Draft {
    fn publish(self, revision: u64) -> Published {
        Published {
            title: self.title,
            revision,
        }
    }
}

fn main() {
    let draft = Draft { title: "Rust".into() };
    let published = draft.publish(1);
    assert_eq!(published, Published { title: "Rust".into(), revision: 1 });
}
```

### `ch54-b006` — 54.12 Iteradores: lazy, ownership y legibilidad

Source: `54.Refactorización-hacia-código-excelente.md:242` · mode: `run`

```rust
#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let users = vec![
        User { name: " Ada ".into() },
        User { name: "  ".into() },
    ];

    let borrowed: Vec<&str> = users
        .iter()
        .map(|user| user.name.trim())
        .filter(|name| !name.is_empty())
        .collect();
    assert_eq!(borrowed, ["Ada"]);

    let owned: Vec<String> = borrowed.into_iter().map(str::to_owned).collect();
    assert_eq!(owned, ["Ada"]);
}
```

### `ch54-b007` — 54.14 La concurrencia forma parte del contrato

Source: `54.Refactorización-hacia-código-excelente.md:285` · mode: `reference`

```text
actor A: contains(email) -> false
actor B: contains(email) -> false
actor A: insert(user)   -> éxito
actor B: insert(user)   -> ¿duplicado?
```

### `ch54-b008` — 54.14 La concurrencia forma parte del contrato

Source: `54.Refactorización-hacia-código-excelente.md:294` · mode: `illustrative`

```rust,ignore
trait UserStore {
    type Error;

    fn insert_unique(
        &mut self,
        user: User,
    ) -> Result<InsertOutcome, Self::Error>;
}
```

### `ch54-b009` — 54.18 Automatización: rustfmt, Clippy y `cargo fix`

Source: `54.Refactorización-hacia-código-excelente.md:354` · mode: `reference`

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fix --workspace --all-features
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc --all-features
```

## 55.Lectura-de-crates-de-alta-calidad

### `ch55-b001` — 55.7 `Cargo.toml`: arquitectura declarada

Source: `55.Lectura-de-crates-de-alta-calidad.md:112` · mode: `reference`

```text
cargo metadata --format-version 1
cargo tree -e normal,build
cargo tree -e features
cargo tree -d
```

### `ch55-b002` — 55.9 `cargo tree`: explicar por qué, no solo qué

Source: `55.Lectura-de-crates-de-alta-calidad.md:141` · mode: `reference`

```text
cargo tree -e features -i nombre-dependencia
cargo tree --target all -e normal,build
cargo tree -d
```

### `ch55-b003` — 55.10 Leer lo que se distribuye

Source: `55.Lectura-de-crates-de-alta-calidad.md:153` · mode: `reference`

```text
cargo package --list
cargo package
```

### `ch55-b004` — 55.11 Dibujar la fachada pública

Source: `55.Lectura-de-crates-de-alta-calidad.md:172` · mode: `reference`

```text
crate root
├── Client
├── Error
├── model::{Id, Record}
├── traits::Transport
└── feature-gated integrations
```

### `ch55-b005` — 55.13 Seguir un corte vertical

Source: `55.Lectura-de-crates-de-alta-calidad.md:214` · mode: `reference`

```text
entrada pública
→ validación / normalización
→ tipo o estado interno
→ selección de backend
→ efecto / syscall / await
→ mapeo de error
→ resultado observable
```

### `ch55-b006` — 55.19 Auditar `unsafe` desde su frontera safe

Source: `55.Lectura-de-crates-de-alta-calidad.md:309` · mode: `reference`

```text
rg -n "unsafe|SAFETY|MaybeUninit|from_raw|into_raw|transmute|NonNull" src
```

### `ch55-b007` — 55.20 Concurrencia y async: dibujar propietarios

Source: `55.Lectura-de-crates-de-alta-calidad.md:331` · mode: `reference`

```text
Service
├── task de admisión       — owner: supervisor
├── N workers              — owner: JoinSet/grupo
├── channel bounded        — cierre: sender principal
└── shutdown deadline      — después: abort + join
```

### `ch55-b008` — 55.26 La ficha de evidencia

Source: `55.Lectura-de-crates-de-alta-calidad.md:433` · mode: `reference`

```text
Pregunta y criterio de cierre:
Versión / commit / checksum:
Toolchain / target / features:
Promesa pública y enlace:
Grafo relevante:
Corte vertical:
Tipos y ownership:
Errores y efectos:
Concurrencia / cancelación:
Unsafe y premisas:
Tests que sostienen el contrato:
Historia relevante:
Hechos:
Inferencias pendientes:
Idea transferible:
Restricción que impide copiarla:
Experimento mínimo y resultado:
```

## 56.Katas-de-ownership-lifetimes-traits-y-concurrencia

### `ch56-b001` — 56.2 Protocolo de práctica

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:30` · mode: `reference`

```text
cargo test --all-targets --all-features
cargo test --doc --all-features
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

### `ch56-b002` — 56.5 Kata 1 — moves en una cola

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:77` · mode: `illustrative`

```rust,ignore
fn drain_ready(queue: &mut Vec<Job>) -> Vec<Job>;
```

### `ch56-b003` — 56.6 Kata 2 — vistas prestadas

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:101` · mode: `illustrative`

```rust,ignore
fn matching_names<'u>(users: &'u [User], prefix: &str) -> Vec<&'u str>;
```

### `ch56-b004` — 56.7 Kata 3 — dos préstamos mutables

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:123` · mode: `illustrative`

```rust,ignore
fn transfer(
    accounts: &mut [Account],
    from: usize,
    to: usize,
    amount: Money,
) -> Result<(), TransferError>;
```

### `ch56-b005` — 56.8 Kata 4 — parser sin copiar tokens

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:150` · mode: `illustrative`

```rust,ignore
struct Command<'a> {
    verb: Verb,
    arguments: Vec<&'a str>,
}
```

### `ch56-b006` — 56.11 Kata 7 — enum frente a `dyn`

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:210` · mode: `illustrative`

```rust,ignore
enum Transform {
    Trim,
    Lowercase,
    Prefix(String),
}
```

### `ch56-b007` — 56.12 Kata 8 — iterador propio

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:234` · mode: `illustrative`

```rust,ignore
struct ChunksExact<'a, T> {
    remaining: &'a [T],
    remainder: &'a [T],
    size: usize,
}
```

### `ch56-b008` — 56.13 Kata 9 — worker con shutdown

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:256` · mode: `illustrative`

```rust,ignore
enum Command {
    Apply(Change),
    Snapshot(Sender<StateSnapshot>),
    Shutdown,
}
```

### `ch56-b009` — 56.14 Kata 10 — atomics e invariantes

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:288` · mode: `reference`

```text
A carga source = 10; B carga source = 10
A decide transferir 7; B decide transferir 7
ambos debitan y acreditan por separado
```

### `ch56-b010` — 56.15 Kata 11 — cancelación async

Source: `56.Katas-de-ownership-lifetimes-traits-y-concurrencia.md:306` · mode: `reference`

```text
download → decode → validate → persist
```

## 57.Mini-crates-y-proyectos-de-consolidación

### `ch57-b001` — 57.3 Contrato común del repositorio

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:40` · mode: `reference`

```text
Cargo.toml
README.md
DESIGN.md
src/
tests/
examples/
benches/       # solo si existe una afirmación de rendimiento
```

### `ch57-b002` — 57.6 Proyecto 1: `domain_ids`

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:78` · mode: `reference`

```text
UserId · OrderId · Email · Percentage · Money
```

### `ch57-b003` — 57.7 Proyecto 2: `record_import`

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:105` · mode: `reference`

```text
bytes → UTF-8 → líneas → parse → validate → deduplicate → report
```

### `ch57-b004` — 57.11 Proyecto 6: `catalog_service`

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:203` · mode: `reference`

```text
crates/
├── catalog-domain
├── catalog-application
└── catalog-adapters
apps/
└── catalog-server
```

### `ch57-b005` — 57.13 Proyecto 8: `native_checksum`

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:249` · mode: `reference`

```text
native-checksum-sys   # bindings raw + build/link
native-checksum       # API safe + ownership + errores
```

### `ch57-b006` — 57.14 Proyecto 9: capstone offline-first

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:275` · mode: `reference`

```text
domain       cambios · versiones · conflictos
application  editar · sincronizar · resolver
storage      documento + log local transaccional
http         servidor Axum
desktop      comandos Tauri
runtime      tasks Tokio supervisadas
```

### `ch57-b007` — 57.15 Arquitectura del capstone

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:304` · mode: `reference`

```text
Tauri ─┐
       ├─> application ─> domain
Axum ──┘        │
                ├─> storage port <─ adapter local
                └─> sync port    <─ adapter HTTP
```

### `ch57-b008` — 57.21 Demo reproducible

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:397` · mode: `reference`

```text
1. checkout limpio y toolchain fijado
2. cargo test --workspace --all-targets --all-features
3. iniciar componentes con puertos efímeros
4. ejecutar escenario y guardar IDs
5. provocar el fallo documentado
6. cerrar y comprobar reporte
7. repetir sin estado residual
```

### `ch57-b009` — 57.22 Puerta automatizada y MSRV

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:413` · mode: `reference`

```text
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### `ch57-b010` — 57.26 Cómo auditar los kernels del libro

Source: `57.Mini-crates-y-proyectos-de-consolidación.md:472` · mode: `reference`

```text
cargo test -p course-solutions --lib projects
cargo clippy -p course-solutions --lib --all-features -- -D warnings
```

## 58.Criterio-de-maestría

### `ch58-b001` — 58.1 La triple explicación

Source: `58.Criterio-de-maestría.md:13` · mode: `illustrative`

```rust,ignore
fn names(users: &[User]) -> impl Iterator<Item = &str> {
    users.iter().map(|user| user.name.as_str())
}
```

### `ch58-b002` — 58.2 Leer una firma completa

Source: `58.Criterio-de-maestría.md:27` · mode: `illustrative`

```rust,ignore
async fn execute<R>(
    repo: &R,
    command: Command,
) -> Result<Event, ExecuteError>
where
    R: Repository + Sync,
```

### `ch58-b003` — 58.15 Paquete mínimo de evidencia

Source: `58.Criterio-de-maestría.md:290` · mode: `reference`

```text
PREDICTION.md    regla y resultado antes de compilar
src/             implementación mínima
tests/           éxito, borde, fallo y contrato negativo
EXPLANATION.md   compila · garantiza · cuesta
EVIDENCE.md      comandos, fuentes, medidas y versiones
CHANGE.md        requisito sorpresa y adaptación
```

### `ch58-b004` — 58.15 Paquete mínimo de evidencia

Source: `58.Criterio-de-maestría.md:301` · mode: `reference`

```text
cargo test -p course-solutions --lib mastery
cargo clippy -p course-solutions --lib --all-features -- -D warnings
```

### `ch58-b005` — 58.22 Puerta automatizada

Source: `58.Criterio-de-maestría.md:403` · mode: `reference`

```text
cargo check --workspace --all-targets --all-features --locked
cargo test --workspace --all-targets --all-features --locked
cargo test --workspace --doc --all-features --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

## 59.Glosario-solucionario-e-indice-de-codigo
