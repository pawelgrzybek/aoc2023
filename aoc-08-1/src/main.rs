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

    let mut steps: i64 = 0;
    let mut index = elements.iter().position(|i| i.name == "AAA").unwrap();

    loop {
        let current_element = elements.get(index).unwrap();
        if current_element.name == "ZZZ" {
            break;
        }

        let next_direction = *instructions
            .get(steps as usize % instructions.len())
            .unwrap() as usize;

        index = *current_element.directions.get(next_direction).unwrap();
        steps += 1;
    }

    println!("Steps: {:#?}", steps);
}
