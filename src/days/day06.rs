#[derive(Copy, Clone)]
struct Game {
    max_time: i64,
    best_distance: i64,
}

#[derive(Copy, Clone)]
struct GameState {
    game: Game,
    travel_distance: i64,
}
impl GameState {
    fn is_winning(self) -> bool {
        return self.travel_distance > self.game.best_distance;
    }
}

impl Game {
    fn new(t: i64, d: i64) -> Game {
        Self {
            max_time: t,
            best_distance: d,
        }
    }

    fn get_possible_states(self) -> Vec<GameState> {
        let mut possible_states = Vec::new();

        for charge_time in 0..self.max_time {
            let ms_remaining = self.max_time - charge_time;
            let travel_distance = ms_remaining * charge_time;
            possible_states.insert(
                possible_states.len(),
                GameState {
                    game: self,
                    travel_distance,
                },
            )
        }

        return possible_states;
    }
}

pub fn solve_part_one(lines: Vec<String>) {
    let time: Vec<i64> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let distance: Vec<i64> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut result = 1;
    for i in 0..time.len() {
        let game = Game::new(time[i], distance[i]);
        let winning_games_count = game
            .get_possible_states()
            .iter()
            .filter(|state| state.is_winning())
            .collect::<Vec<&GameState>>()
            .len();

        if winning_games_count > 0 {
            result *= winning_games_count;
        }
    }

    println!("day6_1 {}", result);
}

pub fn solve_part_two(lines: Vec<String>) {
    let time: i64 = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distance: i64 = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let mut result = 1;
    let game = Game::new(time, distance);
    let winning_games_count = game
        .get_possible_states()
        .iter()
        .filter(|state| state.is_winning())
        .collect::<Vec<&GameState>>()
        .len();

    if winning_games_count > 0 {
        result *= winning_games_count;
    }
    println!("day6_2 {}", result);
}
