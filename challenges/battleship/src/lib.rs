#![allow(unused_variables)]

use regex::Regex;
use core::num;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Mutex;

// A good way to check if there are battleship hits left is to count how many there are

lazy_static::lazy_static! {
    static ref GAME_MAP: Mutex<HashMap<char, [u8; 10]>> = Mutex::new(HashMap::new());
    static ref PLOT_MAP: Mutex<HashMap<char, [char; 10]>> = Mutex::new(HashMap::new());

    static ref RGX_NUM: Regex = Regex::new(r#"[A-Z]"#).unwrap();
    static ref RGX_LETTER: Regex = Regex::new(r#"[\d]+"#).unwrap();
    static ref RGX_INVALID_CHARS: Regex = Regex::new(r"^[A-J]([\d]{1,2})$").unwrap();
}

static Y_AXIS: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

pub fn init() {
    // Load the game map with the txt data
    let file: File = File::open("src/resources/map.txt").unwrap();
    let mut buffer_reader = BufReader::new(file);
    let mut content: String = String::new();

    buffer_reader
        .read_to_string(&mut content)
        .expect("Could not read file");

    // println!("{}", content);

    let mut map = GAME_MAP.lock().unwrap();
    let mut arr: [u8; 10] = [0; 10];
    let mut i = 0;
    let mut j = 0;

    // TODO - For every tenth character, we set j back to 0 and bump up i by one
    // Read every character
    for ch in content.chars() {
        let int_ch: u8 = match ch {
            '0' => 0_u8,
            '1' => 1_u8,
            _ => continue,
        };

        // Do the check before we insert into map
        if j == arr.len() {
            map.insert(Y_AXIS[i], arr);
            i += 1;
            j = 0;
        } else {
            arr[j] = int_ch;
        }

        j += 1;
    }

    let mut plot_map = PLOT_MAP.lock().unwrap();

    // Init the plot map
    for letter in Y_AXIS {
        plot_map.insert(letter, ['-'; 10]);
    }
}

pub fn play() {
    println!("Rules of play: ");
    println!("1: You can quit at any point the games prompts the input by entering 'Q'");
    println!("2: To fire, choose spot you want to shoot at ie: G10");

    loop {
        let line: String = String::new();
        shoot();
        show_map();
    }
}

fn shoot() -> String {
    let mut line: String = String::new();
    let mut formatted_str: String;

    println!("Where would you like to shoot?");

    // If the input is invalid, we ask again
    loop {
        std::io::stdin()
            .read_line(&mut line)
            .expect("Could not get input");

        formatted_str = line.replace(" ", "").to_uppercase();

        // Remove whitespace and validate the input
        if is_valid_coord(&formatted_str) {
            return formatted_str;
        }
        
        println!("Invalid input, please try again");
    }
}

fn str_to_coord(string_coord: String) -> (char, u8) {
    // Split will return what hasn't matched
    let num: String = RGX_NUM.split(&string_coord).collect();
    let letter: String = RGX_LETTER.split(&string_coord).collect();

    // Get the first char, and the u8 and put them in the tuple
    (letter.chars().nth(0).unwrap(), num.parse::<u8>().unwrap())
}

// Creates the 10/10 grid
pub fn show_map() {
    let plot_map = PLOT_MAP.lock().unwrap();

    for i in 0..Y_AXIS.len() {
        print!(" {} ", i + 1);
    }

    println!();

    for ch in Y_AXIS {
        print!("{}", ch);
        if plot_map.get(&ch).is_some() {
            for plot in plot_map.get(&ch).unwrap() {
                print!(" {} ", plot);
            }
            println!();
        }
    }
}

pub fn place(coord: &Coords) {
    let point = get_point(coord);
    let x_point: usize = x_to_element(point.1) as usize;

    let mut map = PLOT_MAP.lock().unwrap();

    // First, plot on the map
    if let Some(values) = map.get_mut(&point.0) {
        values[x_point] = hit_marker(coord);
    }
}

fn hit_marker(coord: &Coords) -> char {
    let point = get_point(coord);

    // Check if the coord is a hit or not
    let map = GAME_MAP.lock().unwrap();

    // Grab the arr out of the map
    let arr = map.get(&point.0).unwrap();
    let x_point = x_to_element(point.1) as usize;

    if arr[x_point] == 1 {
        println!("Hit!");
        'x'
    } else {
        println!("Miss!");
        'o'
    }
}

fn get_point(coord: &Coords) -> (char, u8) {
    let point: (char, u8) = match coord {
        Coords::Coord((y_coord, x_coord)) => (*y_coord, *x_coord),
        Coords::None => ('\0', u8::MAX),
    };

    point
}

fn x_to_element(x: u8) -> u8 {
    // Determine if the x point is 0 or not
    if x == 0 {
        0
    } else {
        x - 1 // To adjust for array indicies
    }
}

pub enum Coords {
    Coord((char, u8)),
    None,
}

////
// Other non map related utilities
////
fn is_valid_coord(user_input: &String) -> bool {
    // If the input contains special chars, just say no
    if !RGX_INVALID_CHARS.is_match(user_input) {
        return false;
    }

    // Pull out the numeric value
    let num_str: String = RGX_NUM.split(user_input).collect();
    
    if num_str.is_empty() || !num_str.chars().all(char::is_numeric) {
        return false;
    }

    let num: u8 = num_str.parse::<u8>().expect("Could not parse numeric str to u8");

    if num > 10 {
        return false;
    }

    true
}

////
// Tests
////
#[test]
fn test_str_to_coord() {
    let str: String = String::from("G10");
    let expected_coord: (char, u8) = ('G', 10);

    assert_eq!(str_to_coord(str), expected_coord);
}

#[test]
fn test_valid_string() {
    let str: String = String::from("F10");

    assert!(is_valid_coord(&str));
}

#[test]
fn test_one_num() {
    let str: String = String::from("F1");

    assert!(is_valid_coord(&str));
}

#[test]
fn test_three_nums() {
    let str: String = String::from("F100");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_no_nums() {
    let str: String = String::from("F");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_num_out_of_range() {
    let str: String = String::from("F12");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_two_chars() {
    let str: String = String::from("FF1");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_no_chars() {
    let str: String = String::from("1");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_chars_empty() {
    let str: String = String::from("");

    assert!(!is_valid_coord(&str));
}

#[test]
fn test_two_alphas() {
    let str: String = String::from("FF");
    assert!(!is_valid_coord(&str));
}

#[test]
fn test_past_j() {
    let str: String = String::from("K1");

    assert!(!is_valid_coord(&str));
}
