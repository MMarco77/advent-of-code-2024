advent_of_code::solution!(2);

use std::cmp::Ordering;

fn from_str(line: &str) -> Vec<u32> {
    let levels = line.split(" ").collect::<Vec<_>>();
    levels
        .iter()
        .map(|v| v.trim().parse::<u32>().expect("Invalid number {v}"))
        .collect::<Vec<_>>()
}

/// Check if l and right is correct
///
/// 1. l and r always increasing or decreasing.
/// 2. l and r differ by at least one and at most three.
fn check(l: u32, r: u32, ord: Ordering) -> bool {
    match (l, r, r.abs_diff(l), ord) {
        (l, r, d, Ordering::Greater) if l > r && (1..=3).contains(&d) => true,
        (l, r, d, Ordering::Less) if l < r && (1..=3).contains(&d) => true,
        _ => false,
    }
}

fn is_valid_list(binding: &[u32]) -> Result<(), usize> {
    let mut iter = binding.windows(2).enumerate();

    // Detect order
    let ord: Ordering = binding[0].cmp(binding.last().unwrap());

    while let Some((idx, [l, r])) = iter.next() {
        if !check(*l, *r, ord) {
            return Err(idx);
        }
    }
    Ok(())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        // Get u32 list
        let binding = from_str(line);

        if binding[0] == *binding.last().unwrap() {
            return acc;
        }
        if is_valid_list(&binding).is_ok() {
            acc + 1
        } else {
            acc
        }
    }))
}

//
// 367, 456, 363. 290 => 318
pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        // Get u32 list
        let binding = from_str(line);
        if binding[0] == *binding.last().unwrap() {
            return acc;
        }
        if let Err(idx) = is_valid_list(&binding) {
            // Recreate list
            let mut binding_idx = binding.clone();
            binding_idx.remove(idx);

            let mut binding_idx_1 = binding.clone();
            binding_idx_1.remove(idx + 1);

            if is_valid_list(&binding_idx).is_ok() || is_valid_list(&binding_idx_1).is_ok() {
                acc + 1
            } else {
                acc
            }
        } else {
            acc + 1
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
