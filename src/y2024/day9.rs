use itertools::Itertools;
use std::cmp::Ordering;
use std::mem::swap;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut map = DiskMap::from_str(input).expect("Failed to parse input");
    map.compact(true);
    map.checksum()
}

fn part2(input: &str) -> usize {
    let mut map = DiskMap::from_str(input).expect("Failed to parse input");
    map.compact(false);
    map.checksum()
}

#[derive(Debug)]
struct Block {
    start: usize,
    file_id: Option<usize>,
    len: usize,
}

impl Block {
    fn new_file(start: usize, len: usize, id: usize) -> Self {
        Self {
            start,
            len,
            file_id: Some(id),
        }
    }

    fn new_free(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
            file_id: None,
        }
    }
}

#[derive(Debug)]
struct DiskMap {
    free: Vec<Block>,
    files: Vec<Block>,
}

impl DiskMap {
    fn compact(&mut self, allow_partials: bool) {
        let mut i = (self.files.len() - 1) as isize;
        while i >= 0 {
            if let Some(free_i) = self.find_next_free(&self.files[i as usize], allow_partials) {
                let free_block = &mut self.free[free_i];
                let file = &mut self.files[i as usize];
                if let Some(remainder) = move_file_into_block(free_block, file) {
                    match remainder.file_id {
                        Some(_) => {
                            self.files.push(remainder);
                            continue;
                        }
                        None => {
                            self.free.push(remainder);
                        }
                    }
                }
            }
            i -= 1;
        }
    }

    fn find_next_free(&self, file: &Block, allow_partial: bool) -> Option<usize> {
        self.free.iter().position(|block| {
            block.start < file.start
                && ((allow_partial && block.len > 0) || (!allow_partial && block.len >= file.len))
        })
    }

    fn checksum(&self) -> usize {
        self.files
            .iter()
            .chain(self.free.iter())
            .sorted_by(|a, b| a.start.cmp(&b.start))
            .flat_map(|block| (0..block.len).map(|_| block.file_id))
            .enumerate()
            .fold(0, |sum, (i, file_id)| match file_id {
                Some(id) => sum + (id * i),
                None => sum,
            })
    }
}

fn move_file_into_block(free_block: &mut Block, file: &mut Block) -> Option<Block> {
    match free_block.len.cmp(&file.len) {
        Ordering::Less => {
            file.len -= free_block.len;
            let tmp = free_block.len;
            free_block.len = 0;
            Some(Block::new_file(
                free_block.start,
                tmp,
                file.file_id.unwrap(),
            ))
        }
        Ordering::Equal => {
            swap(&mut free_block.start, &mut file.start);
            None
        }
        Ordering::Greater => {
            let tmp = file.start;
            file.start = free_block.start;
            free_block.start += file.len;
            free_block.len -= file.len;
            Some(Block::new_free(tmp, file.len))
        }
    }
}

#[derive(Debug)]
struct ParseDiskMapError {}
impl FromStr for DiskMap {
    type Err = ParseDiskMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = 0;
        let mut id = 0;
        let blocks: Vec<Block> = parse_blocks(s)?
            .into_iter()
            .map(|(i, len)| {
                let block = Block {
                    start: pos,
                    file_id: if i % 2 == 0 { Some(id) } else { None },
                    len: len as usize,
                };
                pos += len as usize;
                if i % 2 == 0 {
                    id += 1;
                }
                block
            })
            .collect();
        let (files, free): (Vec<_>, Vec<_>) = blocks
            .into_iter()
            .partition(|block| block.file_id.is_some());
        Ok(Self { free, files })
    }
}

fn parse_blocks(s: &str) -> Result<Vec<(usize, u32)>, ParseDiskMapError> {
    s.trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_digit(10)
                .map(|len| (i, len))
                .ok_or(ParseDiskMapError {})
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
