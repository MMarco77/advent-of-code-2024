use std::collections::HashSet;

advent_of_code::solution!(5);

fn get_rules(input: &str) -> HashSet<String> {
    // Get rules
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("|").expect("Invalid rules");
            format!("{r},{l}")
        })
        .collect()
}

fn is_valid_line(chars_list: Vec<&str>, rules: &HashSet<String>) -> Result<(), usize> {
    for (pos, l) in chars_list.windows(2).enumerate() {
        let pattern = format!("{},{}", l[0], l[1]);
        if rules.contains(&pattern) {
            return Err(pos);
        }
    }
    Ok(())
}

fn get_list(input: &str, rules: &HashSet<String>) -> (Vec<String>, Vec<String>) {
    let mut valid_order: Vec<String> = vec![];
    let mut invalid_order: Vec<String> = vec![];

    for line in input.lines() {
        if is_valid_line(line.split(',').collect::<Vec<_>>(), rules).is_ok() {
            valid_order.push(line.to_string());
        } else {
            invalid_order.push(line.to_string());
        }
    }

    (valid_order, invalid_order)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_str, prints) = input.split_once("\n\n").expect("Invalid input");
    let rules: HashSet<String> = get_rules(rules_str);
    let (res, _) = get_list(prints, &rules);
    Some(res.iter().fold(0, |acc, line| {
        let chars_list = line.split(',').collect::<Vec<_>>();
        acc + chars_list[chars_list.len() / 2]
            .parse::<u32>()
            .expect("Invalid value")
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_str, prints) = input.split_once("\n\n").expect("Invalid input");
    let rules: HashSet<String> = get_rules(rules_str);
    let (_, res) = get_list(prints, &rules);
    Some(res.iter().fold(0, |acc, order| {
        let mut chars_list = order.split(',').collect::<Vec<_>>();
        loop {
            if let Err(pos) = is_valid_line(chars_list.clone(), &rules) {
                // Re-order
                chars_list.swap(pos, pos + 1);
            } else {
                return acc
                    + chars_list[chars_list.len() / 2]
                        .parse::<u32>()
                        .expect("Invalid value");
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
