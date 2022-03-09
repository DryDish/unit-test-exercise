#[derive(Debug)]
#[derive(PartialEq)]
/// Unit to define length unit.
/// Only supports Metric and Imperial
pub enum Unit {
    Metric,
    Imperial
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Unit::Metric => { write!(f, "Metric")},
            Unit::Imperial => { write!(f, "Imperial")}
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Temperature::Celsius => { write!(f, "Celsius")},
            Temperature::Fahrenheit => { write!(f, "Fahrenheit")},
            Temperature::Kelvin => { write!(f, "Kelvin")}
        }
    }
}