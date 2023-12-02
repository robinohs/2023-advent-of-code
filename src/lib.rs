use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(name: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let path = Path::new(name);
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
