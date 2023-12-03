use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Number {
    pub line: usize,
    pub num: u32,
    pub start: usize,
    pub end: usize,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.num == other.num
            && self.start == other.start
            && self.end == other.end
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.line.hash(state);
        self.num.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl Eq for Number {}

impl Number {
    fn parse(raw: &str, line: usize, start: usize, end: usize) -> Number {
        let num = raw.parse::<u32>().unwrap();
        Number {
            line,
            num,
            start,
            end,
        }
    }

    pub fn get_positions(&self) -> Vec<usize> {
        (self.start..=self.end).collect()
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub raw: char,
    pub pos: usize,
}

impl Symbol {
    fn new(char: char, pos: usize) -> Symbol {
        Symbol { raw: char, pos }
    }
}

#[derive(Debug)]
pub struct Line {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
    pub length: usize,
}

impl Line {
    pub fn parse(index: usize, line: String) -> Line {
        let mut buffer = String::new();
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        let mut numbers = vec![];
        let mut symbols = vec![];

        for (pos, char) in line.char_indices() {
            match char {
                char if char.is_ascii_digit() => {
                    start.get_or_insert(pos);
                    end = Some(pos);
                    buffer.push(char)
                }
                '.' => {
                    // handle old buffer
                    if !buffer.is_empty() {
                        numbers.push(Number::parse(
                            &buffer,
                            index,
                            start.take().unwrap(),
                            end.take().unwrap(),
                        ));
                        buffer.clear();
                    }
                }
                char => {
                    // handle old buffer
                    if !buffer.is_empty() {
                        numbers.push(Number::parse(
                            &buffer,
                            index,
                            start.take().unwrap(),
                            end.take().unwrap(),
                        ));
                        buffer.clear();
                    }

                    symbols.push(Symbol::new(char, pos));
                }
            }
        }

        // handle if there is still something in buffer
        if !buffer.is_empty() {
            numbers.push(Number::parse(
                &buffer,
                index,
                start.take().unwrap(),
                end.take().unwrap(),
            ));
            buffer.clear();
        }

        Line {
            numbers,
            symbols,
            length: line.len(),
        }
    }

    pub fn has_symbol_at(&self, pos: usize) -> bool {
        self.symbols.iter().any(|s| s.pos == pos)
    }

    pub fn get_number_at(&self, pos: usize) -> Option<&Number> {
        self.numbers
            .iter()
            .find(|n| n.get_positions().contains(&pos))
    }
}
