mod days;

use std::fs::read_to_string;
use std::path::Path;

use days::{day01, day02, day03};

fn read_lines(relative_path: &str) -> Vec<String> {
    let mut result = Vec::new();
    let path = Path::new(relative_path);

    for line in read_to_string(&path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn day01() {
    // 54927
    day01::solve_part_one(read_lines("src/inputs/day01.txt"));
    // 54581
    day01::solve_part_two(read_lines("src/inputs/day01.txt"));
}

fn day02() {
    // 2331
    day02::solve_part_one(read_lines("src/inputs/day02.txt"));
    // 71585
    day02::solve_part_two(read_lines("src/inputs/day02.txt"));
}

fn day03() {
    // 2331
    day03::solve_part_one(read_lines("src/inputs/day03.txt"));
    // 71585
    day03::solve_part_two(read_lines("src/inputs/day03.txt"));
}

fn main() {
    day01();
    day02();
    day03();
}
