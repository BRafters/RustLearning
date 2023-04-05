fn main() {
    // Ownership is what makes the safty guarantees possible
    
    // There are three rules to ownership:
    // 1. Each value has an owner - There is no value in memory that doesn't have a variable that owns it
    // 2. Only one owner - No variables may share ownership
    // 3. Value gets dropped if its owner goes out of scope
    let mut s1 = String::from("abc");

    // To make a copy of s1, we can use the .clone() method
    // Rust reserves the term 'copy' for when only stack data is being copied
    // .clone() is used if there is heap data and pointer updates involved
    let s2 = s1.clone(); 

    do_stuff(&mut s1);

    println!("{}", s1); // The value for s1 is moved to s2 because only one variable can own a value
}

// If we pass s1 to this function, s1 is moved into the local variable 's' in do_stuff()
// Passing ownership of a value to a function usually means a function is going to consume the passed in value
// Here, it would be a good idea to use references
fn do_stuff(s: &mut String) {
    // '&' indicates a reference to a type
    // do_stuff() borrows a reference to the value
    //// The dot operator for a method or a field auto-dereferences down to the actual value
    //// (*s) for manually dereferencing a reference
    s.insert_str(0, "Hi, ");
}
