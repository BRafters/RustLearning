fn main() {
    // Variables must be initialized before you can use them
    let enigma: i32;

    if true {
        enigma = 42;
    }
    // Still wont work, not guaranteed that enigma will be initialized
    else {
        enigma = 5;
    } // Will work, guaranteed that will enigma will be initialized

    // As long as the compiler can guarantee something is safe, it will let you do it
    println!("{}", enigma);
}
