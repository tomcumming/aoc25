use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<(String, BTreeSet<String>)> {
    inpt.lines()
        .map(|s| match s.split(':').collect::<Vec<_>>().as_slice() {
            [s1, ss] => (
                (*s1).to_owned(),
                ss.split_whitespace().map(|s2| s2.to_owned()).collect(),
            ),
            _ => panic!(),
        })
        .collect()
}

fn day_11_part_1(inpt: &str) -> usize {
    let neighbours: BTreeMap<String, BTreeSet<String>> = parse_input(inpt).into_iter().collect();

    let mut state: BTreeMap<String, usize> = [("you".to_string(), 1)].into_iter().collect();

    loop {
        let mut next_state = BTreeMap::new();

        for (k, c) in state.iter() {
            match neighbours.get(k) {
                None => {
                    next_state
                        .entry(k.clone())
                        .and_modify(|c2| *c2 += *c)
                        .or_insert(*c);
                }
                Some(ns) => {
                    for n in ns {
                        next_state
                            .entry(n.clone())
                            .and_modify(|c2| *c2 += *c)
                            .or_insert(*c);
                    }
                }
            }
        }

        if state == next_state {
            break next_state.get("out").cloned().unwrap_or(0);
        }

        state = next_state;
    }
}

fn _day_11_part_2(_inpt: &str) -> usize {
    todo!()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_11_part_1(&inpt));
    // println!("Part 2\t{}", day_11_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::day_11_part_1;

    const STR_INPT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn example() {
        assert_eq!(day_11_part_1(STR_INPT), 5);
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(day_11_part_2(STR_INPT), _);
    // }
}
