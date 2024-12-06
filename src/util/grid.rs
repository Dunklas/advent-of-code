use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct ParseGridError {}

impl<T> Grid<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
{
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Result<Vec<Vec<T>>, ParseGridError> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| T::from_str(&c.to_string()).map_err(|e| ParseGridError {}))
                    .collect()
            })
            .collect();
        grid.map(Grid::new)
    }
}
