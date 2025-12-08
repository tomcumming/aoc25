use std::{
    collections::BTreeSet,
    io::{self, Read},
};

fn mag2(a: &Vec<isize>, b: &Vec<isize>) -> usize {
    a.iter()
        .cloned()
        .zip(b.iter().cloned())
        .map(|(c, d)| c - d)
        .map(|x| (x * x) as usize)
        .sum()
}

fn day_8_part_1(inpt: &str, connections: usize) -> usize {
    let coords: Vec<Vec<isize>> = inpt
        .lines()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut pairs: Vec<(usize, usize)> = (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
        .collect();
    pairs.sort_by_key(|(i, j)| mag2(&coords[*i], &coords[*j]));
    pairs.reverse();

    let mut circuits: Vec<BTreeSet<usize>> = coords
        .iter()
        .enumerate()
        .map(|(i, _)| [i].into_iter().collect())
        .collect();

    for _ in 0..connections {
        let (i, j) = pairs.pop().unwrap();
        let c1 = (0..circuits.len())
            .find(|c| circuits[*c].contains(&i))
            .unwrap();
        let c2 = (0..circuits.len())
            .find(|c| circuits[*c].contains(&j))
            .unwrap();

        if c1 != c2 {
            let mut cs1 = circuits.remove(usize::max(c1, c2));
            let mut cs2 = circuits.remove(usize::min(c1, c2));

            cs1.append(&mut cs2);
            circuits.push(cs1);
        }
    }

    let mut circuit_lens: Vec<usize> = circuits.into_iter().map(|c| c.len()).collect();
    circuit_lens.sort();
    circuit_lens.reverse();
    circuit_lens.into_iter().take(3).product()
}

fn _day_8_part_2(_inpt: &str) -> usize {
    todo!()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_8_part_1(&inpt, 1000 + 1));
    // println!("Part 2\t{}", day_8_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::day_8_part_1;

    const STR_INPT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn example() {
        assert_eq!(day_8_part_1(STR_INPT, 10), 40);
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(day_8_part_2(STR_INPT), 0);
    // }
}
