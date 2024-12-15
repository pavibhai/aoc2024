use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const WALL: u8 = 9;
const EMPTY: u8 = 0;
const OBSTACLE: u8 = 1;
const OBSTACLE_2: u8 = 2;

pub fn part1(p: &Puzzle) -> u64 {
    let mut pm = p.clone();
    p.moves.iter().for_each(|m| {
        pm.apply(m);
    });
    pm.gps()
}

pub fn part2(p: &Puzzle) -> u64 {
    let mut pm = p.clone();
    pm.double();
    p.moves.iter().for_each(|m| {
        pm.apply_double(m);
    });
    pm.gps()
}

pub fn generator(input: &str) -> Puzzle {
    Puzzle::new(input)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn apply(&mut self, m: &XY) {
        self.x += m.x;
        self.y += m.y;
    }

    fn revert(&mut self, m: &XY) {
        self.x -= m.x;
        self.y -= m.y;
    }
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    robot: XY,
    map: Vec<Vec<u8>>,
    moves: Vec<XY>,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, s)| {
                match s {
                    &WALL => output.push('#'),
                    &OBSTACLE => output.push('['),
                    &OBSTACLE_2 => output.push(']'),
                    &EMPTY if y as i32 == self.robot.y && x as i32 == self.robot.x => output.push('@'),
                    _ => output.push('.'),
                }
            });
            output.push('\n');
        });
        output.pop();
        write!(f, "{output}")
    }
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let mut robot: Option<XY> = None;
        let map: Vec<Vec<u8>> = map.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().map(|(x, c)| {
                match c {
                    '.' => EMPTY,
                    '#' => WALL,
                    'O' => OBSTACLE,
                    '@' if robot.is_none() => {
                        robot = Some(XY { x: x as i32, y: y as i32 });
                        EMPTY
                    }
                    '@' => panic!("Found two positions of the robot"),
                    _ => panic!("Invalid character in map: {}", c),
                }
            }).collect()
        }).collect();
        let moves: Vec<XY> = moves.lines().flat_map(|l| {
            l.chars().map(|c| {
                match c {
                    '^' => XY { x: 0, y: -1 },
                    '>' => XY { x: 1, y: 0 },
                    'v' => XY { x: 0, y: 1 },
                    '<' => XY { x: -1, y: 0 },
                    _ => panic!("Unexpected character {c}")
                }
            })
        }).collect();

        Puzzle {
            map,
            robot: robot.unwrap(),
            moves,
        }
    }

    fn apply(&mut self, m: &XY) -> bool {
        let mut pos = self.robot.clone();
        pos.apply(m);
        let mut times = 0;
        while self.map[pos.y as usize][pos.x as usize] != EMPTY && self.map[pos.y as usize][pos.x as usize] != WALL {
            times += 1;
            pos.apply(&m);
        }

        match self.map[pos.y as usize][pos.x as usize] {
            EMPTY => {
                // Move everything forward
                for _ in 0..times {
                    self.map[pos.y as usize][pos.x as usize] = self.map[(pos.y - m.y) as usize][(pos.x - m.x) as usize];
                    pos.revert(&m);
                }
                self.map[pos.y as usize][pos.x as usize] = EMPTY;
                self.robot = pos;
                true
            }
            WALL => {
                // Move failed
                false
            }
            _ => panic!("Unexpected!!!")
        }
    }

    fn gps(&self) -> u64 {
        self.map.iter().enumerate().flat_map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, v)| {
                if v == &OBSTACLE {
                    Some(100 * y as u64 + x as u64)
                } else {
                    None
                }
            })
        }).sum()
    }

    fn add_check(&self, p: XY, to_check: &mut HashSet<XY>) {
        match self.map[p.y as usize][p.x as usize] {
            OBSTACLE => {
                to_check.insert(XY { x: p.x + 1, y: p.y });
            }
            OBSTACLE_2 => {
                to_check.insert(XY { x: p.x - 1, y: p.y });
            }
            _ => {}
        }
        to_check.insert(p);
    }

    fn apply_double(&mut self, m: &XY) -> bool {
        if m.y == 0 {
            return self.apply(m);
        }

        // Handle the vertical movement
        let mut pos = self.robot.clone();
        pos.apply(m);
        let mut to_check = HashSet::new();
        let mut to_change = Vec::new();
        self.add_check(pos, &mut to_check);

        while !to_check.is_empty() {
            let to_process: Vec<XY> = to_check.drain().collect();
            for pos in to_process {
                match self.map[pos.y as usize][pos.x as usize] {
                    WALL => { return false; }
                    EMPTY => {}
                    _ => {
                        let mut pos = pos.clone();
                        pos.apply(m);
                        self.add_check(pos, &mut to_check);
                    }
                }
                to_change.push(pos);
            }
        }
        let change_items: HashSet<XY> = to_change.clone().into_iter().collect();

        while !to_change.is_empty() {
            let p = to_change.pop().unwrap();
            self.map[p.y as usize][p.x as usize] = if change_items.contains(&XY { x: p.x - m.x, y: p.y - m.y }) {
                self.map[(p.y - m.y) as usize][(p.x - m.x) as usize]
            } else {
                EMPTY
            };
        }
        self.robot.apply(m);
        true
    }

    fn double(&mut self) {
        self.map.iter_mut().for_each(|m| {
            let mut n = vec![EMPTY; m.len() * 2];
            m.iter().enumerate().for_each(|(x, y)| {
                match y {
                    &WALL => {
                        n[x * 2] = WALL;
                        n[x * 2 + 1] = WALL;
                    }
                    &OBSTACLE => {
                        n[x * 2] = OBSTACLE;
                        n[x * 2 + 1] = OBSTACLE_2;
                    }
                    _ => {}
                }
            });
            *m = n;
        });
        self.robot.x *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, EMPTY, OBSTACLE, WALL, XY};

    const INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_generator() {
        let p = generator(&INPUT_SMALL);
        assert_eq!(p.robot, XY { x: 2, y: 2 });
        assert_eq!(p.map.len(), 8);
        assert_eq!(p.map[0].len(), 8);
        assert_eq!(p.map[0][0], WALL);
        assert_eq!(p.map[2][1], WALL);
        assert_eq!(p.map[1][3], OBSTACLE);

        let p = generator(&INPUT_LARGE);
        assert_eq!(p.moves.len(), 700);
    }

    #[test]
    fn test_moves() {
        let mut p = generator(&INPUT_SMALL);
        assert_eq!(p.apply(&XY { x: -1, y: 0 }), false);
        assert_eq!(p.apply(&XY { x: 0, y: -1 }), true);
        assert_eq!(p.robot, XY { x: 2, y: 1 });
        assert_eq!(OBSTACLE, p.map[1][3]);
        assert_eq!(EMPTY, p.map[1][4]);
        assert_eq!(p.apply(&XY { x: 1, y: 0 }), true);
        assert_eq!(p.robot, XY { x: 3, y: 1 });
        assert_eq!(EMPTY, p.map[1][3]);
        assert_eq!(OBSTACLE, p.map[1][4]);
    }

    #[test]
    fn test_part_1() {
        let p = generator(&INPUT_SMALL);
        assert_eq!(part1(&p), 2028);

        let p = generator(&INPUT_LARGE);
        assert_eq!(part1(&p), 10092);
    }

    #[test]
    fn test_double() {
        let mut p = generator("#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^");
        p.double();
        assert_eq!(p.map.len(), 7);
        assert_eq!(p.map[0].len(), 14);
        assert_eq!(p.robot, XY { x: 10, y: 3 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############");
        p.apply_double(&XY { x: -1, y: 0 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############");
        p.apply_double(&XY { x: 0, y: 1 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############");
        p.apply_double(&XY { x: 0, y: 1 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############");
        p.apply_double(&XY { x: -1, y: 0 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############");

        p.apply_double(&XY { x: -1, y: 0 });
        assert_eq!(p.to_string(), "##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############");

        p.apply_double(&XY { x: 0, y: -1 });
        assert_eq!(p.to_string(), "##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############");

        p.apply_double(&XY { x: 0, y: -1 });
        assert_eq!(p.to_string(), "##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############");

        p.apply_double(&XY { x: -1, y: 0 });
        p.apply_double(&XY { x: -1, y: 0 });
        assert_eq!(p.to_string(), "##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############");
        p.apply_double(&XY { x: 0, y: -1 });
        assert_eq!(p.to_string(), "##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############");
        p.apply_double(&XY { x: 0, y: -1 });
        assert_eq!(p.to_string(), "##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############");
    }

    #[test]
    fn test_part_2() {
        let p = generator(&INPUT_LARGE);
        assert_eq!(part2(&p), 9021);
    }
}
