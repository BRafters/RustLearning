fn main() {
    // We use closures to spawn threads, or when you want to do some functional programmming with iterators
    // A closure is an anonymous function that can borrow or capture some data from the scope it is nested in
    
    // The syntax is a parameter list between two pipes without type annotations, followed by a block
    // This creates an anonymous function (closure) that you can call later
    // The types of args and return value are all inferred from how you use the arguments and what you return
    // |x: i32, y: i32| {x + y};

    let add = |x, y| {x + y};

    // You can leave the parameter list empty

    let subtract = |x, y| {x - y};

    println!("{}", add(1, 2)); // Returns 3
    println!("{}", subtract(3, 1)); // Returns 2
}

fn closure_borrowing() {
    // A closure will borrow a reference to values in the enclosing scope
    let s = "asdf".to_string();
    
    // Here, we create a string, and a closure that borrows a reference to s
    // This works because the println! macro wants a reference anyway

    // Closures also support move semantics, so we can force the closure to move any varaibles it uses into itself and take ownership of them
    // 's' will become part of the closure and it will live until the closure itself goes out of scope and gets dropped
    // This ensures that the closure has exclusive access to their values
    let f = move || {
        println!("{}", s);
    };

    f();
}
