use core::panic;
use std::{env, fs, ops::Range};

#[derive(Debug)]
struct Mapper {
    source: Range<i64>,
    delta: i64,
}

fn resolve_mapping(map: &[Mapper], source_number: i64) -> i64 {
    let mapper = map.iter().find(|i| i.source.contains(&source_number));
    match mapper {
        Some(Mapper { source: _, delta }) => source_number - delta,
        None => source_number,
    }
}

const SEEDS: &str = "seeds";
const SEED_TO_SOIL: &str = "seed-to-soil map";
const SOIL_TO_FERTILIZER: &str = "soil-to-fertilizer map";
const FERTILIZER_TO_WATER: &str = "fertilizer-to-water map";
const WATER_TO_LIGHT: &str = "water-to-light map";
const LIGHT_TO_TEMPERATURE: &str = "light-to-temperature map";
const TEMPERATURE_TO_HUMIDITY: &str = "temperature-to-humidity map";
const HUMIDITY_TO_LOCATION: &str = "humidity-to-location map";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        None => panic!("🚨 Please, pass a path to a file."),
        Some(p) => fs::read_to_string(p).unwrap(),
    };
    let input_chunks = input.split("\n\n");
    let mut seeds: Vec<i64> = vec![];

    let mut seed_to_soil_map: Vec<Mapper> = vec![];
    let mut soil_to_fertilizer_map: Vec<Mapper> = vec![];
    let mut fertilizer_to_water_map: Vec<Mapper> = vec![];
    let mut water_to_light_map: Vec<Mapper> = vec![];
    let mut light_to_temperature_map: Vec<Mapper> = vec![];
    let mut temperature_to_humidity_map: Vec<Mapper> = vec![];
    let mut humidity_to_location_map: Vec<Mapper> = vec![];

    for chunk in input_chunks {
        let (prefix, sufix) = chunk.split_once(':').unwrap();
        match prefix {
            SEEDS => {
                let temp = sufix
                    .split_whitespace()
                    .map(|i| i.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                for (index, &item) in temp.iter().enumerate() {
                    if index % 2 == 1 {
                        continue;
                    }

                    let next_item = temp.get(index + 1).unwrap();
                    for s in item..item + next_item {
                        seeds.push(s)
                    }
                }
            }
            _ => {
                let results: Vec<&str> = sufix.trim().split('\n').collect();
                for result in results {
                    let k = result
                        .split_whitespace()
                        .map(|i| i.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    let destination = k.first().unwrap();
                    let source = k.get(1).unwrap();
                    let lenght = k.get(2).unwrap();

                    let item = Mapper {
                        source: *source..source + lenght,
                        delta: *source - *destination,
                    };

                    match prefix {
                        SEED_TO_SOIL => seed_to_soil_map.push(item),
                        SOIL_TO_FERTILIZER => soil_to_fertilizer_map.push(item),
                        FERTILIZER_TO_WATER => fertilizer_to_water_map.push(item),
                        WATER_TO_LIGHT => water_to_light_map.push(item),
                        LIGHT_TO_TEMPERATURE => light_to_temperature_map.push(item),
                        TEMPERATURE_TO_HUMIDITY => temperature_to_humidity_map.push(item),
                        HUMIDITY_TO_LOCATION => humidity_to_location_map.push(item),
                        _ => panic!("Uuuups!"),
                    };
                }
            }
        }
    }

    println!("Seeds collected ✅: {}", seeds.len());

    let result = seeds
        .iter()
        .map(|i| resolve_mapping(&seed_to_soil_map, *i))
        .map(|i| resolve_mapping(&soil_to_fertilizer_map, i))
        .map(|i| resolve_mapping(&fertilizer_to_water_map, i))
        .map(|i| resolve_mapping(&water_to_light_map, i))
        .map(|i| resolve_mapping(&light_to_temperature_map, i))
        .map(|i| resolve_mapping(&temperature_to_humidity_map, i))
        .map(|i| resolve_mapping(&humidity_to_location_map, i))
        .min()
        .unwrap();

    println!("Result: {:#?}", result)
}
