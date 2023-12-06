use regex::Regex;
use std::{env, fs};

#[derive(Debug)]
struct Part {
    part: u16,
    surroundings: Vec<char>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };
    let mut parts: Vec<Part> = Vec::new();
    let pattern = Regex::new(r"\d+").unwrap();
    let input_lines = input.lines();
    let input_lines_lenght = input_lines.clone().count();

    for (index, line) in input_lines.clone().enumerate() {
        let line_length = line.len();

        for m in pattern.find_iter(line) {
            let mut surroundings = vec![];

            // do not collect chars above first line
            if index != 0 {
                let line_above = input_lines.clone().nth(index - 1).unwrap();
                let start_index = if m.start() == 0 { 0 } else { m.start() - 1 };
                surroundings.extend(line_above.chars().skip(start_index).take(m.len() + 2));
             }

            // do not collect chars below the last line
            if index != input_lines_lenght - 1 {
                let line_above = input_lines.clone().nth(index + 1).unwrap();
                let start_index = if m.start() == 0 { 0 } else { m.start() - 1 };
                surroundings.extend(line_above.chars().skip(start_index).take(m.len() + 2));
            }

            // collect char before
            if m.start() != 0 {
                surroundings.push(line.chars().nth(m.start() - 1).unwrap_or('.'));
            }

            // collect char after
            if m.end() != line_length {
                surroundings.push(line.chars().nth(m.end()).unwrap_or('.'));
            }

            parts.push(Part {
                part: m.as_str().parse().unwrap_or(0) as u16,
                surroundings,
            });
        }
    }

    let mut parts_filtered = vec![];
    for part in parts {
        if !part.surroundings.iter().all(|&s| s == '.') {
            parts_filtered.push(part.part);
        }
    }
    let result: i32 = parts_filtered.iter().map(|&i| i as i32).sum();

    println!("Result: {:#?}", result);
}
