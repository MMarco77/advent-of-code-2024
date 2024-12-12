use ibig::{ubig, UBig};

advent_of_code::solution!(11);

fn blink(stones: &[UBig]) -> Vec<UBig> {
    stones.iter().flat_map(|stone| {
        if *stone == ubig!(0) {
            vec![ubig!(1)]
        } else {
            let str_stone = stone.to_string();
            if (str_stone.len() % 2) == 0 {
                let (left, right) = str_stone.split_at(str_stone.len()/2);
                vec![
                    left.parse::<UBig>().expect("Invalid left"),
                    right.parse::<UBig>().expect("Invalid left")
                ]
            } else {
                vec![stone*ubig!(2024)]
            }
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones: Vec<UBig> = input
        .split(" ")
        .map(|c| c.parse().expect("Invalid Big Int"))
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = blink(&stones);
    }

    Some(stones.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones: Vec<UBig> = input
        .split(" ")
        .map(|c| c.parse().expect("Invalid Big Int"))
        .collect::<Vec<_>>();

    for step in 0..75 {
        stones = blink(&stones);
        // println!("Step {} [{}]", step, stones.len());
    }

    Some(stones.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
