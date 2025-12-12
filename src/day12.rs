use std::io::{self, Read};

fn day_10_part_1(inpt: &str) -> usize {
    let mut shapes: Vec<usize> = Vec::new();

    let mut lines = inpt.lines().peekable();

    loop {
        if let Some(s) = lines.peek()
            && s.contains("x")
        {
            break;
        }

        lines.next(); // skip number
        let area: usize = (0..3)
            .flat_map(|_| lines.next())
            .flat_map(|s| s.chars())
            .filter(|c| *c == '#')
            .count();

        shapes.push(area);
        lines.next(); // skip ws
    }

    lines
        .map(|line| {
            let ps1: Vec<_> = line.split(": ").collect();
            let ps2: Vec<_> = ps1[0].split("x").collect();
            let w: usize = ps2[0].parse().unwrap();
            let h: usize = ps2[1].parse().unwrap();

            let ss: Vec<usize> = ps1[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            (w * h, ss)
        })
        .filter(|(area, counts)| {
            *area
                >= counts
                    .iter()
                    .zip(shapes.iter().cloned())
                    .map(|(c, s)| *c * s)
                    .sum()
        })
        .count()
}

fn main() {
    let inpt = {
        let mut inpt = String::new();
        io::stdin().read_to_string(&mut inpt).unwrap();
        inpt
    };
    println!("Part 1\t{}", day_10_part_1(&inpt));
}
