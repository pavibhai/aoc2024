use std::iter::Iterator;
use std::mem::swap;

pub fn part1(equations: &[Equation]) -> u64 {
    equations.iter()
        .filter_map(|e| if e.solve_simple { Some(e.value) } else { None }).sum()
}

pub fn part2(equations: &[Equation]) -> u64 {
    equations.iter().filter_map(|e| {
        if e.solve_simple || e.solve::<true>() > 0 {
            Some(e.value)
        } else {
            None
        }
    })
        .sum()
}

pub fn generator(input: &str) -> Vec<Equation> {
    input.lines()
        .map(|l| Equation::new(l.trim()))
        .collect()
}

#[derive(Debug)]
pub struct Equation {
    value: u64,
    operands: Vec<u64>,
    solve_simple: bool,
}

impl Equation {
    fn new(input: &str) -> Self {
        let (value, operands) = input.split_once(": ").unwrap();
        let value = value.parse().unwrap();
        let operands = operands.split(" ")
            .map(|o| o.parse().unwrap()).collect();

        let mut e = Equation {
            value,
            operands,
            solve_simple: false,
        };
        if e.solve::<false>() > 0 {
            e.solve_simple = true;
        }
        e
    }

    fn solve<const CONCATENATE:bool>(&self) -> usize {
        let mut next = vec![self.value];
        // Go backwards to minimize the explosion
        let values = self.operands.iter().enumerate().rev().peekable();
        let mut input = Vec::new();

        for (idx, o) in values {
            swap(&mut input, &mut next);
            next.clear();
            input.iter().for_each(|v| {
                // If the operand is larger than the value then we can do no meaningful operation
                if o > v {
                    return;
                }
                if o > &0 && v % o == 0 {
                    match v / o {
                        1 if idx == 0 => {
                            next.push(0);
                        }
                        x => next.push(x),
                    }
                }

                if v >= o {
                    next.push(v - o);
                }

                if CONCATENATE {
                    if o > &0 {
                        let mut divisor = 10;
                        while &divisor <= o {
                            divisor *= 10;
                        }
                        if v % divisor == *o {
                            next.push(v / divisor);
                        }
                    } else if o == &0 && v % 10 == 0 {
                        next.push(v / 10);
                    }
                }
            })
        }

        next.iter().filter(|&v| v == &0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, Equation};

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_generator() {
        let equations = generator(&INPUT);
        assert_eq!(9, equations.len());
    }

    #[test]
    fn test_part_1() {
        let equations = generator(&INPUT);
        assert_eq!(3749, part1(&equations));
    }

    #[test]
    fn test_part_2() {
        let equations = generator(&INPUT);
        assert_eq!(11387, part2(&equations));
    }

    #[test]
    fn test_solve() {
        let e = Equation::new("161011: 16 10 13");
        assert_eq!(0, e.solve::<false>());

        let e = Equation::new("83: 17 5");
        assert_eq!(0, e.solve::<false>());

        let e = Equation::new("77502824880: 9 574 9 6 8 8 7 2 26 8 5 3");
        assert_eq!(0, e.solve::<false>());
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(0, Equation::new("83: 8 4").solve::<true>());
        assert_eq!(false, Equation::new("161011: 16 10 13").solve::<true>() > 0);
        assert_eq!(false, Equation::new("100: 1 0 0").solve::<false>() > 0);
        assert_eq!(true, Equation::new("100: 1 0 0").solve::<true>() > 0);
    }
}
