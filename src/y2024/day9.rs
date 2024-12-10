use std::cmp::Ordering;
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

#[derive(Debug)]
struct DiskMap {
    free: Vec<Block>,
    files: Vec<Block>,
}

impl DiskMap {
    fn compact(&mut self, allow_partials: bool) {
        let mut moved = Vec::new();
        while let Some(mut file) = self.files.pop() {
            if let Some(free_i) = self.find_next_free(&file, allow_partials) {
                let free_block = &mut self.free[free_i];
                if let Some(remainder) = move_file_into_block(free_block, &mut file) {
                    match remainder.file_id {
                        Some(_) => {
                            self.files.push(remainder);
                        }
                        None => {
                            self.free.push(remainder);
                        }
                    }
                }
            }
            moved.push(file);
        }
        self.files = moved;
        self.free.retain(|file| file.len != 0);
    }

    fn find_next_free(&self, file: &Block, allow_partial: bool) -> Option<usize> {
        self.free.iter().position(|block| {
            block.start < file.start
                && ((allow_partial && block.len > 0) || (!allow_partial && block.len >= file.len))
        })
    }

    fn checksum(&self) -> usize {
        let mut tmp = self.free.iter().collect::<Vec<_>>();
        tmp.extend(self.files.iter().collect::<Vec<_>>());
        tmp.sort_by(|a, b| a.start.cmp(&b.start));
        let mut sum = 0;
        let mut i = 0;
        for b in tmp {
            for _ in 0..b.len {
                if let Some(id) = b.file_id {
                    sum += i * id;
                }
                i += 1;
            }
        }
        sum
    }
}

fn move_file_into_block(free_block: &mut Block, file: &mut Block) -> Option<Block> {
    match free_block.len.cmp(&file.len) {
        Ordering::Less => {
            let remaining = Block {
                start: file.start + free_block.len,
                file_id: file.file_id,
                len: file.len - free_block.len,
            };
            file.start = free_block.start;
            file.len = free_block.len;
            free_block.len = 0;
            Some(remaining)
        }
        Ordering::Equal => {
            let tmp = free_block.start;
            free_block.start = file.start;
            file.start = tmp;
            None
        }
        Ordering::Greater => {
            let remaining = Block {
                start: file.start,
                file_id: None,
                len: file.len,
            };
            file.start = free_block.start;
            free_block.start += file.len;
            free_block.len -= file.len;
            Some(remaining)
        }
    }
}

#[derive(Debug)]
struct ParseDiskMapError {}
impl FromStr for DiskMap {
    type Err = ParseDiskMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut free = Vec::new();
        let mut files = Vec::new();
        let mut pos = 0;
        let mut id = 0;
        for (i, len) in parse_blocks(s)? {
            if i % 2 == 0 {
                files.push(Block {
                    start: pos,
                    file_id: Some(id),
                    len: len as usize,
                });
                id += 1;
            } else {
                free.push(Block {
                    start: pos,
                    file_id: None,
                    len: len as usize,
                });
            }
            pos += len as usize;
        }
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
