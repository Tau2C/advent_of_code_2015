use std::cmp::min;

fn main() {
    let input = std::fs::read_to_string("day02/input.txt").unwrap();

    let (p1, presents) = puzzle1(&input);
    let p2 = puzzle2(&input, presents);
    println!("P1: {p1}\nP2: {:?}", p2);
}

fn puzzle1(input: &str) -> (u32, Vec<(u8, u8, u8)>) {
    let presents: Vec<(u8, u8, u8)> = input
        .split_whitespace()
        .map(|s| {
            s.split('x')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter_map(|w| Some((w[0], w[1], w[2])))
        .collect();

    let mut paper = 0;

    for present in &presents {
        let a = present.0 as u32 * present.1 as u32;
        let b = present.0 as u32 * present.2 as u32;
        let c = present.1 as u32 * present.2 as u32;
        let slack = min(a, min(b, c));

        paper += 2 * a + 2 * b + 2 * c + slack;
    }
    (paper, presents)
}

fn puzzle2(input: &str, presents: Vec<(u8, u8, u8)>) -> u32 {
    let mut ribbon = 0;
    for present in presents {
        let a = present.0 as u32;
        let b = present.1 as u32;
        let c = present.2 as u32;
        let shortest = min(a, min(b, c));
        if a == shortest {
            let shortest = min(b, c);
            ribbon += 2 * a + 2 * shortest;
        } else if b == shortest {
            let shortest = min(a, c);
            ribbon += 2 * b + 2 * shortest;
        } else if c == shortest {
            let shortest = min(a, b);
            ribbon += 2 * c + 2 * shortest;
        }
        ribbon += a * b * c;
    }
    ribbon
}
