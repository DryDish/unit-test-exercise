mod length;
mod temperature;
mod units;
mod utils;
mod weight;

use crate::length::Length;
use crate::temperature::Temperature;
use crate::units::TemperatureType::{Celsius, Fahrenheit, Kelvin};
use crate::units::WeightType::{Imperial, Metric};
use crate::weight::Weight;

fn main() {
    divider("Length");
    length_run();

    divider("Weight");
    weight_run();

    divider("Temperature");
    temperature_run();
}

/// Runs length related functions
fn length_run() {
    let metric_length = Length::new(123.14, Metric);
    let imperial_length = Length::new(48.48, Imperial);
    println!("{}", metric_length);
    println!("Converted to Imperial: {}", metric_length.convert());
    println!("{}", imperial_length);
    println!("Converted to Metric: {}", imperial_length.convert());
}

/// Runs weight related functions
fn weight_run() {
    let metric_weight = Weight::new(25.0, Metric);
    let imperial_weight = Weight::new(2.0, Imperial);
    println!("{}", metric_weight);
    println!("Converted to Imperial: {}", metric_weight.convert());
    println!("{}", imperial_weight);
    println!("Converted to Metric: {}", imperial_weight.convert());
}

fn temperature_run() {
    let celsius_temp = Temperature::new(0.0, Celsius);
    let fahrenheit_temp = Temperature::new(0.0, Fahrenheit);
    let kelvin_temp = Temperature::new(0.0, Kelvin);

    println!("{}", celsius_temp);
    println!(
        "Converted to Fahrenheit: {}",
        celsius_temp.convert(Fahrenheit)
    );
    println!("Converted to Kelvin: {}", celsius_temp.convert(Kelvin));

    println!("{}", fahrenheit_temp);
    println!("Converted to Celsius: {}", fahrenheit_temp.convert(Celsius));
    println!("Converted to Kelvin: {}", fahrenheit_temp.convert(Kelvin));

    println!("{}", kelvin_temp);
    println!("Converted to Celsius: {}", kelvin_temp.convert(Celsius));
    println!(
        "Converted to Fahrenheit: {}",
        kelvin_temp.convert(Fahrenheit)
    );
}

/// Prints text in the middle of evenly spaced dashes
fn divider(text: &str) {
    let txt_ln = text.len();
    let base_ln = 20;
    let space = base_ln + txt_ln;
    println!("{:->space$}{:->base_ln$}", text, "-");
}
