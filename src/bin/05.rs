use std::collections::HashSet;

advent_of_code::solution!(5);

fn get_rules(iter_line: &mut std::str::Lines<'_>) -> HashSet<String> {
    let mut rules: HashSet<String> = HashSet::new();

    // Get rules
    loop {
        let line = iter_line.next().expect("No more line");
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once("|").expect("Invalid rules");
        rules.insert(format!("{r},{l}"));
    }
    rules
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

fn get_list(
    iter_line: &mut std::str::Lines<'_>,
    rules: &HashSet<String>,
) -> (Vec<String>, Vec<String>) {
    let mut valid_order: Vec<String> = vec![];
    let mut invalid_order: Vec<String> = vec![];

    'scanner: loop {
        if let Some(line) = iter_line.next() {
            if is_valid_line(line.split(',').collect::<Vec<_>>(), rules).is_ok() {
                valid_order.push(line.to_string());
            } else {
                invalid_order.push(line.to_string());
            }
            continue 'scanner;
        }
        break 'scanner;
    }

    (valid_order, invalid_order)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut line_iter: std::str::Lines<'_> = input.lines();
    let rules: HashSet<String> = get_rules(&mut line_iter);

    let (res, _) = get_list(&mut line_iter, &rules);
    Some(res.iter().fold(0, |acc, line| {
        let chars_list = line.split(',').collect::<Vec<_>>();
        acc + chars_list[chars_list.len() / 2]
            .parse::<u32>()
            .expect("Invalid value")
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut line_iter: std::str::Lines<'_> = input.lines();
    let rules: HashSet<String> = get_rules(&mut line_iter);
    let (_, res) = get_list(&mut line_iter, &rules);
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
