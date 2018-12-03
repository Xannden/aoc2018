#![allow(dead_code)]

use std::collections::HashSet;

pub fn part1(input: String) {
    let sum = input.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>();

    println!("{}", sum);
}

pub fn part2(input: String) {
    let mut set : HashSet<i32> = HashSet::new();

    let value = input.lines().map(|l| l.parse::<i32>().unwrap()).cycle().try_fold(0, |frequency, v| {
        let new = frequency + v;

        if set.insert(new) {
           Ok(new)
        } else {
            Err(new)
        }

    }).unwrap_err();

    println!("{}", value);
}

