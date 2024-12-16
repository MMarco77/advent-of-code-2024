use regex::Regex;
use std::str::FromStr;
use fraction::GenericFraction;

advent_of_code::solution!(13);

#[derive(Debug)]
struct ErrorParsing;

/* ========================================================================= */

#[derive(Debug, Clone, Copy)]
struct Button {
    x: usize,
    y: usize,
}

impl FromStr for Button {
    type Err = ErrorParsing;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"Button [A|B]\: X\+(\d+), Y\+(\d+)"#).expect("Button regexp");
        let caps = re.captures(s).unwrap();

        Ok(Self {
            x: caps
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Invalid X"),
            y: caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Invalid Y"),
        })
    }
}

/* ========================================================================= */

#[derive(Debug, Clone)]
struct Prize {
    prize_x: usize,
    prize_y: usize,
}

impl FromStr for Prize {
    type Err = ErrorParsing;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"Prize\: X=(\d+), Y=(\d+)"#).expect("Prize regexp");
        let caps = re.captures(s).unwrap();
        Ok(Self {
            prize_x: caps
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Invalid Prize X"),
            prize_y: caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Invalid Prize Y"),
        })
    }
}

/* ========================================================================= */

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl FromStr for Machine {
    type Err = ErrorParsing;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs: Vec<String> = s.lines().map(|c| c.to_string()).collect();
        let mut iter = inputs.iter();

        Ok(Self {
            button_a: Button::from_str(iter.next().expect("Invalid button A"))?,
            button_b: Button::from_str(iter.next().expect("Invalid button B"))?,
            prize: Prize::from_str(iter.next().expect("Invalid Prize"))?,
        })
    }
}

fn echelon(matrix: &mut [Vec<f32>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f32 {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [Vec<f32>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f32 {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}

pub fn gaussian_elimination(matrix: &mut [Vec<f32>]) -> Vec<f32> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    // Disable cargo clippy warnings about needless range loops.
    // Checking the diagonal like this is simpler than any alternative.
    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == 0f32 {
            println!("Infinitely many solutions");
        }
    }

    let mut result: Vec<f32> = vec![0f32; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

impl Machine {
    fn solve(&self) -> Option<(usize, usize)> {
        // let mut matrix =  vec![
        //     vec![self.button_a.x as f32, self.self.button_b.x as f32, self.prize.prize_x as f32],
        //     vec![self.button_a.y as f32, self.self.button_b.y as f32, self.prize.prize_y as f32],
        // ];
        // let result = gaussian_elimination(&mut matrix);
        // if result.len() != 2 {
        //     None
        // } else {
        //     let mut iter = result.iter();
        //     let pos_x = iter.next().expect("Invalid x");
        //     let pos_y = iter.next().expect("Invalid y");
        //     if *pos_x < 0.0 || *pos_x >= 100.0 || *pos_y >= 100.0 || *pos_y < 0.0 {
        //         None
        //     } else {
        //         eprintln!("result ({pos_x},{pos_y})");
        //         Some((pos_x.round() as usize,pos_y.round() as usize))
        //     }
        // }

        let dy1_dx1:GenericFraction<usize>  = GenericFraction::new(self.button_a.y, self.button_a.x);
        let f2 = (GenericFraction::from(self.prize.prize_y) - GenericFraction::from(self.prize.prize_x) * dy1_dx1)
            / (GenericFraction::from(self.button_b.y) - GenericFraction::from(self.button_b.x) * dy1_dx1);
        let f1 = (GenericFraction::from(self.prize.prize_x) - f2 * self.button_b.x) / GenericFraction::from(self.button_a.x);
        let f2: usize = f2.try_into().ok()?;
        let f1: usize = f1.try_into().ok()?;

        Some((f1, f2))

    }
}

/* ========================================================================= */

// 28138
pub fn part_one(input: &str) -> Option<usize> {
    let casino: Vec<String> = input
        .split("\n\n")
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    let machines = casino
        .iter()
        .map(|m| Machine::from_str(m).expect("Invalid machine"))
        .collect::<Vec<_>>();

    let winners: Vec<(usize, usize)> = machines
        .iter()
        .filter_map(|m| {
            m.solve()
        })
        .collect();

    Some(winners.iter().fold(0, |acc, win| {
        acc + win.0 * 3 + win.1
    }))
}

// 108394825772874
pub fn part_two(input: &str) -> Option<usize> {
    let casino: Vec<String> = input
        .split("\n\n")
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    let machines = casino
        .iter()
        .map(|m| {
            let mut m = Machine::from_str(m).expect("Invalid machine");
            m.prize.prize_x += 10_000_000_000_000;
            m.prize.prize_y += 10_000_000_000_000;
            m
        })
        .collect::<Vec<_>>();

    let winners: Vec<(usize, usize)> = machines
        .iter()
        .filter_map(|m| {
            m.solve()
        })
        .collect();

    Some(winners.iter().fold(0, |acc, win| {
        acc + win.0 * 3 + win.1
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
