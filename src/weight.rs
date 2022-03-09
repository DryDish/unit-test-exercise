use crate::units::Unit;

pub struct Weight {
    amount: f64,
    system: Unit
}

impl Weight {
    /// Create a new object of type Weight.
    /// 
    /// Must provide the amount of weight and the unit.
    pub fn new(amount: f64, system: Unit) -> Self {
        Self { 
            amount,
            system
        }
    }
    /// Convert from the current unit to the other.
    /// 
    /// For example from Metric to Imperial
    /// 
    /// Usage:
    /// ```
    /// let metric_unit = Weight::new(25.0, Unit::Metric);
    /// let converted_unit = metric_unit.convert();
    /// 
    /// assert_eq!(converted_unit, 55,12);
    /// ```
    pub fn convert(self) -> f64 {
        let ratio = 2.20462262185;
        match self.system {
            Unit::Metric => { return (self.amount * ratio * 100.0).round() / 100.0 },
            Unit::Imperial => { return (self.amount / ratio * 100.0).round() / 100.0 },
        }
    }
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "amount: {}, system: {}", self.amount, self.system)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_kg_to_lb() {
        // Arrange 
        let size = 25.0;
        let system = Unit::Metric;
        let expected = 55.12;

        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_kg_to_lb_rounding_up() {
        // Arrange 
        let size = 21.1;
        let system = Unit::Metric;
        let expected = 46.52; // 46.51754

        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_kg_to_lb_rounding_down() {
        // Arrange 
        let size = 22.5;
        let system = Unit::Metric;
        let expected = 49.60; // 49.60401

        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_lb_to_kg() {
        // Arrange 
        let size = 25.0;
        let system = Unit::Imperial;
        let expected = 11.34;
        
        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_lb_to_kg_rounding_up() {
        // Arrange 
        let size = 25.1;
        let system = Unit::Imperial;
        let expected = 11.39; // 11.38517
        
        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_lb_to_kg_rounding_down() {
        // Arrange 
        let size = 26.2;
        let system = Unit::Imperial;
        let expected = 11.88; // 11.88412
        
        // Act
        let unit = Weight::new(size, system);
        let result = unit.convert();

        // Assert
        assert_eq!(result, expected);
    }

}