mod utils;

use crate::utils::currency::Currency;
use crate::utils::length::Length;
use crate::utils::temperature::Temperature;
use crate::utils::units::CurrencyTypes;
use crate::utils::units::TemperatureType::{Celsius, Fahrenheit, Kelvin};
use crate::utils::units::WeightType::{Imperial, Metric};
use crate::utils::weight::Weight;

extern crate dotenv;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    divider("Length");
    length_run();

    divider("Weight");
    weight_run();

    divider("Temperature");
    temperature_run();

    // divider("Currency Exchange");
    // currency_run().await;
}

#[cfg(not(tarpaulin_include))]
/// Runs length related functions
fn length_run() {
    let metric_length = Length::new(123.14, Metric);
    let imperial_length = Length::new(48.48, Imperial);
    println!("{}", metric_length);
    println!("Converted to Imperial: {}", metric_length.convert());
    println!("{}", imperial_length);
    println!("Converted to Metric: {}", imperial_length.convert());
}

#[cfg(not(tarpaulin_include))]
/// Runs weight related functions
fn weight_run() {
    let metric_weight = Weight::new(25.0, Metric);
    let imperial_weight = Weight::new(2.0, Imperial);
    println!("{}", metric_weight);
    println!("Converted to Imperial: {}", metric_weight.convert());
    println!("{}", imperial_weight);
    println!("Converted to Metric: {}", imperial_weight.convert());
}

#[cfg(not(tarpaulin_include))]
/// Runs temperature related functions
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

#[cfg(not(tarpaulin_include))]
async fn currency_run() {
    let kroner = Currency::new(CurrencyTypes::DKK);
    let dollars = Currency::new(CurrencyTypes::USD);

    let dkk_eur = kroner.convert(100.).await;
    let usd_eur = dollars.convert(20.).await;

    println!("100 DKK to EUR is: {}", dkk_eur);
    println!("20 USD to EUR is: {}", usd_eur);
}

#[cfg(not(tarpaulin_include))]
/// Prints text in the middle of evenly spaced dashes
fn divider(text: &str) {
    let txt_ln = text.len();
    let base_ln = 20;
    let space = base_ln + txt_ln;
    println!("{:->space$}{:->base_ln$}", text, "-");
}
