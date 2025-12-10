use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Read},
};

use itertools::Itertools;
use pathfinding::directed::astar::astar;

struct Line {
    lights: BTreeSet<usize>,
    buttons: Vec<BTreeSet<usize>>,
    joltage: Vec<usize>,
}

fn parse_input(inpt: &str) -> Vec<Line> {
    fn parse_lights(s: &str) -> BTreeSet<usize> {
        s.chars()
            .skip(1)
            .take(s.len() - 2)
            .enumerate()
            .filter(|(_i, c)| *c == '#')
            .map(|(i, _c)| i)
            .collect()
    }
    fn parse_button(s: &str) -> BTreeSet<usize> {
        parse_joltage(s).into_iter().collect()
    }
    fn parse_joltage(s: &str) -> Vec<usize> {
        s[1..s.len() - 1]
            .split(',')
            .map(|sn| sn.parse().unwrap())
            .collect()
    }
    inpt.lines()
        .map(|line| {
            let ws: Vec<_> = line.split_whitespace().collect();
            Line {
                lights: parse_lights(ws[0]),
                buttons: ws[1..ws.len() - 1]
                    .iter()
                    .cloned()
                    .map(parse_button)
                    .collect(),
                joltage: parse_joltage(ws[ws.len() - 1]),
            }
        })
        .collect()
}

fn do_buttons(bs: &[&BTreeSet<usize>]) -> BTreeSet<usize> {
    bs.iter()
        .flat_map(|b| b.iter().cloned())
        .fold(BTreeSet::new(), |mut p, i| {
            if p.contains(&i) {
                p.remove(&i);
            } else {
                p.insert(i);
            }
            p
        })
}

fn day_10_part_1(inpt: &str) -> usize {
    let lines = parse_input(inpt);
    lines
        .into_iter()
        .map(|line| {
            line.buttons
                .iter()
                .powerset()
                .find(|bs| do_buttons(bs) == line.lights)
                .unwrap()
                .len()
        })
        .sum()
}

fn day_10_part_2(inpt: &str) -> usize {
    let lines = parse_input(inpt);
    lines
        .into_iter()
        .map(|line| {
            let target: BTreeMap<usize, usize> = line
                .joltage
                .into_iter()
                .enumerate()
                .filter(|(_k, v)| *v > 0)
                .collect();

            let neighbours = |s: &BTreeMap<usize, usize>| -> Vec<(BTreeMap<usize, usize>, usize)> {
                line.buttons
                    .iter()
                    .map(|b| {
                        (
                            b.iter().cloned().fold(s.clone(), |mut p, c| {
                                p.entry(c).and_modify(|v| *v += 1).or_insert(1);
                                p
                            }),
                            1,
                        )
                    })
                    .collect()
            };

            let heuristic = |s: &BTreeMap<usize, usize>| -> usize {
                target
                    .iter()
                    .map(|(k, v)| (s.get(k).cloned().unwrap_or(0) as isize - *v as isize).abs())
                    .sum::<isize>() as usize
            };

            let success = |s: &BTreeMap<usize, usize>| s == &target;

            let path = astar(&BTreeMap::new(), neighbours, heuristic, success);

            path.expect("Cant solve!?").1
        })
        .inspect(|x| println!("got {}", x))
        .sum()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_10_part_1(&inpt));
    println!("Part 2\t{}", day_10_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::{day_10_part_1, day_10_part_2};

    const STR_INPT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn example() {
        assert_eq!(day_10_part_1(STR_INPT), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(day_10_part_2(STR_INPT), 33);
    }
}
