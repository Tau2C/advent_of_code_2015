fn puzzle1(input: &str) -> (usize, usize) {
    let mut lines = Vec::new();
    let mut code_num = 0;
    let mut line_num = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let blackspace = line
            .chars()
            .filter(|p| !p.is_whitespace())
            .collect::<Vec<char>>();

        let mut data = String::new();

        let mut i = 0;
        while i < line.len() {
            match blackspace[i] {
                '\\' => match blackspace[i + 1] {
                    '\\' => {
                        data += "\\";
                        i += 2;
                        line_num += 1;
                        continue;
                    }
                    '\"' => {
                        data += "\"";
                        i += 2;
                        line_num += 1;
                        continue;
                    }
                    'x' => {
                        let ascii = blackspace[i + 2].to_string() + &blackspace[i + 3].to_string();
                        let code = u8::from_str_radix(&ascii[0..2], 16).unwrap();
                        data += &(code as char).to_string();
                        i += 4;
                        line_num += 1;
                        continue;
                    }
                    _ => panic!("UNKNOWN ESCAPE SEQUENCE"),
                },
                _ => {
                    data += &blackspace[i].to_string();
                    line_num += 1;
                }
            }
            i += 1;
        }
        data = data[1..(data.len() - 1)].to_string();
        println!("{data}");

        code_num += blackspace.len();
        line_num -= 2;

        lines.push(data);
    }

    println!("{} {}", code_num, line_num);

    (code_num - line_num, code_num)
}

fn encode(line: &str) -> String {
    let mut encoded = String::new() + "\"";
    for c in line.chars() {
        match c {
            '"' => encoded += "\\\"",
            '\\' => encoded += "\\\\",
            _ => encoded += &c.to_string(),
        }
    }
    encoded += "\"";
    encoded
}

fn puzzle2(input: &str) -> (usize, usize) {
    let code_num: usize = input
        .split('\n')
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).count())
        .sum();
    let encoded_num: usize = input
        .split('\n')
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect::<String>())
        .map(|s| encode(&s).len())
        .sum();

    (encoded_num - code_num, code_num)
}

fn main() {
    let input = std::fs::read_to_string("day08/input.txt").unwrap();

    let (p1, code_num1) = puzzle1(&input);
    println!("P1: {p1}");
    let (p2, code_num2) = puzzle2(&input);
    println!("P2: {}", p2);
    println!("{}", code_num1 == code_num2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let example = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(puzzle2(&example).0, 19)
    }
}
