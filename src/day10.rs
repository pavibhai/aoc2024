use std::iter::Iterator;

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part1(p: &Puzzle) -> u32 {
    p.score_all(true)
}

pub fn part2(p: &Puzzle) -> u32 {
    p.score_all(false)
}

pub fn generator(input: &str) -> Puzzle {
    Puzzle::new(input)
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct XY {
    x: i32,
    y: i32,
}

pub struct Puzzle {
    map: Vec<Vec<u8>>,
    width: i32,
    height: i32,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u8>> = input.lines()
            .map(|l| { l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect() })
            .collect();
        let height = map.len() as i32;
        let width = map[0].len() as i32;
        Puzzle { map, width, height }
    }

    fn score_all(&self, unique: bool) -> u32 {
        self.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, pixel)| {
                if pixel == &0 {
                    return Some(self.score(XY { x: x as i32, y: y as i32 }, unique));
                } else {
                    None
                }
            })
        }).sum()
    }

    fn score(&self, p: XY, unique: bool) -> u32 {
        let mut stack = Vec::new();
        stack.push(XY { x: p.x, y: p.y });
        let mut visited = Vec::new();

        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            let ch = self.height(&c).unwrap();
            if ch == 9 {
                visited.push(c);
                continue;
            }

            NEIGHBORS.iter().for_each(|(dx, dy)| {
                let p = XY { x: c.x + dx, y: c.y + dy };
                match self.height(&p) {
                    Some(h) if ch + 1 == h => stack.push(p),
                    _ => {}
                }
            });
        }

        if unique {
            visited.sort_by_key(|xy| (xy.x, xy.y));
            visited.dedup();
        }
        visited.len() as u32
    }

    fn height(&self, p: &XY) -> Option<u8> {
        if p.x > -1 && p.x < self.width
            && p.y > -1 && p.y < self.height {
            Some(self.map[p.y as usize][p.x as usize])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_generator() {
        let p = generator(&INPUT);
        assert_eq!(p.height, 8);
        assert_eq!(p.width, 8);
    }

    #[test]
    fn test_part_1() {
        let p = generator(&INPUT);
        assert_eq!(part1(&p), 36);
    }

    #[test]
    fn test_part_2() {
        let p = generator(&INPUT);
        assert_eq!(part2(&p), 81);
    }
}
