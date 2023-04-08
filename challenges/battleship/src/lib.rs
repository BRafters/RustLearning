use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GAME_MAP: Mutex<HashMap<char, [u8; 10]>> = Mutex::new(HashMap::new());
}

static Y_AXIS: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

pub fn init() {
    // Create the map
    load_map(Coords::None);

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

    println!("{:?}", map);
}

// Creates the 10/10 grid
pub fn load_map(coord: Coords) {
    for i in 1..=10 {
        print!("{:>3}", i);
    }

    println!();

    // load in the battleship data as we are printing in the letters
    for letter in Y_AXIS {
        println!("{}", letter);
    }
}

fn place() {}

pub enum Coords {
    Coord((char, u8)),
    None,
}
