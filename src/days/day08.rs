use std::collections::HashMap;

use regex::Regex;

pub fn solve_part_one(mut lines: Vec<String>) {
    let instructions = lines.remove(0);
    lines.remove(0);
    let map = build_map(lines);

    let mut position: String = String::from("AAA");
    let mut count = 0;
    let goal = String::from("ZZZ");
    while position != goal {
        for instruction in instructions.chars() {
            count += 1;
            let lr = map.get(&position).unwrap();
            if instruction == 'l' {
                position = lr.0.clone();
            }

            if instruction == 'R' {
                position = lr.1.clone();
            }
        }
    }

    println!("day8_1 {}", count)
}

fn build_map(ll: Vec<String>) -> HashMap<String, (String, String)> {
    let re = Regex::new(r"(?<key>\w+) = [(](?<left>\w+), (?<right>\w+)[)]").unwrap();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in ll {
        let Some(mapping) = re.captures(line.as_str()) else {
            panic!("line does not match regex");
        };
        map.insert(
            mapping["key"].to_string(),
            (mapping["left"].to_string(), mapping["right"].to_string()),
        );
    }
    map
}

pub fn solve_part_two(mut lines: Vec<String>) {
    let instructions = lines.remove(0).chars().collect::<Vec<char>>();
    lines.remove(0);
    let map = build_map(lines);
    let queue = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let mut ans = Vec::new();
    for entry in queue {
        let mut current = entry.clone();
        let mut count = 0;

        while !current.ends_with('Z') {
            let lr = map.get(&current).unwrap();
            let instruction = instructions[count % instructions.len()];
            if instruction == 'L' {
                current = lr.0.clone();
            }

            if instruction == 'R' {
                current = lr.1.clone();
            }

            count += 1;
        }

        ans.push(count);
    }

    println!("day8_2 {}", lcm_of(&ans));
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let x = *iter.next().unwrap();
    let y = *iter.next().unwrap();
    let mut ans = lcm(x, y);
    while let Some(&next) = iter.next() {
        ans = lcm(ans, next);
    }
    ans
}

// fn gcd_of(list: &[usize]) -> usize {
//     let mut iter = list.iter();
//     let x = *iter.next().unwrap();
//     let y = *iter.next().unwrap();
//     let mut ans = gcd(x, y);
//     while let Some(&next) = iter.next() {
//         ans = gcd(ans, next);
//     }
//     ans
// }
