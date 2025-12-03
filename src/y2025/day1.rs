pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    let parsed_input = input
        .lines()
        .map(|line| {
            let (dir, value) = line.split_at(1);
            let value = value.parse::<isize>().unwrap();
            (dir, value)
        })
        .collect::<Vec<_>>();
    let mut sum = 50;
    let mut result = 0;
    for (dir, value) in parsed_input {
        let v = match dir {
            "L" => -value,
            "R" => value,
            _ => panic!("Bye"),
        };
        sum = (sum + v).rem_euclid(100);
        if sum == 0 {
            result += 1;
        }
    }
    result
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3);
    }
}
