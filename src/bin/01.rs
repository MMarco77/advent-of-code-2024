use std::collections::{hash_map::Entry, HashMap};

advent_of_code::solution!(1);

fn parse_list(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let mut l_list: Vec<u32> = Vec::new();
    let mut r_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once("   ").expect("Failed to parse '{line}'");
        l_list.push(
            l.trim()
                .parse::<u32>()
                .expect("Failed to convert left of '{line}'"),
        );
        r_list.push(
            r.trim()
                .parse::<u32>()
                .expect("Failed to convert right of '{line}'"),
        );
    }
    Ok((l_list, r_list))
}

fn count_similar(array: &[u32]) -> HashMap<u32, u32> {
    let mut res: HashMap<u32, u32> = HashMap::new();
    for value in array.iter() {
        let values = match res.entry(*value) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(0),
        };
        *values += 1;
    }

    res
}

/// Order left and right list then iterate to compute the absolute
/// difference.
pub fn part_one(input: &str) -> Option<u32> {
    let (mut l_list, mut r_list) = parse_list(input).unwrap();
    // Order list
    l_list.sort();
    r_list.sort();

    let iter = l_list.iter().zip(r_list.iter());

    Some(iter.fold(0, |acc, (l, r)| -> u32 { acc + l.abs_diff(*r) }))
}

/// Count the number of same value Left/Right
///
/// If left value is available on right list =>
///     Sum(left_value*count_right*count_left)
pub fn part_two(input: &str) -> Option<u32> {
    let (l_list, r_list) = parse_list(input).unwrap();

    let l_map = count_similar(&l_list);
    let mut r_map = count_similar(&r_list);

    Some(l_map.iter().fold(0, |acc, (key, count_left)| -> u32 {
        let count_right = match r_map.entry(*key) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(_) => 0,
        };

        acc + key * count_left * count_right
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
