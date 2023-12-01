pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split("x"))
        .map(|mut parts| {
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .map(|(l, w, h)| packaging(w, l, h))
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split("x"))
        .map(|mut parts| {
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .map(|(l, w, h)| ribbon(l, w, h))
        .sum()
}

fn packaging(l: i32, w: i32, h: i32) -> i32 {
    let sum = 2 * l * w + 2 * w * h + 2 * h * l;
    let mut all = vec![l, w, h];
    all.sort();
    let extra = all[0] * all[1];
    sum + extra
}

fn ribbon(l: i32, w: i32, h: i32) -> i32 {
    let mut all = vec![l, w, h];
    all.sort();
    let ribbon_len = all[0] * 2 + all[1] * 2;
    let bow: i32 = all.iter().product();
    ribbon_len + bow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packaging_surface_area() {
        assert_eq!(58, packaging(2, 3, 4));
        assert_eq!(43, packaging(1, 1, 10));
    }

    #[test]
    fn ribbon_length() {
        assert_eq!(34, ribbon(2, 3, 4));
        assert_eq!(14, ribbon(1, 1, 10));
    }
}
