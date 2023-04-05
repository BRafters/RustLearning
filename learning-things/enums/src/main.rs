#![allow(dead_code, unused_variables)]

use std::fs::File;

fn main() {
    let my_variable: MyOption<i32> = MyOption::Some(32);

    // Creating a None variant of an Option
    let mut x: Option<i32> = None;

    println!("{:?}", x);

    // If you use Option with a concrete type, then the compiler will infer the type
    x = Some(5);
    println!("{:?}", x);

    // Returns true if x is Some varaint
    println!("{}", x.is_some()); // x.is_some() checks if x is the some variant
    println!("{}", x.is_none()); // x.is_none() checks if x is the none variant

    // Option implements the Into-iterator trait, so you can treat it similar to a vector of 0 or 1 items
    for i in x {
        println!("{}", i);
    }

    // Because enums can represent all sorts of data, you need to use patterns to examine them
    // If you want to check for a single variant, you use 'if let'

    // 'if let' takes a pattern (Some()), if it matches, then the condition is true and the variables 'x' inside the pattern are created
    // if let Some(x) = my_variable {
    //     println!("Value is {}", x);
    // }

    // 'match' is good for more than one variant to check
    // Match expressions require you to write a branch arm for every possible outcome
    let x = match my_variable {
        // Use a variable whos type supports matching, like enums
        MyOption::Some(x) => x * x + 1_i32,
        MyOption::None => 0_i32,
        // _ all by itself is a pattern that matches anything and can be used for a default or anything else branch
        _ => 0_i32,
    };

    println!("{}", x);

    // Result is used whenever something might have a useful result, or might have an error
}

enum DispenserItem {
    Empty,                    // You can always have a named variant with no data
    Ammo(u8),                 // A variant can have a single type of data
    Things(String, i32),      // A variant can have a tuple of data
    Place { x: u32, y: u32 }, // A variant can have an anonymous struct of data
}

// You can also create enums with generics
// The Option enum represents when something is either absent or present
// - You either have some value wrappend in the Some variant, or you have None
enum MyOption<T> {
    Some(T),
    None,
}

// You can implement functions and methods for an enum
impl DispenserItem {
    fn display(&self) {}
}

enum Color {
    Red,
    Green,
    Blue,
}

// Definition of the result enum

// #[must_use] makes it a compiler warning to silently drop a result
// enum Result<T, E> {
//      Ok(T),
//      Err(E),
// }

fn open_file() {
    let res = File::open("foo");

    // Simplest way is to unwrap the Result with the unwrap() method
    // If result is Ok, it gives you the file struct you wanted
    // Else if result is Err, then this crashes the program
    // .expect() is the exact same as unwrap, except that the string you pass is what will be printed to you in case of an exception
    let f = res.expect("Error message");
}
