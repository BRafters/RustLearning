fn main() {
    let x = 5 + 5;
    println!("{}", x);

    let y = "BRUH";
    let z = "BLEGH";

    println!("{0}: {1}", y, z);

    // Interpolating strings by named arguments
    println!("{urmum}", urmum=1);

    // Right justify width
    // 5 spaces before num char
    println!("{0:>5}", 1);
}