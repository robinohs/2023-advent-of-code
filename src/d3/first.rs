use std::error::Error;

use aoc::read_lines;
use prev_iter::PrevPeekable;

use crate::data::Line;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D3-1!");
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
            .numbers
            .iter()
            .filter(|n| {
                println!("Checking {}", n.num);
                let mut positions_upper_lower_line = n.get_positions();
                let mut positions_same_line = vec![];
                // add index of left pos if possible
                if let Some(start) = positions_upper_lower_line.first().cloned() {
                    if start > 0 {
                        positions_upper_lower_line.insert(0, start - 1);
                        positions_same_line.insert(0, start - 1);
                    }
                }
                // add index of right pos if possible
                if let Some(end) = positions_upper_lower_line.last().cloned() {
                    if end < line.length {
                        positions_upper_lower_line.push(end + 1);
                        positions_same_line.push(end + 1);
                    }
                }

                println!("{:?}", positions_upper_lower_line);
                println!("{:?}", positions_same_line);
                println!("{:?}", positions_upper_lower_line);

                for pos in positions_same_line.clone() {
                    if line.has_symbol_at(pos) {
                        println!("SameLine: found at {}", pos);
                        return true;
                    }
                }

                for pos in positions_upper_lower_line.clone() {
                    if let Some(upper) = prev {
                        if upper.has_symbol_at(pos) {
                            println!("UpperLine: found at {}", pos);
                            return true;
                        }
                    }
                    if let Some(lower) = next {
                        if lower.has_symbol_at(pos) {
                            println!("LowerLine: found at {}", pos);
                            return true;
                        }
                    }
                }
                println!("No adjacent symbol found.");
                false
            })
            .map(|n| n.num)
            .sum::<u32>();
        overall_sum += sum;
    }

    println!("Result: {}", overall_sum);

    Ok(())
}
