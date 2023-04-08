fn main() {
    // Compound types gather multiple values of other types into one type
    // Tuples store multiple values of any type
    let info = (1, 3.3, 999);

    // There are two ways to access members of a tuple
    // Dot syntax
    let cars = info.0; // "Field access expression"

    // You can use a pattern to destructure and access all the elements of a tuple
    // Tuples have a maximum arity of twelve (at least with full functionality)
    // Arity = how many items are in the tuple
    let (cars, fuel, km) = info; // Arity of 3

    // An array
    let arr_one = [1,2,3];

    // Another array            
    let arr_two = [0; 3]; // <- How many
    //                       ^ val
}
