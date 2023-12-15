use num::integer::lcm;
use std::{env, fs};

#[derive(Debug)]
struct Element {
    name: String,
    directions: [usize; 2],
}

fn main() {
    let input = match env::args().collect::<Vec<String>>().get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    let instructions: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|i| if i == 'L' { 0 } else { 1 })
        .collect();

    let nodes: Vec<&str> = input.lines().skip(2).collect();
    let elements: Vec<Element> = nodes
        .iter()
        .map(|&i| {
            let name = i[0..3].to_string();
            let left = nodes
                .iter()
                .position(|&x| x.starts_with(&i[7..10]))
                .unwrap() as usize;
            let right = nodes
                .iter()
                .position(|&x| x.starts_with(&i[12..15]))
                .unwrap() as usize;

            Element {
                name,
                directions: [left, right],
            }
        })
        .collect();

    let indices: Vec<&Element> = elements.iter().filter(|&i| i.name.ends_with('A')).collect();
    let mut total_steps = vec![];

    for index in indices {
        let mut steps: i64 = 0;
        let mut current_index = index;

        loop {
            if current_index.name.ends_with('Z') {
                break;
            }

            let next_direction = *instructions
                .get(steps as usize % instructions.len())
                .unwrap() as usize;

            current_index = &elements[*current_index.directions.get(next_direction).unwrap()];
            steps += 1;
        }

        total_steps.push(steps);
    }

    let result: i64 = total_steps.iter().fold(1, |prev, &x| lcm(prev, x));
    println!("Steps: {:#?}", result);
}
