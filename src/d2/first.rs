use std::error::Error;

use aoc::read_lines;

use crate::data::Game;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D2-1!");
    let sum = read_lines("d2-input.txt")?
        .map(|line| line.unwrap())
        .map(|line| Game::parse(line))
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum::<u32>();
    println!("Result is: {}", sum);
    Ok(())
}
