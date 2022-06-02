// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn inspect(s: &String) {
    println!("{}", if s.ends_with("s") {"plural" } else {"singular"})
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s")
    }
}

fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        true
    } else {
        false
    }
}

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}
