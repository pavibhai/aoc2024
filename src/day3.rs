use std::iter::Iterator;

pub fn part1(instructions: &str) -> u32 {
    instructions.split("mul(")
        .filter_map(|x| {
            x.split_once(",")
                .map(|(l, r)| {
                    let r = r.split(')').next().unwrap();
                    if l.is_empty() || l.len() > 3 || r.is_empty() || r.len() > 4 {
                        None
                    } else {
                        let l: Result<u32, _> = l.parse();
                        let r: Result<u32, _> = r.parse();
                        match (l, r) {
                            (Ok(l), Ok(r)) => Some(l * r),
                            _ => None
                        }
                    }
                }).unwrap_or(None)
        })
        .sum()
}

pub fn part2(instructions: &str) -> u32 {
    instructions.split("do()")
        .map(|d| d.split("don't()").next().unwrap())
        .map(|s| part1(s))
        .sum()
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_generator() {
        let s = generator(&INPUT);
        assert_eq!(&s, INPUT);
    }

    #[test]
    fn test_part_1() {
        let instructions = generator(&INPUT);
        assert_eq!(part1(&instructions), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}
