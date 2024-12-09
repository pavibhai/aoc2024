use std::iter::Iterator;

pub fn part1(disk_map: &DiskMap) -> u64 {
    disk_map.compact(false)
}

pub fn part2(disk_map: &DiskMap) -> u64 {
    disk_map.compact(true)
}

pub fn generator(input: &str) -> DiskMap {
    DiskMap::new(input)
}

#[derive(Debug, Clone, Copy)]
struct Block {
    id: u32,
    pos: u32,
    size: u32,
}

impl Block {
    fn checksum(&self) -> u64 {
        ((self.pos..(self.pos + self.size)).sum::<u32>() * self.id) as u64
    }
}

#[derive(Debug, Clone, Copy)]
struct Space {
    pos: u32,
    size: u32,
}

impl Space {
    fn allocate(&mut self, b: &mut Block) -> Option<Block> {
        if b.size <= self.size {
            b.pos = self.pos;
            self.pos += b.size;
            self.size -= b.size;
            None
        } else {
            b.size -= self.size;
            let r = Some(Block { id: b.id, pos: self.pos, size: self.size });
            self.size = 0;
            r
        }
    }
}

pub struct DiskMap {
    blocks: Vec<Block>,
    free: Vec<Space>,
}

impl DiskMap {
    fn new(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut free = Vec::new();
        let chars: Vec<char> = input.lines().next().unwrap().chars().collect();
        let mut pos = 0;
        let mut id = 0;
        chars.chunks(2).for_each(|chunk| {
            blocks.push(Block { id, pos, size: chunk[0].to_digit(10).unwrap() });
            id += 1;
            pos += blocks.last().unwrap().size;
            if chunk.len() == 2 {
                free.push(Space { pos, size: chunk[1].to_digit(10).unwrap() });
                pos += free.last().unwrap().size;
            }
        });
        if blocks.len() != free.len() + 1 {
            panic!("invalid input");
        }

        DiskMap {
            blocks,
            free,
        }
    }

    fn compact(&self, full: bool) -> u64 {
        let mut free = self.free.clone();
        let mut checksum = 0u64;
        let mut free_start = 0usize;

        for b in self.blocks.iter().rev() {
            if b.id < free_start as u32 {
                checksum += b.checksum();
                continue;
            }

            let mut splits: Vec<Block> = if full {
                vec![b.clone()]
            } else {
                (0..b.size).rev()
                    .map(|i| Block { id: b.id, pos: b.pos + i, size: 1 })
                    .collect()
            };

            for b in splits.iter_mut() {
                match free[free_start..b.id as usize].iter_mut().find(|f| f.size >= b.size) {
                    Some(i) => {
                        i.allocate(b);
                    }
                    _ => {}
                }
                if free[free_start].size == 0 {
                    free_start += 1;
                }
                checksum += b.checksum()
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_generator() {
        let map = generator(&INPUT);
        assert_eq!(map.blocks.len(), 10);
        assert_eq!(map.free.len(), 9);
    }

    #[test]
    fn test_part_1() {
        let map = generator(&INPUT);
        assert_eq!(part1(&map), 1928);
    }

    #[test]
    fn test_part_2() {
        let map = generator(&INPUT);
        assert_eq!(part2(&map), 2858);
    }
}
