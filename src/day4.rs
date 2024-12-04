use std::iter::Iterator;

pub fn part1(word_search: &Vec<Vec<char>>) -> usize {
    let directions = vec![
        vec![(1, 0), (2, 0), (3, 0)],         // horizontal
        vec![(0, -1), (0, -2), (0, -3)],      // vertical
        vec![(-1, -1), (-2, -2), (-3, -3)],   // backslash
        vec![(-1, 1), (-2, 2), (-3, 3)]       // forward slash
    ];
    let expected = ['M', 'A', 'S'];
    let mut count = 0;
    for y in 0..word_search.len() {
        for x in 0..word_search[0].len() {
            if word_search[y][x] == 'X' {
                count += count_occurrence(word_search, x as i32, y as i32, &directions, &expected)
            }
        }
    }
    count
}

fn count_occurrence(word_search: &Vec<Vec<char>>, x: i32, y: i32,
                    directions: &[Vec<(i32, i32)>], expected: &[char]) -> usize {
    let mut count = 0;
    for dir in directions {
        for direction in [-1, 1] {
            if dir.iter().zip(expected).all(|((dx, dy), c)| {
                let dx = dx * direction;
                let dy = dy * direction;
                x + dx > -1 && x + dx < word_search[0].len() as i32
                    && y + dy > -1 && y + dy < word_search.len() as i32
                    && word_search[(y + dy) as usize][(x + dx) as usize] == *c
            }) {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(word_search: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let directions = vec![
        vec![(-1, -1), (1, 1)],  // backslash
        vec![(-1, 1), (1, -1)]   // forward slash
    ];
    let expected = ['M', 'S'];
    for y in 0..word_search.len() {
        for x in 0..word_search[0].len() {
            if word_search[y][x] == 'A' {
                if count_occurrence(word_search, x as i32, y as i32, &directions, &expected) == 2 {
                    count += 1
                }
            }
        }
    }
    count
}

pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines()
         .map(|line| line.chars().collect())
         .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_generator() {
        let word_search = generator(&INPUT);
        assert_eq!(word_search.len(), 10);
        assert_eq!(word_search[0].len(), 10);
    }

    #[test]
    fn test_part_1() {
        let word_search = generator(&INPUT);
        assert_eq!(part1(&word_search), 18);
    }

    #[test]
    fn test_part_2() {
        let word_search = generator(&INPUT);
        assert_eq!(part2(&word_search), 9);
    }
}
