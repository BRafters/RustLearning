#![allow(unused_variables)]

use std::collections::hash_map::HashMap;

fn main() {
    println!("Vectors");
    vectors();

    println!();

    println!("HashMaps");
    hash_map();
}

fn vectors() {
    // Vector is a generic collection that holds a bunch of one type
    // Vec<T>
    let mut v: Vec<i32> = Vec::new();

    // Macro vec!
    let mut y: Vec<i32> = vec![2,4,6];

    // Once you have the vector, you can push values to it
    // Vectors act like a stack
    // Push appends to end, pop removes the item at the end and returns it
    v.push(2);
    v.push(3);
    v.push(6);

    let x = v.pop(); // x = 6

    println!("{}", v[1]); // 3
    println!("{}", v[0]);
    println!("{:?}", y);
}

fn hash_map(){
    // HashMap is a generic collection of K-V pairs
    //                 K   V
    let mut h: HashMap<u8, bool> = HashMap::new();

    h.insert(5, true);
    h.insert(6, false);

    let five = h.remove(&5).unwrap();
}