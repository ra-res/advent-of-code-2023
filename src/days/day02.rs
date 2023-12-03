use std::cmp;

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
            for dice in game_iteration.split(",").map(str::to_string) {
                match dice {
                    dice if dice.contains("blue") => {
                        let dice_count = get_dice_count(dice);
                        if dice_count > max_blue {
                            is_game_invalid = true;
                        }
                    }
                    dice if dice.contains("green") => {
                        let dice_count = get_dice_count(dice);
                        if dice_count > max_green {
                            is_game_invalid = true;
                        }
                    }
                    dice if dice.contains("red") => {
                        let dice_count = get_dice_count(dice);
                        if dice_count > max_red {
                            is_game_invalid = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !is_game_invalid {
            invalid_games += game_id.parse::<i64>().unwrap();
        }
    }
    println!("{}", invalid_games)
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

        let mut highest_blue = 1;
        let mut highest_red = 1;
        let mut highest_green = 1;
        for game_iteration in game_scores {
            for dice in game_iteration.split(",").map(str::to_string) {
                match dice {
                    dice if dice.contains("blue") => {
                        let dice_count = get_dice_count(dice);
                        highest_blue = cmp::max(dice_count, highest_blue);
                    }
                    dice if dice.contains("green") => {
                        let dice_count = get_dice_count(dice);
                        highest_green = cmp::max(dice_count, highest_green);
                    }
                    dice if dice.contains("red") => {
                        let dice_count = get_dice_count(dice);
                        highest_red = cmp::max(dice_count, highest_red);
                    }
                    _ => {}
                }
            }
        }
        sum_of_powers += highest_blue * highest_green * highest_red;
    }
    println!("{}", sum_of_powers)
}
