// allow unused imports for now
#![allow(unused_imports)]
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
}

fn parse_char(input: char) -> Direction {
    match input {
        '(' => Direction::Up,
        ')' => Direction::Down,
        _ => panic!("Invalid input"),
    }
}
fn convert_dir_to_int(dir: Direction) -> i64 {
    match dir {
        Direction::Up => 1,
        Direction::Down => -1,
    }
}

fn dirs_to_int(dirs: Vec<Direction>) -> i64 {
    let mut result = 0;
    for d in dirs.iter() {
        result += convert_dir_to_int(*d);
    }
    result
}

fn dirs_to_basement(dirs: Vec<Direction>) -> usize {
    let mut floor: i64 = 0;
    for (i, d) in dirs.iter().enumerate() {
        if floor < 0 {
            return i;
        }
        floor += convert_dir_to_int(*d)
    }
    return 0;
}

fn main() {
    // Day 1 Part 1
    let input = fs::read_to_string("src/input2015d1p1.txt")
        .expect("Should have been able to read the file");
    let dirs: Vec<Direction> = input.chars().map(parse_char).collect();
    println!("D1 P1: {}", dirs_to_int(dirs.clone()));

    // Day 1 Part 2
    // Find when floor goes negative first
    println!("D1 P2: {}", dirs_to_basement(dirs))
}
