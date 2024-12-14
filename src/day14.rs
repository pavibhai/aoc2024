const SAFETY_FACTOR_TIME: u32 = 100;
const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;
const SEARCH_MAX_TIME: u32 = 10000;

pub fn part1(ebhq: &EBHQ) -> u32 {
    let mut ebhq = ebhq.clone();
    ebhq.safety_factor(SAFETY_FACTOR_TIME)
}

pub fn part2(ebhq: &EBHQ) -> u32 {
    let mut ebhq = ebhq.clone();

    for i in 1..=SEARCH_MAX_TIME {
        ebhq.elapse_time(1);
        if ebhq.maybe_like_christmas_tree() {
            return i;
        }
    }
    panic!("No solution found!");
}

pub fn generator(input: &str) -> EBHQ {
    EBHQ::new(input, HEIGHT, WIDTH)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(input: &str) -> Self {
        let (x, y) = input.split_once(",").unwrap();
        XY {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: XY,
    velocity: XY,
}

impl Robot {
    fn new(input: &str) -> Self {
        let (p, v) = input.split_once(" ").unwrap();
        Robot {
            pos: XY::new(p.split_once("=").unwrap().1),
            velocity: XY::new(v.split_once("=").unwrap().1),
        }
    }

    fn navigate(&mut self, time: u32, height: &i32, width: &i32) {
        self.pos.x += self.velocity.x * time as i32;
        self.pos.y += self.velocity.y * time as i32;
        self.pos.x = self.pos.x.rem_euclid(*width);
        self.pos.y = self.pos.y.rem_euclid(*height);
    }

    fn quadrant(&self, mid_height: &i32, mid_width: &i32) -> Option<usize> {
        if &self.pos.x == mid_width || &self.pos.y == mid_height {
            None
        } else {
            match (&self.pos.x < mid_width, &self.pos.y < mid_height) {
                (true, true) => Some(0),
                (true, false) => Some(1),
                (false, true) => Some(2),
                (false, false) => Some(3),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EBHQ {
    height: i32,
    width: i32,
    robots: Vec<Robot>,
    map: Vec<Vec<u32>>,
}

impl EBHQ {
    fn new(input: &str, height: i32, width: i32) -> Self {
        if height % 2 == 0 || width % 2 == 0 {
            panic!("Invalid height/width: {height}/{width}, should not be even!!!");
        }

        let mut map = vec![vec![0; width as usize]; height as usize];
        let robots: Vec<Robot> = input.lines().map(|line| Robot::new(line)).collect();
        robots.iter().for_each(|robot| {
            map[robot.pos.y as usize][robot.pos.x as usize] += 1;
        });

        EBHQ {
            height,
            width,
            robots,
            map,
        }
    }


    fn safety_factor(&mut self, time: u32) -> u32 {
        self.elapse_time(time);

        let mid_height = self.height / 2;
        let mid_width = self.width / 2;

        let mut quadrants = [0, 0, 0, 0];
        self.robots.iter().for_each(|r| {
            match r.quadrant(&mid_height, &mid_width) {
                Some(q) => quadrants[q] += 1,
                None => (),
            }
        });
        quadrants.iter().product()
    }

    fn elapse_time(&mut self, time: u32) {
        self.robots.iter_mut().for_each(|r| {
            self.map[r.pos.y as usize][r.pos.x as usize] -= 1;
            r.navigate(time, &self.height, &self.width);
            self.map[r.pos.y as usize][r.pos.x as usize] += 1;
        })
    }

    fn maybe_like_christmas_tree(&self) -> bool {
        (0..self.height - 3).any(|y| {
            (2..self.width - 2).any(|x| {
                (0..3).all(|dy| {
                    (-dy..=dy).all(|dx| {
                        self.map[(y + dy) as usize][(x + dx) as usize] > 0
                    })
                })
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, Robot, EBHQ, XY};
    const HEIGHT: i32 = 7;
    const WIDTH: i32 = 11;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_generator() {
        let ebhq = EBHQ::new(&INPUT, HEIGHT, WIDTH);
        assert_eq!(ebhq.robots.len(), 12);
        assert_eq!(ebhq.robots.first().unwrap().pos, XY { x: 0, y: 4 });
        assert_eq!(ebhq.robots.first().unwrap().velocity, XY { x: 3, y: -3 });
        assert_eq!(ebhq.robots.last().unwrap().pos, XY { x: 9, y: 5 });
        assert_eq!(ebhq.robots.last().unwrap().velocity, XY { x: -3, y: -3 });
    }

    #[test]
    fn test_robot() {
        let r = Robot::new("p=2,4 v=2,-3");
        assert_eq!(r.pos, XY { x: 2, y: 4 });
        assert_eq!(r.velocity, XY { x: 2, y: -3 });

        let mut r = Robot::new("p=2,4 v=2,-3");
        r.navigate(5, &HEIGHT, &WIDTH);
        assert_eq!(r.pos, XY { x: 1, y: 3 });
    }

    #[test]
    fn test_part_1() {
        let ebhq = EBHQ::new(INPUT, HEIGHT, WIDTH);
        assert_eq!(part1(&ebhq), 12);
    }

    #[test]
    #[should_panic]
    fn test_part_2() {
        let ebhq = EBHQ::new(&INPUT, HEIGHT, WIDTH);
        part2(&ebhq);
    }
}
