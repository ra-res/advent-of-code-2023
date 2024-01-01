mod days;

#[allow(dead_code)]
mod run {
    use crate::days::{day01, day02, day03, day04, day05, day06, day07, day08};
    use std::fs::read_to_string;
    use std::path::Path;

    fn read_lines(relative_path: &str) -> Vec<String> {
        let mut result = Vec::new();
        let path = Path::new(relative_path);

        for line in read_to_string(&path).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }

    pub fn day01() {
        day01::solve_part_one(read_lines("src/inputs/day01.txt"));
        day01::solve_part_two(read_lines("src/inputs/day01.txt"));
    }

    pub fn day02() {
        day02::solve_part_one(read_lines("src/inputs/day02.txt"));
        day02::solve_part_two(read_lines("src/inputs/day02.txt"));
    }

    pub fn day03() {
        day03::solve_part_one(read_lines("src/inputs/day03.txt"));
        day03::solve_part_two(read_lines("src/inputs/day03.txt"));
    }

    pub fn day04() {
        day04::solve_part_one(read_lines("src/inputs/day04.txt"));
        day04::solve_part_two(read_lines("src/inputs/day04.txt"));
    }

    pub fn day05() {
        day05::solve_part_one(read_lines("src/inputs/day05.txt"));
        day05::solve_part_two(read_lines("src/inputs/day05.txt"));
    }

    pub fn day06() {
        day06::solve_part_one(read_lines("src/inputs/day06.txt"));
        day06::solve_part_two(read_lines("src/inputs/day06.txt"));
    }

    pub fn day07() {
        day07::solve_part_one(read_lines("src/inputs/day07.txt"));
        day07::solve_part_two(read_lines("src/inputs/day07.txt"));
    }

    pub fn day08() {
        day08::solve_part_one(read_lines("src/inputs/day08.txt"));
        day08::solve_part_two(read_lines("src/inputs/day08.txt"));
    }
}

fn main() {
    run::day08();
}
