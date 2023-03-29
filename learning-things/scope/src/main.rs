fn example() {
    // Variables can also be shadowed 
    // Variables are always local to their scope
    // You can even shadow them in the same scope
    let mut x = 5;
    // Shadowing a mutable variable with an immutable variable
    let x = 10;

    println!("{}", x);

    // You can even shadow a variable to a different type in the same scope
    let meme = "More Cowbell!!!";
    let meme = 24;

    println!("{}", meme);
}

fn main() {
    // Scope of a variable begins where it is created and extends to the end of the block

    let x = 5; // Main block
    {
        // x is shadowed with a new value '99'
        // These 'x' vars are two different variables with different values
        let x: i32 = 100; 
        let y = 99;
        println!("{}, {}", x, y);

        // Once this block ends, the inner x is dropped and the outer x is once again accessible
    } // Nested block, there is no garbage collector, variables are terminated here

    // println!("{}, {}", x, y); // error
    example();
}
