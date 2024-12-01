fn main() {
    let input = String::new() + "bgvyzdsv";

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

fn puzzle1(input: &str) -> u32 {
    let data = input;
    let mut n = 0;
    loop {
        let hash = md5::compute(format!("{data}{n}"));
        if format!("{:X?}", hash).starts_with("00000") {
            break n;
        }
        n += 1;
    }
}

fn puzzle2(input: &str) -> i32 {
    let data = input;
    let mut n = 0;
    loop {
        let hash = md5::compute(format!("{data}{n}"));
        if format!("{:X?}", hash).starts_with("000000") {
            break n;
        }
        n += 1;
    }
}
