use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Display, Formatter},
    str::FromStr,
};

macro_rules! is_between {
    ($value:expr, $min:expr, $max:expr) => {{
        $value >= $min && $value < $max
    }};
}

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

/* ============================================================================ */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn antinodes(
        left: &Point,
        right: &Point,
        max_x_pos: usize,
        max_y_pos: usize,
    ) -> Vec<Point> {
        let left_2_right_x = right.x - left.x;
        let left_2_right_y = right.y - left.y;
        let mut res = vec![];

        let antinode_1 = Point {
            x: left.x - left_2_right_x,
            y: left.y - left_2_right_y,
        };

        let antinode_2 = Point {
            x: right.x + left_2_right_x,
            y: right.y + left_2_right_y,
        };

        let antinode_3 = Point {
            x: left.x + left_2_right_x,
            y: left.y + left_2_right_y,
        };

        let antinode_4 = Point {
            x: right.x - left_2_right_x,
            y: right.y - left_2_right_y,
        };

        if antinode_1 != *left
            && antinode_1 != *right
            && is_between!(antinode_1.x, 0, max_x_pos as i32)
            && is_between!(antinode_1.y, 0, max_y_pos as i32)
        {
            res.push(antinode_1)
        }

        if antinode_2 != *left
            && antinode_2 != *right
            && is_between!(antinode_2.x, 0, max_x_pos as i32)
            && is_between!(antinode_2.y, 0, max_y_pos as i32)
        {
            res.push(antinode_2)
        }

        if antinode_3 != *left
            && antinode_3 != *right
            && is_between!(antinode_3.x, 0, max_x_pos as i32)
            && is_between!(antinode_3.y, 0, max_y_pos as i32)
        {
            res.push(antinode_3)
        }

        if antinode_4 != *left
            && antinode_4 != *right
            && is_between!(antinode_4.x, 0, max_x_pos as i32)
            && is_between!(antinode_4.y, 0, max_y_pos as i32)
        {
            res.push(antinode_4)
        }

        res
    }

    pub fn antinodes2(
        antennas: HashMap<String, Vec<Point>>,
        max_x_pos: usize,
        max_y_pos: usize,
    ) -> HashSet<Point> {
        let mut antinodes: HashSet<Point> = HashSet::new();

        for (_, positions) in antennas {
            let n = positions.len();

            for i in 0..n {
                for j in i + 1..n {
                    let p1 = positions[i];
                    let p2 = positions[j];

                    // Add antenna
                    antinodes.insert(p1);
                    antinodes.insert(p2);

                    // Compute align
                    let dx = p2.x - p1.x;
                    let dy = p2.y - p1.y;

                    for k in 1.. {
                        let antinode_1 = Point {
                            x: p1.x + k * dx,
                            y: p1.y + k * dy,
                        };
                        let antinode_2 = Point {
                            x: p2.x - k * dx,
                            y: p2.y - k * dy,
                        };

                        let mut have_break = true;
                        if is_between!(antinode_1.x, 0, max_x_pos as i32)
                            && is_between!(antinode_1.y, 0, max_y_pos as i32)
                        {
                            antinodes.insert(antinode_1);
                            have_break = false;
                        }

                        if is_between!(antinode_2.x, 0, max_x_pos as i32)
                            && is_between!(antinode_2.y, 0, max_y_pos as i32)
                        {
                            antinodes.insert(antinode_2);
                            have_break = false;
                        }

                        if have_break {
                            break;
                        }
                    }
                }
            }
        }

        antinodes
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:02};{:02}]", self.x, self.y)
    }
}

/* ============================================================================ */

#[derive(Clone)]
enum Glyph {
    Empty,
    Antenna(String),
    AntiNode,
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Glyph::Empty => write!(f, "."),
            Glyph::Antenna(c) => write!(f, "{}", c),
            Glyph::AntiNode => write!(f, "#"),
        }
    }
}

impl FromStr for Glyph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Glyph::Empty),
            "#" => Ok(Glyph::AntiNode),
            _ => Ok(Glyph::Antenna(s.to_owned())),
        }
    }
}

/* ============================================================================ */

struct Map {
    pub grid: Vec<Vec<Glyph>>,
    pub width: usize,
    pub height: usize,
    pub antenna_list: HashMap<String, Vec<Point>>,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antenna_list: HashMap<String, Vec<Point>> = HashMap::new();

        // Parse
        let grid: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match Glyph::from_str(&c.to_string()) {
                        Ok(Glyph::Antenna(label)) => {
                            let values = match antenna_list.entry(label) {
                                Entry::Occupied(o) => o.into_mut(),
                                Entry::Vacant(v) => v.insert(vec![]),
                            };
                            values.push(Point {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            });
                            Glyph::Antenna(c.to_string())
                        }
                        _ => Glyph::Empty,
                    })
                    .collect()
            })
            .collect();

        // Compute grid dimension
        let width = s.lines().last().unwrap().chars().count();
        let height = s.lines().count();

        Ok(Self {
            grid,
            width,
            height,
            antenna_list,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lab Map [{} x {}]:", self.width, self.height)?;
        for line in &self.grid {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/* ============================================================================ */

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = Map::from_str(input).expect("Invalid input");
    let mut antinode_list: HashSet<Point> = HashSet::new();

    for (_, pos) in map.antenna_list.iter() {
        let pairs = pos.iter().combinations(2);
        for pair in pairs {
            Point::antinodes(pair[0], pair[1], map.width, map.height)
                .into_iter()
                .for_each(|antinode| {
                    antinode_list.insert(antinode);
                });
        }
    }

    // for pt in antinode_list.clone() {
    //
    // }

    for pt in antinode_list.clone() {
        if let Some(row) = map.grid.get_mut(pt.y as usize) {
            row[pt.x as usize] = Glyph::AntiNode
        }
    }

    Some(antinode_list.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from_str(input).expect("Invalid input");
    Some(Point::antinodes2(map.antenna_list, map.width, map.height).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // 361
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
