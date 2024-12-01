use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("day03/input.txt").unwrap();

    let p1 = puzzle1(&input);
    let p2 = puzzle2(&input);
    println!("P1: {p1}\nP2: {:?}", p2);
}

fn puzzle1(input: &str) -> u32 {
    let mut presents_map: HashMap<(i32, i32), u32> = HashMap::new();

    let mut houses = 1;

    let mut x = 0;
    let mut y = 0;

    presents_map.insert((x, y), 1);

    for i in 0..input.len() {
        let mov = input.chars().nth(i).unwrap();
        match mov {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!(),
        }
        let value = match presents_map.get(&(x, y)) {
            Some(val) => val.clone(),
            None => {
                houses += 1;
                0
            }
        };
        presents_map.insert((x, y), value + 1);
    }
    houses
}

fn puzzle2(input: &str) -> i32 {
    let mut presents_map: HashMap<(i32, i32), u32> = HashMap::new();

    let mut houses = 1;

    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;

    presents_map.insert((0, 0), 2);

    for i in 0..input.len() {
        let mov = input.chars().nth(i).unwrap();
        if i % 2 == 0 {
            match mov {
                '<' => x1 -= 1,
                '>' => x1 += 1,
                '^' => y1 += 1,
                'v' => y1 -= 1,
                _ => panic!(),
            }
            let value = match presents_map.get(&(x1, y1)) {
                Some(val) => val.clone(),
                None => {
                    houses += 1;
                    0
                }
            };
            presents_map.insert((x1, y1), value + 1);
        } else {
            match mov {
                '<' => x2 -= 1,
                '>' => x2 += 1,
                '^' => y2 += 1,
                'v' => y2 -= 1,
                _ => panic!(),
            }
            let value = match presents_map.get(&(x2, y2)) {
                Some(val) => val.clone(),
                None => {
                    houses += 1;
                    0
                }
            };
            presents_map.insert((x2, y2), value + 1);
        }
    }
    houses
}
