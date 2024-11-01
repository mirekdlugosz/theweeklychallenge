pub struct Point {
    x: f32,
    y: f32,
}

struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn from_points(one: &Point, two: &Point) -> Vector {
        Vector {
            x: two.x - one.x,
            y: two.y - one.y,
        }
    }
}

pub fn is_boomerang(one: &Point, two: &Point, three: &Point) -> bool {
    if (one.x == two.x && two.x == three.x) || (one.y == two.y && two.y == three.y) {
        return false;
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
    if 0.000001 > (dot_product - (magnitude_v * magnitude_w)).abs() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 2.0, y: 3.0 },
            &Point { x: 3.0, y: 2.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 2.0, y: 2.0 },
            &Point { x: 3.0, y: 3.0 },
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 1.0, y: 2.0 },
            &Point { x: 2.0, y: 3.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 1.0, y: 2.0 },
            &Point { x: 1.0, y: 3.0 },
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 2.0, y: 1.0 },
            &Point { x: 3.0, y: 1.0 },
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let result = is_boomerang(
            &Point { x: 0.0, y: 0.0 },
            &Point { x: 2.0, y: 3.0 },
            &Point { x: 4.0, y: 5.0 },
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test7() {
        let result = is_boomerang(
            &Point { x: 1.0, y: 1.0 },
            &Point { x: 3.0, y: 4.0 },
            &Point { x: 9.0, y: 13.0 },
        );
        assert_eq!(result, false);
    }
}
