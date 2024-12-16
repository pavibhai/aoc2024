use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::day16::Space::{EMPTY, WALL};

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part1(m: &Maze) -> u32 {
    m.score::<false>()
}

pub fn part2(m: &Maze) -> u32 {
    m.score::<true>()
}

pub fn generator(input: &str) -> Maze {
    Maze::new(input)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Space {
    WALL,
    EMPTY(Vec<XY>),
}

impl Space {}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn apply(&mut self, dir: &XY) {
        self.x += dir.x;
        self.y += dir.y;
    }
}

pub struct Maze {
    map: Vec<Vec<Space>>,
    start: XY,
    dir: XY,
    end: XY,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let char_map: Vec<Vec<char>> = input.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => start = Some(XY { x: x as i32, y: y as i32 }),
                    'E' => end = Some(XY { x: x as i32, y: y as i32 }),
                    _ => {}
                }
                c
            }).collect()
        }).collect();

        let mut map = vec![vec![WALL; char_map[0].len()]; char_map.len()];
        char_map.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| {
                if c == &'#' { return; }
                let paths: Vec<XY> = NEIGHBORS.iter().filter_map(|(dx, dy)| {
                    if char_map[(y as i32 + dy) as usize][(x as i32 + dx) as usize] != '#' {
                        Some(XY { x: *dx, y: *dy })
                    } else {
                        None
                    }
                }).collect();
                map[y][x] = EMPTY(paths);
            });
        });

        Maze {
            map,
            start: start.unwrap(),
            dir: XY { x: 1, y: 0 },
            end: end.unwrap(),
        }
    }

    fn score<const COUNT_BEST_POS: bool>(&self) -> u32 {
        let mut path = Path::new(self.start.clone(), self.dir.clone());
        path.visited.push(self.start.clone());
        let mut best_scores = HashMap::new();
        let mut best_spots: HashSet<XY> = HashSet::new();
        let mut best_score = None;
        let mut heap = BinaryHeap::new();
        heap.push(path);

        while !heap.is_empty() {
            let p = heap.pop().unwrap();
            if COUNT_BEST_POS && best_score.is_some() && p.score > best_score.unwrap() {
                break;
            } else if p.pos == self.end {
                best_score = Some(p.score);
                if !COUNT_BEST_POS {
                    break;
                }
                for pos in p.visited.iter() {
                    best_spots.insert(pos.clone());
                }
            }

            match best_scores.get_mut(&(p.pos.clone(), p.dir.clone())) {
                None => {
                    best_scores.insert((p.pos.clone(), p.dir.clone()), p.score);
                }
                Some(score) if *score > p.score || (*score == p.score && COUNT_BEST_POS) => {
                    *score = p.score;
                }
                _ => { continue }
            }

            match &self.map[p.pos.y as usize][p.pos.x as usize] {
                EMPTY(choices) => {
                    choices.iter().filter(|&c| {
                        let mut next = p.pos.clone();
                        next.apply(c);
                        (c.x + p.dir.x != 0) || (c.y + p.dir.y != 0)
                    })
                        .for_each(|c| {
                            match c {
                                c if c == &p.dir => {
                                    let mut p = p.clone();
                                    p.pos.apply(&c);
                                    p.score += 1;
                                    p.visited.push(p.pos.clone());
                                    heap.push(p);
                                }
                                _ => {
                                    let mut p = p.clone();
                                    p.dir.x = c.x;
                                    p.dir.y = c.y;
                                    p.score += 1001;
                                    p.pos.apply(&c);
                                    p.visited.push(p.pos.clone());
                                    heap.push(p);
                                }
                            }
                        })
                }
                _ => unreachable!()
            }
        }

        if COUNT_BEST_POS {
            best_spots.len() as u32
        } else {
            best_score.unwrap()
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    score: u32,
    pos: XY,
    dir: XY,
    visited: Vec<XY>,
}

impl Path {
    fn new(pos: XY, dir: XY) -> Self {
        Path {
            pos,
            dir,
            score: 0,
            visited: Vec::new(),
        }
    }
}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Path {}

impl PartialOrd<Self> for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score).reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::Space::{EMPTY, WALL};
    use super::{generator, part1, part2, XY};

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_generator() {
        let m = generator(&INPUT);
        assert_eq!(m.dir, XY { x: 1, y: 0 });
        assert_eq!(m.end, XY { x: 13, y: 1 });
        assert_eq!(m.start, XY { x: 1, y: 13 });
        assert_eq!(m.map[0][0], WALL);
        assert_eq!(m.map[1][13], EMPTY(vec![XY { x: -1, y: 0 }, XY { x: 0, y: 1 }]));
        assert_eq!(m.map[13][1], EMPTY(vec![XY { x: 1, y: 0 }, XY { x: 0, y: -1 }]));
    }

    #[test]
    fn test_part_1() {
        let m = generator(&INPUT);
        assert_eq!(part1(&m), 7036);

        let m = generator(&INPUT_2);
        assert_eq!(part1(&m), 11048);
    }

    #[test]
    fn test_part_2() {
        let m = generator(&INPUT);
        assert_eq!(part2(&m), 45);

        let m = generator(&INPUT_2);
        assert_eq!(part2(&m), 64);
    }
}
