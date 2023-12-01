pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => sum,
    })
}

fn part2(input: &str) -> i32 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            return 1 + i as i32;
        }
    }
    -1
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

    #[test]
    fn part_2() {
        let input = vec![(")", 1), ("()())", 5)];
        for (input, expected) in input {
            let result = part2(input);
            assert_eq!(expected, result);
        }
    }
}
