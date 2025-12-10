use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Read},
};

use itertools::Itertools;

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

fn bags_of(
    elems: usize,
    bag_size: usize,
) -> Box<dyn Iterator<Item = std::collections::BTreeMap<usize, usize>>> {
    if bag_size == 0 {
        return Box::new(std::iter::once(BTreeMap::new()));
    }
    Box::new(bags_of(elems, bag_size - 1).flat_map(move |bag| {
        let max_elem = bag.first_key_value().map(|(k, _)| *k).unwrap_or(elems - 1);

        (0..=max_elem).map(move |i| {
            let mut bag_ = bag.clone();
            bag_.entry(i).and_modify(|c| *c += 1).or_insert(1);
            bag_
        })
    }))
}

fn do_joltage(bs: &[BTreeSet<usize>], js: &BTreeMap<usize, usize>) -> BTreeMap<usize, usize> {
    js.iter()
        .flat_map(|(k, v)| bs.get(*k).unwrap().iter().map(|i| (*i, *v)))
        .fold(BTreeMap::new(), |mut p, (k, v)| {
            p.entry(k).and_modify(|v_| *v_ += v).or_insert(v);
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

            (0..)
                .find(|n| {
                    bags_of(line.buttons.len(), *n).any(|presses| {
                        let total = do_joltage(&line.buttons, &presses);
                        total == target
                    })
                })
                .unwrap()
        })
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
