#[derive(Debug)]
pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn parse(draw: &str) -> Set {
        let mut red = None;
        let mut green = None;
        let mut blue = None;
        for cube in draw.split(",") {
            let cube = cube.trim();
            let vals: Vec<&str> = cube.split(" ").collect();
            let num = vals[0].parse::<u32>().unwrap();
            let color = vals[1];

            match color {
                "red" => red = Some(num),
                "green" => green = Some(num),
                "blue" => blue = Some(num),
                _ => panic!("Unexpected color!"),
            };
        }
        Set {
            red: red.unwrap_or(0),
            green: green.unwrap_or(0),
            blue: blue.unwrap_or(0),
        }
    }

    fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn parse(line: String) -> Game {
        let line = &line[5..];
        let data: Vec<&str> = line.split(":").collect();

        // extract id
        let id = data[0].parse::<u32>().unwrap();

        // extract draws
        let sets = data[1]
            .trim()
            .split(";")
            .map(|draw| draw.trim())
            .map(|draw| Set::parse(draw))
            .collect();

        Game { id, sets }
    }

    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.sets
            .iter()
            .filter(|s| !s.possible(red, green, blue))
            .count()
            == 0
    }

    pub fn power(&self) -> u32 {
        let red = self.sets.iter().map(|s| s.red).max().unwrap_or(0);
        let green = self.sets.iter().map(|s| s.green).max().unwrap_or(0);
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap_or(0);
        red * green * blue
    }
}
