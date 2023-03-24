fn printMathStuffs() {
    // Integer addition
    println!("1 + 2 = {}", 1u8 + 2u8);

    // Integer subtraction
    println!("1 - 2 = {}", 1i8 - 2i8);

    // Use underscores for readability
    println!("One thousand => {}", 1_000);
}

fn printTuples() {
    
}

fn main() {
    // Rust provides access to a wide variety of primitives

    // Scalar types:
    // Signed ints: i8, i16, i32, i64, i128, isize (for pointer size)
    // Unsigned ints :u8, u16, u32, u64, u128, usize(for pointer size)
    // char (4 bytes each)
    // bool
    // the unit type (), whose only possible val is an empty tuple

    // Compound types:
    // Arrays
    // Tuples

    // Can be type annotated
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular notation
    let a_int = 1i32; // Suffix notation

    // Or a default will be used
    let x = 24; // i32
    let y = 2.4; // f64

    // Type can be inferred by context
    // A mutable variables value can be changed
    let mut inferred_int = 12; // i64 is inferred from another line
    inferred_int = 2342473948724i64;

    // The type of a variable cant be changed

    // Variables can be overwritten with shadowing
    let x = "tree";

    // Underscores can be inserted in numeric literals to improve readability
    // 1_000 => 1,000
    
    printMathStuffs();
}
