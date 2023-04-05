use std::fmt;

fn printMathStuffs() {
    // Integer addition
    println!("1 + 2 = {}", 1u8 + 2u8);

    // Integer subtraction
    println!("1 - 2 = {}", 1i8 - 2i8);

    // Use underscores for readability
    println!("One thousand => {}", 1_000);
}

// Tuples can be used as function args and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // Tuple is a collection of values of different types and are constructed using ()

    // 'let' can be used to bind members of a tublle to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(matrix: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let (i_one, i_two, i_three, i_four) = matrix;

    (i_one, i_four, i_three, i_two)
}

fn playWithTuples() {
    let short_tuple = (1u8, false, 3i8, true);

    // Values can be extracted from the tuple using tuple indexing
    println!("Tuple first value: {}", short_tuple.0);
    println!("Tuple fourth value: {}", short_tuple.3);

    // Tuples can be tuple members
    let tuple_of_tuples = ((5u32,));

    // Tuples are printable
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("The reversed pair is: {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    println!("{:?}", (5u8,));

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}

struct Matrix(f32, f32, f32, f32);

// Need to manually implement fmt::Display for the type
impl fmt::Display for Matrix {
     // Trait requires fmt with this signature
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i: u16 = 0;
        // Write strictly the first element into the supplied output stream 'f'
        // Returns fmt::Result which indicates whether the operation succeeded or failed
        write!(f, "({}, {}\n{}, {})", self.0, self.1, self.2, self.3)
     }
}

fn doActivity() {
    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));

    // Transpose 
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    let mut tuple = (matrix.0, matrix.1, matrix.2, matrix.3);

    tuple = transpose(tuple);

    println!("{:?}", tuple);
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
    playWithTuples();
    doActivity();
}
