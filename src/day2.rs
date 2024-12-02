use std::iter::Iterator;

pub fn part1(reports: &Vec<Vec<i32>>) -> u32 {
    reports
        .iter()
        .map(|r| is_safe(r, false))
        .sum()
}

fn is_safe(r: &Vec<i32>, dampener: bool) -> u32 {
    let mut result = is_safe_fwd(r, dampener);
    if result == 0 {
        let mut r = r.clone();
        r.reverse();
        result = is_safe_fwd(&r, dampener);
    }
    result
}

fn is_safe_fwd(r: &Vec<i32>, dampener: bool) -> u32 {
    let changes: Vec<i32> = r.windows(2).map(|w| w[1] - w[0]).collect();
    let increases_forward: Vec<&i32> = changes.iter().take_while(|c| *c > &0 && *c < &4).collect();
    let increases_backward: Vec<&i32> = changes
        .iter()
        .rev()
        .take_while(|c| *c > &0 && *c < &4)
        .collect();

    if increases_forward.len() == changes.len() || increases_backward.len() == changes.len() {
        return 1;
    } else if !dampener {
        return 0;
    }

    if increases_forward.len() + increases_backward.len() + 1 == changes.len()
        && (increases_forward.is_empty() || increases_backward.is_empty())
    {
        return 1;
    }

    if changes.len() - increases_forward.len() - increases_backward.len() > 2 {
        return 0;
    }

    match changes.len() - increases_forward.len() - increases_backward.len() {
        1 => {
            let f_diff = changes[increases_forward.len()] + *increases_forward.last().unwrap();
            let b_diff = changes[increases_forward.len()] + *increases_backward.last().unwrap();
            if (f_diff > 0 && f_diff < 4) || (b_diff > 0 && b_diff < 4) {
                1
            } else {
                0
            }
        }
        2 => {
            let diff = changes[increases_forward.len()] + changes[increases_forward.len() + 1];
            if diff > 0 && diff < 4 {
                1
            } else {
                0
            }
        }
        _ => 0
    }
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
    use crate::day2::{generator, is_safe, part1, part2};

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
