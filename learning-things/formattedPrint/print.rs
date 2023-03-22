fn main() {
    let pi = 3.141592;

    // Printing is handled by a series of macros defined in std::fmt

    // {} will be replaced with any args
    println!("{} days", 31);

    // Positional args can be used
    // Int in {} determins which arg will be replaced
    println!("{0} days, {1} hours, {0} minutes", 1, 48);

    // Named arguments
    println!("Name: {name}, Age: {age}", name="Bob", age=49);

    // Different formatting by using ';'
    println!("Base 10: {}", 69420);
    println!("Base 2 (Binary): {:b}", 69420);

    // Can add white space to text with a specified width
    // outputs "    1", 4 spaces and a "1" for a total width of 5
    println!("Right justify: '{number:>5}'", number=1);

    // Can pad numbers with extra zeroes
    // Can left-adjust by flipping the sign
    println!("Padding: {number:0<5}", number=1);
    println!("Padding: {number:0>5}", number=1);

    // Can use named args in the format specifier by appending '$'
    println!("Padding named args: {number:0<width$}", number = 1, width=10);
    println!("Padding named args: {number:0>width$}", number = 1, width=10);

    // '.' for num of decimal places
    println!("PI is roughly {0:>.decimal_places$}", pi, decimal_places=2);
}