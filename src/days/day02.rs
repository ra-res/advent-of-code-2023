fn get_dice_count(dice: String) -> i64 {
    return dice
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .parse::<i64>()
        .unwrap();
}

pub fn solve_part_one(lines: Vec<String>) {
    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    let mut invalid_games = 0;
    for line in lines {
        let game_info: Vec<String> = line.split(":").map(str::to_string).collect::<Vec<_>>();
        let game_id: String = game_info
            .first()
            .unwrap()
            .split_whitespace()
            .map(str::to_string)
            .last()
            .unwrap();

        let game_scores: Vec<String> = game_info
            .last()
            .unwrap()
            .split(";")
            .map(str::to_string)
            .collect::<Vec<_>>();

        let mut is_game_invalid = false;
        for game_iteration in game_scores {
            for dice in game_iteration.split(",") {
                let dice_count = get_dice_count(dice.to_string());
                if dice.contains("blue") {
                    if dice_count > max_blue {
                        is_game_invalid = true;
                    }
                }
                if dice.contains("green") {
                    if dice_count > max_green {
                        is_game_invalid = true;
                    }
                }
                if dice.contains("red") {
                    if dice_count > max_red {
                        is_game_invalid = true;
                    }
                }
            }
        }

        if !is_game_invalid {
            invalid_games += game_id.parse::<i64>().unwrap();
        }
    }
    println!("day2_1 {}", invalid_games)
}

pub fn solve_part_two(lines: Vec<String>) {
    let mut sum_of_powers = 0;
    for line in lines {
        let game_info: Vec<String> = line.split(":").map(str::to_string).collect::<Vec<_>>();
        let game_scores: Vec<String> = game_info
            .last()
            .unwrap()
            .split(";")
            .map(str::to_string)
            .collect::<Vec<_>>();

        let mut blue = 1;
        let mut red = 1;
        let mut green = 1;
        for game_iteration in game_scores {
            for dice in game_iteration.split(",") {
                let dice_count = get_dice_count(dice.to_string());
                if dice.contains("blue") {
                    blue = blue.max(dice_count);
                }
                if dice.contains("green") {
                    green = green.max(dice_count);
                }
                if dice.contains("red") {
                    red = red.max(dice_count);
                }
            }
        }
        sum_of_powers += blue * green * red;
    }
    println!("day2_2 {}", sum_of_powers)
}
