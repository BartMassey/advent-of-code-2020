// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 3.  
//! Bart Massey 2020

use aoc::*;

type Map = Vec<Vec<bool>>;

fn read_map() -> Map {
    let mut map = Vec::new();
    for line in input_lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => row.push(true),
                '.' => row.push(false),
                '\n' => (),
                _ => panic!("unexpected char"),
            }
        }
        map.push(row);
    }
    map
}

#[allow(clippy::ptr_arg)]
fn count_trees(map: &Map, right: usize, down: usize) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut col = 0;
    let mut ntrees = 0;
    for row in (0..height).step_by(down) {
        ntrees += usize::from(map[row][col]);
        col = (col + right) % width;
    }
    ntrees
}

fn main() {
    let map = read_map();
    match get_part() {
        Part1 => {
            let ntrees = count_trees(&map, 3, 1);
            println!("{}", ntrees);
        }
        Part2 => {
            let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
            let tree_mult: usize = slopes
                .into_iter()
                .map(|(down, right)| count_trees(&map, down, right))
                .product();
            println!("{}", tree_mult);
        }
    }
}
