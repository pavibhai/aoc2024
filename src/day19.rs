use std::collections::{HashMap};
use crate::day19::Stripe::{BLACK, BLUE, GREEN, RED, WHITE};

pub fn part1(p: &Puzzle) -> u64 {
    p.designs.iter().filter(|&d| p.count_possible(d, false) > 0).count() as u64
}

pub fn part2(p: &Puzzle) -> u64 {
    p.designs.iter().map(|d| p.count_possible(d, true)).sum()
}

pub fn generator(input: &str) -> Puzzle {
    Puzzle::new(input)
}

#[derive(Debug, Default)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new(towels: &str) -> Self {
        let mut tree: Tree = Tree::default();
        tree.nodes.push(Node::default());
        for towel in towels.split(", ") {
            let stripes: Vec<char> = towel.chars().map(|c| c).collect();
            tree.build_into_tree(0, &stripes);
        }
        tree
    }

    fn build_into_tree(&mut self, idx: usize, stripes: &[char]) {
        if stripes.is_empty() {
            self.nodes[idx].is_towel = true;
            return;
        }

        let stripe = Stripe::new(&stripes[0]);
        let idx = match self.nodes[idx].children.get(&stripe) {
            Some(idx) => *idx,
            None => {
                let new_idx = self.nodes.len();
                self.nodes.push(Node::default());
                self.nodes[idx].children.insert(stripe, new_idx);
                new_idx
            }
        };
        self.build_into_tree(idx, &stripes[1..])
    }

    fn count_valid(&self, design: &[Stripe], node_idx: usize, cache: &mut [Option<u64>], all: bool) -> u64 {
        if design.is_empty() {
            if self.nodes[node_idx].is_towel {
                return 1;
            }
            return 0;
        }

        if node_idx == 0 {
            return match cache[design.len() - 1] {
                Some(c) => {
                    c
                }
                None => {
                    cache[design.len() - 1] = Some(self.compute_valid(design, node_idx, cache, all));
                    cache[design.len() - 1].unwrap()
                }
            }
        }

        self.compute_valid(design, node_idx, cache, all)
    }

    fn compute_valid(&self, design: &[Stripe], node_idx: usize, cache: &mut [Option<u64>], all: bool) -> u64 {
        let mut count = match self.nodes[node_idx].children.get(&design[0]) {
            Some(c_idx) => {
                self.count_valid(&design[1..], *c_idx, cache, all)
            }
            None => 0,
        };

        if all || count == 0 {
            if self.nodes[node_idx].is_towel {
                count += self.count_valid(design, 0, cache, all);
            }
        }
        count
    }
}

#[derive(Debug, Default)]
struct Node {
    is_towel: bool,
    children: HashMap<Stripe, usize>,
}

impl Node {}

pub struct Puzzle {
    towels_as_tree: Tree,
    designs: Vec<Vec<Stripe>>,
}

impl Puzzle {
    pub fn new(input: &str) -> Puzzle {
        let (towels, designs) = input.split_once("\n\n").unwrap();
        let towels_as_tree = Tree::new(towels.trim());

        let designs = designs.trim().lines()
            .map(|d| d.chars().map(|c| Stripe::new(&c)).collect())
            .collect();

        Puzzle {
            towels_as_tree,
            designs,
        }
    }

    fn count_possible(&self, design: &[Stripe], all: bool) -> u64 {
        self.towels_as_tree.count_valid(design, 0, &mut vec![None; design.len()], all)
    }
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Hash)]
enum Stripe {
    WHITE,
    BLUE,
    BLACK,
    RED,
    GREEN,
}

impl Stripe {
    fn new(input: &char) -> Self {
        match input {
            'w' => WHITE,
            'u' => BLUE,
            'b' => BLACK,
            'r' => RED,
            'g' => GREEN,
            _ => unreachable!("Found unknown stripe: {}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::Stripe::{BLACK, BLUE, GREEN, RED, WHITE};
    use super::{generator, part1, part2, Stripe};

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_generator() {
        let p = generator(&INPUT);
        assert_eq!(p.designs.len(), 8);
    }

    #[test]
    fn test_part_1() {
        let p = generator(&INPUT);
        assert_eq!(part1(&p), 6);
        assert_eq!(p.count_possible(&[RED], false), 1);
        assert_eq!(p.count_possible(&[RED, BLACK], false), 1);
        assert_eq!(p.count_possible(&[BLACK, RED, WHITE, RED, RED], false), 1);
        assert_eq!(p.count_possible(&[BLUE, BLACK, WHITE, BLUE], false), 0);
        assert_eq!(p.count_possible(&[BLACK, WHITE, RED, GREEN, BLACK], false), 1);
    }

    #[test]
    fn test_big() {
        let p = generator("rrgbgg, gbgbgr, rrb, wrgggbb, rr, bgb, wbb, ruugr, gwugg, ruu, gubw, gru, bgg, uwu, bggwbrgw, wwugbr, ur, urb, bbb, rrbrbw, uww, wggwwu, wwrb, gbg, wuruu, wbgbwbr, ggu, grgwru, g, uugbuu, rwrrwb, uurwub, grgr, wubb, buwu, guu, bbw, rgwgbg, grr, uuuwr, ggubrrg, uuub, gugu, bur, bguub, rgr, bwu, bwbuw, wwwwbw, uurbbgb, rwb, uuw, ggurug, wrr, gww, rgbgu, wgr, rw, uguu, brb, ugrr, brwurgu, rugg, gbuwr, gb, uggbrbr, urwrbgb, rbgwbwb, gg, rur, uuu, wr, grur, uwugb, gbrugru, buug, bbrw, uurugru, u, bb, ubggb, brrb, rrg, bgur, gubub, gwwu, rugurug, ggbgbgbb, ggbwgr, gurw, gbgww, ugbg, gugb, grg, rggwg, buwb, wbu, guwu, rggbgugw, ggb, ruub, rugw, ugb, wrw, www, gwwbr, bru, bgurrbru, rrug, wubgug, brguw, wbugr, ggur, rgwuurrw, wurrg, rrgu, wwrwrb, brg, wwg, ugugu, ugr, rbgr, rg, uggg, rgrg, wwb, rrgrrwg, bgwb, brgbwgu, rurg, brw, brww, brggu, rgwwbbgg, uwgb, rwgggwwb, wrgrbw, bwrgw, wbbw, rbub, gbguuuub, bwrwg, uurruu, rugu, urrwguw, bwru, bu, wuub, ugggr, uuug, gu, uugggb, wbbb, rrr, b, rgwgr, gbr, gwrwwrw, ugu, ggw, wgub, wuw, uubr, ggrgbg, uwbbu, ubwurb, rbb, uur, uwrruug, wrggur, bwrr, wugb, wuwr, wwgb, wgubgg, wg, rgwbgwbw, rggugug, uub, uggug, wgu, brgrrub, rwwrgbr, gbgrgr, grgubbu, ugbbr, urw, ubuubb, burgw, ubu, gwgub, wwwubrgu, wwwww, bgr, bgu, gwwruuw, wgrgw, uwuu, wrb, ubw, rgrwu, brrrrr, wrgbb, gwuuwbg, wwbgu, uugbwrww, brwbg, ub, rbru, ruggwr, gwb, ggwrgg, ggurur, rgrrwrbw, wrgw, bugg, gbu, rgu, rwuw, gwg, uuwwu, uwbwww, wuwwbuu, wb, bbrr, rgrgwru, burr, ugurubu, gwu, buurb, wrgg, bugbrw, burbuw, bwr, urr, grgu, ruw, ugbwgw, rbrug, uuugr, buubwr, rurwg, ugrgw, grru, uwwwbur, uuwu, gwrwr, uruuu, urwgugbb, wrurwur, rwr, rgb, brbub, gwr, gwbb, brrbwb, bwwgw, bguw, ug, rru, wrg, uurrg, rww, bbu, wgbb, grw, ubg, uwg, ww, gbwr, rrwugr, wrrgwb, rrw, wbuuuub, wrurggr, bwrb, bbrbrgg, ggggbrw, ugug, wur, wwbr, gubbrwur, wgwgwb, wbbrr, wgw, rbwwbr, w, bbgw, urg, wbgbg, wbr, bbwbw, uu, wgwguw, wu, bgwgu, wub, buu, rb, bgbw, bgwgb, uwgrbu, guw, rbbgu, wru, rubgggwu, gur, rbrgw, gwug, wguwrr, gubbr, rub, rbwu, rgg, bggug, bwugb, rbbbgr, bbbwg, guwrrb, grrggg, bgwu, uuwbru, bbg, rug, uugu, gwugwu, ubb, gub, uubu, bwwrru, guub, wwr, urbgw, uru, wwu, wwwurwwu, bbwg, ubbbwg, gbug, bwbr, wgggwg, urgwub, bub, bg, uwrg, uwr, ubrrb, bug, rggru, rbw, ggrw, bbwwg, gr, gubu, uubbg, bwg, urggw, uuwrrgw, ugg, bbrg, gug, bwggww, grbb, bbbu, rbu, ru, ubuggr, ruuu, uug, uwb, wwwbug, uggwr, gbwuw, grwr, uwuwgbub, wwbw, bww, rbbwwr, bwww, ggggu, ugw, bwwwu, uuwug, wbbggrg, bwb, grbrgwu, bwwu, bw, ruurr, wbg, ubrg, uwuw, brrg, brwuu, gugw, urbggb, bruw, wbwuggw, ggwwr, rbwr, wbrgr, uugg, bbru, ugwuguwu, rwrg, bgwwwrw, uugwg, bwurrr, bwrwr, wwwrggu, bbr, rruwu, rgw, buw, urbrbg, wgg, gbgb, rwu, gubbu, gbgbgbrr, wbw, ubr, wgb, gw, buuwbr, rugrrur, gbuuwwr, rburubg, gbb, ugrgr, wug, ggggb, ggru, brwwg, bwwgrb, gurwubww, ggg, bwgb, bgugbw, rrrww, ggr, gbbb, bgw, bugu, wuu, grgur, rwg

rugbgbwwbbgrwrbubgugrgbrrbgwrbbgbwurwgrbr
uruuurbrwuwrrrwwurwbrwwguruwgrbgwbbwrugwwgrbr");
        let input: [Stripe; 41] = [RED, BLUE, GREEN, BLACK, GREEN, BLACK, WHITE, WHITE, BLACK, BLACK, GREEN, RED, WHITE, RED, BLACK, BLUE, BLACK, GREEN, BLUE, GREEN, RED, GREEN, BLACK, RED, RED, BLACK, GREEN, WHITE, RED, BLACK, BLACK, GREEN, BLACK, WHITE, BLUE, RED, WHITE, GREEN, RED, BLACK, RED];
        assert_eq!(p.count_possible(&input, false), 0);
    }

    #[test]
    fn test_part_2() {
        let p = generator(&INPUT);
        assert_eq!(16, part2(&p));
    }
}
