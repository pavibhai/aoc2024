use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

pub fn part1(map: &CityMap) -> u32 {
    map.find_antinodes::<false>().len() as u32
}

pub fn part2(map: &CityMap) -> u32 {
    map.find_antinodes::<true>().len() as u32
}

pub fn generator(input: &str) -> CityMap {
    CityMap::new(input)
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_valid(&self, height: &i32, width: &i32) -> bool {
        &self.x > &-1 && &self.x < width && &self.y > &-1 && &self.y < height
    }

    fn adjust(&mut self, dx: &i32, dy: &i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub struct CityMap {
    height: i32,
    width: i32,
    nodes: HashMap<char, Vec<XY>>,
}

impl CityMap {
    fn new(input: &str) -> Self {
        let mut height = 0i32;
        let mut width = 0i32;
        let mut nodes = HashMap::new();
        input.lines().enumerate().for_each(|(y, line)| {
            height += 1;
            width = line.len() as i32;

            line.chars().enumerate().filter(|(_, c)| c != &'.').for_each(|(x, c)| {
                nodes.entry(c)
                    .and_modify(|p: &mut Vec<XY>| p.push(XY::new(x as i32, y as i32)))
                    .or_insert(vec![XY::new(x as i32, y as i32)]);
            })
        });

        CityMap {
            height,
            width,
            nodes,
        }
    }

    fn find_antinodes<const RESONANT_HARMONICS:bool>(&self) -> HashSet<XY> {
        let mut antinodes = HashSet::new();

        self.nodes.iter().for_each(|(_, p)| {
            for i in 0..p.iter().len() - 1 {
                for j in i + 1..p.len() {
                    if RESONANT_HARMONICS {
                        self.collect_anti_nodes_w_harmonics(&p[i], &p[j], &mut antinodes);
                    } else {
                        self.collect_anti_nodes(&p[i], &p[j], &mut antinodes);
                    }
                }
            }
        });
        antinodes
    }

    fn collect_anti_nodes_w_harmonics(&self, p1: &XY, p2: &XY, antinodes: &mut HashSet<XY>) {
        antinodes.insert(p1.clone());
        antinodes.insert(p2.clone());

        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        for (sgn, p) in [(1, p1), (-1, p2)] {
            let mut xy = XY::new(p.x + (sgn * dx), p.y + (sgn * dy));
            while xy.is_valid(&self.height, &self.width) {
                antinodes.insert(xy);
                xy.adjust(&(sgn * dx), &(sgn * dy));
            }
        }
    }

    fn collect_anti_nodes(&self, p1: &XY, p2: &XY, antinodes: &mut HashSet<XY>) {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;

        for xy in [XY::new(p1.x + dx, p1.y + dy), XY::new(p2.x - dx, p2.y - dy)] {
            if xy.is_valid(&self.height, &self.width) {
                antinodes.insert(xy);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, XY};

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_generator() {
        let map = generator(&INPUT);
        assert_eq!(map.width, 12);
        assert_eq!(map.height, 12);
        assert_eq!(map.nodes.len(), 2);
        assert_eq!(map.nodes.get(&'A').unwrap(),
                   &vec![
                       XY::new(6, 5),
                       XY::new(8, 8),
                       XY::new(9, 9),
                   ]);
    }

    #[test]
    fn test_part_1() {
        let map = generator(&INPUT);
        assert_eq!(part1(&map), 14);
    }

    #[test]
    fn test_part_2() {
        let map = generator(&INPUT);
        assert_eq!(part2(&map), 34);
    }
}
