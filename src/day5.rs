use std::collections::HashSet;
use std::iter::Iterator;

pub fn part1(puzzle: &Puzzle) -> u32 {
    puzzle.score_valid_updates()
}

pub fn part2(puzzle: &Puzzle) -> u32 {
    puzzle.score_invalid_updates()
}

pub fn generator(input: &str) -> Puzzle {
    Puzzle::new(input)
}

pub struct Puzzle {
    valid: Vec<Vec<u32>>,
    invalid: Vec<Vec<(u32, i32)>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules: HashSet<(u32, u32)> = rules.lines()
            .map(|l| {
                let (b, a) = l.split_once("|").unwrap();
                (b.parse().unwrap(), a.parse().unwrap())
            }).collect();

        let mut valid = Vec::new();
        let mut invalid: Vec<Vec<(u32, i32)>> = Vec::new();

        updates.lines().for_each(|line| {
            let u: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            let mut pass = true;
            let mut ordering = vec![0i32; u.len()];
            for i in 0..u.len() - 1 {
                for j in i+1..u.len() {
                    if rules.contains(&(u[i], u[j])) {
                        ordering[i] -= 1;
                        ordering[j] += 1;
                    } else {
                        pass = false;
                        ordering[i] += 1;
                        ordering[j] -= 1;
                    }
                }
            }
            if pass {
                valid.push(u);
            } else {
                invalid.push(u.iter().zip(ordering).map(|(&p, o)| (p, o)).collect());
            }
        });

        Puzzle {
            valid,
            invalid,
        }
    }

    fn score_valid_updates(&self) -> u32 {
        self.valid.iter().map(|u| u[u.len() / 2])
            .sum()
    }

    fn score_invalid_updates(&self) -> u32 {
        let mut score = 0;
        self.invalid.iter().for_each(|u| {
            score += u.iter().find_map(|(p, o)| if o == &0 {
                Some(p)
            } else {
                None
            }).unwrap();
        });

        score
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_generator() {
        let puzzle = generator(&INPUT);
        assert_eq!(puzzle.valid.len() + puzzle.invalid.len(), 6);
    }

    #[test]
    fn test_part_1() {
        let puzzle = generator(&INPUT);
        assert_eq!(part1(&puzzle), 143);
    }

    #[test]
    fn test_part_2() {
        let puzzle = generator(&INPUT);
        assert_eq!(part2(&puzzle), 123);
    }
}
