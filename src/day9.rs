use std::io::{self, Read};

type Coord = (isize, isize);
type Pair = (Coord, Coord);

fn area(((sx, sy), (ex, ey)): Pair) -> isize {
    ((ex - sx).abs() + 1) * ((ey - sy).abs() + 1)
}

fn parse_coords(inpt: &str) -> Vec<Coord> {
    inpt.lines()
        .map(|l| match l.split(",").collect::<Vec<_>>().as_slice() {
            [sx, sy] => (sx.parse().unwrap(), sy.parse().unwrap()),
            _ => panic!(),
        })
        .collect()
}

fn day_9_part_1(inpt: &str) -> isize {
    let coords = parse_coords(inpt);
    let pairs = coords
        .iter()
        .flat_map(|s| coords.iter().map(|e| (*s, *e)))
        .filter(|((sx, _sy), (ex, _ey))| sx <= ex);
    let biggest = pairs.max_by_key(|p| area(*p)).unwrap();
    area(biggest)
}

fn day_9_part_2(inpt: &str) -> isize {
    let coords = parse_coords(inpt);

    let lines: Vec<Pair> = (0..coords.len())
        .map(|i| (coords[i], coords[(i + 1) % coords.len()]))
        .collect();

    fn line_though(line: Pair, rect: Pair) -> bool {
        let lx1 = isize::min(line.0.0, line.1.0);
        let lx2 = isize::max(line.0.0, line.1.0);
        let ly1 = isize::min(line.0.1, line.1.1);
        let ly2 = isize::max(line.0.1, line.1.1);
        let ry1 = isize::min(rect.0.1, rect.1.1);
        let ry2 = isize::max(rect.0.1, rect.1.1);
        let rx1 = isize::min(rect.0.0, rect.1.0);
        let rx2 = isize::max(rect.0.0, rect.1.0);
        if line.0.0 == line.1.0 {
            // vert
            let x = line.0.0;
            x > rx1 && x < rx2 && (ly2 > ry1) && (ly1 < ry2)
        } else if line.0.1 == line.1.1 {
            // horiz
            let y = line.0.1;
            y > ry1 && y < ry2 && (lx2 > rx1) && (lx1 < rx2)
        } else {
            panic!()
        }
    }

    let pairs = coords
        .iter()
        .flat_map(|s| coords.iter().map(|e| (*s, *e)))
        .filter(|((sx, _sy), (ex, _ey))| sx <= ex)
        .filter(|p| !lines.iter().any(|l| line_though(*l, *p)));
    let biggest = pairs.max_by_key(|p| area(*p)).unwrap();
    area(biggest)
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_9_part_1(&inpt));
    println!("Part 2\t{}", day_9_part_2(&inpt));
}

#[cfg(test)]
mod tests {
    use super::{day_9_part_1, day_9_part_2};

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

    #[test]
    fn example2() {
        assert_eq!(day_9_part_2(STR_INPT), 24);
    }
}
