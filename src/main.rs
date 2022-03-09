mod length;
mod units;
mod weight;
mod temperature;

use crate::units::WeightType;
use crate::units::TemperatureType;
use crate::length::Length;
use crate::weight::Weight;


fn main() {
    divider("Length");
    length_run();

    divider("Weight");
    weight_run();
}

/// Runs length related functions
fn length_run() {
    let metric_length = Length::new(123.14, WeightType::Metric);
    let imperial_length = Length::new(48.48, WeightType::Imperial);
    println!("{}", metric_length);
    println!("Converted to Imperial: {}", metric_length.convert());
    println!("{}", imperial_length);
    println!("Converted to Metric: {}", imperial_length.convert());
}

/// Runs weight related functions
fn weight_run() {
    let metric_weight = Weight::new(25.0, WeightType::Metric);
    let imperial_weight = Weight::new(2.0, WeightType::Imperial);
    println!("{}", metric_weight);
    println!("Converted to Imperial: {}", metric_weight.convert());
    println!("{}", imperial_weight);
    println!("Converted to Metric: {}", imperial_weight.convert());
}

/// Prints text in the middle of evenly spaced dashes
fn divider(text: &str){
    let txt_ln = text.len();
    let base_ln = 20;
    let space = base_ln + txt_ln;
    println!("{:->space$}{:->base_ln$}", text, "-");
}
