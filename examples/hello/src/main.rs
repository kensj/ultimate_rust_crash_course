use hello::{english, russian, spanish};

trait Movement {
    fn move(&self) ->{
        println!("I'm moving");
    }
}
trait Fly {

}
impl Movement for Fly;
trait Run {

}
imply Movement for Run;

struct Goose {
    beak_length: i8,
}
impl Fly for Goose;
impl Run for Goose;

struct Pegasus {
    wing_length: i8,
}
impl Fly for Pegasus;
impl Run for Pegasus;

struct Horse {
    mane_length: i8,
}
impl Run for Horse;

fn main() {


    english::greet();
    spanish::greet();
    russian::greet();

    println!("{}", my_function(10, 12_i8));
    println!("{}", my_function(10, 12));
}

fn my_function(a: i8, b: i8) -> i8 {
    let c = b - a + 120;
    c
}
