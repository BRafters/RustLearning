const STARTING_MISSILES: u8 = 8;
const READY_AMOUNT: u8 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles.", ready, missiles);
    println!("{} missile(s) left.", missiles - ready);
}
