# Rust Walkthrough
from https://github.com/emandres/rust-tutorial

Rust is a systems programming language. As such, use cases for it typically fall in the same area
that languages such as C and C++ would typically be used. 
Rust, however, is a modern programming language with extensive safety features built into the language.

One of Rust's most distinctive features involves memory safety. Rust introduces the concept of data ownership.
Data can then be lent, or "borrowed" by another scope, either mutably or immutably. The borrow checker is a
compile-time step that ensures that ownership rules are enforced. This eliminates a whole class of errors
that are all too easy to make in unsafe languages like C.

While Rust is a general purpose language, it has implemented several features typical of functional languages.
Usage of the Maybe (Option) and Either (Result) monads are built into the standard library. 
The type system is static and strong. There is no inheritance; instead shared functionality is handled through 
traits (similar to interfaces in OO languages). There is support for several common types found in languages descended from ML
such as discriminated unions, tuples, and records (called structs in Rust).

In many functional languages, immutability is a priority in terms of language design. Rust takes a slightly different approach.
Instead of forbidding mutability (or making it very difficult), Rust makes mutability explicit and opt-in. 

## Chapter 1 - Basics - functions, module, unit tests

In `src/lib.rs`:

```rust
pub mod chapter_1;
```

In `src/chapter_1.rs`

```rust
pub fn meaning_of_life() -> i32 { // 2
    let x = 42; // 2
    x // 3
}

#[cfg(test)] // 4
mod tests {
    use super::*;

    #[test]
    fn test_the_meaning_of_life() {
        assert_eq!(42, meaning_of_life());
    }
}
```

1. Function signature
2. Let binding
3. Return expression
4. Unit test module

## Chapter 2 - Option\<T\>

Option is a type-safe way around null references, which are almost completely absent from Rust (they do exist in unsafe Rust, but play a niche role).

In `src/lib.rs`:

```rust
pub mod chapter_2;
```

In `src/chapter_2.rs`

```rust
pub fn divide_safely(a: i32, b: i32) -> Option<i32> {
    if b == 0 { // 1
        None
    } else {
        Some(a / b)
    }
}

pub fn divide_with_no_remorse(a: i32, b: i32) -> i32 {
    divide_safely(a, b).unwrap() // 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_safely() {
        assert_eq!(divide_safely(4, 2), Some(2));
        assert_eq!(divide_safely(8, 3), Some(2));
        assert_eq!(divide_safely(4, 0), None);
    }

    #[test]
    fn test_divide_with_no_remorse_successfully() {
        assert_eq!(divide_with_no_remorse(4, 2), 2);
        assert_eq!(divide_with_no_remorse(8, 3), 2);
    }

    #[test]
    #[should_panic]
    fn test_divide_with_no_remorse_unsuccessfully() {
        divide_with_no_remorse(4, 0);
    }
}
```

`Option<T>` can be defined like this:

```rust
enum Option<T> {
    Some(T),
    None
}
```

## Chapter 3 - Results 

The Result type partially replaces the exception throw pattern available in many languages.
Instead of stopping execution and bubbling up to the nearest `catch`, Rust has a built in type 
that for expressing the failure of an operation. There are also some language features that make working with
results a bit more ergonomic.

The `Result` type is an implementation of the `Either` monad. It has two possible states: `Ok(T)` or `Err(E)`.
It is very common in Rust code to see Result aliased to remove the need to type the error type `E` everywhere it is used. 
For instance in the following examples, anywhere that `Result<Value>` is used, it actually resolves to `Result<Value, serde_json::Error>`.

In `Cargo.toml`

```toml
[dependencies]
serde_json = "1.0.0"
```

In `src/lib.rs`:

```rust
extern crate serde_json;
pub mod chapter_3
```

In `src/chapter_3.rs`

```rust
use serde_json::{self, Value, Result};

pub fn parse_input_to_json_value(input: &str) -> Result<Value> {
    serde_json::from_str(input)
}

pub fn get_meaning_of_life(input: &str) -> Result<i64> {
    match parse_input_to_json_value(input) {
        Ok(json) => Ok(json["meaningOfLife"].as_i64().unwrap()),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_json() {
        assert!(parse_input_to_json_value("42").is_ok())
    }

    #[test]
    fn parse_invalid_json() {
        assert!(parse_input_to_json_value("'asdf'").is_err());
    }

    #[test]
    fn test_get_meaning_of_life() {
        match get_meaning_of_life(r#"{"meaningOfLife": 42}"#) {
            Ok(result) => assert_eq!(result, 42),
            Err(_) => assert!(false)
        }
    }
}

// let json = parse_input_to_json_value(input)?;
// Ok(json["meaningOfLife"].as_i64().unwrap())
```

1. The `?` operator
2. Matching on Result

## Chapter 4 - (Im)mutability

Values in Rust are immutable by default. The compiler will only allow mutation if a `let` binding contains the `mut` keyword.

```rust
pub fn does_not_compile() -> i32 {
    let x = 42;
    x = 19;
    x
}
```


```rust
pub fn does_compile() -> i32 {
    let mut x = 42;
    x = 19;
    x
}
```

## Chapter 5 - The Stack and the Heap

There is no garbage collection in Rust, consistent with it's position as a systems programming language.
As such, it is often necessary to think about where your data lives.

```rust
fn smart_pointers() {
    let _a = 42; //allocated on stack

    let _b = Box::new(43); //a smart pointer to a location on the heap

    let _c = Vec::<i32>::new(); //a smart pointer to an array on the heap

    let _d = Rc::new(44); // a reference counting smart pointer

    let _e = Arc::new(45); // an atomic reference counting smart pointer (thread safe)
}
```

Rust follows the C++ paradigm of RAII (Resource allocation is initialization). 
The owning scope of data will automatically deallocate it when the scope ends.
To customize this behavior, the `Drop` trait may be implemented.

## Chapter 6 - Ownership and Borrowing

In order to share data without transfering ownership we can use references.
Normal references (denoted `&x`) are immutable, i.e. they're a readonly view into the state of the object.

```rust
pub fn does_not_compile() {
    let jaunty_phrase =  "Everything is awesome".to_string();

    print_jaunty_phrase(jaunty_phrase);

    println!("\"{}\" is my favorite phrase", jaunty_phrase);
}

fn print_jaunty_phrase(phrase: String) {
    println!("{}", phrase);
}
```

```rust
pub fn does_compile() {
    let jaunty_phrase =  "Everything is awesome".to_string();

    print_jaunty_phrase(&jaunty_phrase);

    println!("\"{}\" is my favorite phrase", jaunty_phrase);
}

fn print_jaunty_phrase(phrase: &String) {
    println!("{}", phrase);
}
```

```rust
pub fn multiple_references() {
    let s = "woohoo!".to_string();
    let a = vec![&s];
    let b = vec![&s];

    print_vec(a);
    print_vec(b);
}

fn print_vec(v: Vec<&String>) {
    println!("{:?}", v);
}
```

```rust
pub fn single_mutable_reference() {
    let s = "woohoo!".to_string();
    let a = vec![&mut s];
    let b = vec![&mut s];

    print_vec_2(a);
    print_vec_2(b);
}

fn print_vec_2(v: Vec<&mut String>) {
    println!("{:?}", v);
}
```

## Chapter 7 - Structs and Enums

```rust
struct Walrus {
    pub name: String,
    gender: Gender,
    stomach: Vec<Food>
}

enum Gender {
    Male,
    Female
}

struct Food(String); // a tuple-struct

struct Empty; // a unit-type struct

impl Walrus {
    pub fn eat(&mut self, food: Food) {
        self.stomach.push(food);
    }
}
```

## Chapter 8 - Polymorphism through Traits

```rust
struct Cat;
struct Dog;
struct Human;

trait Speak {
    fn speak(message: String) {
        println!("{}", message);
    }
}

impl Speak for Cat {
    fn speak(_message: String) {
        println!("Meow");
    }
}

impl Speak for Dog {
    fn speak(_message: String) {
        println!("Woof");
    }
}
```
