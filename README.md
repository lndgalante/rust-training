# rust-training


## Rust for TS-devs

### Intro

- Variables

```rust
let foo = 5; // constante
let mut foo = 5; // mutable
```

- Shadowing

Se puede cambiar tipos de variables

```rust
let foo = 5;
let foo = "5";
```

- Condicionales

No se usan parentesis en la condición.

```rust
if is_even(number) && number % 4 == 0 {
    println!("number is divisible by 4");
}
```

- Loops

Sin incluir el último elemento

```rust
for i in 0..10 {
  println!("i: {}", i);
}
```

Incluyendo el último elemento

```rust
for i in 0..=10 {
  println!("i: {}", i);
}
```

- While

```rust
while true {

}
```

- Loops infinitos

```rust
loop {
  println!("loop");
}
```

- Colecciones

```rust
for x in &some_array {
  println!("x: {}", x);
}
```

También se puede mapear sobre un array

```rust
vec![1, 2, 3, 4, 5]
  .iter()
  .map(|x| x + 1)
  .collect()
```

- Funciones

```rust
fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
```

- Closures

```rust
|x| {
  return x;
}
```

```rust
|x| x + 1
```

- Clases

```rust
struct Foo {
  properties // private
  pub properties // public
}

impl Foo {
  fn this() // static fn
  pub fn this() // public static fn


  fn this(&self) // method
  fn this(&mut self) // change methods on itself

  pub fn this(self) // consume self
}
```

Los structs definen las propiedades de un objeto.

- Interfaces

```rust
trait Foo {
  // no properties
  fn method(&self) -> return_type;
}

impl Foo for MyStruct {
  fn method(&self) -> return_type {
    // method implementation
  }
}
```

Los traits definen los métodos que un objeto debe tener.


#### Numbers y Strings

- Numbers

Se tiene que definir un número de manera más granular.

- i<number>: un entero que puede ser positivo o negativo (signed)
- u<number>: un entero que solo puede ser positivo (unsigned)
- f<number>: un número que requiere decimales
- usize: un u<number> que depende de la arquitectura del sistema (64 o 32)
- isize: un i<number> que depende de la arquitectura del sistema (64 o 32)

```rust
let a: i32 = -10;
let b: u32 = 20;
let c: f32 = 30.0;
```

```rust
4 / 3 = 1;

4.0 / 3 // No se puede, hay que castear el decimal a entero o viceversa
4.0 / 3.0 // Se puede

let foo = 4u32;
foo * -1 // Error, no se puede multiplicar un entero sin signo por uno negativo
```

- Strings y &str

- String
  - Los String se alojan en el heap
  - Los String pueden ser mutables

- &str
  - &str es un puntero a una cadena de caracteres en memoria, es una vista a un String
  - &str es inmutable
  - &str es análogo a &[u8]

```rust
let x = "Hello, world!";
let y = x[0..5]; // "Hello" -> This is a &str
```

#### Recomendaciones

Esta bien usar:
- unwraps
- clones

Mientras se esta aprendiendo Rust.


#### Vector, Tuple, y Struct

- Vect (rust)

```rust
let a = vec![1, 2, 3];

// Mutacion
a.push(6); // Error: a is not mutable

// Para esto se tiene que usar una variable mutable
let mut a = a;
a.push(6); // Ok
```

Como acceder a un elemento de un vector?

```rust
let a = vec![1, 2, 3, 4, 5];

let item = a[2]; // funciona, pero si se usa un indice que no existe, crashea
let item = a.get(2); // devuelve un Option<T> donde T puede ser un i32

```

Un Option<T> es un posible valor undefined. Todas las cosas que pueden fallar deben ser especificadas con un Option.

- Tuple

Es una estructura de datos que puede tener varios tipos de datos pero que es fija.

```rust
let a = (1, String::from("Hello"));

let (x, y) = a;
```

También funciona en las funciones

```rust
let a = (1, String::from("Hello"));

fn add_one((my_int, my_string): (i32, String)) -> (i32, String) {
  return (my_int + 1, my_string);
}

let a = add_one(a);
```

- Struct

```rust
struct MyStruct {
  x: usize,
  y: usize,
  z: usize,
}

fn bar(MyStruct { x, y, z }: MyStruct) {
  println!("x: {}, y: {}, z: {}", x, y, z);
}

fn main() {
  let foo = MyStruct { x: 1, y: 2, z: 3 };

  let MyStruct { x, .. } = foo;
  let MyStruct { y, z, ..} = foo;

  if let MyStruct { x, .. } = foo {
    println!("things about x: {}", x);
  }
}
```

#### Todo, Unreachable, and Unwrap

- todo

Se utiliza para indicar que aún no se ha implementado algo.

```rust
fn not_completed_fn(x: usize) -> usize {
  if x < 10 {
    return true;
  }

  todo!("hey, me, finish this later");
}
```

- unreachable

Se utiliza para indicar que una parte del código nunca se ejecutará.

```rust
fn only_evens(x: usize) -> bool {
  if x % 2 == 1 {
    unreachable!("this should never happen");
  }

  todo!("hey, me, finish this later");
}

```

- unwrap

No deberiamos de usarlo ya que puede causar un crash, pero es útil para aprender en un principio.

Nos permite obtener el valor de un Option<T> (parecido a null o undefined) o un Result<T, E>.

```rust
fn main() {
   let foo = None;

   let bar = foo.unwrap(); // Error: no value present
}

```

### Codeando en Rust

#### Valor, Mutable y No mutable


```rust
let mut x = 5; // valor
let y = &x; // referencia a x, de solo lectura
let y = &mut x; // referencia a x, de lectura y escritura
// Para que este último funcione, x tiene que ser mutable
```

Ejemplo: en JS no podrías usar `.reverse()` en un array sino esta definido como `&mut`.

Esta distinción es importante ya que nos permite generar funciones que: mutan y funciones que no.

- Pueden leer
```rust
fn do_this(x: &i32) {
  println!("x: {}", x);
}
```

- Pueden mutar sus propios valores

```rust
fn do_this(x: &mut i32) {
  *x += 1;
}
```

En resumen:

- Reference, significa solo lectura, y se define con &
- Mutable reference, significa lectura y escritura, y se define con &mut
- Value, significa que es un valor en sí, y se define normalmente


#### Unit

() es llamada Unit, y efectivamente es un valor que no tiene nada.

Ejemplo:

```rust
fn only_evens(x: usize) -> Result<()> {

  return Ok(());
}
```

#### Codeando un iterador

Un iterador es una estructura de datos que nos permite iterar sobre otra colección de datos, por ej un `vec![1,2,3]`.

```typescript
const newArray = [1, 2, 3].map(item => item + 1);
console.log(newArray);
```

```rust
let numbers_vector: Vec<_> = vec![1, 2, 3].iter().map(|item| item + 10).collect();
```

```rust
Vec<_> // el _ infiere los datos internos del vector
```

Para volver a convertir iterador hacia una estructura, tenemos que hacer 2 cosas:
- Definir 1ro el tipo en la variable que estemos trabajando
- Ejecturar `.collect()` al final de nuestra iteración
