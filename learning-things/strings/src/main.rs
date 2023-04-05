fn main() {
    // There are at least six types of strings in the Rust standard library
    // We mostly care about two of them that overlap eachother
    
    //// String slice - you will almost always see it as a borrowed string slice &str
    // A borrowed string slice is often referred to as a string
    let msg = "Hello World"; // A literal string is always a borrowed string slice

    // A borrowed string slice is internally made up of a pointer to some bytes and a length
    // &str:
    // ptr -> | a | b |
    // len = 4

    // A borrowed string slice is a subset of a string which is why they share a lot of characteristics

    // The data in a borrowed string slice cannot be modified while the data in a String can be modified
    // You will often create a string by calling the to_string() method on a borrowed string slice
    // Or by passing a borrowed string slice to String::from
    let new_msg = "ab".to_string();

    //// String (with a capital S)
    
    // A String is made up of a pointer to some bytes, a length, and a capacity that may be higher that what is currently being used
    // String:
    // ptr -> | a | b |  |  |
    // len = 4
    // capacity = 8

    // Strings cannot be indexed by character position ie: my_string[3], Strings are unicode
    // .chars() can be used to retrieve an iterator that you can use to iterate through the unicode scalars
    // these iterators have a method called .nth() athat you can use in place of indexing

}
