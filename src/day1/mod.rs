#![allow(dead_code)]

use std::collections::HashSet;

pub fn process_input(input: String) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn part1(input: &[i32]) {
    println!("{}", input.iter().sum::<i32>());
}

pub fn part2(input: &[i32]) {
    let mut set : HashSet<i32> = HashSet::new();

    let value = input.iter().cycle().try_fold(0, |frequency, v| {
        let new = frequency + v;

        if set.insert(new) {
           Ok(new)
        } else {
            Err(new)
        }

    }).unwrap_err();

    println!("{}", value);
}

