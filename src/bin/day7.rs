use std::fs::File;
use std::io::Read;

/// Generated all permutations of the given vector `of`
#[allow(dead_code)]
fn permutation<T: std::clone::Clone>(ret: &mut Vec<Vec<T>>, of: &mut [T], idx: usize) {
    if idx == of.len() - 1 {
        ret.push(of.to_vec());
        return;
    }
    for j in idx..of.len() {
        of.swap(idx, j);
        permutation(ret, of, idx + 1);
        of.swap(idx, j);
    }
}

/// Generate permutations of arbitrary length of '+', '*' and '|'
fn permutation2(
    ret: &mut Vec<Vec<char>>,
    arr: &mut Vec<char>,
    curr_depth: usize,
    max_depth: usize,
    partb: bool,
) {
    if curr_depth >= max_depth {
        ret.push(arr.to_vec());
        return;
    }

    arr.push('+');
    permutation2(ret, arr, curr_depth + 1, max_depth, partb);
    arr.pop().unwrap();

    arr.push('*');
    permutation2(ret, arr, curr_depth + 1, max_depth, partb);
    arr.pop().unwrap();

    if partb {
        arr.push('|');
        permutation2(ret, arr, curr_depth + 1, max_depth, partb);
        arr.pop().unwrap();
    }
}

// https://github.com/maciekbartczak/aoc-2024/blob/main/src/bin/07.rs
fn solve(input: &str, partb: bool) {
    let input = input
        .lines()
        .map(|line| {
            let (test_val, values) = line.split_once(':').expect("Expected a ':'");
            let test_val = test_val
                .parse::<usize>()
                .expect("Expected the test value to be `usize`");
            let values = values
                .split_whitespace()
                .map(|v| {
                    v.parse::<usize>()
                        .expect("Expected expression values to be `usize`")
                })
                .collect::<Vec<_>>();
            (test_val, values)
        })
        .collect::<Vec<_>>();

    let mut calibration_res = 0;
    for (test_val, values) in input {
        if values.len() <= 1 {
            continue;
        }

        let mut perms: Vec<Vec<char>> = Default::default();
        let mut arr: Vec<char> = Default::default();

        permutation2(&mut perms, &mut arr, 0, values.len() - 1, partb);

        let mut adds_up = false;
        for perm in perms {
            let mut v = values.clone();
            v.reverse();

            let mut running_total = v.pop().unwrap();
            for ops in perm {
                if ops == '+' {
                    running_total += v.pop().unwrap();
                } else if ops == '*' {
                    running_total *= v.pop().unwrap();
                } else if ops == '|' {
                    let val = v.pop().unwrap();
                    let val = format!("{}{}", running_total.to_string(), val.to_string());
                    running_total = val.parse().unwrap();
                }
            }

            if test_val == running_total {
                adds_up = true;
                break;
            }
        }

        if adds_up {
            calibration_res += test_val;
        }
    }

    println!("{calibration_res}");
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/7.txt").unwrap().read_to_string(&mut str);

    solve(&str, true);
}
