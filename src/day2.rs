use std::{
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<(u64, u64)> {
    inpt.trim()
        .split(",")
        .map(|s| match s.split("-").collect::<Vec<_>>().as_slice() {
            [sl, sr] => (sl.parse().unwrap(), sr.parse().unwrap()),
            _ => panic!("Parsing failed!"),
        })
        .collect()
}

fn day_2(inpt: &[(u64, u64)]) -> u64 {
    inpt.iter()
        .cloned()
        .flat_map(|(l, h)| {
            (l..=h).filter(|n| {
                let m = n.ilog10() + 1;
                let o = (10u64).pow(m / 2);
                let p = n / o;
                let q = n % o;
                m % 2 == 0 && p == q
            })
        })
        .sum()
}

fn day_2_part2(_inpt: &[(u64, u64)]) -> u64 {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_2(&inpt),
        [_, s] if s == "part2" => day_2_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_2, parse_input};

    const STR_INPT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_2(inpt), 1227775554);
    }
}
