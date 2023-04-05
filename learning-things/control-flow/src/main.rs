fn main() {
    // You do not need parentheses around the conditional
    // Everything between the "if" and the opening curly brace is the condition
    let num: i8 = 5;
    let msg: &str;

    // "if" is an expression, not a statement
    // Statements don't return values, expressions do
    msg = if num == 5 {
        "five" // The values get returned as tail expressions
    } else if num == 4 {
        "four" // No semi-colons after these values
    } else {
        "other" // Note: "return" only works in function bodies
    }; // Note: All of the blocks return the same type

    // Braces are required

    // An unconditional loop
    // If the compiler knows a loop is unconditional, there's some pretty good optimizations it can do
    // Unconditional loops need to end eventually "break"
    loop {
        println!("Hello!");
        break;
    }

    // You can annotate the loop you want to break out from of with a label
    //Label
    'bob: loop {
        loop {
            loop {
                // When this code hits the break statment in the inner loop, it will break all the way out of the outermost loop
                break 'bob; // You can tell break which loop you want to break out of
            }
        }
    }

    let mut i: i8 = 0;
    'bob: loop {
        loop {
            loop {
                if i == 5 {
                    break 'bob;
                }

                i += 1;
                // If you give it a label, then it continues the named loop
                continue 'bob; // By itself, it continues the innermost loop
            }
        }
    }

    // Rust's 'for' loop iterates over any iterabble value
    // .iter() iterates over all items in a collection in order
    for num in [7, 8, 9].iter() {
        println!("{}", num);
    }

    // ranges (Start == inclusive, end == exclusive)
    // ..= will make the end inclusive as well
    for num in 0..50 {
        println!("{}", num);
    }

    println!("{}", msg);
}
