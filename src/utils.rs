pub fn round_to_two_decimals(number: f64) -> f64 {
    return (number * 100.0).round() / 100.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_zero() {
        // Arrange
        let number = 0.0;
        let expected = 0.00;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_long_number() {
        // Arrange
        let number = 0.0199999999999999999999999999999999999999999999999999999999999;
        let expected = 0.02;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_up_positive() {
        // Arrange
        let number = 123.4598;
        let expected = 123.46;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_down_positive() {
        // Arrange
        let number = 123.45491;
        let expected = 123.45;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_up_negative() {
        // Arrange
        let number = -941.567167;
        let expected = -941.57;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_down_negative() {
        // Arrange
        let number = -941.564161;
        let expected = -941.56;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_up_ranged() {
        // Arrange
        let number = 541.567649;
        let expected = 541.57;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn rounding_down_ranged() {
        // Arrange
        let number = 541.56499;
        let expected = 541.56;
        // Act
        let result = round_to_two_decimals(number);
        // Assert
        assert_eq!(result, expected);
    }
}
