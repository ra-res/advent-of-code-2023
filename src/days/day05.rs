use std::collections::HashMap;

use regex::Regex;

fn get_seeds(lines: Vec<String>) -> Vec<String> {
    return lines
        .get(0)
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
}

pub fn solve_part_one(lines: Vec<String>) {
    let mut cpy = lines.clone();

    let seeds: Vec<String> = get_seeds(cpy.clone());
    cpy.remove(0); // remove seeds
    cpy.remove(0); // remove empty line after seeds
    let (keys, map) = build_map(cpy);
    let mut lowest = std::i64::MAX;

    for seed in seeds {
        let mut current_key = seed.clone();
        for map_key in &keys {
            let local_map = map.get(map_key).unwrap();
            let value = local_map.get(&current_key);
            if value.is_some() {
                current_key = value.unwrap().clone();
            }
            // println!("{} {}", seed, current_key);
        }
        lowest = lowest.min(current_key.parse::<i64>().unwrap());
    }

    println!("day5_1 {}", lowest)
}

fn build_map(cpy: Vec<String>) -> (Vec<String>, HashMap<String, HashMap<String, String>>) {
    let re: Regex = Regex::new(r"(?<name>.*) map").unwrap();
    let mut maps: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    let mut info: String = String::new();
    let mut map: HashMap<String, String> = HashMap::new();

    for line in cpy {
        if line.is_empty() {
            info = String::new();
            map = HashMap::new();
            continue;
        }

        if let Some(map) = re.captures(&line) {
            info = map.name("name").unwrap().as_str().to_string();
            keys.insert(keys.len(), info.clone());
        } else {
            // Each line within a map contains three numbers:
            // the destination range start,
            // the source range start,
            // and the range length.
            let nums: Vec<i64> = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();


            let mut dest = nums[0].clone();
            let mut src = nums[1].clone();
            let range = nums[2];

            for _ in 0..range {
                map.insert(src.to_string(), dest.to_string());
                src += 1;
                dest += 1;
            }

            maps.insert(info.clone(), map.clone());
        }
    }

    return (keys, maps);
}

pub fn solve_part_two(lines: Vec<String>) {
    println!("day5_2 {}", 1);
}
