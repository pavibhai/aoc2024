use std::iter::Iterator;

pub fn part1(locations: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (l_locs, r_locs) = locations;
    l_locs.iter().zip(r_locs.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

pub fn part2(locations: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (l_locs, r_locs) = locations;

    let mut similarity_score = 0;
    let mut r_idx: usize = 0;
    let mut l_prev: Option<(u32, u32)> = None;
    for l_id in l_locs {
        if l_prev.is_some() && &l_prev.unwrap().0 == l_id {
            similarity_score += l_prev.unwrap().1;
            continue;
        }

        while r_idx < r_locs.len() && l_id > &r_locs[r_idx] {
            r_idx += 1;
        }
        let mut score: u32 = 0;
        while r_idx < r_locs.len() && l_id == &r_locs[r_idx] {
            score += r_locs[r_idx];
            r_idx += 1;
        }
        l_prev = Some((*l_id, score));
        similarity_score += score;
    }
    similarity_score
}

pub fn generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_locations: Vec<u32> = Vec::new();
    let mut right_locations: Vec<u32> = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left_locations.push(l.parse::<u32>().unwrap());
        right_locations.push(r.parse::<u32>().unwrap());
    }
    left_locations.sort();
    right_locations.sort();

    (left_locations, right_locations)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    fn input() -> String {
        ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"].join("\n")
    }

    #[test]
    fn test_generator() {
        let (l, r) = generator(&input());
        assert_eq!(l.len(), 6);
        assert_eq!(r.len(), 6);
        assert_eq!(l, vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(r, vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn test_part_1() {
        let values = generator(&input());
        assert_eq!(part1(&values), 11);
    }

    #[test]
    fn test_part_2() {
        let values = generator(&input());
        assert_eq!(part2(&values), 31);
    }
}
