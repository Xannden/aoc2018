#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, x: i32, y: i32) -> i32 {
        i32::abs(self.x.saturating_sub(x)) + i32::abs(self.y.saturating_sub(y))
    }
}

pub fn run(input: String) {
    let data = input.lines().map( |l| {
       let mut split = l.split(", ");

        Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        }
    }).collect::<Vec<_>>();

    part1(&data);
    part2(&data);
}

fn part1(input: &[Point]) {
    let min_x = input.iter().fold(input[0].x, |a, v| {
       i32::min(a, v.x)
    });
    let max_x = input.iter().fold(0, |a, v| {
        i32::max(a, v.x)
    });
    let min_y = input.iter().fold(input[0].y, |a, v| {
        i32::min(a, v.y)
    });
    let max_y = input.iter().fold(0, |a, v| {
        i32::max(a, v.y)
    });

    let mut map = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut distance = HashMap::new();
            for point in input {
                distance.insert(point, point.distance(x, y));
            }

            let min = distance.values().min().unwrap();

            if distance.values().fold(0, |a ,f| if f == min { a + 1} else {a}) > 1 {
                continue;
            }

            let closest = **distance.iter().find(|p| p.1 == min).unwrap().0;

            map.entry(closest).and_modify(|v: &mut i32| *v = v.saturating_add(1))
                .or_insert(1);

            if x == min_x || x == max_x || y == min_y || y == max_y {
                map.entry(closest).and_modify(|v| *v = i32::max_value());
            }
        }
    }
    println!("{:?}", map.values().fold(0, |a, v| {
        if *v != i32::max_value() && *v > a {
            *v
        } else {
            a
        }
    }));
}

fn part2(input: &[Point]) {
    let min_x = input.iter().fold(input[0].x, |a, v| {
        i32::min(a, v.x)
    });
    let max_x = input.iter().fold(0, |a, v| {
        i32::max(a, v.x)
    });
    let min_y = input.iter().fold(input[0].y, |a, v| {
        i32::min(a, v.y)
    });
    let max_y = input.iter().fold(0, |a, v| {
        i32::max(a, v.y)
    });

    let mut area = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut distance = 0;

            for point in input {
                distance += point.distance(x, y);
            }

            if distance < 10000 {
                area += 1;
            }
        }
    }

    println!("{}", area);
}