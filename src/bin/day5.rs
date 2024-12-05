// https://youtu.be/LA4RiCDPUlI

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
fn partb(input: &str) {
    let (page_ord_rules, update_list) = parse_input(input);
    let mut sum_mid = 0;
    for mut update in update_list {
        if !follows_print_rules(&update, &page_ord_rules) {
            // Bubble Sort with custom predicate
            loop {
                let mut is_sorted = true;
                for idx in 0..update.len() - 1 {
                    let x = update[idx];
                    let y = update[idx + 1];
                    if page_ord_rules.contains(&(y, x)) {
                        is_sorted = false;
                        update.swap(idx, idx + 1);
                    }
                }
                if is_sorted {
                    break;
                }
            }

            sum_mid += update[update.len() / 2];
        }
    }
    println!("Sum of mid pages: {sum_mid}");
}

fn follows_print_rules(update: &[usize], rules: &Vec<(usize, usize)>) -> bool {
    let mut page_idxes: HashMap<usize, usize> = HashMap::new();
    for (idx, page) in update.iter().enumerate() {
        page_idxes.entry(*page).or_insert(idx);
    }

    for (a, b) in rules {
        if (page_idxes.contains_key(a) && page_idxes.contains_key(b))
            && page_idxes.get(a) > page_idxes.get(b)
        {
            return false;
        }
    }

    // Approach with having rules as HashMap<usize, Vec<usize> (page -> should come before)
    // for (idx, page) in update.iter().enumerate() {
    //     if let Some(page_should_come_before) = page_ord_rules.get(page) {
    //         for next_page in &update[idx+1..] {
    //             if !page_should_come_before.contains(next_page) {
    //                 continue 'next
    //             }
    //         }
    //     }
    // }

    true
}

#[allow(dead_code)]
fn parta(input: &str) {
    let (page_ord_rules, update_list) = parse_input(input);
    let mut sum_mid = 0;
    for update in update_list {
        if follows_print_rules(&update, &page_ord_rules) {
            sum_mid += update[update.len() / 2];
        }
    }
    println!("Sum of mid pages: {sum_mid}");
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (page_ord_rules, updates) = input
        .split_once("\n\n")
        .expect("ERROR: Unable to separate out 'page ordering rules' and 'pages to produce'");

    // My take on this type was to have HashMap<usize, Vec<usize>>
    // page -> should come before these pages
    //
    // This appoach fails in the case where there is a page in `value` (which should come before
    // `key`) which comes after `key` but we don't know that as the `value` page doesn't have a
    // `key` in here. You know what I'm saying
    //
    // Example:
    // 29|13
    //
    // 29,13
    // 13,29
    //
    // HashMap = {
    //     29: [13]
    // }
    //
    // First update: fine, 29 comes before 13
    // Second update: there is no key for 13, what now?
    let page_ord_rules: Vec<(usize, usize)> = page_ord_rules
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once('|')
                .expect("Expected `|` to be present in page rules");
            (
                x.parse::<usize>()
                    .expect("Expected page rules to be `usize`"),
                y.parse::<usize>()
                    .expect("Expected page rules to be `usize`"),
            )
        })
        .collect();

    let update_list = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| {
                    page.parse::<usize>()
                        .expect("Expected page number to be `usize`")
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    (page_ord_rules, update_list)
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/5.txt").unwrap().read_to_string(&mut str);

    parta(&str);
    partb(&str);
}
