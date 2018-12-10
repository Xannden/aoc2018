use std::collections::{
    VecDeque,
    HashMap,
};

use std::fs::File;
use std::io::Read;

use regex;
use std::io::Write;
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn run(input: String) {
    let reg = regex::Regex::new(r"position=<([\d\-\s]*), ([\d\-\s]*)> velocity=<([\d\-\s]*), ([\d\-\s]*)>").unwrap();

    let input = input.lines().map(|l| {
        let caps = reg.captures(l).unwrap();

        Point {
            position: (caps[1].trim().parse().unwrap(), caps[2].trim().parse().unwrap()),
            velocity: (caps[3].trim().parse().unwrap(), caps[4].trim().parse().unwrap()),
        }
    }).collect::<Vec<_>>();

    let mut file = File::create("output.txt").unwrap();

    part1(input.clone());
}

fn write(data: &[Point]) {


    let mut map = HashSet::new();

    for point in data {
        map.insert(point.position);
    }

    let ((p_x, p_y), (w, h)) = bounding_box(&data);

    for y in p_y..=(p_y + h) {
        let mut line = String::new();

        for x in p_x..=(p_x + w) {

            if map.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push(' ');
            }
        }

        println!("{}", line);
    }
}

fn part1(mut data: Vec<Point>) {

    let (_, (w, h)) = bounding_box(&data);

    let mut smallest = w + h;

    let mut seconds = 0;

    loop {
        seconds += 1;

        for point in data.iter_mut() {
            point.position.0 += point.velocity.0;
            point.position.1 += point.velocity.1;
        }

        let (_, size) = bounding_box(&data);

       if (size.0 + size.1) < smallest {
           smallest = size.0 + size.1;
       } else if (size.0 + size.1) > smallest {
           break;
       }
    }

    for point in data.iter_mut() {
        point.position.0 -= point.velocity.0;
        point.position.1 -= point.velocity.1;
    }

    write(&data);

    println!("{}", seconds - 1);
}

fn bounding_box(data: &[Point]) -> ((i32, i32), (i32, i32)) {
    let min_x = data.iter().fold(i32::max_value(), |a, v| {
        i32::min(a, v.position.0)
    });
    let min_y = data.iter().fold(i32::max_value(), |a, v| {
        i32::min(a, v.position.1)
    });
    let max_x = data.iter().fold(0, |a, v| {
        i32::max(a, v.position.0)
    });
    let max_y = data.iter().fold(0, |a, v| {
        i32::max(a, v.position.1)
    });

    ((min_x, min_y), (max_x - min_x, max_y - min_y))
}