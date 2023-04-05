// The absolute path is the library name, which is the same as the name of your project in Cargo.toml
// Followed by the scope operator '::'
// Then the name of the function ie: greet
use hello::greet; // Since the use statement is outside of any smaller scope, this brings greet() into scope for all of main
use rand::{Rng, thread_rng};

// The standard library is always available by default 

fn main() {
    // Let for declaring vars
    // Strongly typed lang
    let bunnies = 2; // i32 is a default
    let bunnies_64: i64 = 5; // Variables are immutable by default

    println!("{}, {}", bunnies, bunnies_64);

    // 'let' can destructure data on the right hand side
    // Initializes variables inside of a corresponding pattern on the left side
    let (bunnies, carrots) = (8, 50);

    // Type annotations are required for constants
    // The value must be a constant expression that can
    // be determined at compile time
    // Are global variables, can use it anywhere you want
    // Really fast
    const WARP_FACTOR: f32 = 9.09;

    greet(); // Can omit the library name and just use the function name
    println!("{}", generate_random());
}

fn generate_random() -> i8 {
    let x: i8 = thread_rng().gen_range(0, 100);
    x
}