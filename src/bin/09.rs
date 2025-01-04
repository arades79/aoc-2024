use std::ops::AddAssign;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut hard_drive: Vec<Option<usize>> = Vec::with_capacity(input.len() * 10);
    let mut next_free_block: usize = 0;
    for (id, file_size) in input.char_indices() {
        if id % 2 == 0 {
            // file
            let id = id >> 1;
            let file_size = file_size.to_digit(10)?;
            for _ in 0..file_size {
                hard_drive.push(Some(id))
            }
        } else {
            // blank space
            let blank_size = file_size.to_digit(10)?;
            for _ in 0..blank_size {
                hard_drive.push(None);
            }
        }
    }
    let mut last_file_block = hard_drive.len() - 1;
    let find_next_free = |start: &mut usize, hdd: &[Option<usize>]| {
        while hdd[*start].is_some() {
            start.add_assign(1);
        }
    };
    find_next_free(&mut next_free_block, &hard_drive);

    while last_file_block > next_free_block + 1 {
        if hard_drive[last_file_block].is_some() {
            hard_drive.swap(next_free_block, last_file_block);
        }
        find_next_free(&mut next_free_block, &hard_drive);
        while hard_drive[last_file_block].is_none() {
            last_file_block -= 1;
        }
    }
    let checksum = hard_drive
        .into_par_iter()
        .enumerate()
        .filter_map(|(index, block)| block.map(|block| block * index))
        .sum();

    Some(checksum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Span {
    start: u32,
    size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]

enum Content {
    #[default]
    Empty,
    File(u32), // id
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Block {
    span: Span,
    content: Content,
}

impl Block {
    fn checksum(self) -> usize {
        if let Content::File(id) = self.content {
            let (start, size, id) = (
                self.span.start as usize,
                self.span.size as usize,
                id as usize,
            );
            (start..(start + size)).map(|index| index * id).sum()
        } else {
            0
        }
    }
    const fn is_file(&self) -> bool {
        match self.content {
            Content::Empty => false,
            Content::File(_) => true,
        }
    }
    const fn is_empty(&self) -> bool {
        !self.is_file()
    }
    fn fill_with(&mut self, other: &mut Block) -> Block {
        let moved = Block {
            span: Span {
                start: self.span.start,
                size: other.span.size,
            },
            content: other.content,
        };
        other.content = Content::Empty;
        self.span.size -= other.span.size;
        self.span.start += other.span.size;
        moved
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut hard_drive: Vec<Block> = Vec::with_capacity(input.len());
    let mut next_block_start: u32 = 0;
    for (id, file_size) in input.char_indices() {
        let file_size = file_size.to_digit(10)?;
        hard_drive.push(Block {
            span: Span {
                start: next_block_start,
                size: file_size,
            },
            content: if id % 2 == 0 {
                Content::File((id >> 1) as u32)
            } else {
                Content::Empty
            },
        });
        next_block_start += file_size;
    }
    for block_index in (0..hard_drive.len()).rev() {
        if hard_drive[block_index].is_file() {
            let mut file = hard_drive[block_index];
            if let Some((free_block_index, mut free_block)) = hard_drive
                .iter()
                .copied()
                .enumerate()
                .find(|(index, block)| {
                    block.is_empty() && file.span.size <= block.span.size && *index < block_index
                })
            {
                let new_block = free_block.fill_with(&mut file);
                hard_drive[free_block_index] = free_block;
                hard_drive[block_index] = file;
                hard_drive.push(new_block);
            }
        }
    }
    let checksum = hard_drive.into_iter().map(|f| f.checksum()).sum();
    Some(checksum)
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
