use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}
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

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32,
}

#[derive(Debug)]
struct Pocket {
    rx: Range,
    ry: Range,
    rz: Range,
    grid: HashMap<Position, State>,
}

struct Cube {
    pos: Position,
    state: State,
}

fn new_state(vs: &Vec<Cube>) -> State {
    let active_neighbors = vs.iter().filter(|&x| x.state == State::Active).count();
    if active_neighbors == 2 || active_neighbors == 3 {
        State::Active
    } else {
        State::Inactive
    }
}

impl Pocket {
    fn new() -> Pocket {
        Pocket {
            grid: HashMap::new(),
            rx: Range { min: 0, max: 0 },
            ry: Range { min: 0, max: 0 },
            rz: Range { min: 0, max: 0 },
        }
    }
    fn add_cube(&mut self, x: i32, y: i32, z: i32, state: State) {
        let pos = Position { x, y, z };
        self.grid.insert(pos, state);
    }

    fn get(&self, x: i32, y: i32, z: i32) -> State {
        if let Some(&state) = self.grid.get(&Position { x, y, z }) {
            return state;
        }
        return State::Inactive;
    }
    fn set(&mut self, pos: Position, new_state: State) {
        self.grid.insert(pos, new_state);
    }
    fn update_ranges(&mut self) {
        let mut not_init = true;
        let mut rx = Range { max: 0, min: 0 };
        let mut ry = Range { max: 0, min: 0 };
        let mut rz = Range { max: 0, min: 0 };

        for (pos, _) in &self.grid {
            if not_init || pos.x < rx.min {
                rx.min = pos.x;
            }
            if not_init || pos.y < ry.min {
                ry.min = pos.y;
            }
            if not_init || pos.z < rz.min {
                rz.min = pos.z;
            }
            if not_init || pos.x > rx.max {
                rx.max = pos.x;
            }
            if not_init || pos.y > ry.max {
                ry.max = pos.y;
            }
            if not_init || pos.z > rz.max {
                rz.max = pos.z;
            }
            not_init = false;
        }
        self.rx = rx;
        self.ry = ry;
        self.rz = rz;
    }

    fn neighbors(&self, pos: Position) -> Vec<Cube> {
        let mut v: Vec<Cube> = Vec::new();
        for dz in -1..1 {
            for dy in -1..1 {
                for dx in -1..1 {
                    let x = pos.x + dx;
                    let y = pos.y + dy;
                    let z = pos.z + dz;

                    if x == pos.x && y == pos.y && z == pos.z {
                        continue;
                    }
                    v.push(Cube {
                        pos: Position { x, y, z },
                        state: self.get(x, y, z),
                    });
                }
            }
        }
        v
    }

    fn cycle(&self) -> Pocket {
        let mut new_pocket: Pocket = Pocket::new();

        for (pos, _) in &self.grid {
            // update the state for the current cube
            let neighbors = self.neighbors(*pos);
            new_pocket.set(*pos, new_state(&neighbors));
            // now update the status for the neighbors
            neighbors.iter().for_each(|cube| {
                let local_neighbors = self.neighbors(cube.pos);
                new_pocket.set(*pos, new_state(&local_neighbors));
            });
        }
        new_pocket.update_ranges();

        new_pocket
    }
}

impl fmt::Display for Pocket {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vs: Vec<String> = Vec::new();

        for z in self.rz.min..self.rz.max {
            vs.push(fmt::format(format_args!("z={}\n", z)));
            for y in self.ry.min..self.ry.max {
                for x in self.rx.min..self.rx.max {
                    let pos = Position { x, y, z };
                    if let Some(&state) = self.grid.get(&pos) {
                        if state == State::Active {
                            vs.push("#".to_owned())
                        } else {
                            vs.push(".".to_owned())
                        }
                    } else {
                        vs.push(".".to_owned())
                    }
                }
                vs.push("\n".to_owned());
            }
            vs.push("\n".to_owned());
        }

        write!(f, "{}", vs.join(""))
    }
}

fn load_fromfile(fname: &str) -> Pocket {
    let mut pocket: Pocket = Pocket::new();

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
    pocket.update_ranges();
    pocket
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut pocket = load_fromfile("./data/input.txt");

    println!("Pocket:\n{:?}", pocket);

    for i in 0..1 {
        pocket = pocket.cycle();
        println!("round {}:\n{}", i + 1, pocket);
    }
}
