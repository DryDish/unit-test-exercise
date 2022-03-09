mod length;
use crate::length::{Length, Unit};


fn main() {
    let unit = Length::new(123.13, Unit::Metric);
    println!("{}", unit);
    println!("{}", unit.convert());
}
