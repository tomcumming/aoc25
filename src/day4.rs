use std::{
    env,
    io::{self, Read},
};

fn parse_input(inpt: &str) -> Vec<Vec<bool>> {
    inpt.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Unexpected char: {:?}", c),
                })
                .collect()
        })
        .collect()
}

fn lookup_cell(grid: &[Vec<bool>], x: usize, y: usize) -> Option<bool> {
    grid.get(y).and_then(|row| row.get(x)).cloned()
}

fn day_4(inpt: &[Vec<bool>]) -> usize {
    let check_neighbours = |x: usize, y: usize| -> usize {
        (x.saturating_sub(1)..=x + 1)
            .flat_map(|nx| (y.saturating_sub(1)..=y + 1).map(move |ny| (nx, ny)))
            .filter(|(nx, ny)| *nx != x || *ny != y)
            .filter(|(nx, ny)| lookup_cell(inpt, *nx, *ny).unwrap_or(false))
            .count()
    };

    inpt.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .filter(|(x, cell)| *cell && check_neighbours(*x, y) < 4)
                .count()
        })
        .sum()
}

fn day_4_part2(_inpt: &Vec<Vec<bool>>) -> usize {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut str_input = String::new();
    io::stdin().read_to_string(&mut str_input).unwrap();
    let inpt = parse_input(&str_input);
    let out = match args.as_slice() {
        [_] => day_4(&inpt),
        [_, s] if s == "part2" => day_4_part2(&inpt),
        _ => panic!("Unexpected args"),
    };
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::{day_4, parse_input};

    const STR_INPT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn example() {
        let inpt = &parse_input(STR_INPT);

        assert_eq!(day_4(inpt), 13);
    }
}
