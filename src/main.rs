mod length;
use crate::length::{Length, Unit};


fn main() {
    length_run();
}

fn length_run() {
    let metric_length = Length::new(123.13, Unit::Metric);
    let imperial_length = Length::new(12.1, Unit::Imperial);
    println!("{}", metric_length);
    println!("Converted to Imperial: {}", metric_length.convert());
    println!("{}", imperial_length);
    println!("Converted to Metric: {}", imperial_length.convert());
}