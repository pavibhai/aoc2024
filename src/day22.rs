use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

const PRUNE_VALUE: u32 = 16777216 - 1;
const PRICE_CHANGES: usize = 4;

pub fn part1(secrets: &Secrets) -> u64 {
    secrets.numbers.iter().map(|n| *secrets.compute_n_secrets(*n, 2000).last().unwrap() as u64)
        .sum()
}

pub fn part2(secrets: &Secrets) -> u64 {
    let mut total_sequences = HashMap::new();

    for secret in secrets.numbers.iter() {
        let mut sequences = secrets.get_sequences(*secret, 2000);
        for (s, v) in sequences.drain() {
            total_sequences.entry(s)
                .and_modify(|total| *total += v as u64)
                .or_insert(v as u64);
        }
    }

    *total_sequences.values().max().unwrap()
}

pub fn generator(input: &str) -> Secrets {
    Secrets::new(input)
}

pub struct Secrets {
    numbers: Vec<u32>,
}

impl Secrets {
    fn new(input: &str) -> Self {
        let numbers = input.lines().map(|l| l.parse().unwrap()).collect();
        Self {
            numbers,
        }
    }

    fn compute_n_secrets(&self, start: u32, n: usize) -> Vec<u32> {
        let mut secrets = Vec::new();
        secrets.push(start);
        let mut curr = start;
        for _ in 0..n {
            curr = self.next_secret(curr);
            secrets.push(curr);
        }
        secrets
    }

    fn next_secret(&self, n: u32) -> u32 {
        let mut next = n;
        next ^= next << 6;
        next &= PRUNE_VALUE;

        next ^= next >> 5;
        next &= PRUNE_VALUE;

        next ^= next << 11;
        next &= PRUNE_VALUE;
        next
    }

    fn get_sequences(&self, secret: u32, n: usize) -> HashMap<Vec<i8>, u32> {
        let mut sequences = HashMap::new();
        let next_secrets = self.compute_n_secrets(secret, n);
        let mut sequence = VecDeque::new();
        for i in 1..PRICE_CHANGES {
            sequence.push_back((next_secrets[i] % 10) as i8 - (next_secrets[i - 1] % 10) as i8);
        }

        for i in PRICE_CHANGES..n {
            sequence.push_back((next_secrets[i] % 10) as i8 - (next_secrets[i - 1] % 10) as i8);
            let v = sequence.iter().cloned().collect_vec();
            if !sequences.contains_key(&v) {
                sequences.insert(v, next_secrets[i] % 10);
            }
            sequence.pop_front();
        }
        sequences
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_generator() {
        let s = generator(&INPUT);
        assert_eq!(s.numbers[0], 1);
        assert_eq!(s.numbers[1], 10);
        assert_eq!(s.numbers[2], 100);
        assert_eq!(s.numbers[3], 2024);
        assert_eq!(s.numbers.len(), 4);
    }

    #[test]
    fn test_next_secret() {
        let s = generator(&INPUT);
        assert_eq!(s.next_secret(123), 15887950);
        assert_eq!(s.next_secret(15887950), 16495136);
        assert_eq!(s.next_secret(16495136), 527345);

        assert_eq!(s.compute_n_secrets(1, 2000).last().unwrap(), &8685429);
        assert_eq!(s.compute_n_secrets(10, 2000).last().unwrap(), &4700978);
    }

    #[test]
    fn test_part_1() {
        let s = generator(&INPUT);
        assert_eq!(part1(&s), 37327623);
    }

    #[test]
    fn test_sequences() {}

    #[test]
    fn test_part_2() {
        let s = generator("1
2
3
2024");
        assert_eq!(part2(&s), 23);
    }
}
