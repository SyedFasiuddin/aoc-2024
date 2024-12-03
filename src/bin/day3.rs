use regex::Regex;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn part_b(input: &str) {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)")
        .expect("ERROR: Unable to compile regex");
    let mul_re =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("ERROR: Unable to compile regex");

    let mut res = 0;
    let mut state = true;
    for (pat, _) in re.captures_iter(input).map(|c| c.extract::<0>()) {
        match pat {
            "don't()" => state = false,
            "do()" => state = true,
            mul if state => {
                res += mul_re
                    .captures_iter(mul)
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
                    .next() // This is because we know there will be only one match
                    .unwrap_or_default()
            }
            _ => (),
        }
    }

    println!("Sum of Mul, conditional: {}", res);
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/3.txt").unwrap().read_to_string(&mut str);

    part_b(&str);
}
