use std::collections::HashMap;
use std::fmt::Display;
use crate::day21::DKey::{ADir, Down, EDir, Left, Right, Up};
use crate::day21::NKey::{Eight, Five, Four, ANum, Nine, One, Seven, Six, Three, Two, Zero};

const DIRECTION_PAD: [[DKey; 3]; 2] = [
    [EDir, Up, ADir],
    [Left, Down, Right]
];

const NUMERIC_PAD: [[NKey; 3]; 4] = [
    [Seven, Eight, Nine],
    [Four, Five, Six],
    [One, Two, Three],
    [NKey::ENum, Zero, ANum]
];

pub fn part1(codes: &Vec<Vec<NKey>>) -> u64 {
    let mut p = Puzzle::new(2);
    codes.iter()
        .map(|code| {
            let l = p.press_keys(code);
            let v = NKey::value(code) as u64;
            l * v
        }).sum()
}

pub fn part2(codes: &Vec<Vec<NKey>>) -> u64 {
    let mut p = Puzzle::new(25);
    codes.iter()
        .map(|code| {
            let l = p.press_keys(code);
            let v = NKey::value(code) as u64;
            l * v
        }).sum()
}

pub fn generator(input: &str) -> Vec<Vec<NKey>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(NKey::new)
                .collect()
        }).collect()
}

struct Puzzle {
    n_pad: NPad,
    d_pad: DPad,
    bots: usize,
    cache: HashMap<(DKey, DKey, usize), u64>,
}

impl Puzzle {
    fn new(bots: usize) -> Self {
        Self {
            bots,
            n_pad: NPad::default(),
            d_pad: DPad::default(),
            cache: HashMap::new(),
        }
    }

    fn press_keys(&mut self, keys: &[NKey]) -> u64 {
        let mut collect_1 = 0u64;
        let mut collect_2 = 0u64;
        keys.iter().for_each(|key| {
            let (mut d_keys, reversible) = self.n_pad.press(key);
            collect_1 += self.press_dpad_keys(&d_keys, 0);

            if reversible {
                d_keys.pop();
                d_keys.reverse();
                d_keys.push(ADir);
                collect_2 += self.press_dpad_keys(&d_keys, 0);
                if collect_1 > collect_2 {
                    collect_1 = collect_2;
                } else {
                    collect_2 = collect_1;
                }
            } else {
                collect_2 = collect_1;
            }
        });
        collect_1
    }

    fn press_dpad_keys(&mut self, keys: &[DKey], level: usize) -> u64 {
        let mut result = 0u64;
        for (i, k) in keys.iter().enumerate() {
            let prev = if i == 0 {
                ADir
            } else {
                keys[i - 1]
            };

            match self.cache.get(&(prev, k.clone(), level)) {
                Some(value) => {
                    result += *value;
                    continue;
                }
                _ => {}
            }

            let (mut keys, reversible) = self.d_pad.press(&prev, k);
            if level + 1 == self.bots {
                result += keys.len() as u64;
                continue;
            }

            let v1 = self.press_dpad_keys(&keys, level + 1);
            if reversible {
                keys.pop();
                keys.reverse();
                keys.push(ADir);
                let v2 = self.press_dpad_keys(&keys, level + 1);
                if v1 > v2 {
                    self.cache.insert((prev, k.clone(), level), v2);
                    result += v2;
                } else {
                    self.cache.insert((prev, k.clone(), level), v1);
                    result += v1;
                }
            } else {
                self.cache.insert((prev, k.clone(), level), v1);
                result += v1;
            }
        }
        result
    }
}

#[derive(Debug, Copy, Clone)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum DKey {
    ADir,
    Up,
    Down,
    Left,
    Right,
    EDir,
}

impl Display for DKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            ADir => 'A',
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
            _ => unreachable!(),
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NKey {
    ANum,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    ENum,
}

impl NKey {
    fn new(c: char) -> Self {
        match c {
            'A' => ANum,
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => unreachable!()
        }
    }

    fn to_number(&self) -> u32 {
        match self {
            Zero => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            _ => unreachable!()
        }
    }

    fn value(codes: &[NKey]) -> u32 {
        let mut result = 0;
        codes[..codes.len() - 1].iter()
            .for_each(|code| result = result * 10 + code.to_number());
        result
    }
}

#[derive(Clone, Debug)]
struct DPad {
    keys: HashMap<DKey, XY>,
    avoid: XY,
}

impl DPad {
    fn default() -> Self {
        let mut keys = HashMap::new();
        for (y, line) in DIRECTION_PAD.iter().enumerate() {
            for (x, key) in line.iter().enumerate() {
                keys.insert(key.clone(), XY { x: x as i32, y: y as i32 });
            }
        }
        let avoid = keys.get(&EDir).unwrap().clone();
        DPad { keys, avoid }
    }

    fn press(&mut self, start: &DKey, key: &DKey) -> (Vec<DKey>, bool) {
        let start = self.keys.get(start).unwrap();
        let end = self.keys.get(key).unwrap();

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let mut moves = Vec::new();
        // Down
        if dy > 0 { (0..dy.abs()).for_each(|_| { moves.push(Down) }); }
        // Right
        if dx > 0 { (0..dx.abs()).for_each(|_| { moves.push(Right) }); }
        // Up
        if dy < 0 { (0..dy.abs()).for_each(|_| { moves.push(Up) }); }
        // Left
        if dx < 0 { (0..dx.abs()).for_each(|_| { moves.push(Left) }); }
        moves.push(ADir);
        let dont_reverse = (self.avoid.y == start.y && self.avoid.x == end.x)
            || (self.avoid.x == start.x && self.avoid.y == end.y);
        (moves, !dont_reverse)
    }
}

struct NPad {
    keys: HashMap<NKey, XY>,
    curr: NKey,
    avoid: XY,
}

impl NPad {
    fn default() -> Self {
        let mut keys = HashMap::new();
        for (y, line) in NUMERIC_PAD.iter().enumerate() {
            for (x, key) in line.iter().enumerate() {
                keys.insert(key.clone(), XY { x: x as i32, y: y as i32 });
            }
        }
        let avoid = keys.get(&NKey::ENum).unwrap().clone();
        NPad { keys, curr: ANum, avoid }
    }

    fn press(&mut self, key: &NKey) -> (Vec<DKey>, bool) {
        let start = self.keys.get(&self.curr).unwrap();
        let end = self.keys.get(key).unwrap();
        self.curr = *key;

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let mut moves = Vec::new();
        //Up
        if dy < 0 { (0..dy.abs()).for_each(|_| { moves.push(Up) }); }
        //Right
        if dx > 0 { (0..dx.abs()).for_each(|_| { moves.push(Right) }); }
        //Left
        if dx < 0 { (0..dx.abs()).for_each(|_| { moves.push(Left) }); }
        //Down
        if dy > 0 { (0..dy.abs()).for_each(|_| { moves.push(Down) }); }
        moves.push(ADir);
        let dont_reverse = (self.avoid.y == start.y && self.avoid.x == end.x)
            || (self.avoid.x == start.x && self.avoid.y == end.y);
        (moves, !dont_reverse)
    }
}

#[cfg(test)]
mod tests {
    use crate::day21::DKey::{ADir, Down, Left, Right, Up};
    use crate::day21::NKey::{ANum, Eight, Nine, One, Two, Zero, Seven};
    use super::{generator, part1, part2, DPad, NPad, Puzzle};

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_generator() {
        let codes = generator(&INPUT);
        assert_eq!(codes[0], [Zero, Two, Nine, ANum]);
        assert_eq!(codes[1], [Nine, Eight, Zero, ANum]);
    }

    #[test]
    fn test_part_1() {
        let mut n_pad = NPad::default();
        assert_eq!(n_pad.press(&Zero).0, vec![Left, ADir]);
        assert_eq!(n_pad.press(&Two).0, vec![Up, ADir]);
        assert_eq!(n_pad.press(&Nine).0, vec![Up, Up, Right, ADir]);
        assert_eq!(n_pad.press(&ANum).0, vec![Down, Down, Down, ADir]);

        let mut d_pad = DPad::default();
        assert_eq!(d_pad.press(&ADir, &Left).0, vec![Down, Left, Left, ADir]);

        let mut p = Puzzle::new(2);
        assert_eq!(p.press_keys(&[Zero, Two, Nine, ANum]),
                   "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len() as u64);
        let result = p.press_keys(&[One, Seven, Nine, ANum]);
        assert_eq!(result,
                   "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len() as u64);

        let codes = generator(&INPUT);
        assert_eq!(part1(&codes), 126384);
    }

    #[test]
    fn test_part_2() {
        let p = generator(&INPUT);
        assert_eq!(part2(&p), 154115708116294);
    }
}
