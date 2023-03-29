fn main() {
    let x = do_stuff(2, 12);
    println!("{}", x);
}

// Snake case
//         parameter list
//                           return type
// Functions don't support varargs
// Macros however do support varargs
fn do_stuff(qty: u32, oz: u32) -> u32 { 
    // return 45; Can use return for returning a val
    qty * oz // Shorthand for return. This is a "tail expression"
}