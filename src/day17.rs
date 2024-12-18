use crate::day17::Instruction::{ADV, BDV, BST, BXC, BXL, CDV, JNZ, OUT};

pub fn part1(d: &Debugger) -> String {
    let mut d = d.clone();
    d.run().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

pub fn part2(d: &Debugger) -> u64 {
    let mut d = d.clone();
    let expected = d.to_instructions();
    let bits = d.optimize().unwrap();

    find_value(&mut d, bits, 0, expected.len() - 1, &expected).unwrap()
}

fn find_value(d: &mut Debugger, bits: u32, a_value: u64, a_idx: usize, expected: &Vec<u64>) -> Option<u64> {
    for v in 0..2u64.pow(bits) {
        let curr = a_value | (v << (bits * a_idx as u32));
        if curr == 0 { continue; }
        d.state.reset(curr);
        let o = d.run();
        if o.len() == expected.len() && o[a_idx..] == expected[a_idx..] {
            if a_idx == 0 {
                return Some(curr);
            }
            match find_value(d, bits, curr, a_idx - 1, expected) {
                Some(v) => return Some(v),
                None => continue,
            }
        }
    }
    None
}

pub fn generator(input: &str) -> Debugger {
    Debugger::new(input)
}

#[derive(Debug, Clone)]
pub struct Debugger {
    state: State,
    program: Vec<Instruction>,
}

impl Debugger {
    fn new(input: &str) -> Self {
        let (state, program) = input.trim().split_once("\n\n").unwrap();
        let program: Vec<&str> = program.split_once(' ').unwrap().1
            .split(',')
            .collect();
        let program = program.chunks(2).map(|v| Instruction::new(v[0], v[1])).collect();

        Debugger {
            program,
            state: State::new(state),
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output: Vec<u64> = Vec::new();
        while self.state.ip < self.program.len() as u64 {
            if let Some(o) = self.program[self.state.ip as usize].perform(&mut self.state) {
                output.push(o);
            }
        }
        output
    }

    fn optimize(&self) -> Option<u32> {
        let mut found_adv = None;

        let deps: Vec<(u64, (u64, u64))> = self.program.iter().map(|i| {
            match i {
                ADV(o) if o < &4 => {
                    found_adv = Some(*o as u32);
                    (4, (4, *o))
                }
                ADV(o) => (4, (4, *o)),
                BXL(o) => (5, (5, *o)),
                BST(o) => (5, (0, *o)),
                JNZ(_) => (0, (0, 0)),
                BXC(_) => (5, (5, 6)),
                OUT(_) => (0, (0, 0)),
                BDV(o) => (5, (4, *o)),
                CDV(o) => (5, (4, *o)),
            }
        }).collect();

        match deps.iter().find(|(t, _)| t == &4).unwrap() {
            (_, (o1, o2)) if o1 > &5 || o2 > &5 => return None,
            _ => {}
        }

        let first_b = deps.iter().enumerate()
            .find(|(_, (t, _))| t == &5)
            .map(|(i, (_, (o1, o2)))| (i, o1 == &6 || o2 == &6 || o1 == &5 || o2 == &5));
        let first_c = deps.iter().enumerate()
            .find(|(_, (t, _))| t == &6)
            .map(|(i, (_, (o1, o2)))| (i, o1 == &6 || o2 == &6 || o1 == &5 || o2 == &5));

        match (first_b, first_c) {
            (None, None) => found_adv,
            (Some((i, true)), Some((j, false))) if i > j => found_adv,
            (Some((i, false)), Some((j, true))) if j > i => found_adv,
            (Some((_, false)), None) => found_adv,
            (None, Some((_, false))) => found_adv,
            _ => None
        }
    }

    fn to_instructions(&self) -> Vec<u64> {
        let mut expected = Vec::new();
        for i in &self.program {
            match i {
                ADV(op) => {
                    expected.push(0);
                    expected.push(*op);
                }
                BXL(op) => {
                    expected.push(1);
                    expected.push(*op);
                }
                BST(op) => {
                    expected.push(2);
                    expected.push(*op);
                }
                JNZ(op) => {
                    expected.push(3);
                    expected.push(*op);
                }
                BXC(op) => {
                    expected.push(4);
                    expected.push(*op);
                }
                OUT(op) => {
                    expected.push(5);
                    expected.push(*op);
                }
                BDV(op) => {
                    expected.push(6);
                    expected.push(*op);
                }
                CDV(op) => {
                    expected.push(7);
                    expected.push(*op);
                }
            }
        }
        expected
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    ip: u64,
}

impl State {
    fn new(input: &str) -> Self {
        let mut state = State::default();
        let mut lines = input.lines();
        state.a = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();
        state.b = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();
        state.c = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();

        state
    }

    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
    }

    fn combo(&self, op: &u64) -> u64 {
        match op {
            op if op < &4 => *op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    ADV(u64),
    BXL(u64),
    BST(u64),
    JNZ(u64),
    BXC(u64),
    OUT(u64),
    BDV(u64),
    CDV(u64),
}

impl Instruction {
    fn new(inst: &str, op: &str) -> Self {
        let inst = inst.parse::<u64>().unwrap();
        let op = op.parse::<u64>().unwrap();
        match inst {
            0 => ADV(op),
            1 => BXL(op),
            2 => BST(op),
            3 => JNZ(op),
            4 => BXC(op),
            5 => OUT(op),
            6 => BDV(op),
            7 => CDV(op),
            _ => unreachable!()
        }
    }

    fn perform(&self, state: &mut State) -> Option<u64> {
        state.ip += 1;
        match self {
            ADV(op) => {
                state.a >>= state.combo(op);
            }
            BXL(op) => state.b ^= op,
            BST(op) => state.b = state.combo(op) % 8,
            JNZ(op) => {
                if state.a != 0 {
                    state.ip = *op / 2;
                }
            }
            BXC(_) => state.b ^= state.c,
            OUT(op) => {
                return Some(state.combo(op) % 8)
            }
            BDV(op) => state.b = state.a / 2u64.pow(state.combo(op) as u32),
            CDV(op) => state.c = state.a / 2u64.pow(state.combo(op) as u32),
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};
    use crate::day17::Instruction::ADV;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    const INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_generator() {
        let d = generator(&INPUT);
        assert_eq!(d.program.len(), 3);
        assert_eq!(d.program[0], ADV(1));
        assert_eq!(d.state.a, 729);
        assert_eq!(d.state.b, 0);
        assert_eq!(d.state.c, 0);
        assert_eq!(d.state.ip, 0);
    }

    #[test]
    fn test_part_1() {
        let d = generator(&INPUT);
        assert_eq!(part1(&d), "4,6,3,5,6,3,5,2,1,0");

        let d = generator("Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4");
        assert_eq!(part1(&d), "0,1,2");

        let d = generator("Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0");
        assert_eq!(part1(&d), "4,2,5,6,7,7,7,7,3,1,0");

        let d = generator("Register A: 105981155568026
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0");
        assert_eq!(part1(&d), "2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0");
    }

    #[test]
    fn test_optimize() {
        let d = generator(&INPUT);
        assert_eq!(Some(1), d.optimize());

        let d = generator(&INPUT_2);
        assert_eq!(Some(3), d.optimize());

        let d = generator("Register A: 2024
Register B: 0
Register C: 0

Program: 0,4,5,4,3,0");
        assert_eq!(None, d.optimize());

        let d = generator("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,1,3,5,4,3,0");
        assert_eq!(None, d.optimize());
    }

    #[test]
    fn test_part_2() {
        let d = generator(&INPUT_2);
        assert_eq!(117440, part2(&d));
    }
}
