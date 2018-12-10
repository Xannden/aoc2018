#![allow(dead_code)]
#![allow(unused_variables)]

pub fn run(input: String) {
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {


    println!("{:?}", reduce(input));
}

fn part2(input: &str) {
    let input = String::from_utf8(reduce(input)).unwrap();

    let mut min = ( 0, usize::max_value());

    for letter in b'a'..b'z' {
        let result = reduce(&input.replace(letter as char, "").replace(letter.to_ascii_uppercase() as char, "")).len();

        if result < min.1 {
            min = (letter, result);
        }
    }

    println!("{}, {}", min.0 as char, min.1);
}

fn reduce(input: &str) -> Vec<u8> {
    let mut data = Vec::from(input.as_bytes());
    let mut done = false;

    while !done {
        for i in 0..data.len() {
            if i == data.len() - 1 {
                done = true;
                break;
            }

            if data[i].to_ascii_lowercase() == data[i + 1].to_ascii_lowercase() {
                if (data[i].is_ascii_lowercase() && data[i + 1].is_ascii_uppercase()) || (data[i].is_ascii_uppercase() && data[i + 1].is_ascii_lowercase()) {
                    data.remove(i);
                    data.remove(i);
                    break;
                }
            }


        }
    }

    data
}