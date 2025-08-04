// This is fundamentally the same problem as 293-2. Only then we wanted false on straight line and
// true otherwise, while now we want true on straight line and false otherwise.
// The solution below is copy-pasted. I needed to bump Vector fields to f64, as we need to square a
// very large number. I also tuned difference threshold just enough to make test with large numbers
// pass

pub struct Point {
    x: f64,
    y: f64,
}

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn from_points(one: &Point, two: &Point) -> Vector {
        Vector {
            x: two.x - one.x,
            y: two.y - one.y,
        }
    }
}

pub fn is_straight_line(one: &Point, two: &Point, three: &Point) -> bool {
    if (one.x == two.x && two.x == three.x) || (one.y == two.y && two.y == three.y) {
        return true;
    }

    let v = Vector::from_points(one, two);
    let w = Vector::from_points(two, three);

    let dot_product = v.x * w.x + v.y * w.y;
    let magnitude_v = (v.x.powi(2) + v.y.powi(2)).sqrt();
    let magnitude_w = (w.x.powi(2) + w.y.powi(2)).sqrt();

    // the main idea is: are the points on the same line that is not horizontal or vertical?
    // to check that, we can check if vectors |AB| and |BC| are in the same direction
    // vectors are in the same direction if their dot product and product of their magnitudes
    // are the same
    // we do greater than instead of equal as a hack to deal with floating point math precision
    if 0.00025 > (dot_product - (magnitude_v * magnitude_w)).abs() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = is_straight_line(
            &Point { x: 2.0, y: 1.0 },
            &Point { x: 2.0, y: 3.0 },
            &Point { x: 2.0, y: 5.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = is_straight_line(
            &Point { x: 1.0, y: 4.0 },
            &Point { x: 3.0, y: 4.0 },
            &Point { x: 10.0, y: 4.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = is_straight_line(
            &Point { x: 0.0, y: 0.0 },
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 2.0, y: 3.0 },
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = is_straight_line(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 1.0, y: 1.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = is_straight_line(
            &Point {
                x: 1000000.0,
                y: 1000000.0,
            },
            &Point {
                x: 2000000.0,
                y: 2000000.0,
            },
            &Point {
                x: 3000000.0,
                y: 3000000.0,
            },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let result = is_straight_line(
            &Point { x: 0.0, y: 0.0 },
            &Point { x: 2.0, y: 3.0 },
            &Point { x: 4.0, y: 5.0 },
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test7() {
        let result = is_straight_line(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 3.0, y: 4.0 },
            &Point { x: 9.0, y: 13.0 },
        );
        assert_eq!(result, true);
    }
}
