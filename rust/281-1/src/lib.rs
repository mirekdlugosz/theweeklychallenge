pub fn square_is_light(coordinates: &str) -> bool {
    let mut coordinates_iter = coordinates.chars();
    // cast letter to number in ASCII table. We abuse the fact that uppercase
    // and lowercase letters are 32 characters apart, which is even. This way
    // we don't have to unify case to handle both "A8" and "a8"
    // A is assigned odd number, B is even etc.
    let letter = coordinates_iter.next().unwrap() as u8;
    let number = coordinates_iter.next().unwrap().to_digit(10).unwrap();

    let letter_is_even = u32::from(letter % 2);
    let number_is_even = number % 2;

    // this comes from following table:
    // letter | number | result
    //  even  |  even  | dark (false)
    //  even  |  odd   | light (true)
    //  odd   |  even  | light (true)
    //  odd   |  odd   | dark (false)
    // dark tile is if both letter and number identifier are even (or odd)
    // if one is even and other is odd, then it's light tile

    letter_is_even != number_is_even
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = square_is_light("d3");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = square_is_light("g5");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = square_is_light("e6");
        assert_eq!(result, true);
    }
}
