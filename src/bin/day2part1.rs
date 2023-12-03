use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug, PartialEq)]
pub struct Set {
    pub reds: i32,
    pub blues: i32,
    pub greens: i32,
}

impl Set {
    pub fn new(reds: i32, blues: i32, greens: i32) -> Self {
        Self {
            reds: reds,
            blues: blues,
            greens: greens,
        }
    }
}

#[derive(Default)]
struct Game {
    num: i32,
    sets: Vec<Set>,
}

impl Game {
    pub fn print(&self) {
        print!("Game {}: ", self.num);
        for set in &self.sets {
            print!(
                "{} red, {} green, {} blue; ",
                set.reds, set.greens, set.blues
            )
        }
        println!("");
    }
}

fn parse_set(line: &str) -> Option<Set> {
    let mut set = Set::default();

    let cubes: Vec<_> = line.split(',').map(|x| x.trim_start()).collect();
    for cube_line in cubes {
        let cube_info: Vec<_> = cube_line.split(' ').collect();
        let cube_amount: i32 = cube_info[0].parse().unwrap();
        let cube_type = cube_info[1];

        match cube_type {
            "red" => set.reds = set.reds + cube_amount,
            "green" => set.greens = set.greens + cube_amount,
            "blue" => set.blues = set.blues + cube_amount,
            _ => return None,
        }
    }

    Some(set)
}

fn parse_game(line: &str) -> Option<Game> {
    let mut game = Game::default();

    // get game number
    let side: Vec<_> = line.split(':').collect();

    game.num = side[0].split(' ').collect::<Vec<_>>()[1].parse().unwrap();

    // 'lil split and trim action
    let sets: Vec<_> = side[1].split(';').map(|x| x.trim_start()).collect();
    for set_line in sets {
        let set = parse_set(set_line);
        game.sets.push(set.unwrap());
    }

    Some(game)
}

const NUM_RED_CUBES: i32 = 12;
const NUM_GREEN_CUBES: i32 = 13;
const NUM_BLUE_CUBES: i32 = 14;

fn main() -> std::io::Result<()> {
    
    let f = File::open("src/data/day2part1_data.txt")?;
    let reader = BufReader::new(f);

    let mut games: Vec<Game> = Vec::<Game>::default();
    for line in reader.lines() {
        let game_line = line.unwrap();
        games.push(parse_game(game_line.as_str()).unwrap());
    }

    let mut valid_games: Vec<_> = Vec::<i32>::default();
    for game in games {
        let mut is_valid = true;
        for set in game.sets {
            if set.reds <= NUM_RED_CUBES && set.greens <= NUM_GREEN_CUBES && set.blues <= NUM_BLUE_CUBES {
                continue;
            } else {
                is_valid = false;
            }
        }
        
        if is_valid {
            valid_games.push(game.num)
        }
    }

    let sum: i32 = valid_games.iter().sum();
    println!("Valid games: {:?}", valid_games);
    println!("Sum of valid games: {}", sum);

    Ok(())
}