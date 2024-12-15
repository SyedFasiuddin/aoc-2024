use regex::Regex;
use std::fs::File;
use std::io::Read;

fn parta(input: &str) {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("ERROR: Unable to compile regex");

    let mut sum = 0.0;

    for [a1, b1, a2, b2, c1, c2] in
        re.captures_iter(input)
            .map(|c| c.extract::<6>())
            .map(|(_, [a1, b1, a2, b2, c1, c2])| {
                let a1 = a1
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `a1` into usize");
                let b1 = b1
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `b1` into usize");
                let a2 = a2
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `a2` into usize");
                let b2 = b2
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `b2` into usize");
                let c1 = c1
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `c1` into usize");
                let c2 = c2
                    .parse::<f32>()
                    .expect("ERROR: Unable to parse `c2` into usize");
                [a1, b1, a2, b2, c1, c2]
            })
    {
        // https://stackoverflow.com/questions/4383959/
        // https://en.wikipedia.org/wiki/Cramer%27s_rule
        let x: f32 = ((c1 * b2 - a2 * c2) / (a1 * b2 - b1 * a2)) as f32;
        let y: f32 = ((a1 * c2 - c1 * b1) / (a1 * b2 - b1 * a2)) as f32;

        if x > 100.0 || y > 100.0 {
            continue;
        }
        // solution for x & y should be whole numbers only
        if x.fract() != 0.0 && y.fract() != 0.0 {
            continue;
        }
        sum += (x * 3.0) + (y * 1.0);
    }
    println!("Sum: {sum}");
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/13.txt")
        .unwrap()
        .read_to_string(&mut str);

    parta(&str);
}
