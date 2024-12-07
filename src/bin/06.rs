use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({};{}) [{}]",
            self.position.x, self.position.y, self.direction
        )
    }
}

impl Guard {
    pub fn tick(&self) -> Result<Self, ()> {
        let new_position = match self.direction {
            Direction::Up => Position {
                x: self.position.x,
                y: if self.position.y == 0 {return Err(())} else {self.position.y.checked_sub(1).expect("Y guard too low")},
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y.checked_add(1).expect("Y guard too hight"),
            },
            Direction::Right => Position {
                x: self.position.x.checked_add(1).expect("X guard too hight"),
                y: self.position.y,
            },
            Direction::Left => Position {
                x: if self.position.x == 0 {return Err(())} else {self.position.x.checked_sub(1).expect("X guard too low")},
                y: self.position.y,
            },
        };

        Ok(Self {
            direction: self.direction,
            position: new_position
        })
    }

    pub fn turn(&mut self) -> Self {
        Self {
            position: self.position,
            direction: match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Right => write!(f, ">"),
            Direction::Left => write!(f, "<"),
        }
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            _ => Err(ParseError),
        }
    }
}

/* ============================================================================ */

#[derive(Copy, Clone)]
enum Glyph {
    Empty,
    Guard(Direction),
    Block,
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Glyph::Empty => write!(f, "."),
            Glyph::Guard(c) => write!(f, "{}", c),
            Glyph::Block => write!(f, "#"),
        }
    }
}

impl FromStr for Glyph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Glyph::Block),
            "." => Ok(Glyph::Empty),
            "^" | "v" | ">" | "<" => Ok(Glyph::Guard(
                Direction::from_str(s).expect("Invalid Direction"),
            )),
            _ => Err(ParseError),
        }
    }
}

/* ============================================================================ */

struct LabMap {
    pub grid: Vec<Vec<Glyph>>,
    pub width: usize,
    pub height: usize,
    pub guard: Guard,
}

impl FromStr for LabMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard = Guard {
            position: Position {
                x: usize::default(),
                y: usize::default(),
            },
            direction: Direction::Up,
        };
        let grid: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(
                        |(x, c)| match Glyph::from_str(&c.to_string()).expect("Invalid Glyph") {
                            Glyph::Empty => Glyph::Empty,
                            Glyph::Guard(direction) => {
                                guard = Guard {
                                    position: Position { x, y },
                                    direction,
                                };
                                Glyph::Empty
                            }
                            Glyph::Block => Glyph::Block,
                        },
                    )
                    .collect()
            })
            .collect();

        // Compute grid dimension
        let width = grid[0].len();
        let height = grid.len();

        Ok(Self {
            grid,
            width,
            height,
            guard,
        })
    }
}

impl Display for LabMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lab Map [{} x {}]:", self.width, self.height)?;
        for line in &self.grid {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "=> Guard ({};{}):",
            self.guard.position.x, self.guard.position.y
        )?;
        Ok(())
    }
}

impl LabMap {
    pub fn get_glyph(&self, position: Position) -> Option<Glyph> {
        if position.x > self.width { return None }
        if position.y > self.height { return None }
        self.grid.get(position.y)
            .and_then(|row| row.get(position.x)).copied()
    }

    pub fn next_pos(&self, guard: &Guard) -> Option<Guard> {
        match guard.direction {
            Direction::Up => {
                if guard.position.y == 0 { return None }
                match self
                    .grid
                    .get(guard.position.y - 1)
                    .and_then(|row| row.get(guard.position.x))
                {
                    Some(Glyph::Empty) => Some(guard.tick().unwrap()),
                    Some(Glyph::Block) => {
                        // // eprintln!("Meet block");
                        let g = guard.clone().turn();
                        Some(g.tick().unwrap())
                    }
                    Some(Glyph::Guard(_)) => unreachable!(),
                    None => None,
                }
            }

            Direction::Down => {
                if guard.position.y  == self.height { return None }
                match self
                    .grid
                    .get(guard.position.y + 1)
                    .and_then(|row| row.get(guard.position.x))
                {
                    Some(Glyph::Empty) => Some(guard.tick().unwrap()),
                    Some(Glyph::Block) => {
                        // // eprintln!("Meet block");
                        let g = guard.clone().turn();
                        Some(g.tick().unwrap())
                    }
                    Some(Glyph::Guard(_)) => unreachable!(),
                    None => None,
                }
            }
            Direction::Right => {
                if guard.position.x == self.width { return None }
                match self
                    .grid
                    .get(guard.position.y)
                    .and_then(|row| row.get(guard.position.x + 1))
                {
                    Some(Glyph::Empty) => Some(guard.tick().unwrap()),
                    Some(Glyph::Block) => {
                        // eprintln!("Meet block");
                        let g = guard.clone().turn();
                        Some(g.tick().unwrap())
                    }
                    Some(Glyph::Guard(_)) => unreachable!(),
                    None => None,
                }
            }
            Direction::Left => {
                if guard.position.x == 0 { return None }
                match self
                    .grid
                    .get(guard.position.y)
                    .and_then(|row| row.get(guard.position.x - 1))
                {
                    Some(Glyph::Empty) => Some(guard.tick().unwrap()),
                    Some(Glyph::Block) => {
                        // eprintln!("Meet block");
                        let g = guard.clone().turn();
                        Some(g.tick().unwrap())
                    }
                    Some(Glyph::Guard(_)) => unreachable!(),
                    None => None,
                }
            }
        }
    }
}

/* ============================================================================ */

pub fn part_one(input: &str) -> Option<u32> {
    let lab_map = LabMap::from_str(input).expect("Invalid Map");

    let mut cur_guard_pos = lab_map.guard;
    let mut pos_rec: HashSet<Position> = HashSet::new();
    pos_rec.insert(cur_guard_pos.position);

    // eprintln!("Starting => {}", cur_guard_pos);
    loop {
        cur_guard_pos = match lab_map.next_pos(&cur_guard_pos) {
            Some(v) => {
                // eprintln!("Move {}", v);
                pos_rec.insert(cur_guard_pos.position);
                v
            }
            None => break,
        };
    }

    Some(pos_rec.len() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lab_map = LabMap::from_str(input).expect("Invalid Map");

    let mut cur_guard_pos = lab_map.guard;
    let mut paradox_pos_rec: HashSet<Position> = HashSet::new();
    let mut pos_rec: HashSet<Guard> = HashSet::new();
    pos_rec.insert(cur_guard_pos);
    paradox_pos_rec.insert(cur_guard_pos.position);

    // eprintln!("Starting => {}", cur_guard_pos);
    loop {
        cur_guard_pos = match lab_map.next_pos(&cur_guard_pos) {
            Some(v) => {
                // eprintln!("Move {}", v);
                pos_rec.insert(cur_guard_pos);

                // If i do a turn/tick => found similar guard than previously?
                let g = cur_guard_pos.clone().turn();
                let g = g.tick().unwrap();
                if pos_rec.contains(&g) {
                    // Compute paradoxical block
                    if let Ok(paradox_pos) = cur_guard_pos.clone().tick() {
                        if let Some(Glyph::Empty) = lab_map.get_glyph(paradox_pos.position) {
                            paradox_pos_rec.insert(paradox_pos.position);
                        }
                    }
                }

                v
            }
            None => break,
        };
    }

    Some(paradox_pos_rec.len() as u32 + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
