use std::fmt;
use crate::units::Unit;

/// Object to store lengths.
/// Contains the size as well as the unit type.
pub struct Length { 
    size: f64,
    system: Unit
}

impl Length {
    /// Create a new object of type Length.
    /// 
    /// Must provide the size of the length and the unit.
    pub fn new(size: f64, system: Unit) -> Self {
        Self { 
            size,
            system
        }
    }

    /// Convert from the current unit to the other.
    /// 
    /// For example from Metric to Imperial
    /// 
    /// Usage:
    /// ```
    /// let metric_unit = Length::new(123.13, Unit::Metric);
    /// let converted_unit = metric_unit.convert();
    /// 
    /// assert_eq!(converted_unit, 48.48);
    /// ```
    pub fn convert(self) -> f64 {
        let ratio = 2.54;
        match self.system {
            Unit::Metric => { return (self.size / ratio * 100.0).round() / 100.0 },
            Unit::Imperial => { return (self.size * ratio * 100.0).round() / 100.0 },
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size: {}, system: {}", self.size, self.system)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_cm_to_in() {
        // Arrange 
        let size = 22.0;
        let system = Unit::Metric;
        let expected = 8.66;

        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_cm_to_in_rounding_up() {
        // Arrange 
        let size = 21.1;
        let system = Unit::Metric;
        let expected = 8.31;

        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_cm_to_in_rounding_down() {
        // Arrange 
        let size = 21.5;
        let system = Unit::Metric;
        let expected = 8.46;

        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_in_to_cm() {
        // Arrange 
        let size = 22.0;
        let system = Unit::Imperial;
        let expected = 55.88;
        
        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_in_to_cm_rounding_up() {
        // Arrange 
        let size = 8.2;
        let system = Unit::Imperial;
        let expected = 20.83;
        
        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_in_to_cm_rounding_down() {
        // Arrange 
        let size = 8.3;
        let system = Unit::Imperial;
        let expected = 21.08;
        
        // Act
        let unit = Length::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

}

