use regex;
use std::collections::HashMap;

pub fn run(input: String) {
    let regex = regex::Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let mut data  = HashMap::new();

    for l in input.lines() {
        let captures = regex.captures(l).unwrap();

        let key = captures[2].chars().nth(0).unwrap();
        let value = captures[1].chars().nth(0).unwrap();

        data.entry(key).and_modify(|v: &mut Vec<char>| v.push(value)).or_insert(vec![value]);

        if !data.contains_key(&value) {
            data.insert(value, Vec::new());
        }
    }

    let mut data = data.into_iter().collect::<Vec<_>>();

    data.sort_by(|l, r| l.0.cmp(&r.0));

    part1(&data);
    part2(&data);
}

fn part1(input: &Vec<(char, Vec<char>)>) {
    let mut data = input.clone();

    let mut order = String::new();

    'outer: loop {
        if data.len() == 0 {
            break;
        }

        for (i, (c, v)) in data.clone().into_iter().enumerate() {
            if v.len() == 0 {
                order.push(c);

                data.remove(i);

                for item in data.iter_mut() {
                    match item.1.iter().position(|p| *p == c) {
                        Some(pos) => {item.1.remove(pos);},
                        None => (),
                    }
                }
                break;
            }
        }
    }

    println!("{}", order);


}

fn part2(input: &Vec<(char, Vec<char>)>) {
    let mut data = input.clone();

    let mut order = String::new();

    let mut workers: [Option<(i32, char)>; 5] = [None; 5];

    let mut second = 0;

    'outer: loop {
        if data.len() == 0 && workers.iter().all(|w| w.is_none()){
            break;
        }

        for worker in &mut workers {
            match worker {
                Some(w) => {
                    if w.0 > 0 {
                        w.0 -= 1;
                    } else if w.0 == 0 {
                        order.push(w.1);

                        for item in data.iter_mut() {
                            match item.1.iter().position(|p| *p == w.1) {
                                Some(pos) => { item.1.remove(pos); },
                                None => (),
                            }
                        }

                        *worker = next_item(&mut data);
                    }
                }
                None => *worker = next_item(&mut data),
            }
        }

        for worker in &mut workers {
            match worker {
                Some(_) => (),
                None => *worker = next_item(&mut data),
            }
        }

        print!("{} ", second);

        for worker in &workers {
            print!("{:?} ", worker);
        }

        println!("{}", order);

        second += 1;
    }

    println!("{}", order);
}

fn next_item(data: &mut Vec<(char, Vec<char>)>) -> Option<(i32, char)> {
    let mut result = None;

    for (i, (c, v)) in data.clone().into_iter().enumerate() {
        if v.len() == 0 {
            data.remove(i);

            result = Some((((c as u8 - b'A') as i32) + 60, c));

            break;
        }
    }
    result
}