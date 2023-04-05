fn main() {
    // There are four scalar types
    let i: i8 = 25; // Integers (i32 is default)
    let f: f32 = 22.24; // Floats (f64 default)
    let is_enabled: bool = false; // Booleans
    let c: char = 'b'; // Characters
    
    // Can suffix a literial with the type you want
    let x = 5_u16; // Where underscores help with readability

    // isize is the size of the platforms pointer type and can represent every memory address in the process

    println!("\n{}, {}, {}, {}, {}", i, f, is_enabled, c, x);
}
