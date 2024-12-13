const A_TOKENS: i64 = 3;
const B_TOKENS: i64 = 1;

pub fn part1(machines: &Vec<Machine>) -> i64 {
    solve(machines, false)
}

pub fn part2(machines: &Vec<Machine>) -> i64 {
    solve(machines, true)
}

fn solve(machines: &Vec<Machine>, correct_error: bool) -> i64 {
    let mut a = 0;
    let mut b = 0;
    machines.iter().for_each(|m| {
        let (a_times, b_times) = m.solve(correct_error);
        a += a_times;
        b += b_times;
    });
    (a * A_TOKENS) + (b * B_TOKENS)
}

pub fn generator(input: &str) -> Vec<Machine> {
    input.split("\n\n")
        .map(|chunk| Machine::new(chunk))
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct XY {
    x: i64,
    y: i64,
}

impl XY {
    fn new(input: &str, sep: &str) -> XY {
        let xy = input.split_once(", ").unwrap();
        let x = xy.0.split_once(sep).unwrap().1.parse::<i64>().unwrap();
        let y = xy.1.split_once(sep).unwrap().1.parse::<i64>().unwrap();

        Self {
            x,
            y,
        }
    }
}

pub struct Machine {
    a: XY,
    b: XY,
    prize: XY,
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let a = XY::new(lines.next().unwrap().split_once(": ").unwrap().1, "+");
        let b = XY::new(lines.next().unwrap().split_once(": ").unwrap().1, "+");
        let prize = XY::new(lines.next().unwrap().split_once(": ").unwrap().1, "=");

        Machine {
            a,
            b,
            prize,
        }
    }

    fn solve(&self, correct_error: bool) -> (i64, i64) {
        let px = if correct_error { 10000000000000 + self.prize.x } else { self.prize.x };
        let py = if correct_error { 10000000000000 + self.prize.y } else { self.prize.y };
        let n = (px * self.b.y) - (self.b.x * py);
        let d = (self.a.x * self.b.y) - (self.b.x * self.a.y);
        if n % d != 0 { return (0, 0); }
        let mut a_times = n / d;
        let mut b_times = (px - (self.a.x * a_times)) / self.b.x;
        if self.a.x > self.b.x && self.a.y > self.b.y
            && self.a.x % self.b.x == 0 && self.a.y % self.b.y == 0 {
            match (self.a.x / self.b.x, self.a.y / self.b.y) {
                (a, b) if a == b && a > 3 => {
                    a_times += b_times / a;
                    b_times += b_times % a;
                }
                _ => {}
            }
        }
        (a_times, b_times)
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, XY};

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_generator() {
        let machines = generator(&INPUT);
        assert_eq!(4, machines.len());
        assert_eq!(XY { x: 94, y: 34 }, machines[0].a);
        assert_eq!(XY { x: 22, y: 67 }, machines[0].b);
        assert_eq!(XY { x: 8400, y: 5400 }, machines[0].prize);
    }

    #[test]
    fn test_part_1() {
        let machine = generator(&INPUT);
        assert_eq!(part1(&machine), 480);
    }

    #[test]
    fn test_part_2() {
        let machine = generator(&INPUT);
        assert_eq!(part2(&machine), 875318608908);
    }
}
