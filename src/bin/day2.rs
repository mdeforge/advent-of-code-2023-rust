use std::default;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Default)]
struct Game {
    num: i32,
    reds: i32,
    blues: i32,
    greens: i32
}

fn main() -> std::io::Result<()> {
    let f = File::open("src/bin/day2part1_data.txt")?;
    let reader = BufReader::new(f);

    // Game | 1 |  | 3 | blue |  | 4 | red |  | 1 | red |  | 2 | green |  | 6 | blue |  | 2 | 
    let mut games: Vec<Game> = Vec::<Game>::default();

    for line in reader.lines() {
        // get game number
        let game_half: Vec<&str> = line?.split(':').collect();
        let game_part: Vec<&str> = game_half[0].split(' ').collect();
        let num: i32 = game_part[1].parse().unwrap();

        //let game_section: Vec<&str> = game.split(&[' ', ':', ' ', ';', ','][..]).collect();
        //game_sum = game_sum + game_section
        // for v in game_section {
        //     print!("{} | ", v);
        // }

        println!("{}", num);
    }

    Ok(())
}