use std::{
    cmp::{max, min},
    collections::HashMap,
    error::Error,
};

use aoc::read_lines;

use crate::data::Card;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from D4-1");
    let cards = read_lines("d4-input.txt")?
        .map(|l| l.unwrap())
        .map(|f| Card::try_parse(&f).unwrap())
        .map(|c| (c.id, c))
        .collect::<HashMap<usize, Card>>();

    let mut open_cards: Vec<Card> = cards.iter().map(|(_, b)| b.clone()).collect();

    let max_id = open_cards.len();

    let mut card_count = 0;

    while let Some(card) = open_cards.pop() {
        card_count += 1;
        let winners = card.id + 1..=min(max_id, card.id + card.drawn_winner_count() as usize);
        for c in winners {
            open_cards.push(cards.get(&c).cloned().unwrap());
        }
    }

    // let result = deck.iter().map(|(_, v)| v).sum::<usize>();

    println!("Result: {}", card_count);
    Ok(())
}
