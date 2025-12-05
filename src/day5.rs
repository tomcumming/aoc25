use std::{
    collections::BTreeSet,
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let ranges = inpt
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| match l.split("-").collect::<Vec<_>>().as_slice() {
            [low, high] => (low.parse().unwrap(), high.parse().unwrap()),
            _ => panic!("Can't parse range!"),
        })
        .collect();
    let ids = inpt
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    (ranges, ids)
}

fn day_5((ranges, ids): &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let all_ids: BTreeSet<usize> = ids.iter().cloned().collect();
    ranges
        .iter()
        .fold(BTreeSet::<usize>::new(), |mut fresh, (low, high)| {
            for id in all_ids.range(low..=high) {
                fresh.insert(*id);
            }
            fresh
        })
        .len()
}

fn day_5_part2(_inpt: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_5(&inpt),
        [_, s] if s == "part2" => day_5_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_5, parse_input};

    const STR_INPT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_5(inpt), 3);
    }
}
