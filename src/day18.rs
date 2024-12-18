use std::collections::VecDeque;

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part1(m: &Memory) -> u32 {
    m.walk(1024).unwrap()
}

pub fn part2(m: &Memory) -> String {
    let result = m.blocking_byte(0, m.max_byte as usize);
    format!("{},{}", result.x, result.y)
}

pub fn generator(input: &str) -> Memory {
    Memory::new(input, 70, 70)
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(input: &str) -> XY {
        let (x, y) = input.split_once(",").unwrap();
        XY {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn apply(&mut self, dx: &i32, dy: &i32) {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug)]
pub struct Memory {
    max_byte: u32,
    bytes: Vec<Vec<u32>>,
    start: XY,
    exit: XY,
}

impl Memory {
    pub fn new(input: &str, dest_x: i32, dest_y: i32) -> Memory {
        let mut bytes = vec![vec![u32::MAX; dest_x as usize + 1]; dest_y as usize + 1];
        let mut max_byte = 0;
        input.lines().enumerate().for_each(|(y, line)| {
            let p = XY::new(line);
            bytes[p.y as usize][p.x as usize] = (y + 1) as u32;
            max_byte += 1;
        });

        Memory {
            max_byte,
            bytes,
            start: XY::default(),
            exit: XY { x: dest_x, y: dest_y },
        }
    }

    fn walk(&self, after: u32) -> Option<u32> {
        let mut min_steps = vec![vec![u32::MAX; self.exit.x as usize + 1]; self.exit.y as usize + 1];
        let mut queue = VecDeque::new();
        min_steps[self.start.y as usize][self.start.x as usize] = 0;
        queue.push_back(self.start.clone());

        while !queue.is_empty() {
            let b = queue.pop_front().unwrap();
            if b == self.exit {
                return Some(min_steps[b.y as usize][b.x as usize]);
            }

            for (dx, dy) in NEIGHBORS.iter() {
                let mut next = b.clone();
                next.apply(dx, dy);
                if self.is_valid(&next)
                    && self.bytes[next.y as usize][next.x as usize] > after
                    && min_steps[next.y as usize][next.x as usize] > min_steps[b.y as usize][b.x as usize] + 1 {
                    min_steps[next.y as usize][next.x as usize] = min_steps[b.y as usize][b.x as usize] + 1;
                    queue.push_back(next);
                }
            }
        }
        None
    }

    fn is_valid(&self, b: &XY) -> bool {
        b.x >= self.start.x && b.x <= self.exit.x
            && b.y >= self.start.y && b.y <= self.exit.y
    }

    fn blocking_byte(&self, start: usize, end: usize) -> XY {
        if start >= end {
            for (y, row) in self.bytes.iter().enumerate() {
                for (x, v) in row.iter().enumerate() {
                    if *v == end as u32 {
                        return XY { x: x as i32, y: y as i32 };
                    }
                }
            }
            unreachable!();
        }

        let mid = (start + end) / 2;
        match self.walk(mid as u32) {
            Some(_) => self.blocking_byte(mid + 1, end),
            None => self.blocking_byte(start, mid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Memory, XY};

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_generator() {
        let m = Memory::new(&INPUT, 6, 6);
        assert_eq!(m.start, XY { x: 0, y: 0 });
        assert_eq!(m.exit, XY { x: 6, y: 6 });
        assert_eq!(m.bytes[4][5], 1);
    }

    #[test]
    fn test_part_1() {
        let m = Memory::new(&INPUT, 6, 6);
        assert_eq!(m.walk(12).unwrap(), 22);
    }

    #[test]
    fn test_part_2() {
        let m = Memory::new(&INPUT, 6, 6);
        assert_eq!(m.blocking_byte(0, m.max_byte as usize), XY { x: 6, y: 1 });
    }
}
