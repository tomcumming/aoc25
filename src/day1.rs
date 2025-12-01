use std::io::{self, Read};

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
            let s_ = (s + i) % 100;
            (s_, c + if s_ == 0 { 1 } else { 0 })
        })
        .1
}

fn main() {
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let out = day_1(&parse_input(&str_input));
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_1, parse_input};

    #[test]
    fn example() {
        let str_inpt = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let inpt = &parse_input(str_inpt);

        assert_eq!(day_1(inpt), 3);
    }
}
