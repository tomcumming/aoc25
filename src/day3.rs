use std::{
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<Vec<u8>> {
    inpt.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn day_2(inpt: &[Vec<u8>]) -> u64 {
    inpt.iter()
        .map(|digits| {
            let rev_tails = (0..digits.len() - 1)
                .rev()
                .map(|i| (digits[i], &digits[i + 1..]));

            let (d1, ds) = rev_tails.max_by_key(|(d, _)| *d).unwrap();
            let d2 = ds.iter().max().cloned().unwrap();

            (d1 * 10 + d2) as u64
        })
        .sum()
}

fn day_2_part2(_inpt: &[Vec<u8>]) -> u64 {
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

    const STR_INPT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_2(inpt), 357);
    }
}
