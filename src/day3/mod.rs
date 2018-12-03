#![allow(dead_code)]
#![allow(unused_variables)]

use regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rect {
    id: i64,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

impl Rect {
    fn overlap(&self, other: &Rect) -> bool {
        self.x <= (other.x + other.width) && (self.x + self.width) >= other.x && (self.y + self.height) >= other.y && self.y <= (other.y + other.height)
    }
}

pub fn process_input(input: String) -> Vec<Rect> {
    let re = regex::Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();

        Rect {
            id: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            x: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            y: caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            width: caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            height: caps.get(5).unwrap().as_str().parse::<i64>().unwrap(),
        }
    }).collect::<Vec<_>>()
}

pub fn part1(input: &[Rect]) {
    let mut board = HashMap::new();

    for i in 0..input.len() {
        for x in 0..input[i].width {
            for y in 0..input[i].height {
                board.entry((x + input[i].x, y + input[i].y)).and_modify(|v| *v = true).or_insert(false);
            }
         }
    }

    println!("{}", board.values().fold(0, |acc, b|  if *b { acc + 1} else {acc}));
}

pub fn part2(input: &[Rect]) {
    for i in 0..input.len() {
        let mut found_overlap = false;
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            if input[i].overlap(&input[j]) {
                found_overlap = true;
            }
        }

        if !found_overlap {
            println!("{}", input[i].id);
        }
    }
}