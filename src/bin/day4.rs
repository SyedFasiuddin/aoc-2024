// https://www.geeksforgeeks.org/search-a-word-in-a-2d-grid-of-characters/
// https://leetcode.com/problems/word-search/description/

use std::fs::File;
use std::io::Read;

fn make_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    input
        .lines()
        .for_each(|line| grid.push(line.chars().collect()));
    grid
}

fn idx_valid(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    (i >= 0 && j >= 0) && (i < grid.len() as i32 && j < grid[0].len() as i32)
}

#[allow(dead_code)]
fn parta(input: &str) {
    let grid = make_grid(input);
    let word = ['X', 'M', 'A', 'S'];
    let dir_x: [i8; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_y: [i8; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    let mut num_appeared = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != word[0] {
                continue;
            }

            // Should check in 8 directions
            'next_direction: for dir in 0..8 {
                // i,j are curr idx which is already checked if it is 'X'
                let mut curr_i = i as i32 + dir_x[dir] as i32;
                let mut curr_j = j as i32 + dir_y[dir] as i32;

                // Check 3 more chars in this directions
                for idx in 1..4 {
                    if !idx_valid(curr_i, curr_j, &grid) {
                        continue 'next_direction;
                    }
                    if grid[curr_i as usize][curr_j as usize] != word[idx] {
                        continue 'next_direction;
                    }
                    curr_i += dir_x[dir] as i32;
                    curr_j += dir_y[dir] as i32;
                }
                num_appeared += 1;
            }
        }
    }

    println!("Num of XMAS: {num_appeared}");
}

#[allow(dead_code)]
fn partb(input: &str) {
    let grid = make_grid(input);
    let (m, a, s)  = ('M', 'A', 'S');
    let dir_x: [i8; 4] = [-1, -1, 1, 1];
    let dir_y: [i8; 4] = [-1, 1, -1, 1];

    let mut num_appeared = 0;
    for i in 0..grid.len() {
        'next:  for j in 0..grid[0].len() {
            if grid[i][j] != a {
                continue;
            }

            // Should check in 4 directions
            for dir in 0..4 {
                let curr_i = i as i32 + dir_x[dir] as i32;
                let curr_j = j as i32 + dir_y[dir] as i32;

                if !idx_valid(curr_i, curr_j, &grid) {
                    continue 'next;
                }
                if grid[curr_i as usize][curr_j as usize] != m &&
                    grid[curr_i as usize][curr_j as usize] != s
                {
                    continue 'next;
                }
            }

            // It is an X (all indexes are valid) but is it X-MAS?
            if grid[i-1][j-1] == m && grid[i+1][j+1] == s {
                if grid[i-1][j+1] == m && grid[i+1][j-1] == s {
                    num_appeared += 1;
                }
                if grid[i-1][j+1] == s && grid[i+1][j-1] == m {
                    num_appeared += 1;
                }
            }

            if grid[i-1][j-1] == s && grid[i+1][j+1] == m {
                if grid[i-1][j+1] == m && grid[i+1][j-1] == s {
                    num_appeared += 1;
                }
                if grid[i-1][j+1] == s && grid[i+1][j-1] == m {
                    num_appeared += 1;
                }
            }
        }
    }

    println!("Num of X-MAS: {num_appeared}");
}

fn main() {
    let mut str = String::new();
    let _ = File::open("inputs/4.txt").unwrap().read_to_string(&mut str);

    partb(&str);
}
