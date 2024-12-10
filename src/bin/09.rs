use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(9);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

/* ============================================================================ */

#[derive(Copy, Clone, PartialEq, Eq)]
struct File {
    begin: usize,
    length: usize,
    file_id: usize
}

struct HardDrive {
    data: Vec<File>
}

impl From<&str> for HardDrive {
    fn from(input: &str) -> Self {
        let mut data = Vec::new();
        let mut is_file = true; // Tracks whether we're processing a file or free space
        let mut file_id = 0;
        let mut current_pos: usize = 0;

        for ch in input.chars() {
            let length = ch.to_digit(10).expect(&format!("Invalid {ch}"));
            if is_file {
                data.push(File {
                    begin: current_pos,
                    length: length as usize,
                    file_id,
                });
                current_pos += length as usize;
                file_id += 1; // Increment the file ID for the next file
            } else {
                current_pos += length as usize;
            }
            is_file = !is_file; // Alternate between file and free space
        }

        Self {
            data
        }
    }
}

impl HardDrive {
    fn defragment(&mut self) {
        let total_length: usize = self.data.iter().map(|f| f.begin + f.length).max().unwrap_or(0);
        let mut disk: Vec<char> = vec!['.'; total_length];

        for file in &self.data {
            for i in 0..file.length {
                disk[file.begin + i] = std::char::from_digit(file.file_id as u32, 10).unwrap();
            }
        }

        let mut rev_read_pos = total_length - 1;
        let mut read_pos = 0;
        loop {
            if rev_read_pos == read_pos { break;}
            if disk[read_pos] == '.' {
                if disk[rev_read_pos] == '.' {
                    rev_read_pos -= 1;
                    continue;
                }
                disk[read_pos] = disk[rev_read_pos];
                disk[rev_read_pos] = '.';
                rev_read_pos -= 1;
            }
            read_pos += 1;
        }

        let mut data: Vec<File> = vec![];
        let mut chars = disk.iter().enumerate();

        let mut current_file_id: Option<char> = None;
        let mut begin = 0;
        let mut last_index = disk.len();

        while let Some((index, ch)) = chars.next() {
            last_index = index;
            if *ch == '.' {
                break;
            }

            if Some(ch) != current_file_id.as_ref() {
                if let Some(file_id) = current_file_id {
                    let length = index - begin;
                    data.push(File {
                        begin,
                        length,
                        file_id: file_id.to_digit(10).unwrap() as usize,
                    });
                }
                current_file_id = Some(*ch);
                begin = index;
            }
        }

        if let Some(file_id) = current_file_id {
            let length = last_index - begin;
            data.push(File {
                begin,
                length,
                file_id: file_id.to_digit(10).unwrap() as usize,
            });
        }
        // println!("{}", HardDrive { data: data.clone()});
        self.data = data;

    }

    fn checksum(&self) -> u32 {
        let mut checksum = 0;

        for file in &self.data {
            for i in 0..file.length {
                let position = file.begin + i;
                checksum += position as u32 * file.file_id as u32;
            }
        }

        checksum
    }
}

impl Display for HardDrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_length: usize = self.data.iter().map(|f| f.begin + f.length).max().unwrap_or(0);
        let mut disk = vec!['.'; total_length];

        for file in &self.data {
            for i in 0..file.length {
                disk[file.begin + i] = std::char::from_digit(file.file_id as u32, 10).unwrap();
            }
        }

        write!(f, "{}", disk.iter().collect::<String>())
    }
}

/* ============================================================================ */


pub fn part_one(input: &str) -> Option<u32> {
    // println!("Parse input");
    let mut hdd = HardDrive::from(input);
    println!("HDD => {}", hdd);
    // println!("Defragment");
    hdd.defragment();
    // println!("Defrag>  => {}", hdd);
    // println!("Compute Checksum");
    Some(hdd.checksum())

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
