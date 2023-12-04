use std::default;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl Grid {
    pub fn new(width: usize, height: usize, fill: char) -> Self {
        Self {
            cells: vec![vec![fill; width]; height],
            width: width,
            height: height
        }
    }

    pub fn at(&self, x: i32, y: i32) -> Option<char> {


        None
    }

    pub fn print(&self) {
        // This seems like the prettiest way to print a 2d grid
        // println!("{:?}", self.cells) has a bunch of brackets in it
        self.cells.iter().for_each(|it| {
            println!("{:?}", it);
        })
    }
}

fn main() -> std::io::Result<()> {
    
    let f = File::open("src/data/day3part1_test.txt")?;
    let mut reader = BufReader::new(f);
    
    //let mut buf = String::new();
    //let bytes = reader.read_line(&mut buf)?;
    //let size = buf.trim().chars().count();

    //println!("Size: {}", size);

    let mut flat: Vec<_> = Vec::<char>::default();
    //let grid = Grid::new(size, size, '.');

    for line in reader.lines() {
        for c in line?.trim().chars() {
            flat.push(c);
        }
    }

    let v = 2;
    println!("{}", flat[v]);
    println!("{}", flat[v + 11]);

    //grid.print();

    Ok(())
}