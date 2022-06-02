const STARTING_MISSILES: i8 = 8;
const READY: i8 = 2;

fn main() {
    let (missiles, ready): (i8, i8) = (STARTING_MISSILES, READY);

    println!("Hello, world!");
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
