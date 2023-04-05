fn main() {
    // Rust takes the composition over inheritance approach
    print_noise(RedFox::new());

    // There is a special trait called 'Copy' 
    // If your type implements Copy, then it will be copied instead of moved in move situations
}

struct RedFox {
    enemy: bool,
    life: u32,
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}

// Traits define required behavior - functions and methods that a struct must implement if it wants to have that trait
trait Noisy {
    // trait specifies that the struct must have a method named get_noise() that returns a borrowed string slice
    fn get_noise(&self) -> &str;
}

// Adding an implementation of the noisy trait to redfox
impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

// Once we have a trait involved, we can start writing generic functions that accept any value that implements the trait
fn print_noise<T: Noisy>(item: T) {
    // item: T, which is defined to be anything that implements the Noisy trait
    println!("{}", item.get_noise()); // The function can use any behavior on item that the Noisy trait defines
}

// As long as one of either the trait or the struct is defined in your project, you can implement any trait for any struct
