advent_of_code::solution!(7);

fn concat_value(higher: &u64, lower: &u64) -> u64 {
     let str1 = higher.to_string();
     let str2 = lower.to_string();
     let concatenated = format!("{}{}", str1, str2);
     concatenated.parse().unwrap()
    // higher * 10u64.pow(lower.ilog10() + 1) + lower
}

fn is_calibration_valid(res: u64, current_value: u64, calibrations: &[u64]) -> bool {
    if calibrations.is_empty() {
        return current_value == res;
    }
    if current_value > res {
        return false;
    }

    let next_value = calibrations.first().expect("No more value");
    let mut next_calibrations = calibrations.to_vec().clone();
    next_calibrations.remove(0);

    // Sum
    is_calibration_valid(res, next_value + current_value, &next_calibrations)
        || is_calibration_valid(res, next_value * current_value, &next_calibrations)
}

fn is_calibration_valid2(res: u64, current_value: u64, calibrations: &[u64]) -> bool {
    if calibrations.is_empty() {
        return current_value == res;
    }
    if current_value > res {
        return false;
    }

    let next_value = calibrations.first().expect("No more value");
    let mut next_calibrations = calibrations.to_vec().clone();
    next_calibrations.remove(0);

    is_calibration_valid2(res, next_value + current_value, &next_calibrations)
        || is_calibration_valid2(res, next_value * current_value, &next_calibrations)
        || is_calibration_valid2(
            res,
            concat_value(next_value, &current_value),
            &next_calibrations)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().fold(0, |acc, line| {
        let (res, nbrs) = line.split_once(":").expect("Invalid line");
        let nbr_list: Vec<u64> = nbrs
            .trim()
            .split(" ")
            .map(|v| v.trim().parse::<u64>().expect("Invalid value"))
            .collect::<Vec<_>>();
        let res = res
            .parse::<u64>()
            .expect(&format!("Invalid result for '{res}'"));
        if is_calibration_valid(res, 0, &nbr_list) {
            acc + res
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().fold(0, |acc, line| {
        let (res, nbrs) = line.split_once(":").expect("Invalid line");
        let nbr_list: Vec<u64> = nbrs
            .trim()
            .split(" ")
            .map(|v| v.trim().parse::<u64>().expect("Invalid value"))
            .collect::<Vec<_>>();
        let res = res
            .parse::<u64>()
            .expect(&format!("Invalid result for '{res}'"));
        if is_calibration_valid2(res, 0, &nbr_list) {
            acc + res
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
