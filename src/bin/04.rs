use std::{fmt::{Display, Formatter}, str::FromStr};

advent_of_code::solution!(4);

static BORDER: char = '#';


#[derive(Debug, PartialEq, Eq)]
pub struct ParseElfWordsError;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EflWords {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl EflWords {
    fn find_pattern(&self, x: usize, y: usize) -> u32 {
        let mut counter = 0_u32;

        let x = x + 3;
        let y = y + 3;

         // hz right
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x+1).unwrap().get(y).unwrap() == 'M' &&
        *self.grid.get(x+2).unwrap().get(y).unwrap() == 'A' &&
        *self.grid.get(x+3).unwrap().get(y).unwrap() == 'S' { counter += 1}

        // diag up right
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x+1).unwrap().get(y-1).unwrap() == 'M' &&
        *self.grid.get(x+2).unwrap().get(y-2).unwrap() == 'A' &&
        *self.grid.get(x+3).unwrap().get(y-3).unwrap() == 'S' { counter += 1}

         // up
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x).unwrap().get(y-1).unwrap() == 'M' &&
        *self.grid.get(x).unwrap().get(y-2).unwrap() == 'A' &&
        *self.grid.get(x).unwrap().get(y-3).unwrap() == 'S' { counter += 1}

        // diag up left
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x-1).unwrap().get(y-1).unwrap() == 'M' &&
        *self.grid.get(x-2).unwrap().get(y-2).unwrap() == 'A' &&
        *self.grid.get(x-3).unwrap().get(y-3).unwrap() == 'S' { counter += 1}

        // hz left
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x-1).unwrap().get(y).unwrap() == 'M' &&
        *self.grid.get(x-2).unwrap().get(y).unwrap() == 'A' &&
        *self.grid.get(x-3).unwrap().get(y).unwrap() == 'S' { counter += 1}

        // dialg down left
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x-1).unwrap().get(y+1).unwrap() == 'M' &&
        *self.grid.get(x-2).unwrap().get(y+2).unwrap() == 'A' &&
        *self.grid.get(x-3).unwrap().get(y+3).unwrap() == 'S' { counter += 1}

        // left
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x).unwrap().get(y+1).unwrap() == 'M' &&
        *self.grid.get(x).unwrap().get(y+2).unwrap() == 'A' &&
        *self.grid.get(x).unwrap().get(y+3).unwrap() == 'S' { counter += 1}

        // diag down right
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'X' &&
        *self.grid.get(x+1).unwrap().get(y+1).unwrap() == 'M' &&
        *self.grid.get(x+2).unwrap().get(y+2).unwrap() == 'A' &&
        *self.grid.get(x+3).unwrap().get(y+3).unwrap() == 'S' { counter += 1}

        counter
    }

    fn find_pattern2(&self, x: usize, y: usize) -> u32 {
        let mut counter = 0_u32;

        let x = x + 3;
        let y = y + 3;

        // Reverse
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'A' &&
        *self.grid.get(x-1).unwrap().get(y-1).unwrap() == 'S' &&
        *self.grid.get(x+1).unwrap().get(y-1).unwrap() == 'S' &&
        *self.grid.get(x-1).unwrap().get(y+1).unwrap() == 'M' &&
        *self.grid.get(x+1).unwrap().get(y+1).unwrap() == 'M' { counter += 1}

        // order
        if  *self.grid.get(x).unwrap().get(y).unwrap() == 'A' &&
        *self.grid.get(x-1).unwrap().get(y-1).unwrap() == 'M' &&
        *self.grid.get(x+1).unwrap().get(y-1).unwrap() == 'M' &&
        *self.grid.get(x-1).unwrap().get(y+1).unwrap() == 'S' &&
        *self.grid.get(x+1).unwrap().get(y+1).unwrap() == 'S' { counter += 1}


        counter
    }

}

impl Display for EflWords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Elf's words:")?;
        for line in &self.grid {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for EflWords {
    type Err = ParseElfWordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<_>> = s.lines().map(|line| {
                line.chars().collect()
            })
            .collect();

        // Compute grid dimension
        let width = grid[0].len();
        let height = grid.len();

        // Add empty around letter
        let grid: Vec<Vec<_>> = std::iter::repeat_n(vec![BORDER; width + 6], 3)
        .chain(grid.into_iter().map(|line| {
            std::iter::repeat_n(BORDER, 3)
                .chain(line)
                .chain(std::iter::repeat_n(BORDER, 3))
                .collect()
        }))
        .chain(std::iter::repeat_n(vec![BORDER; width + 6], 3))
        .collect();

        Ok(Self {
            grid,
            width,
            height
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = EflWords::from_str(input).expect("Failed to parse grid");

    // Solve
    let mut total = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            total += grid.find_pattern(x,y);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = EflWords::from_str(input).expect("Failed to parse grid");

    // Solve
    let mut total = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            total += grid.find_pattern2(x,y);
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
