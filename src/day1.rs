use std::{
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<isize> {
    fn parse_line(line: &str) -> isize {
        let dir = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            _ => panic!("Unknown direction in {:?}", line),
        };
        let amt: isize = line[1..].parse().expect("Could not read amount");
        amt * dir
    }
    inpt.lines().map(parse_line).collect()
}

fn day_1(inpt: &[isize]) -> isize {
    inpt.iter()
        .fold((50, 0), |(s, c), i| {
            let s_ = (s + i).rem_euclid(100);
            (s_, c + if s_ == 0 { 1 } else { 0 })
        })
        .1
}

fn day_1_part2(inpt: &[isize]) -> isize {
    inpt.iter()
        .fold((50, 0), |(s, c), j| {
            let full_turns = (*j) / 100;
            let i = *j - full_turns * 100;
            // grim
            let part_turns = if i >= 0 {
                (s + i) / 100
            } else if s == 0 {
                0
            } else {
                (100 - s - i) / 100
            };
            ((s + i).rem_euclid(100), c + part_turns + full_turns.abs())
        })
        .1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_1(&inpt),
        [_, s] if s == "part2" => day_1_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_1, day_1_part2, parse_input};

    const STR_INPT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_1(inpt), 3);
    }
    #[test]
    fn example_part2() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_1_part2(inpt), 6);
    }
}
