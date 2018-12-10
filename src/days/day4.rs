use regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Time {
    hour: u32,
    minute: u32,
}


#[derive(Debug, Eq, PartialEq)]
struct Entry {
    date: Date,
    time: Time,
    message: String,
    event: Event,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let date = self.date.cmp(&other.date);

        if date == Ordering::Equal {
            self.time.cmp(&other.time)
        } else {
            date
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        let date = self.date.partial_cmp(&other.date)?;

        if date == Ordering::Equal {
            self.time.partial_cmp(&other.time)
        }
        else {
            Some(date)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Event {
    Start(u32),
    Asleep,
    Awake,
}


pub fn run(input: String) {
    let regex = regex::Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)\] (.*)").unwrap();
    let start = regex::Regex::new(r"Guard #(\d*) begins shift").unwrap();

    let mut data = input.lines().map(|l| {
       let captures = regex.captures(l).unwrap();

        let event = if start.is_match(&captures[6]) {
            let temp = start.captures(&captures[6]).unwrap();
            Event::Start(temp[1].parse().unwrap())
        } else if &captures[6] == "falls asleep" {
            Event::Asleep
        } else {
            Event::Awake
        };

        Entry {
            date: Date {
                year: captures[1].parse().unwrap(),
                month: captures[2].parse().unwrap(),
                day: captures[3].parse().unwrap(),
            },
            time: Time {
                hour: captures[4].parse().unwrap(),
                minute: captures[5].parse().unwrap(),
            },
            message: captures[6].to_string(),
            event,
        }
    }).collect::<Vec<_>>();

    data.sort();

    let mut times : HashMap<u32, Vec<(Time,Time)>> = HashMap::new();

    let mut start = Time {hour: 0, minute: 0};
    let mut guard_id= 0;

    for entry in data {
        match entry.event {
            Event::Start(id) => guard_id = id,
            Event::Asleep => start = entry.time,
            Event::Awake => {times.entry(guard_id).and_modify(|v| {
                v.push((start, entry.time))
            }).or_insert({
                let mut vec = Vec::new();
                vec.push((start, entry.time));
                vec
            });},
        }
    }

    part1(&times);
    part2(&times);
}

fn part1(times: &HashMap<u32, Vec<(Time,Time)>>) {
    let mut longest_sleep = (0, 0);

    for (g, e) in times {
        let mut total_time = 0;

        for time in e {
            total_time += time.1.minute - time.0.minute;
        }

        if total_time > longest_sleep.1 {
            longest_sleep = (*g, total_time);
        }
    }

    let mut minutes = HashMap::new();

    for times in times.get(&longest_sleep.0) {
        for time in times {
            for m in time.0.minute..time.1.minute {
                minutes.entry(m).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    println!("{:?}", longest_sleep.0 * minutes.iter().fold((0, 0), |a, e| {
        if *e.1 > a.1 {
            (*e.0, *e.1)
        } else {
            a
        }
    }).0);
}

fn part2(times: &HashMap<u32, Vec<(Time,Time)>>) {
    let mut sleep = HashMap::new();

    for (guard, times) in times {
        for time in times {
            for m in time.0.minute..time.1.minute {
                sleep.entry((*guard, m)).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    let result = sleep.iter().fold(((0,0),0), |a, v| {
        if *v.1 > a.1 {
            (*v.0, *v.1)
        } else {
            a
        }
    }).0;

    println!("{:?}", result.0 * result.1);
}