use std::{
    collections::BTreeSet,
    io::{self, Read},
};

fn day_7_part_1(inpt: &str) -> usize {
    let (width, start_x) = {
        let first_line = inpt.lines().next().unwrap();
        let start_x = first_line
            .char_indices()
            .find(|(_, c)| *c == 'S')
            .map(|(i, _)| i)
            .unwrap();
        (first_line.len(), start_x)
    };

    let (splits, _) = inpt.lines().fold(
        (0, [start_x].into_iter().collect::<BTreeSet<usize>>()),
        |(splits, beams), line_str| {
            let line: Vec<char> = line_str.chars().collect();
            let not_split: BTreeSet<usize> = beams
                .iter()
                .cloned()
                .filter(|x| line[*x] == '.' || line[*x] == 'S')
                .collect();
            let split = beams.iter().cloned().filter(|x| line[*x] == '^');
            let children: BTreeSet<usize> = split
                .clone()
                .flat_map(|x| [x as isize - 1, x as isize + 1])
                .filter(|x| *x >= 0 && *x < width as isize)
                .map(|x| x as usize)
                .collect();

            (
                splits + split.count(),
                not_split.union(&children).cloned().collect(),
            )
        },
    );

    splits
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_7_part_1(&inpt));
}

#[cfg(test)]
mod tests {
    use super::day_7_part_1;

    const STR_INPT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn example() {
        assert_eq!(day_7_part_1(STR_INPT), 21);
    }
}
