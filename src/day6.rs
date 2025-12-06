use std::{
    env,
    io::{self, Read},
};

enum Op {
    Add,
    Mul,
}

fn parse_input(inpt: &str) -> (Vec<Vec<usize>>, Vec<Op>) {
    fn line_is_num_row(l: &str) -> bool {
        l.chars()
            .find(|c| !c.is_whitespace())
            .unwrap()
            .is_ascii_digit()
    }
    let numbers = inpt
        .lines()
        .take_while(|l| line_is_num_row(l))
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    let ops = inpt
        .lines()
        .find(|l| !line_is_num_row(l))
        .map(|l| {
            l.split_whitespace()
                .map(|o| match o {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!("Unknown op: {:?}", o),
                })
                .collect()
        })
        .unwrap();
    (numbers, ops)
}

fn day_6((numbers, ops): &(Vec<Vec<usize>>, Vec<Op>)) -> usize {
    let col_count = numbers[0].len();
    (0..col_count)
        .map(|x| {
            let col = numbers.iter().map(|row| row[x]);
            match ops[x] {
                Op::Add => col.sum::<usize>(),
                Op::Mul => col.product(),
            }
        })
        .sum()
}

fn day_6_part2(_inpt: &(Vec<Vec<usize>>, Vec<Op>)) -> usize {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_6(&inpt),
        [_, s] if s == "part2" => day_6_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_6, parse_input};

    const STR_INPT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_6(inpt), 4277556);
    }
}
