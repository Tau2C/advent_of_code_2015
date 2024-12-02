#![allow(dead_code)]
fn main() {
    let input = std::fs::read_to_string("day06/input.txt").unwrap();

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

enum Operation {
    TurnOn(((u32, u32), (u32, u32))),
    TurnOff(((u32, u32), (u32, u32))),
    Toggle(((u32, u32), (u32, u32))),
}

fn parse_op(op: &str) -> Operation {
    let words = op.split_whitespace().collect::<Vec<&str>>();
    if op.starts_with("turn off") {
        let pos1 = words[2]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let pos2 = words[4]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        return Operation::TurnOff(((pos1[0], pos1[1]), (pos2[0], pos2[1])));
    } else if op.starts_with("turn on") {
        let pos1 = words[2]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let pos2 = words[4]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        return Operation::TurnOn(((pos1[0], pos1[1]), (pos2[0], pos2[1])));
    } else if op.starts_with("toggle") {
        let pos1 = words[1]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let pos2 = words[3]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        return Operation::Toggle(((pos1[0], pos1[1]), (pos2[0], pos2[1])));
    }
    panic!()
}

fn puzzle1(input: &str) -> i32 {
    let input = input.split('\n').collect::<Vec<&str>>();

    let mut map = [[false; 1000]; 1000];

    for op in input {
        if op.is_empty() {
            continue;
        }
        match parse_op(op) {
            Operation::TurnOn(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        map[y as usize][x as usize] = true;
                    }
                }
            }
            Operation::TurnOff(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        map[y as usize][x as usize] = false;
                    }
                }
            }
            Operation::Toggle(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        map[y as usize][x as usize] = !map[y as usize][x as usize];
                    }
                }
            }
        }
    }

    let mut lamps_on = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            lamps_on += map[y][x] as i32;
        }
    }
    lamps_on
}

fn puzzle2(input: &str) -> u32 {
    let input = input.split('\n').collect::<Vec<&str>>();

    let mut map: Vec<[u32; 1000]> = Vec::new();

    for _ in 0..1000 {
        map.push([0; 1000]);
    }

    for op in input {
        if op.is_empty() {
            continue;
        }
        match parse_op(op) {
            Operation::TurnOn(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        map[y as usize][x as usize] += 1;
                    }
                }
            }
            Operation::TurnOff(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        if map[y as usize][x as usize] != 0 {
                            map[y as usize][x as usize] -= 1;
                        }
                    }
                }
            }
            Operation::Toggle(((x1, y1), (x2, y2))) => {
                for y in y1..(y2 + 1) {
                    for x in x1..(x2 + 1) {
                        map[y as usize][x as usize] += 2;
                    }
                }
            }
        }
    }

    let mut brightness = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            brightness += map[y][x];
        }
    }
    brightness
}
