
use hello::greet;
// Use brings an item from some path into some scope 
use lib::greet;

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

    greet();
}
