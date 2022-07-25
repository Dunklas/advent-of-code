pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }
    return floor;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, expected) in input {
            let result = part1(input);
            assert_eq!(expected, result);
        }
    }
}
