use regex::Regex;
use std::fs::File;
use std::io::Read;

fn part_a(input: &str) {
    let re =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("ERROR: Unable to compile regex");
    let res: usize = re
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_, [a, b])| {
            let a = a
                .parse::<usize>()
                .expect("ERROR: Unable to parse `a` into usize");
            let b = b
                .parse::<usize>()
                .expect("ERROR: Unable to parse `b` into usize");
            a * b
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("Sum of Mul: {}", res);
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/3.txt").unwrap().read_to_string(&mut str);

    part_a(&str);
}
