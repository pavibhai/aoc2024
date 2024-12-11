use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(stones: &Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    stones.iter().map(|s| blink_dfs(*s, 25, &mut cache)).sum()
}

pub fn part2(stones: &Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    stones.iter().map(|s| blink_dfs(*s, 75, &mut cache)).sum()
}

pub fn generator(input: &str) -> Vec<u64> {
    input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink_dfs(n: u64, times: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if times == 0 {
        1
    } else {
        match n {
            0 => blink_dfs(1, times - 1, cache),
            n => {
                match digits(n) {
                    d if d % 2 == 0 => {
                        let divisor = 10u64.pow(d / 2);
                        from_cache_or_compute(n / divisor, times - 1, cache)
                            + from_cache_or_compute(n % divisor, times - 1, cache)
                    }
                    _ => from_cache_or_compute(n * 2024, times - 1, cache)
                }
            }
        }
    }
}

fn from_cache_or_compute(n: u64, times: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    match cache.get(&(n, times)) {
        Some(n) => *n,
        None => {
            let r = blink_dfs(n, times, cache);
            cache.insert((n, times), r);
            r
        }
    }
}

fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}

pub fn blink(stones: &mut VecDeque<u64>, seen: &mut HashSet<u64>) {
    let n = stones.len();
    for _ in 0..n {
        let s = stones.pop_front().unwrap();
        seen.insert(s);
        match s {
            0 => {
                stones.push_back(1)
            }
            _ => {
                match digits(s) {
                    n if n % 2 == 0 => {
                        let divisor = 10u64.pow(n / 2);
                        stones.push_back(s / divisor);
                        stones.push_back(s % divisor);
                    }
                    _ => stones.push_back(s * 2024)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "125 17";

    #[test]
    fn test_generator() {
        let stones = generator(&INPUT);
        assert_eq!(2, stones.len());
    }

    #[test]
    fn test_part_1() {
        let stones = generator(&INPUT);
        assert_eq!(55312, part1(&stones));
    }

    #[test]
    fn test_part_2() {
        let stones = generator(&INPUT);
        assert_eq!(65601038650482, part2(&stones));
    }
}
