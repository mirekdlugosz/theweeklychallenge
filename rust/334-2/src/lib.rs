pub struct Point {
    x: isize,
    y: isize,
}

trait Manhattanable {
    fn same_line(&self, other: &Point) -> bool;
    fn distance(&self, other: &Point) -> usize;
}

impl Manhattanable for Point {
    fn same_line(&self, other: &Point) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn manhattan_distance(x: isize, y: isize, points: &[Point]) -> isize {
    let reference = Point { x, y };
    points
        .iter()
        .enumerate()
        .filter_map(|(idx, other)| {
            reference.same_line(other)
                .then_some((idx, reference.distance(other)))
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .map_or(-1, |x| x.0 as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = manhattan_distance(
            3,
            4,
            &[
                Point { x: 1, y: 2 },
                Point { x: 3, y: 1 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 3 },
            ],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = manhattan_distance(
            2,
            5,
            &[
                Point { x: 3, y: 4 },
                Point { x: 2, y: 3 },
                Point { x: 1, y: 5 },
                Point { x: 2, y: 5 },
            ],
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = manhattan_distance(
            1,
            1,
            &[
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 4 },
            ],
        );
        assert_eq!(result, -1);
    }

    #[test]
    fn test4() {
        let result = manhattan_distance(
            0,
            0,
            &[
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 2, y: 0 },
            ],
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = manhattan_distance(
            5,
            5,
            &[
                Point { x: 5, y: 6 },
                Point { x: 6, y: 5 },
                Point { x: 5, y: 4 },
                Point { x: 4, y: 5 },
            ],
        );
        assert_eq!(result, 0);
    }
}
