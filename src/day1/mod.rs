#![allow(dead_code)]

use std::collections::HashSet;

pub fn run(input: String) {
    let data = input.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<_>>();

    println!("Day 1");
    part1(&data);
    part2(&data);
}

fn part1(input: &[i32]) {
    println!("{}", input.iter().sum::<i32>());
}

fn part2(input: &[i32]) {
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

