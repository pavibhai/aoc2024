use std::iter::Iterator;

const INCREASES: [i32; 3] = [1, 2, 3];
const DECREASES: [i32; 3] = [-1, -2, -3];

pub fn part1(reports: &Vec<Vec<i32>>) -> u32 {
    reports
        .iter()
        .map(|r| is_safe(r, false))
        .sum()
}

fn is_safe(r: &Vec<i32>, dampener: bool) -> u32 {
    let changes: Vec<i32> = r.windows(2).map(|w| w[1] - w[0]).collect();
    for allowed in [DECREASES, INCREASES] {
        let forward = changes.iter().take_while(|c| allowed.contains(c)).count();
        let backward = changes
            .iter()
            .rev()
            .take_while(|c| allowed.contains(c))
            .count();

        if forward == changes.len() || backward == changes.len() {
            return 1;
        } else if !dampener {
            continue;
        }

        match changes.len() - forward - backward {
            1 if forward == 0 || backward == 0 => return 1,
            1 => {
                let f_diff = changes[forward] + changes[forward - 1];
                let b_diff = changes[forward] + changes[forward + 1];
                if allowed.contains(&f_diff) || allowed.contains(&b_diff) {
                    return 1;
                } else {
                    continue;
                }
            }
            2 => {
                let diff = changes[forward] + changes[forward + 1];
                if allowed.contains(&diff) {
                    return 1;
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }
    0
}

pub fn part2(reports: &Vec<Vec<i32>>) -> u32 {
    reports.iter().map(|r| is_safe(r, true)).sum()
}

pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, is_safe, part1, part2};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_generator() {
        let reports = generator(&INPUT);
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(reports[1], vec![1, 2, 7, 8, 9]);
    }

    #[test]
    fn test_part_1() {
        let reports = generator(&INPUT);
        assert_eq!(part1(&reports), 2);
    }

    #[test]
    fn test_part_2() {
        let reports = generator(&INPUT);
        assert_eq!(part2(&reports), 4);
    }

    #[test]
    fn test_is_safe() {
        let inputs = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![9, 2, 3, 4, 5, 6],
            vec![1, 3, 2, 4, 5, 6],
            vec![7, 6, 4, 2, 1],
            vec![1, 6, 2, 4, 6],
            vec![10, 7, 8, 5, 4, 3],
            vec![1, 2, 3, 7, 6, 7, 9],
            vec![1, 5, 6, 7, 8, 9],
            vec![10, 10, 13, 14, 17],
            vec![2, 3, 10, 5, 6, 7, 8, 9],
            vec![4, 2, 7, 8, 9],
            vec![1, 4, 2, 3, 5],
        ];

        for i in inputs {
            assert_eq!(is_safe(&i, true), 1);
            let mut i = i.clone();
            i.reverse();
            assert_eq!(is_safe(&i, true), 1);
        }
    }

    #[test]
    fn test_is_unsafe() {
        let inputs = vec![vec![2, 9, 1, 3, 4, 5, 6]];

        for i in inputs {
            assert_eq!(is_safe(&i, true), 0);
            let mut i = i.clone();
            i.reverse();
            assert_eq!(is_safe(&i, true), 0);
        }
    }
}
