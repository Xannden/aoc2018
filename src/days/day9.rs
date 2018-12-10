use std::collections::{
    HashMap,
    vec_deque::VecDeque,
};

use regex;

pub fn run(input: String) {
    let reg = regex::Regex::new(r"(\d*) players; last marble is worth (\d*) points").unwrap();

    let caps = reg.captures(&input).unwrap();

    part1(caps[1].parse().unwrap(), caps[2].parse().unwrap());
    part2(caps[1].parse().unwrap(), caps[2].parse().unwrap());
}

fn part1(players: u32, last_marble: u32) {
    let mut ring = VecDeque::new();
    let mut scores = HashMap::new();

    ring.push_front(0);

    for i in 1..last_marble {
        if i % 23 == 0 && i != 0{
            scores.entry(i % players).and_modify(|s| *s += i).or_insert(i);

            rotate(&mut ring, 7);

            scores.entry(i % players).and_modify(|s| *s += ring.pop_back().unwrap());

            rotate(&mut ring, -1);
        } else {
            rotate(&mut ring, -1);
            ring.push_back(i);
        }
    }

    println!("{:?}", scores.values().max());
}

fn part2(players: u32, last_marble: u32) {
    part1(players, last_marble * 100);
}

fn rotate(vec: &mut VecDeque<u32>, amount: i32) {
    if amount > 0 {
        for _ in 0..amount {
            let temp = vec.pop_back().unwrap();
            vec.push_front(temp);
        }
    } else {
        for _ in amount..0 {
            let temp = vec.pop_front().unwrap();
            vec.push_back(temp);
        }
    }
}