# Rust Card Decks

Small project for learning the **Rust Language**.

## Concepts

### Traits

#### Deriving Traits

* In Rust, `traits` can be thought of as **interfaces**.
* In `main.rs`, we derive the `Debug` trait. `Debug` is a trait that requires the type to provide a way to print its contents in a human-readable or developer-friendly form.
* When you use `#[derive(Debug)]`, you're not inheriting behavior from a parent class like in object-oriented languages. Instead, you're automatically implementing a trait for your struct.

### Mutability (and Immutability)

* Variables are `immutable` in Rust by default. Meaning once they are assigned a value, this value can't be changed.
* Example of an `immutable` variable declaration: `let values = ["A", "2", "3"];`
* On the other hand, `mutable` variables allow for such changes. In Rust, this how you declare `mutable` variables: `let mut cards = vec![];`, for instance.

### Macros

* Rust works with the concept of macros. Macris are metaprogramming tools that allow code to generate other code. They are used to reduce boilerplate code by expanding into more complex expressions at compile time.
* For instance, when we do `let mut cards = vec![];` we are using a macro which initializes a `Vec<T>` with the provided elements.

### Ownership

* Rust has a unique ownership model that enforces **memory safety** without needing a garbage collector. Every value in Rust has a single owner at any given time, and when the owner goes out of scope, that value is automatically dropped (deallocated).

#### Passing by reference

* To avoid moving ownership (and thereby making the original value unusable), Rust allows **borrowing** using **references**.
* A `reference` is a way to allow multiple parts of the program to access a value without taking ownership of it.
* **Syntax**
  * **Immutable reference**: `&T`
  * **Mutable reference**: `&mut T`
