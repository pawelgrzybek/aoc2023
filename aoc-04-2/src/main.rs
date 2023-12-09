use std::{env, fs};

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    points: usize,
    instances: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    let min_points: usize = 1;
    let mut cards: Vec<Card> = vec![];

    for (line_index, line) in input.lines().enumerate() {
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
        let card = Card {
            index: line_index,
            points: numbers_same,
            instances: 1,
        };
        cards.push(card);
    }

    for index in 0..cards.len() {
        let card = cards.get(index).unwrap().clone();
        let Card {
            index,
            points,
            instances: _,
        } = card;

        if points.ge(&min_points) {
            let index_start: usize = index + 1;
            let index_stop: usize = index + card.points;
            for winning_card in cards[index_start..=index_stop].iter_mut() {
                winning_card.instances += card.instances;
            }
        }
    }

    let result: usize = cards.iter().map(|i| i.instances).sum();
    println!("Cards: {:#?}", result)
}
