pub fn run(input: String) {
    println!("Day 2");
    part1(&input);
    part2(&input);
}

pub fn part1(input: &str) {
    let mut two = 0;
    let mut three = 0;
    let mut array;

    for line in input.lines() {
        array = [0; 26];

        for c in line.chars() {
            array[((c as u8) - b'a') as usize] += 1;

        }

        if array.iter().any( |v| *v == 2) {
            two += 1;
        }

        if array.iter().any(|v| *v == 3) {
            three += 1;
        }
    }


    println!("{}", two * three);
}

pub fn part2(input: &str){
    for first in input.lines() {
        for sec in input.lines() {
            let mut diff = (0, 0);

            for (i, (fc, sc)) in first.chars().zip(sec.chars()).enumerate() {
                if fc != sc {
                    diff.1 += 1;
                    diff.0 = i;
                }
            }

            if diff.1 == 1 {
                println!("{}{}", &first[0..diff.0], &first[(diff.0+1)..]);
                return;
            }
        }
    }

}
