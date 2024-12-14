use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(13);

#[derive(Debug)]
struct ErrorParsing;

/* ========================================================================= */

#[derive(Debug, Clone, Copy)]
struct Button {
    x: u32,
    y: u32,
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
                .parse::<u32>()
                .expect("Invalid X"),
            y: caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("Invalid Y"),
        })
    }
}

/* ========================================================================= */

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Prize {
    prize_x: u32,
    prize_y: u32,
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
                .parse::<u32>()
                .expect("Invalid Prize X"),
            prize_y: caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("Invalid Prize Y"),
        })
    }
}

impl Prize {
    pub fn is_over(&self, prize: Prize) -> bool {

    impl Ord for Prize {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.prize_x == other.prize_x && self.prize_y == other.prize_y {
                return Ordering::Equal;
            } else if self.prize_x > other.prize_x || self.prize_y > other.prize_y {
                return Ordering::Greater;
            } else {
                return Ordering::Greater;
            }
        }
    }
}

/* ========================================================================= */

#[derive(Debug)]
enum ButtonToClick {
    BtnA(Button),
    BtnB(Button)
}

#[derive(Debug)]
struct Claw {
    btn_a_count: u32,
    btn_b_count: u32,
    prize: Prize,
}

impl Claw {
    pub fn new() -> Self {
        Self {
            btn_a_count: 0,
            btn_b_count: 0,
            prize: Prize {
                prize_x: 0,
                prize_y: 0,
            },
        }
    }

    pub fn add(&self, btn: ButtonToClick) -> Self {
        match btn {
            ButtonToClick::BtnA(button) => Self {
                btn_a_count: self.btn_a_count + 1,
                btn_b_count: self.btn_a_count,
                prize: Prize {
                    prize_x: self.prize.prize_x + button.x,
                    prize_y: self.prize.prize_y + button.y,
                },
            },
            ButtonToClick::BtnB(button) => Self {
                btn_a_count: self.btn_a_count,
                btn_b_count: self.btn_a_count + 1,
                prize: Prize {
                    prize_x: self.prize.prize_x + button.x,
                    prize_y: self.prize.prize_y + button.y,
                },
            },
        }
    }
}

/* ========================================================================= */

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

impl Machine {
    fn play(&self, current_claw: &Claw, btn_2_click: ButtonToClick) -> Option<Claw> {
        // eprintln!("current_claw {:#?} | btn_2_click {:#?}", current_claw.prize, btn_2_click);
        // eprintln!("current_claw {:#?}", current_claw.prize);
        let next_claw = current_claw.add(btn_2_click);
        if current_claw.prize == next_claw.prize { return Some(next_claw) }
        eprintln!("{:#?} > {:#?} => ", current_claw.prize, next_claw.prize, );
        if current_claw.prize > next_claw.prize { return None }
        if current_claw.btn_a_count >= 100 || current_claw.btn_b_count >= 100 { return None }

        if let Some(winner) = self.play(&next_claw, ButtonToClick::BtnA(self.button_a)) {
            Some(winner)
        } else {
            self.play(&next_claw, ButtonToClick::BtnB(self.button_b))
        }
    }
}

/* ========================================================================= */

pub fn part_one(input: &str) -> Option<u32> {
    let casino: Vec<String> = input
        .split("\n\n")
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    let machines = casino
        .iter()
        .map(|m| Machine::from_str(m).expect("Invalid machine"))
        .collect::<Vec<_>>();

    eprintln!("Found {} machines", machines.len());

    let winners: Vec<Claw> = machines.iter().filter_map(|m| {
        eprintln!("Process machine");
        let claw = Claw::new();
        if let Some(winner) = m.play(&claw, ButtonToClick::BtnA(m.button_a)) {
            Some(winner)
        } else {
            // Restart with new claw
            m.play(&claw, ButtonToClick::BtnB(m.button_b))
        }
    }).collect();
    eprintln!("Found {} winners", winners.len());

    Some (
        winners.iter().fold(0, |acc, win| {
            acc + win.btn_a_count*3 + win.btn_b_count
        })
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
