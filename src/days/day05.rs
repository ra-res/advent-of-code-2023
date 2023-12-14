use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, fmt};

#[derive(Clone)]
struct Range {
    dest: i64,
    src: i64,
    range: i64,
}

impl Range {
    fn contains_seed(&self, seed: i64) -> bool {
        let min = self.src;
        let max = self.src + self.range;
        return seed >= min && seed < max;
    }

    fn get_seed(&self, seed: i64) -> i64 {
        let d = seed - self.src;
        let x = self.dest + d.abs();
        return x;
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "dest: {}, src: {}, range: {}",
            self.dest, self.src, self.range
        )
    }
}

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
        let mut current_seed = seed.clone().parse::<i64>().unwrap();
        for map_key in &keys {
            let ranges = map.get(map_key).unwrap();
            for range in ranges {
                if range.contains_seed(current_seed) {
                    current_seed = range.get_seed(current_seed.clone());
                    break;
                }
            }
        }

        lowest = lowest.min(current_seed);
    }

    println!("day5_1 {}", lowest)
}

fn build_map(cpy: Vec<String>) -> (Vec<String>, HashMap<String, Vec<Range>>) {
    let re: Regex = Regex::new(r"(?<name>.*) map:").unwrap();
    let mut maps: HashMap<String, Vec<Range>> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    let mut info: String = String::new();
    let mut ranges: Vec<Range> = Vec::new();

    for line in cpy {
        if line.is_empty() {
            maps.insert(info.to_string(), ranges.clone());
            ranges = Vec::new();
            info = String::new();
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

            let dest = nums[0].clone();
            let src = nums[1].clone();
            let range = nums[2].clone();

            ranges.insert(0, Range { dest, src, range })
        }
    }

    return (keys, maps);
}

fn unbox_seeds(seeds: Vec<String>) -> Vec<Vec<i64>> {
    let par_iter = seeds.par_chunks_exact(2).map(|v| {
        let range_start = v.get(0).unwrap().parse::<i64>().unwrap();
        let range_end = range_start + v.get(1).unwrap().parse::<i64>().unwrap();
        let range = (range_start..range_end).into_iter().collect::<Vec<i64>>();
        return range;
    });

    let mut target: Vec<Vec<i64>> = Vec::new();
    par_iter.collect_into_vec(&mut target);
    return target;
}

pub fn solve_part_two(lines: Vec<String>) {
    let mut cpy = lines.clone();
    let batches: Vec<Vec<i64>> = unbox_seeds(get_seeds(cpy.clone()));
    cpy.remove(0); // remove seeds
    cpy.remove(0); // remove empty line after seeds

    let (keys, map) = build_map(cpy);
    let min = batches
        .par_iter()
        .map(|seed_batch| {
            let batch_result = seed_batch
                .par_iter()
                .map(|seed| {
                    let mut current_seed = seed.clone();
                    for map_key in &keys {
                        let ranges = map.get(map_key).unwrap();
                        for range in ranges {
                            if range.contains_seed(current_seed) {
                                current_seed = range.get_seed(current_seed.clone());
                                break;
                            }
                        }
                    }
                    return current_seed;
                })
                .min()
                .unwrap();
            return batch_result;
        })
        .min()
        .unwrap();

    println!("day5_2 {}", min);
}
