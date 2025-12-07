use std::{
    collections::{BTreeMap, BTreeSet},
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

fn day_7_part_2(inpt: &str) -> usize {
    let (width, start_x) = {
        let first_line = inpt.lines().next().unwrap();
        let start_x = first_line
            .char_indices()
            .find(|(_, c)| *c == 'S')
            .map(|(i, _)| i)
            .unwrap();
        (first_line.len(), start_x)
    };

    let beam_counts = inpt.lines().fold(
        [(start_x, 1)]
            .into_iter()
            .collect::<BTreeMap<usize, usize>>(),
        |beam_counts, line_str| {
            let line: Vec<char> = line_str.chars().collect();
            let not_split = beam_counts
                .iter()
                .map(|(k, v)| (*k, *v)) // why not cloned?
                .filter(|(x, _)| line[*x] == '.' || line[*x] == 'S');
            let split = beam_counts
                .iter()
                .map(|(k, v)| (*k, *v)) // why not cloned?
                .filter(|(x, _)| line[*x] == '^')
                .flat_map(|(k, v)| [(k as isize - 1, v), (k as isize + 1, v)])
                .filter(|(k, _)| *k >= 0 && *k < width as isize)
                .map(|(k, v)| (k as usize, v));

            not_split
                .chain(split)
                .fold(BTreeMap::new(), |mut p, (k, v)| {
                    p.entry(k).and_modify(|c| *c += v).or_insert(v);
                    p
                })
        },
    );

    beam_counts.into_values().sum()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_7_part_1(&inpt));
    println!("Part 2\t{}", day_7_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::{day_7_part_1, day_7_part_2};

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

    #[test]
    fn example2() {
        assert_eq!(day_7_part_2(STR_INPT), 40);
    }
}
