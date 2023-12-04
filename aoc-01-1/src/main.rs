use std::{env, fs};

fn main() {
    let mut sum = 0;

    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        let digit_first = digits.first().unwrap().to_digit(10).unwrap();
        let digit_last = digits.last().unwrap().to_digit(10).unwrap();

        sum += digit_first * 10 + digit_last;
    }

    println!("Sum is {:#?}", sum);
}
