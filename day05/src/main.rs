fn main() {
    let input = std::fs::read_to_string("day05/input.txt").unwrap();

    let p1 = puzzle1(&input);
    println!("P1: {p1}");
    let p2 = puzzle2(&input);
    println!("P2: {p2}");
}

fn puzzle1(input: &str) -> u32 {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let not_allowed = ["ab", "cd", "pq", "xy"];

    let mut nice = 0;
    for word in input.split_whitespace().collect::<Vec<&str>>() {
        let mut vowel_count = 0;
        let mut double = false;
        let mut forced = false;
        for i in 0..word.len() {
            let c = word.chars().nth(i).unwrap();
            if vowels.contains(&c) {
                vowel_count += 1;
            }
            if i + 1 < word.len() {
                let next = word.chars().nth(i + 1).unwrap();
                if not_allowed.contains(&(format!("{c}{next}").as_str())) {
                    forced = true;
                    break;
                }
                if c == next {
                    double = true;
                }
            }
        }

        if !forced {
            if vowel_count >= 3 && double {
                nice += 1;
            }
        }
    }
    nice
}

fn puzzle2(input: &str) -> u32 {
    let nice = input
        .split_whitespace()
        .filter(|s| {
            for i in 0..(s.len() - 2) {
                if s.chars().nth(i) == s.chars().nth(i + 2) {
                    return true;
                }
            }
            false
        })
        .filter(|input| {
            for high in input
                .char_indices()
                .flat_map(|(start, _)| input.get(start..))
            {
                let pair_h = high.get(0..2);
                if pair_h.is_none() {
                    continue;
                }
                let pair_h = pair_h.unwrap();
                for pair_l in high
                    .char_indices()
                    .flat_map(|(start, _)| high.get((start + 2)..).and_then(|s| s.get(0..2)))
                {
                    if pair_h == pair_l {
                        return true;
                    }
                }
            }
            false
        })
        .collect::<Vec<&str>>();
    nice.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let test = "qjhvhtzxzqqjkmpb";
        assert_eq!(puzzle2(&test), 1);
    }

    #[test]
    fn test_example2() {
        let test = "xxyxx";
        assert_eq!(puzzle2(&test), 1);
    }

    #[test]
    fn test_example3() {
        let test = "uurcxstgmygtbstg";
        assert_eq!(puzzle2(&test), 0);
    }

    #[test]
    fn test_example4() {
        let test = "ieodomkazucvgmuy";
        assert_eq!(puzzle2(&test), 0);
    }
}
