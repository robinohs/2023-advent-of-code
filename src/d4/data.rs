use std::{collections::HashSet, error::Error, num::ParseIntError};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    winners: HashSet<u32>,
    draws: HashSet<u32>,
}

impl Card {
    pub fn try_parse(raw: &str) -> Result<Card, Box<dyn Error>> {
        let re = Regex::new(
            r"Card(?: )*(?<id>\d+):(?: )*(?<winners>(?:(?:\d+)(?: )*)+)\|(?: )*(?<draws>(?:(?:\d+)(?: )*)+)",
        )?;

        let caps = re
            .captures(raw)
            .ok_or("Wrong format, expected: Card <id>: [<winner>+] | [<draw>+]")?;

        // Parse the ID
        let id = caps.name("id").unwrap().as_str().trim();
        let id = id.parse::<usize>()?;

        let re_number = Regex::new(r"(\d+)").unwrap();

        // parse winner numbers
        let winners_raw = caps.name("winners").unwrap().as_str().trim();
        let winners = re_number
            .find_iter(winners_raw)
            .map(|m| m.as_str())
            .map(|s| s.parse::<u32>())
            .collect::<Result<HashSet<u32>, ParseIntError>>()?;

        // parse draw numbers
        let draws_raw = caps.name("draws").unwrap().as_str().trim();
        let draws = re_number
            .find_iter(draws_raw)
            .map(|m| m.as_str())
            .map(|s| s.parse::<u32>())
            .collect::<Result<HashSet<u32>, ParseIntError>>()?;

        let card = Card { id, winners, draws };
        Ok(card)
    }

    pub fn drawn_winner_count(&self) -> u32 {
        self.draws
            .iter()
            .filter(|v| self.winners.contains(v))
            .count() as u32
    }
}
