use std::{env, fs};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn get_winning_options(&self) -> i64 {
        let range_start = 1;
        let range_end = self.time - 1;

        (range_start..=range_end)
            .enumerate()
            .map(|(index, time)| time * (range_end - index as i64))
            .filter(|&i| i > self.distance)
            .count() as i64
    }
}

fn main() {
    let input = match env::args().collect::<Vec<String>>().get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    let time = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();

    let race = Race { time, distance };

    println!("Result: {:#?}", race.get_winning_options());
}
