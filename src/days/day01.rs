fn get_digits(str: String) -> Vec<char> {
    return str
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .filter(|&c: &char| c.is_digit(10))
        .collect();
}

fn str_to_digits(str: String) -> String {
    return str
        .clone()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
}

pub fn solve_part_one(lines: Vec<String>) {
    let mut result: i64 = 0;

    for line in lines {
        let digits: Vec<char> = get_digits(line);

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let addition = format!("{}{}", first, last).parse::<i64>();
        result += addition.unwrap();
    }
    println!("day1_1 {}", result)
}

pub fn solve_part_two(lines: Vec<String>) {
    let mut result: i64 = 0;

    for line in lines {
        let digits: Vec<char> = get_digits(str_to_digits(line));
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let addition = format!("{}{}", first, last).parse::<i64>().unwrap();
        result += addition;
    }
    println!("day2_2 {}", result)
}
