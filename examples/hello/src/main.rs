use hello::{english, russian, spanish};

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
