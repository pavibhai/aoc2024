use crate::day20::Space::{TRACK, WALL};

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part1(race_track: &RaceTrack) -> u32 {
    race_track.compute_savings(2, 100, true)
}

pub fn part2(race_track: &RaceTrack) -> u32 {
    race_track.compute_savings(20, 100, true)
}

pub fn generator(input: &str) -> RaceTrack {
    RaceTrack::new(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Space {
    TRACK,
    WALL,
}

impl Space {
    fn new(space: &char) -> Space {
        match space {
            '.' | 'S' | 'E' => TRACK,
            '#' => WALL,
            _ => unreachable!("Invalid space: {}", space),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn is_same(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }

    fn manhattan_distance(&self, other: &XY) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct RaceTrack {
    path: Vec<XY>,
}

impl RaceTrack {
    fn new(input: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let map = input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                if c == 'S' {
                    start = Some(XY { x: x as i32, y: y as i32 });
                } else if c == 'E' {
                    end = Some(XY { x: x as i32, y: y as i32 });
                }
                Space::new(&c)
            }).collect()
        }).collect();
        let path = Self::compute_path(map, start.unwrap(), end.unwrap());

        Self {
            path
        }
    }

    fn compute_path(map: Vec<Vec<Space>>, start: XY, end: XY) -> Vec<XY> {
        let mut path = Vec::new();
        let mut curr = start.clone();
        path.push(start);
        while curr != end {
            for (dx, dy) in NEIGHBORS.iter() {
                if curr.y + dy > -1 && curr.y + dy < map.len() as i32
                    && curr.x + dx > -1 && curr.x + dx < map[0].len() as i32
                    && (path.len() == 1 || !path[path.len() - 2].is_same(curr.x + dx, curr.y + dy))
                    && map[(curr.y + dy) as usize][(curr.x + dx) as usize] == TRACK {
                    curr.x += dx;
                    curr.y += dy;
                    path.push(curr.clone());
                    break;
                }
            }
        }
        path
    }

    fn compute_savings(&self, cheat_time: u32, saving: u32, minimum: bool) -> u32 {
        let mut cheats = 0;
        for i in 0..self.path.len() - saving as usize {
            for j in i + saving as usize..self.path.iter().len() {
                match self.path[i].manhattan_distance(&self.path[j]) {
                    d if d <= cheat_time && j as u32 - i as u32 - d == saving => {
                        cheats += 1;
                    }
                    d if minimum && d <= cheat_time && j as u32 - i as u32 - d > saving => {
                        cheats += 1;
                    }
                    _ => {}
                }
            }
        }
        cheats
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, XY};

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_generator() {
        let rt = generator(&INPUT);
        assert_eq!(rt.path[0], XY { x: 1, y: 3 });
        assert_eq!(rt.path.last().unwrap(), &XY { x: 5, y: 7 });
        assert_eq!(rt.path.len(), 85);
    }

    #[test]
    fn test_part_1() {
        let rt = generator(&INPUT);

        assert_eq!(14, rt.compute_savings(2, 4, false));
        assert_eq!(2, rt.compute_savings(2, 6, false));
        assert_eq!(4, rt.compute_savings(2, 8, false));
        assert_eq!(2, rt.compute_savings(2, 10, false));
        assert_eq!(3, rt.compute_savings(2, 12, false));
        assert_eq!(1, rt.compute_savings(2, 20, false));
        assert_eq!(1, rt.compute_savings(2, 36, false));
        assert_eq!(1, rt.compute_savings(2, 38, false));
        assert_eq!(1, rt.compute_savings(2, 40, false));
        assert_eq!(1, rt.compute_savings(2, 64, false));
    }

    #[test]
    fn test_part_2() {
        let rt = generator(&INPUT);
        assert_eq!(1, rt.compute_savings(6, 76, false));

        assert_eq!(32, rt.compute_savings(20, 50, false));
        assert_eq!(31, rt.compute_savings(20, 52, false));
        assert_eq!(29, rt.compute_savings(20, 54, false));
        assert_eq!(39, rt.compute_savings(20, 56, false));
        assert_eq!(25, rt.compute_savings(20, 58, false));
        assert_eq!(23, rt.compute_savings(20, 60, false));
        assert_eq!(20, rt.compute_savings(20, 62, false));
        assert_eq!(19, rt.compute_savings(20, 64, false));
        assert_eq!(12, rt.compute_savings(20, 66, false));
        assert_eq!(14, rt.compute_savings(20, 68, false));
        assert_eq!(12, rt.compute_savings(20, 70, false));
        assert_eq!(22, rt.compute_savings(20, 72, false));
        assert_eq!(4, rt.compute_savings(20, 74, false));
        assert_eq!(3, rt.compute_savings(20, 76, false));
        assert_eq!(285, rt.compute_savings(20, 50, true));
    }
}
