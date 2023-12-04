use std::{cmp::max, error::Error};

use aoc::read_lines;

use crate::data::Card;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D4-1");
    let result = read_lines("d4-input.txt")?
        .map(|l| l.unwrap())
        .map(|f| Card::try_parse(&f))
        .collect::<Result<Vec<Card>, Box<dyn Error>>>()?
        .iter()
        .map(|c| c.drawn_winner_count())
        // .inspect(|w_c| {
        //     println!("{}", w_c);
        // })
        .map(|winner_count| match winner_count {
            count if count > 0 => 2_u32.pow(winner_count - 1),
            _ => 0,
        })
        .sum::<u32>();

    println!("Result: {}", result);
    Ok(())
}
