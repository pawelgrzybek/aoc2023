use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug)]
struct Hand {
    hand_type: i32,
    hand_score: String,
    bid: i32,
}

fn main() {
    let input = match env::args().collect::<Vec<String>>().get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };

    let points = HashMap::from([
        ('A', 'a'),
        ('K', 'b'),
        ('Q', 'c'),
        ('J', 'd'),
        ('T', 'e'),
        ('9', 'f'),
        ('8', 'g'),
        ('7', 'h'),
        ('6', 'i'),
        ('5', 'j'),
        ('4', 'k'),
        ('3', 'l'),
        ('2', 'm'),
    ]);
    let mut hands = vec![];

    for line in input.lines() {
        let hand = line.split_whitespace().next().unwrap().to_string();
        let bid = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut hand_type = hand.clone().chars().collect::<Vec<char>>();
        let mut hand_type_unique = HashSet::new();
        hand_type.retain(|x| hand_type_unique.insert(*x));
        let mut hand_typ_numeric = hand_type
            .iter()
            .map(|c| hand.chars().filter(move |d| d == c).count() as i32)
            .collect::<Vec<i32>>();

        hand_typ_numeric.sort();
        hand_typ_numeric.reverse();
        hand_typ_numeric.resize(5, 0);
        let hand_type = hand_typ_numeric.iter().fold(0, |acc, i| acc * 10 + i);
        let hand_score = hand.chars().map(|i| points.get(&i).unwrap()).collect();

        hands.push(Hand {
            bid,
            hand_type,
            hand_score,
        })
    }

    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then(b.hand_score.cmp(&a.hand_score))
    });
    let result: i32 = hands
        .iter()
        .enumerate()
        .map(|(a, b)| b.bid * (a as i32 + 1))
        .sum();

    println!("Result: {:#?}", result)
}
