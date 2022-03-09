pub fn round_to_two_decimals(number: f64) -> f64 {
    return (number * 100.0).round() / 100.0;
}
