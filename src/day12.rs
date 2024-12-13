use crate::day12::Direction::{EAST, NORTH, SOUTH, WEST};
use std::collections::{HashMap, HashSet};

pub fn part1(garden: &Garden) -> u32 {
    garden.fence_price(false)
}

pub fn part2(garden: &Garden) -> u32 {
    garden.fence_price(true)
}

pub fn generator(input: &str) -> Garden {
    let arrangement: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions = [NORTH, SOUTH, WEST, EAST];
    let mut region_lots = compute_regions(&arrangement, &directions);
    let mut regions = Vec::new();
    while !region_lots.is_empty() {
        let plots = region_lots.pop().unwrap();
        let edges = compute_region_edges(&plots);
        regions.push(Region { plots, edges });
    }

    Garden {
        regions
    }
}

fn compute_region_edges(region: &HashSet<XY>) -> HashSet<Edge> {
    let mut edges: HashMap<Edge, u32> = HashMap::new();
    region.iter().for_each(|r| {
        [NORTH, SOUTH, WEST, EAST].iter().for_each(|d| {
            let edge = Edge::new(&r.x, &r.y, d);
            edges.entry(edge)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        })
    });
    edges.retain(|_, v| v == &1);
    edges.into_keys().collect()
}

fn compute_regions(arrangement: &Vec<Vec<char>>, directions: &[Direction]) -> Vec<HashSet<XY>> {
    let mut regions = Vec::new();
    let mut allocated = vec![vec![false; arrangement[0].len()]; arrangement.len()];
    let mut stack = Vec::new();
    let width: i32 = arrangement[0].len() as i32;
    let height: i32 = arrangement.len() as i32;

    arrangement.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            if allocated[y][x] {
                return;
            }
            stack.clear();
            stack.push(XY { x: x as i32, y: y as i32 });
            let plant = arrangement[y][x];
            let mut plots = HashSet::new();
            while !stack.is_empty() {
                let p = stack.pop().unwrap();
                if allocated[p.y as usize][p.x as usize] {
                    continue;
                }
                for d in directions {
                    let n = d.adjust(&p);
                    if n.x > -1 && n.x < width
                        && n.y > -1 && n.y < height
                        && &arrangement[n.y as usize][n.x as usize] == &plant {
                        stack.push(n);
                    }
                }
                allocated[p.y as usize][p.x as usize] = true;
                plots.insert(p);
            }
            regions.push(plots);
        })
    });
    regions
}

pub struct Region {
    plots: HashSet<XY>,
    edges: HashSet<Edge>,
}

impl Region {
    fn fence_price(&self, bulk_discount: bool) -> u32 {
        if bulk_discount {
            (self.plots.len() * self.compute_sides()) as u32
        } else {
            (self.plots.len() * self.edges.len()) as u32
        }
    }

    fn compute_sides(&self) -> usize {
        self.edges.iter().filter(|e| {
            if e.x1 == e.x2 {
                let p = Edge { x1: e.x1 - 1, x2: e.x1 - 1, y1: e.y1, y2: e.y2 };
                // check that x-1, y1 and x,y1 are in region or x-1, y2 and x, y2 are in region
                !self.edges.contains(&p)
                    || self.plots.contains(&XY { x: p.x1, y: e.y1 }) != self.plots.contains(&XY { x: e.x1, y: e.y1 })
                    || self.plots.contains(&XY { x: p.x1, y: e.y2 }) != self.plots.contains(&XY { x: e.x1, y: e.y2 })
            } else {
                let p = Edge { x1: e.x1, x2: e.x2, y1: e.y1 - 1, y2: e.y1 - 1 };
                // check that x1, y-1 and x2, y-1 are in region or x1, y and x2, y
                !self.edges.contains(&p)
                    || self.plots.contains(&XY { x: e.x1, y: p.y1 }) != self.plots.contains(&XY { x: e.x1, y: e.y1 })
                    || self.plots.contains(&XY { x: e.x1, y: p.y1 }) != self.plots.contains(&XY { x: e.x1, y: e.y2 })
            }
        }).count()
    }
}

pub struct Garden {
    regions: Vec<Region>,
}

impl Garden {
    fn fence_price(&self, bulk_discount: bool) -> u32 {
        self.regions.iter().map(|r| {
            r.fence_price(bulk_discount)
        }).sum()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn adjust(&self, xy: &XY) -> XY {
        match self {
            NORTH => XY { x: xy.x, y: xy.y - 1 },
            SOUTH => XY { x: xy.x, y: xy.y + 1 },
            EAST => XY { x: xy.x + 1, y: xy.y },
            WEST => XY { x: xy.x - 1, y: xy.y },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Edge {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Edge {
    fn new(x: &i32, y: &i32, d: &Direction) -> Self {
        match d {
            NORTH => Self { x1: *x, x2: *x, y1: y - 1, y2: *y },
            SOUTH => Self { x1: *x, x2: *x, y1: *y, y2: y + 1 },
            EAST => Self { x1: *x, x2: x + 1, y1: *y, y2: *y },
            WEST => Self { x1: x - 1, x2: *x, y1: *y, y2: *y },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_generator() {
        let g = generator(&INPUT);
        assert_eq!(g.regions.len(), 5);
    }

    #[test]
    fn test_part_1() {
        let g = generator(&INPUT);
        assert_eq!(part1(&g), 140);

        let g = generator("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO");
        assert_eq!(part1(&g), 772);

        let g = generator("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE");
        assert_eq!(part1(&g), 1930);
    }

    #[test]
    fn test_part_2() {
        let g = generator(&INPUT);
        assert_eq!(part2(&g), 80);

        let g = generator("EBBA
EBBA
ECCA
CCDA");
        assert_eq!(part2(&g), 80);

        let g = generator("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO");
        assert_eq!(part2(&g), 436);

        let g = generator("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE");
        assert_eq!(part2(&g), 236);

        let g = generator("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE");
        assert_eq!(part2(&g), 236);

        let g = generator("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE");
        assert_eq!(part2(&g), 1206);
    }
}
