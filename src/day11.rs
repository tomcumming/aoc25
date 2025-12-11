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

fn lookup(
    index: &BTreeMap<String, BTreeSet<String>>,
    memo: &mut BTreeMap<String, BTreeMap<u8, usize>>,
    node: &str,
) -> BTreeMap<u8, usize> {
    if node == "svr" {
        [(0, 1)].into_iter().collect()
    } else if let Some(cached) = memo.get(node) {
        cached.clone()
    } else {
        let res = index
            .get(node)
            .iter()
            .flat_map(|x| x.iter())
            .map(|x| lookup(index, memo, x))
            .flat_map(|x| x.into_iter())
            .map(|(flags, count)| {
                (
                    flags
                        | match node {
                            "fft" => 0x01,
                            "dac" => 0x10,
                            _ => 0,
                        },
                    count,
                )
            })
            .fold(BTreeMap::new(), |mut p, (flags, c)| {
                p.entry(flags).and_modify(|fc| *fc += c).or_insert(c);
                p
            });
        memo.insert(node.to_owned(), res.clone());
        res
    }
}

// We could generalize these two functions but who can be bothered?
fn day_11_part_2(inpt: &str) -> usize {
    let neighbours: BTreeMap<String, BTreeSet<String>> = parse_input(inpt).into_iter().collect();
    let index: BTreeMap<String, BTreeSet<String>> = neighbours
        .into_iter()
        .flat_map(|(s, es)| es.into_iter().map(move |e| (e, s.clone())))
        .fold(BTreeMap::new(), |mut p, (e, s)| {
            p.entry(e)
                .and_modify(|ss| {
                    ss.insert(s.clone());
                })
                .or_insert([s].into_iter().collect());
            p
        });

    let mut memo: BTreeMap<String, BTreeMap<u8, usize>> = BTreeMap::new();

    lookup(&index, &mut memo, "out")
        .get(&0x11)
        .cloned()
        .unwrap_or(0)
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_11_part_1(&inpt));
    println!("Part 2\t{}", day_11_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::{day_11_part_1, day_11_part_2};

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

    const STR_INPT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn example2() {
        assert_eq!(day_11_part_2(STR_INPT_2), 2);
    }
}
