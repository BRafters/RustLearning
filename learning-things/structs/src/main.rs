use rand::{Rng, thread_rng};

fn main() {
    let another_fox = RedFox::new(); // The scope operator is used to access parts of namespace-like things

    println!("The Red Fox has {} life", another_fox.life);
    another_fox.maneuver();
    another_fox.attack();
}

// Structs can have data fields, methods, and associated functions
// Name of structs are Pascal cased
struct RedFox {
    enemy: bool,
    life: u8,
}

// Typically, you would implement an associated function to use as a constructor to create a struct with default values
// methods and associated functions are defined in an implementation block that is separate from the struct definition
impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    // Methods are also defined in the impl block
    fn maneuver(&self) {
        println!("The red fox moved");
    }

    fn attack(&self) {
        println!("The red fox attacked and done {} damage.", generate_random(50, 15));
    }
}

fn generate_random(high: i128, low: i128) -> i128 {
    thread_rng().gen_range(low..=high)
}


