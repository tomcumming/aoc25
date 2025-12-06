use std::{
    env,
    io::{self, Read},
};

enum Op {
    Add,
    Mul,
}

fn parse_input(inpt: &str) -> (Vec<Vec<char>>, Vec<Op>) {
    let line_count = inpt.lines().count();
    let ls = inpt
        .lines()
        .take(line_count - 1)
        .map(|l| l.chars().collect())
        .collect();
    let ops = inpt
        .lines()
        .last()
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
    (ls, ops)
}

fn day_6((lines, ops): &(Vec<Vec<char>>, Vec<Op>)) -> usize {
    let numbers: Vec<Vec<usize>> = lines
        .iter()
        .map(|cs| cs.iter().collect())
        .map(|l: String| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

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

fn group_cols(mut cols: Vec<String>) -> Vec<Vec<usize>> {
    let mut groups = Vec::new();

    while !cols.is_empty() {
        let mut group = Vec::new();
        while let Some(s) = cols.pop() {
            if s.trim().is_empty() {
                break;
            } else {
                group.push(s.trim().parse().unwrap());
            }
        }
        groups.push(group);
    }

    groups.reverse();
    groups
}

fn day_6_part2((inpt_ls, ops): &(Vec<Vec<char>>, Vec<Op>)) -> usize {
    let line_len = inpt_ls[0].len();
    let all_cols = (0..line_len)
        .map(|x| inpt_ls.iter().map(|l| l[x]).collect::<String>())
        .collect();
    group_cols(all_cols)
        .into_iter()
        .zip(ops)
        .map(|(ns, op)| match op {
            Op::Add => ns.iter().sum::<usize>(),
            Op::Mul => ns.iter().product(),
        })
        .sum()
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
    use super::{day_6, day_6_part2, parse_input};

    const STR_INPT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_6(inpt), 4277556);
    }

    #[test]
    fn example2() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_6_part2(inpt), 3263827);
    }
}
