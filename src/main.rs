#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod days;

use crate::days::*;

fn main() {
//    day1::run(read_file("day1.txt"));

//    day2::run(read_file("day2.txt"));

//    day3::run(read_file("day3.txt"));

//    day4::run(read_file("day4.txt"));

//    day5::run(read_file("day5.txt"));

//    day6::run(read_file("day6.txt"));

//    day7::run(read_file("day7.txt"));

//    day8::run(read_file("day8.txt"));

//    day9::run(read_file("day9.txt"));

//    day10::run(read_file("day10.txt"));

    day11::run(read_file("day11.txt"));
}

pub fn read_file(path: &str) -> String {
    let mut file = File::open("D:/Code/Rust/aoc/Data/".to_owned() + path).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();


    input
}