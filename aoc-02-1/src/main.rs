use std::{env, fs};

fn main() {
    #[derive(Debug)]
    struct Round {
        red: u8,
        green: u8,
        blue: u8,
    }

    #[derive(Debug)]
    struct Game {
        game: u8,
        rounds: Vec<Round>,
    }

    const MAX_RED_CUBES: u8 = 12;
    const MAX_GREEN_CUBES: u8 = 13;
    const MAX_BLUE_CUBES: u8 = 14;
    const DEFAULT_CUBES: u8 = 0;

    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let line = &line.split_once(' ').unwrap().1.split_once(": ").unwrap();
        let (game, rounds) = line;
        let game_id = game.parse().unwrap();

        let rounds_parsed: Vec<&str> = rounds.split(';').collect();
        let mut rounds_vec: Vec<Round> = Vec::new();

        for round in rounds_parsed.iter() {
            let colors: Vec<&str> = round.trim().split(", ").collect();
            let colors_iter = colors.iter();

            rounds_vec.push(Round {
                red: colors_iter
                    .clone()
                    .find(|&i| i.ends_with(" red"))
                    .unwrap_or(&"0 red")
                    .replace(" red", "")
                    .parse()
                    .unwrap_or(DEFAULT_CUBES),
                green: colors_iter
                    .clone()
                    .find(|&i| i.ends_with(" green"))
                    .unwrap_or(&"0 green")
                    .replace(" green", "")
                    .parse()
                    .unwrap_or(DEFAULT_CUBES),
                blue: colors_iter
                    .clone()
                    .find(|&i| i.ends_with(" blue"))
                    .unwrap_or(&"0 blue")
                    .replace(" blue", "")
                    .parse()
                    .unwrap_or(DEFAULT_CUBES),
            })
        }

        games.push(Game {
            game: game_id,
            rounds: rounds_vec,
        });
    }

    let mut sum: u16 = 0;

    for game in games {
        if game.rounds.iter().all(|x| {
            x.red <= MAX_RED_CUBES && x.green <= MAX_GREEN_CUBES && x.blue <= MAX_BLUE_CUBES
        }) {
            sum += game.game as u16;
        }
    }

    println!("Result {:#?}", sum)
}
