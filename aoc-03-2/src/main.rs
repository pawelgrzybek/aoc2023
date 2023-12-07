use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };
    let digit_pattern = Regex::new(r"\d+").unwrap();
    let input_lines = input.lines();
    let input_lines_lenght = input_lines.clone().count();
    let mut sum = 0;

    for (line_index, line) in input_lines.clone().enumerate() {
        let gear_indices: Vec<usize> = line
            .char_indices()
            .filter(|(_0, _1)| *_1 == '*')
            .map(|(_0, _1)| _0)
            .collect();

        // skip line if there is no gear in the line
        if gear_indices.is_empty() {
            continue;
        }

        for gear in gear_indices {
            let start_index = gear - 1;
            let end_index = gear + 1;
            let valid_overlap_range: Vec<usize> = (start_index..=end_index).collect();
            let mut multipliers: Vec<i32> = Vec::new();

            // do not collect numbers from the line above if first line
            if line_index != 0 {
                let line_above = input_lines.clone().nth(line_index - 1).unwrap();
                let mut line_above_digits: Vec<(Vec<usize>, usize)> = digit_pattern
                    .find_iter(line_above)
                    .map(|m| {
                        (
                            (m.start()..=m.end() - 1).collect(),
                            m.as_str().parse().unwrap(),
                        )
                    })
                    .collect();
                line_above_digits
                    .retain(|(_0, _1)| _0.iter().any(|&i| valid_overlap_range.contains(&i)));
                let part_numbers: Vec<i32> = line_above_digits.iter().map(|i| i.1 as i32).collect();
                for part in part_numbers {
                    multipliers.push(part);
                }
            }

            // do not collect numbers from the line below if last line
            if line_index != input_lines_lenght - 1 {
                let line_below = input_lines.clone().nth(line_index + 1).unwrap();
                let mut line_below_digits: Vec<(Vec<usize>, usize)> = digit_pattern
                    .find_iter(line_below)
                    .map(|m| {
                        (
                            (m.start()..=m.end() - 1).collect(),
                            m.as_str().parse().unwrap(),
                        )
                    })
                    .collect();
                line_below_digits
                    .retain(|(_0, _1)| _0.iter().any(|&i| valid_overlap_range.contains(&i)));

                let part_numbers: Vec<i32> = line_below_digits.iter().map(|i| i.1 as i32).collect();
                for part in part_numbers {
                    multipliers.push(part);
                }
            }

            let mut line_digits: Vec<(Vec<usize>, usize)> = digit_pattern
                .find_iter(line)
                .map(|m| {
                    (
                        (m.start()..=m.end() - 1).collect(),
                        m.as_str().parse().unwrap(),
                    )
                })
                .collect();
            line_digits.retain(|(_0, _1)| _0.iter().any(|&i| valid_overlap_range.contains(&i)));

            let part_numbers: Vec<i32> = line_digits.iter().map(|i| i.1 as i32).collect();
            for part in part_numbers {
                multipliers.push(part);
            }

            if multipliers.len() == 2 {
                let product: i32 = multipliers.iter().product();
                sum += product;
            }
        }
    }
    println!("Sum: {}", sum);
}
