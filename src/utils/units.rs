#[derive(Debug, PartialEq)]
/// Unit to define length unit.
/// Only supports Metric and Imperial
pub enum WeightType {
    Metric,
    Imperial,
}

#[derive(Debug, PartialEq)]
pub enum TemperatureType {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for WeightType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WeightType::Metric => {
                write!(f, "Metric")
            }
            WeightType::Imperial => {
                write!(f, "Imperial")
            }
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for TemperatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemperatureType::Celsius => {
                write!(f, "Celsius")
            }
            TemperatureType::Fahrenheit => {
                write!(f, "Fahrenheit")
            }
            TemperatureType::Kelvin => {
                write!(f, "Kelvin")
            }
        }
    }
}
