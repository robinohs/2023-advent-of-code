use std::error::Error;

use aoc::read_lines;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D1-2!");
    let result = read_lines("d1-input.txt")? // reuse of input
        .map(|line| line.unwrap())
        .map(|line| calculate_number(line))
        .sum::<u32>();
    println!("Result is: {}", result);
    Ok(())
}

fn calculate_number(line: String) -> u32 {
    let mut first_number = None;
    let mut second_number = None;

    for (pos, char) in line.char_indices() {
        if char.is_digit(10) {
            let num = char.to_digit(10);
            if first_number.is_none() {
                first_number = num;
            }
            second_number = num;
        } else {
            let mut num = None;
            let slice = &line[pos..];
            if slice.starts_with("one") {
                num = Some(1);
            } else if slice.starts_with("two") {
                num = Some(2);
            } else if slice.starts_with("three") {
                num = Some(3);
            } else if slice.starts_with("four") {
                num = Some(4);
            } else if slice.starts_with("five") {
                num = Some(5);
            } else if slice.starts_with("six") {
                num = Some(6);
            } else if slice.starts_with("seven") {
                num = Some(7);
            } else if slice.starts_with("eight") {
                num = Some(8);
            } else if slice.starts_with("nine") {
                num = Some(9);
            }
            if num.is_some() {
                if first_number.is_none() {
                    first_number = num;
                }
                second_number = num;
            }
        }
    }

    first_number.unwrap() * 10 + second_number.unwrap()
}
