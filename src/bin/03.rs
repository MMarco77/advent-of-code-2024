use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(re.captures_iter(input).fold(0, |acc, caps| {
        let (_, [left, right]) = caps.extract();
        acc + left.parse::<u32>().expect("Invalid left")
            * right.parse::<u32>().expect("Invalid right")
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(don't\(\))|(do\(\))").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut capture = true;

    Some(re.captures_iter(input).fold(0, |acc, caps| {
        // Get alternative (mul, do, don't)
        let (_, [a]) = caps.extract();
        match a {
            "don't()" => {
                capture = false;
                acc
            }
            "do()" => {
                capture = true;
                acc
            }
            _ => {
                if !capture {
                    acc
                } else {
                    let caps = re_mul.captures(a).unwrap();
                    let (_, [left, right]) = caps.extract();
                    acc + left.parse::<u32>().expect("Invalid left")
                        * right.parse::<u32>().expect("Invalid right")
                }
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
