use std::{collections::HashSet, error::Error};

use aoc::read_lines;
use prev_iter::PrevPeekable;

use crate::data::Line;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D3-2!");
    let mut index = 0;
    let lines: Vec<Line> = read_lines("d3-input.txt")?
        // .take(15)
        .map(|line| line.unwrap())
        .map(|line| {
            let tmp = index;
            index += 1;
            Line::parse(tmp, line)
        })
        .collect();

    let mut overall_sum = 0;
    let mut iter = PrevPeekable::new(lines.iter());

    while let Some(line) = iter.next() {
        let prev = iter.prev();
        let next = iter.peek();
        let sum = line
            .symbols
            .iter()
            .filter(|s| s.raw == '*')
            .map(|s| {
                println!("Checking {} at {}", s.raw, s.pos);
                let mut neighbors = HashSet::new();

                let mut positions_upper_lower_line = vec![s.pos];
                let mut positions_same_line = vec![];
                if s.pos > 0 {
                    positions_upper_lower_line.insert(0, s.pos - 1);
                    positions_same_line.insert(0, s.pos - 1);
                }
                if s.pos < line.length {
                    positions_upper_lower_line.push(s.pos + 1);
                    positions_same_line.push(s.pos + 1);
                }

                println!("{:?}", positions_upper_lower_line);
                println!("{:?}", positions_same_line);
                println!("{:?}", positions_upper_lower_line);

                for pos in positions_same_line.clone() {
                    if let Some(num) = line.get_number_at(pos) {
                        println!("SameLine: found {} at {}", num.num, pos);
                        neighbors.insert(num);
                    }
                }

                for pos in positions_upper_lower_line.clone() {
                    if let Some(upper) = prev {
                        if let Some(num) = upper.get_number_at(pos) {
                            println!("UpperLine: found {} at {}", num.num, pos);
                            neighbors.insert(num);
                        }
                    }
                    if let Some(lower) = next {
                        if let Some(num) = lower.get_number_at(pos) {
                            println!("LowerLine: found {} at {}", num.num, pos);
                            neighbors.insert(num);
                        }
                    }
                }

                println!(
                    "Number of neighbors: {}| Gear ratio: {}.",
                    neighbors.len(),
                    neighbors.iter().map(|n| n.num).product::<u32>()
                );
                if neighbors.len() == 2 {
                    neighbors.iter().map(|n| n.num).product()
                } else {
                    0
                }
            })
            .sum::<u32>();
        overall_sum += sum;
    }

    println!("Result: {}", overall_sum);

    Ok(())
}
