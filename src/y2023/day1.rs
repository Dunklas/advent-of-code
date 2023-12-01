pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            let mut x = line.chars()
                .filter(|c| c.is_numeric());
            let first = x.next();
            let last = x.last();
            let b = format!("{}{}", first.unwrap().to_string(), last.unwrap_or(first.unwrap()));
            return b;
        })
        .flat_map(|x| x.parse::<i32>())
        .sum()
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, part1(input));
    }

    #[test]
    fn part_2() {

    }
}
 