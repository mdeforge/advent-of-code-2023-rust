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

    // what is the fewest number of cubes of each color that could have been in the bag to make the game possible
    pub fn find_min_set(&self) -> Set {
        Set::new(
            self.sets.iter().map(|set| set.reds).max().unwrap(),
            self.sets.iter().map(|set| set.greens).max().unwrap(),
            self.sets.iter().map(|set| set.blues).max().unwrap()
        )
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

fn main() -> std::io::Result<()> {
    
    let f = File::open("src/data/day2part1_data.txt")?;
    let reader = BufReader::new(f);

    let mut games: Vec<_> = Vec::<Game>::default();
    for line in reader.lines() {
        let game_line = line.unwrap();
        games.push(parse_game(game_line.as_str()).unwrap());
    }
    
    let mut total_power: i32 = 0;
    for game in &games {
        let min_set = game.find_min_set();
        let power = min_set.reds * min_set.greens * min_set.blues;
        total_power = total_power + power;
    }

    println!("Total power: {}", total_power);

    Ok(())
}