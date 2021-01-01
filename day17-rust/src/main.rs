use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum State {
    Active,
    Inactive,
}

#[derive(Debug, Eq)]
struct Position{x: i32, y: i32, z: i32}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

struct Pocket {
    grid: HashMap<Position, State>,
}

impl Pocket {
    fn add_cube(&mut self, x: i32, y: i32, z: i32, state: State) {
        let pos = Position{x, y, z};
        self.grid.insert(pos, state);
    }
}

fn load_fromfile(fname: &str) -> Pocket {
    let mut pocket: Pocket = Pocket{
        grid: HashMap::new(),
    };

    let z = 0;
    let mut y = 0;
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for raw_line in lines {
            if let Ok(line) = raw_line {
                let mut x = 0;
                for c in line.split("") {
                    let mut state: State = State::Inactive;
                    if c == "#" {
                        state = State::Active;
                    }
                    pocket.add_cube(x, y, z, state);
                    x += 1;
                }
                y += 1;
            }
        }
    }

    pocket
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let pocket = load_fromfile("./data/input.txt");

    println!("Pocket {}", pocket.grid.len());
}
