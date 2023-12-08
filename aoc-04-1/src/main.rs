use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };
    let mut points: Vec<u32> = vec![];

    for line in input.lines() {
        let numbers = line.split(": ").last().unwrap().split(" | ");
        let numbers_winning: Vec<i32> = numbers
            .clone()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let numbers_my: Vec<i32> = numbers
            .clone()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let numbers_same = numbers_my
            .iter()
            .filter(|&x| numbers_winning.contains(x))
            .count();
        points.push(2u32.pow(numbers_same as u32) / 2);
    }

    let result: u32 = points.iter().sum();

    println!("Result: {:#?}", result);
}
