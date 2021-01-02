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
    w: i32,
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
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
    rw: Range,
    grid: HashMap<Position, State>,
}

#[derive(Debug)]
struct Cube {
    pos: Position,
    state: State,
}

fn new_state(current_state: State, vs: &Vec<Cube>) -> State {
    let active_neighbors = vs.iter().filter(|&x| x.state == State::Active).count();
    if current_state == State::Active && (active_neighbors == 2 || active_neighbors == 3) {
        return State::Active;
    }
    if current_state == State::Inactive && active_neighbors == 3 {
        return State::Active;
    }
    State::Inactive
}

impl Pocket {
    fn new() -> Pocket {
        Pocket {
            grid: HashMap::new(),
            rx: Range { min: 0, max: 0 },
            ry: Range { min: 0, max: 0 },
            rz: Range { min: 0, max: 0 },
            rw: Range { min: 0, max: 0 },
        }
    }
    fn add_cube(&mut self, x: i32, y: i32, z: i32, w: i32, state: State) {
        let pos = Position { x, y, z, w };
        self.grid.insert(pos, state);
    }

    fn get_state(&self, x: i32, y: i32, z: i32, w: i32) -> State {
        if let Some(&state) = self.grid.get(&Position { x, y, z, w }) {
            return state;
        }
        return State::Inactive;
    }

    fn set_state(&mut self, pos: Position, new_state: State) {
        self.grid.insert(pos, new_state);
    }

    fn update_ranges(&mut self, pos: &Position) {
        if pos.x < self.rx.min {
            self.rx.min = pos.x;
        }
        if pos.y < self.ry.min {
            self.ry.min = pos.y;
        }
        if pos.z < self.rz.min {
            self.rz.min = pos.z;
        }
        if pos.w < self.rw.min {
            self.rw.min = pos.w;
        }
        if pos.x > self.rx.max {
            self.rx.max = pos.x;
        }
        if pos.y > self.ry.max {
            self.ry.max = pos.y;
        }
        if pos.z > self.rz.max {
            self.rz.max = pos.z;
        }
        if pos.w > self.rw.max {
            self.rw.max = pos.w;
        }
    }

    fn count_active(self) -> u32 {
        self.grid.iter().fold(0, |acc, (_, &state)| {
            if state == State::Active {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn neighbors(&self, pos: &Position) -> Vec<Cube> {
        let mut v: Vec<Cube> = Vec::new();
        for dw in -1..=1 {
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let x = pos.x + dx;
                        let y = pos.y + dy;
                        let z = pos.z + dz;
                        let w = pos.w + dw;

                        if x == pos.x && y == pos.y && z == pos.z && w == pos.w {
                            continue;
                        }
                        v.push(Cube {
                            pos: Position { x, y, z, w },
                            state: self.get_state(x, y, z, w),
                        });
                    }
                }
            }
        }
        v
    }

    fn cycle(&self) -> Pocket {
        let mut new_pocket: Pocket = Pocket::new();

        for (pos, state) in &self.grid {
            // update the state for the current cube
            let neighbors = self.neighbors(pos);
            new_pocket.set_state(*pos, new_state(*state, &neighbors));
            new_pocket.update_ranges(pos);
            // now update the status for the neighbors
            neighbors.iter().for_each(|cube| {
                let local_neighbors = self.neighbors(&cube.pos);
                new_pocket.set_state(cube.pos, new_state(cube.state, &local_neighbors));
                new_pocket.update_ranges(&cube.pos);
            });
        }
        new_pocket
    }
}

impl fmt::Display for Pocket {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vs: Vec<String> = Vec::new();

        for w in self.rw.min..=self.rw.max {
            for z in self.rz.min..=self.rz.max {
                vs.push(fmt::format(format_args!("z={} w={}\n", z, w)));
                for y in self.ry.min..=self.ry.max {
                    for x in self.rx.min..=self.rx.max {
                        let pos = Position { x, y, z, w };
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
        }

        write!(f, "{}", vs.join(""))
    }
}

fn load_fromfile(fname: &str) -> Pocket {
    let mut pocket: Pocket = Pocket::new();

    let z = 0;
    let w = 0;
    let mut y = 0;
    let mut max_x = 0;
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for raw_line in lines {
            if let Ok(line) = raw_line {
                let mut x = 0;
                for c in line.split("") {
                    if c == "#" {
                        pocket.add_cube(x, y, z, w, State::Active);
                        x += 1;
                    } else if c == "." {
                        pocket.add_cube(x, y, z, w, State::Inactive);
                        x += 1;
                    }
                    if max_x < x {
                        max_x = x;
                    }
                }
                y += 1;
            }
        }
    }
    pocket.rx.max = max_x;
    pocket.ry.max = y;
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
    println!("Pocket:\n{}", pocket);

    for _ in 0..6 {
        pocket = pocket.cycle();
        //println!("round {}:\n{}", i + 1, pocket);
    }

    println!("cube active: {}", pocket.count_active());
}
