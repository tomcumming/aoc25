use std::io::{self, Read};

fn day_9_part_1(inpt: &str) -> isize {
    let coords: Vec<(isize, isize)> = inpt
        .lines()
        .map(|l| match l.split(",").collect::<Vec<_>>().as_slice() {
            [sx, sy] => (sx.parse().unwrap(), sy.parse().unwrap()),
            _ => panic!(),
        })
        .collect();

    fn area(((sx, sy), (ex, ey)): ((isize, isize), (isize, isize))) -> isize {
        ((ex - sx).abs() + 1) * ((ey - sy).abs() + 1)
    }

    let pairs = coords
        .iter()
        .flat_map(|s| coords.iter().map(|e| (*s, *e)))
        .filter(|((sx, _sy), (ex, _ey))| sx <= ex);

    let biggest = pairs.max_by_key(|p| area(*p)).unwrap();

    area(biggest)
}

fn _day_9_part_2(_inpt: &str) -> usize {
    todo!()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_9_part_1(&inpt));
    // println!("Part 2\t{}", day_9_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::day_9_part_1;

    const STR_INPT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn example() {
        assert_eq!(day_9_part_1(STR_INPT), 50);
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(day_9_part_2(STR_INPT), _);
    // }
}
