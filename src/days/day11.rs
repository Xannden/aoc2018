use std::collections::{
    VecDeque,
    HashMap,
    HashSet,
};

use rayon;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::time::Instant;


pub fn run(input: String) {
    let input = input.parse::<i32>().unwrap();

    let mut board = [[0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            let rack_id = x + 10;

            let mut power_level = rack_id * y;
            power_level += input;
            power_level *= rack_id;

            power_level = (power_level / 100) % 10;
            power_level -= 5;

            board[x as usize][y as usize] = power_level;
        }
    }

    part1(&board);
}

fn part1(board: &[[i32; 300]; 300]) {
    let mut max = (0, 0, 0, 0);

    for x in 0..298 {
        for y in 0..298 {
            let total = total(board, x, y, 3);

            if total > max.2 {
                max = (x, y, total, 3);
            }
        }
    }

    println!("{:?}", max);

    let now = Instant::now();

   max = (1..301).into_par_iter().fold(|| (0,0,0,0), |a: (usize, usize, i32, i32), v: i32| {
        let mut temp = a;

        for x in 0..=(300 - v as usize) {
            for y in 0..=(300 - v as usize) {
                let total = total(board, x, y, v as usize);

                if total > temp.2 {
                    temp = (x, y, total, v);
                }
            }
        }

        temp
    }).reduce(|| (0,0,0,0), |a, v | {
       if a.2 < v.2 {
           v
       } else {
           a
       }
   });

    println!("{}", now.elapsed().as_secs());
    println!("{:?}", max);
}

fn total(board: &[[i32; 300]; 300], x: usize, y: usize, size: usize) -> i32 {
    let mut sum = 0;

    for x in x..(x+size) {
        for y in y..(y+size) {
            sum += board[x][y];
        }
    }

    sum
}