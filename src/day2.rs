use std::fs::File;
use std::io::Read;

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/2.txt").unwrap().read_to_string(&mut str);

    day2a_another_approach(&str);
}

#[allow(dead_code)]
fn day2a(input: &str) {
    let mut num_safe = 0;
    'outer: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|v| {
                v.parse::<i32>()
                    .expect("ERROR: unable to convert str to usize")
            })
            .collect::<Vec<_>>();

        let diff = levels[0] - levels[1];
        if diff > 0 {
            // All increasing
            for w in levels.windows(2) {
                match &w[1] - &w[0] {
                    -1 | -2 | -3 => continue,
                    _ => continue 'outer,
                }
            }
            num_safe += 1;
        } else {
            // All decreasing
            for w in levels.windows(2) {
                match &w[1] - &w[0] {
                    1 | 2 | 3 => continue,
                    _ => continue 'outer,
                }
            }
            num_safe += 1;
        }
    }
    println!("safe: {:?}", num_safe);
}

#[allow(dead_code)]
fn day2b(input: &str) {
    let mut num_safe = 0;
    'outer: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|v| {
                v.parse::<i32>()
                    .expect("ERROR: unable to convert str to usize")
            })
            .collect::<Vec<_>>();

        let mut damped = false;
        let diff = levels[0] - levels[1];
        if diff > 0 {
            // All increasing
            for w in levels.windows(2) {
                match &w[1] - &w[0] {
                    -1 | -2 | -3 => continue,
                    _ => {
                        if !damped {
                            damped = true
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }
            num_safe += 1;
        } else {
            // All decreasing
            for w in levels.windows(2) {
                match &w[1] - &w[0] {
                    1 | 2 | 3 => continue,
                    _ => {
                        if !damped {
                            damped = true
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }
            num_safe += 1;
        }
    }
    println!("safe: {:?}", num_safe);
}

#[allow(dead_code)]
fn day2a_another_approach(input: &str) {
    let mut num_safe = 0;
    for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|v| {
                v.parse::<i32>()
                    .expect("ERROR: unable to convert str to usize")
            })
            .collect::<Vec<_>>();

        let diffs = levels
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        // Can't use `.all` before doing `.collect`
        if diffs.iter().all(|&val| val == -1 || val == -2 || val == -3) {
            num_safe += 1;
        } else if diffs.iter().all(|&val| val == 1 || val == 2 || val == 3) {
            num_safe += 1;
        }
    }
    println!("safe: {:?}", num_safe);
}
