use std::fs::File;
use std::io::Read;

fn parta(input: &str) {
    println!("{input}");
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/5.txt").unwrap().read_to_string(&mut str);

    parta(&str);
}
