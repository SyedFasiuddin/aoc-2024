use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[allow(dead_code)]
fn day1a(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut distance = 0;
    for (l, r) in left.into_iter().zip(right) {
        distance += r.abs_diff(l);
    }
    distance
}

#[allow(dead_code)]
fn day1b(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut right_hash: HashMap<u32, u32> = Default::default();
    for r in right {
        right_hash.entry(r).and_modify(|entry| *entry += 1).or_insert(1);
    }

    let mut similarity = 0;
    for l in left {
        if let Some(val) = right_hash.get(&l) {
            similarity += l * val;
        }
    }
    similarity
}

#[allow(dead_code)]
fn day1(input: &str) {
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

    // println!("Part A, distance: {}", day1a(left, right));
    println!("Part B, similarity: {}", day1b(left, right));
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/1.txt").unwrap().read_to_string(&mut str);

    day1(&str);
}
