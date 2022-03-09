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