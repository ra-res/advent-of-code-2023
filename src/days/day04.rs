use regex::Regex;

fn str_to_nums(str: &str) -> Vec<i64> {
    return str
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
}

pub fn solve_part_one(lines: Vec<String>) {
    let re: Regex = Regex::new(r"Card( )*([0-9][0-9]*)\:(?<winning>.*)\|(?<my>.*)").unwrap();

    let mut winning_numbers = 0;
    for line in lines {
        if let Some(m) = re.captures(line.as_str()) {
            let winning_nums: Vec<i64> = str_to_nums(m.name("winning").unwrap().as_str());
            let my_nums: Vec<i64> = str_to_nums(m.name("my").unwrap().as_str());

            let mut current = 0;
            for num in my_nums {
                if winning_nums.contains(&num) {
                    if current == 0 {
                        current = 1;
                    } else {
                        current = current * 2;
                    }
                }
            }

            winning_numbers += current;
        }
    }

    println!("day4_1 {}", winning_numbers)
}

pub fn solve_part_two(lines: Vec<String>) {
    println!("day4_2 {}", 1)
}
