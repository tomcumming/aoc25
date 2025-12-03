use std::{
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<Vec<u8>> {
    inpt.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn max_left_most(tail_len: usize, digits: &[u8]) -> Option<(u8, &[u8])> {
    (0..digits.len() - tail_len)
        .rev()
        .map(|i| (digits[i], &digits[i + 1..]))
        .max_by_key(|(d, _)| *d)
}

fn max_digits_reversed(count: usize, digits: &[u8]) -> Option<Vec<u8>> {
    if count == 0 {
        return Some(Vec::new());
    }

    let (d, our_tail_digits) = max_left_most(count - 1, digits)?;
    let mut tail_digits = max_digits_reversed(count - 1, our_tail_digits)?;
    tail_digits.push(d);
    Some(tail_digits)
}

fn reverse_digits_to_number(digits: &[u8]) -> u64 {
    digits.iter().cloned().rfold(0, |p, c| p * 10 + c as u64)
}

fn day_3(inpt: &[Vec<u8>]) -> u64 {
    inpt.iter()
        .map(|digits| reverse_digits_to_number(&max_digits_reversed(2, digits).unwrap()))
        .sum()
}

fn day_3_part2(inpt: &[Vec<u8>]) -> u64 {
    inpt.iter()
        .map(|digits| reverse_digits_to_number(&max_digits_reversed(12, digits).unwrap()))
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_3(&inpt),
        [_, s] if s == "part2" => day_3_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_3, day_3_part2, parse_input};

    const STR_INPT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_3(inpt), 357);
    }

    #[test]
    fn example2() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_3_part2(inpt), 3121910778619);
    }
}
