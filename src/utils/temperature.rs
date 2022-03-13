use crate::utils::units::TemperatureType::{self, Celsius, Fahrenheit, Kelvin};
use crate::utils::functions::round_to_two_decimals;

pub struct Temperature {
    gradient: f64,
    system: TemperatureType,
}

impl Temperature {
    /// Create a new object of type Weight.
    ///
    /// Must provide the amount of weight and the unit.
    pub fn new(gradient: f64, system: TemperatureType) -> Self {
        Self { gradient, system }
    }
    /// Convert from the current unit to the other.
    ///
    /// For example from Metric to Imperial
    ///
    /// Usage:
    /// ```
    /// let metric_unit = Temperature::new(25.0, TemperatureType::Celsius);
    /// let converted_unit = metric_unit.convert();
    ///
    /// assert_eq!(converted_unit, 55,12);
    /// ```
    pub fn convert(&self, other: TemperatureType) -> f64 {
        match (&self.system, other) {
            (Celsius, Fahrenheit) => {
                return convert_celsius_to_fahrenheit(&self.gradient);
            }
            (Celsius, Kelvin) => {
                return convert_celsius_to_kelvin(&self.gradient);
            }
            (Fahrenheit, Celsius) => {
                return convert_fahrenheit_to_celsius(&self.gradient);
            }
            (Fahrenheit, Kelvin) => {
                return convert_fahrenheit_to_kelvin(&self.gradient);
            }
            (Kelvin, Celsius) => {
                return convert_kelvin_to_celsius(&self.gradient);
            }
            (Kelvin, Fahrenheit) => {
                return convert_kelvin_to_fahrenheit(&self.gradient);
            }
            // this is for the pairings (k-k, c-c, f-f)
            // returns current gradient as no conversion is needed
            (_, _) => return self.gradient,
        }
    }
}

fn convert_celsius_to_fahrenheit(celsius: &f64) -> f64 {
    return round_to_two_decimals((celsius * 1.8) + 32.);
}

fn convert_celsius_to_kelvin(celsius: &f64) -> f64 {
    return round_to_two_decimals(celsius + 273.15);
}

fn convert_fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    return round_to_two_decimals((fahrenheit - 32.) / 1.8);
}

/// This first converts to Celsius and then to Kelvin, lol
fn convert_fahrenheit_to_kelvin(fahrenheit: &f64) -> f64 {
    return round_to_two_decimals((fahrenheit - 32.) / 1.8 + 273.15);
}

fn convert_kelvin_to_celsius(kelvin: &f64) -> f64 {
    return round_to_two_decimals(kelvin - 273.15);
}

fn convert_kelvin_to_fahrenheit(kelvin: &f64) -> f64 {
    return round_to_two_decimals(kelvin * 1.8 - 459.67);
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "system: {}, gradient: {}", self.system, self.gradient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_celsius_to_fahrenheit() {
        // Arrange
        let gradient = 25.0;
        let expected = 77.0;

        // Act
        let temperature = Temperature::new(gradient, Celsius);
        let result = temperature.convert(Fahrenheit);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_celsius_to_kelvin() {
        // Arrange
        let gradient = 25.0;
        let expected = 298.15;

        // Act
        let temperature = Temperature::new(gradient, Celsius);
        let result = temperature.convert(Kelvin);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_fahrenheit_to_celsius() {
        // Arrange
        let gradient = 25.0;
        let expected = -3.89; // -3.888889

        // Act
        let temperature = Temperature::new(gradient, Fahrenheit);
        let result = temperature.convert(Celsius);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_fahrenheit_to_kelvin() {
        // Arrange
        let gradient = 25.0;
        let expected = 269.26; // 269.2611

        // Act
        let temperature = Temperature::new(gradient, Fahrenheit);
        let result = temperature.convert(Kelvin);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_kelvin_to_celsius() {
        // Arrange
        let gradient = 25.0;
        let expected = -248.15; // -248.15

        // Act
        let temperature = Temperature::new(gradient, Kelvin);
        let result = temperature.convert(Celsius);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_kelvin_to_fahrenheit() {
        // Arrange
        let gradient = 25.0;
        let expected = -414.67; // -414.67

        // Act
        let temperature = Temperature::new(gradient, Kelvin);
        let result = temperature.convert(Fahrenheit);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_celsius_to_celsius() {
        // Arrange
        let gradient = 25.0;
        let expected = 25.0;

        // Act
        let temperature = Temperature::new(gradient, Celsius);
        let result = temperature.convert(Celsius);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_fahrenheit_to_fahrenheit() {
        // Arrange
        let gradient = 75.0;
        let expected = 75.0;

        // Act
        let temperature = Temperature::new(gradient, Fahrenheit);
        let result = temperature.convert(Fahrenheit);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_kelvin_to_kelvin() {
        // Arrange
        let gradient = 100.0;
        let expected = 100.0;

        // Act
        let temperature = Temperature::new(gradient, Kelvin);
        let result = temperature.convert(Kelvin);

        // Assert
        assert_eq!(result, expected);
    }
}
