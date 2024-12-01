fn main() {
    let input = std::fs::read_to_string("day01/input.txt").unwrap();

    let floor = puzzle1(&input);
    let basement = puzzle2(&input);
    println!("P1: {floor}\nP2: {:?}", basement);
}

fn puzzle1(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
    }
    floor
}

fn puzzle2(input: &str) -> Option<u32> {
    let mut floor = 0;

    for index in 0..input.len() {
        let c = input.chars().nth(index).unwrap();
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
        if floor < 0 {
            return Some((index + 1) as u32);
        }
    }
    return None;
}
