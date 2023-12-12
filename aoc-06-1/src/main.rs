use std::{env, fs};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    fn get_winning_options(&self) -> i32 {
        let range_start = 1;
        let range_end = self.time - 1;

        (range_start..=range_end)
            .enumerate()
            .map(|(index, time)| time * (range_end - index as i32))
            .filter(|&i| i > self.distance)
            .count() as i32
    }
}

fn main() {
    let input = match env::args().collect::<Vec<String>>().get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    let line_time = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let line_distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let races = line_time
        .iter()
        .zip(line_distance)
        .map(|(&time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    let result: i32 = races
        .iter()
        .map(|race| race.get_winning_options())
        .product();

    println!("Result: {:#?}", result);
}
