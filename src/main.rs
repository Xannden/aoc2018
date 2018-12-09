#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
//    day1::run(read_file("day1.txt"));

//    day2::run(read_file("day2.txt"));

//    day3::run(read_file("day3.txt"));

//    day4::run(read_file("day4.txt"));

//    day5::run(read_file("day5.txt"));

    day6::run(read_file("day6.txt"));
}

pub fn read_file(path: &str) -> String {
    let mut file = File::open("D:/Code/Rust/aoc/Data/".to_owned() + path).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();


    input
}