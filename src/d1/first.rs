use std::error::Error;

use aoc::read_lines;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D1-1!");
    let result = read_lines("d1-input.txt")?
        .map(|line| line.unwrap())
        .map(|line| calculate_number(line))
        .sum::<u32>();
    println!("Result is: {}", result);
    Ok(())
}

fn calculate_number(line: String) -> u32 {
    let mut first_number = None;
    let mut second_number = None;
    line.chars()
        .filter(|c: &char| c.is_ascii_digit())
        .for_each(|c| {
            if first_number.is_none() {
                first_number = c.to_digit(10);
            }
            second_number = c.to_digit(10);
        });

    first_number.unwrap() * 10 + second_number.unwrap()
}
