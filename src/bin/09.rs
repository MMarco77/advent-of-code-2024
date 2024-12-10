/* Inspired by AROD */

advent_of_code::solution!(9);
use std::collections::BTreeSet;

/* ============================================================================ */

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Empty,
    File(usize),
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct File {
    position: usize,
    length: usize,
    file_id: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmptySpace {
    pub position: usize,
    pub length: usize,
}

/* ============================================================================ */

struct HardDrive {
    files: Vec<Block>,
}

impl From<&str> for HardDrive {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        Self {
            files: data
                .iter()
                .enumerate()
                .flat_map(|(idx, elt)| {
                    if idx % 2 == 0 {
                        vec![Block::File(idx / 2); *elt]
                    } else {
                        vec![Block::Empty; *elt]
                    }
                })
                .collect(),
        }
    }
}

impl HardDrive {
    fn defragment(&mut self) {
        // Compact it
        let mut first_empty: usize = 0;
        while first_empty < self.files.len() {
            // Look for a valid first_empty position
            while let Block::File(_) = &self.files[first_empty] {
                first_empty += 1;
                if first_empty >= self.files.len() {
                    return;
                }
            }

            // Get the last file from the disk
            #[allow(unused_assignments)]
            let mut last_elem = usize::MAX;
            loop {
                let last = self.files.pop().unwrap();
                match last {
                    Block::Empty => {
                        // check if we emptied the free spaces, in this case we are done
                        if first_empty >= self.files.len() {
                            return;
                        }
                    }
                    Block::File(id) => {
                        last_elem = id;
                        break;
                    }
                }
            }

            // Put the last elem at the first free space
            self.files[first_empty] = Block::File(last_elem);
        }
    }

    /// Compute the checksum of a disk by ignoring empty spaces
    fn checksum(&self) -> usize {
        self.files
            .iter()
            .enumerate()
            .map(|(pos, elem)| match elem {
                Block::Empty => 0,
                Block::File(id) => id * pos,
            })
            .sum()
    }
}

fn compute_first_empty_space(bitmap: &[BTreeSet<usize>], len: usize) -> Option<(usize, usize)> {
    // Look for empty spaces in the tree
    let mut min_pos = (0, usize::MAX);
    for (bracket, item) in bitmap.iter().enumerate().take(10).skip(len) {
        if let Some(position) = item.first() {
            if *position < min_pos.1 {
                min_pos = (bracket, *position);
            }
        }
    }

    if min_pos.1 != usize::MAX {
        Some(min_pos)
    } else {
        None
    }
}

/* ============================================================================ */

pub fn part_one(input: &str) -> Option<usize> {
    let mut hdd = HardDrive::from(input);
    hdd.defragment();
    Some(hdd.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    // Create the list of files
    let mut current_position: usize = 0;
    let (disk, empty_spaces): (Vec<_>, Vec<_>) = data
        .iter()
        .enumerate()
        .map(|(idx, elt)| {
            let file = if idx % 2 == 0 {
                // We are on a file node
                (
                    Some(File {
                        position: current_position,
                        length: *elt,
                        file_id: idx / 2,
                    }),
                    None,
                )
            } else {
                (
                    None,
                    Some(EmptySpace {
                        position: current_position,
                        length: *elt,
                    }),
                )
            };

            // Increase the position
            current_position += *elt;

            // Return the file if it was created
            file
        })
        .collect();

    // Get the list of files
    let mut disk: Vec<_> = disk.into_iter().flatten().collect();

    // Create the bitmap
    // 1. filter out empty elements
    let empty_spaces: Vec<_> = empty_spaces.into_iter().flatten().collect();
    // 2. create the bitmap
    let mut bitmap: Vec<_> = vec![BTreeSet::new(); 10];
    // 3. fill it
    for empty_space in empty_spaces {
        bitmap[empty_space.length].insert(empty_space.position);
    }

    // Try to compact the disk from the end
    for file in disk.iter_mut().rev() {
        if let Some((bracket, position)) = compute_first_empty_space(&bitmap, file.length) {
            // Check if the new position is an improvement
            if position >= file.position {
                continue;
            }

            // Remove the position in the bracket
            bitmap[bracket].remove(&position);

            // If needed, add the remaining free space to the according bracket
            if bracket > file.length {
                bitmap[bracket - file.length].insert(position + file.length);
            }

            // We don't need to mark the file used space as free since we treat them from the back

            //  Move the file
            file.position = position;
        }
    }

    // Compute the checksum
    Some(
        disk.iter()
            .map(|file| {
                (0..file.length)
                    .map(|idx| (file.position + idx) * file.file_id)
                    .sum::<usize>()
            })
            .sum(),
    )
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
        assert_eq!(result, Some(2858));
    }
}
