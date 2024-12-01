use std::fs::File;
use std::io::Read;

fn day1a(input: &str) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input.lines() {
        let v = line
            .split_whitespace()
            .map(|x| x.parse().expect("Unable to convert str to u32"))
            .collect::<Vec<u32>>();
        left.push(v[0]);
        right.push(v[1]);
    }
    left.sort();
    right.sort();
    let mut distance = 0;
    for (l, r) in left.into_iter().zip(right) {
        distance += r.abs_diff(l);
    }
    println!("distance: {}", distance);
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/1.txt").unwrap().read_to_string(&mut str);

    day1a(&str);
}
