#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;

fn main() {
//    day1::part1(read_file("day1.txt"));
//    day1::part2(read_file("day1.txt"));

    day2::part1(read_file("day2.txt"));
    day2::part2(read_file("day2.txt"));

//    day3::part1(read_file("day3.txt"));
//    day3::part2(read_file("day3.txt"));
}

pub fn read_file(path: &str) -> String {
    let mut file = File::open("D:/Code/Rust/aoc/Data/".to_owned() + path).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();


    input
}