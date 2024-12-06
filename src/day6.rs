use std::iter::Iterator;
use std::mem::swap;

pub fn part1(pm: &PatrolMap) -> u32 {
    pm.patrol().len() as u32
}

pub fn part2(pm: &PatrolMap) -> u32 {
    let visited = pm.patrol();

    visited.iter().filter(|&p| {
        p != &pm.guard_start && pm.is_stuck_in_loop(p)
    })
        .count() as u32
}

pub fn generator(input: &str) -> PatrolMap {
    PatrolMap::new(input)
}

fn turn_right(dir: &mut (i32, i32)) {
    swap(&mut dir.0, &mut dir.1);
    if dir.1 == 0 {
        dir.0 = -dir.0;
    }
}

fn dir_to_idx(dir: &(i32, i32)) -> usize {
    match dir {
        (-1, 0) => 0,
        (0, -1) => 1,
        (1, 0) => 2,
        (0, 1) => 3,
        _ => unreachable!(),
    }
}

pub struct PatrolMap {
    obstacles: Vec<Vec<bool>>,
    guard_start: (i32, i32),
    guard_direction: (i32, i32),
    width: i32,
    height: i32,
}

impl PatrolMap {
    fn new(input: &str) -> Self {
        let mut y = -1;
        let mut obstacles = Vec::new();
        let mut guard_start = (0, 0);
        let mut guard_direction = (0, 0);
        let width = input.lines().next().unwrap().chars().count() as i32;

        for line in input.lines() {
            y += 1;
            let mut row = vec![false; width as usize];
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        row[x] = true;
                    }
                    '^' => {
                        guard_start = (x as i32, y);
                        guard_direction = (0, -1);
                    }
                    '>' => {
                        guard_start = (x as i32, y);
                        guard_direction = (1, 0);
                    }
                    'v' => {
                        guard_start = (x as i32, y);
                        guard_direction = (0, -1);
                    }
                    '<' => {
                        guard_start = (x as i32, y);
                        guard_direction = (-1, 0);
                    }
                    _ => {
                        // Ignore paths
                    }
                }
            }
            obstacles.push(row);
        }

        let height = obstacles.len() as i32;

        PatrolMap {
            obstacles,
            guard_start,
            guard_direction,
            height,
            width,
        }
    }

    fn is_valid_pos(&self, curr: &(i32, i32)) -> bool {
        curr.0 > -1 && curr.0 < self.width
            && curr.1 > -1 && curr.1 < self.height
    }

    fn patrol(&self) -> Vec<(i32, i32)> {
        let mut curr = self.guard_start.clone();
        let mut dir = self.guard_direction.clone();
        let mut visited = vec![vec![false; self.width as usize]; self.height as usize];
        while self.is_valid_pos(&curr) {
            visited[curr.1 as usize][curr.0 as usize] = true;

            curr.0 += dir.0;
            curr.1 += dir.1;
            self.point_away_from_obstacle(&mut curr, &mut dir);
        }

        visited.iter().enumerate()
            .flat_map(|(y, r)| {
                r.iter().enumerate().filter_map(move |(x, v)| {
                    if *v {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            }).collect()
    }

    fn point_away_from_obstacle(&self, curr: &mut (i32, i32), dir: &mut (i32, i32)) {
        while self.is_valid_pos(curr) && self.obstacles[curr.1 as usize][curr.0 as usize] {
            curr.0 -= dir.0;
            curr.1 -= dir.1;
            turn_right(dir);
            curr.0 += dir.0;
            curr.1 += dir.1;
        }
    }

    fn point_away_from_obstacle_w_new(&self, curr: &mut (i32, i32), dir: &mut (i32, i32), new_obstacle: &(i32, i32)) {
        while self.is_valid_pos(curr)
            && (self.obstacles[curr.1 as usize][curr.0 as usize] || new_obstacle == curr) {
            curr.0 -= dir.0;
            curr.1 -= dir.1;
            turn_right(dir);
            curr.0 += dir.0;
            curr.1 += dir.1;
        }
    }

    fn is_stuck_in_loop(&self, new_obstacle: &(i32, i32)) -> bool {
        let mut curr = self.guard_start.clone();
        let mut dir = self.guard_direction.clone();
        let mut visited = vec![vec![[false; 4]; self.width as usize]; self.height as usize];
        while self.is_valid_pos(&curr) {
            if visited[curr.1 as usize][curr.0 as usize][dir_to_idx(&dir)] {
                return true;
            }
            visited[curr.1 as usize][curr.0 as usize][dir_to_idx(&dir)] = true;

            curr.0 += dir.0;
            curr.1 += dir.1;
            self.point_away_from_obstacle_w_new(&mut curr, &mut dir, &new_obstacle);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, turn_right};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_generator() {
        let pm = generator(&INPUT);
        assert_eq!(pm.obstacles.len(), 10);
        assert_eq!(pm.obstacles[0][4], true);
        assert_eq!(pm.obstacles[9][6], true);
        assert_eq!(pm.guard_start, (4, 6));
        assert_eq!(pm.guard_direction, (0, -1));
        assert_eq!(pm.height, 10);
        assert_eq!(pm.width, 10);
    }

    #[test]
    fn test_part_1() {
        let pm = generator(&INPUT);
        assert_eq!(part1(&pm), 41);
    }

    #[test]
    fn test_part_2() {
        let pm = generator(&INPUT);
        assert_eq!(part2(&pm), 6);
    }

    #[test]
    fn test_turn_right() {
        let mut dir = (-1, 0);
        turn_right(&mut dir);
        assert_eq!(dir, (0, -1));
        turn_right(&mut dir);
        assert_eq!(dir, (1, 0));
        turn_right(&mut dir);
        assert_eq!(dir, (0, 1));
        turn_right(&mut dir);
        assert_eq!(dir, (-1, 0));
    }
}
