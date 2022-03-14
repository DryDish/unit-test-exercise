use super::units::CurrencyTypes;
use dotenv::dotenv;
use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::env;

/// Object designed to contain currency.
/// Contains a currency code and a value
#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    code: CurrencyTypes,
    value: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    EUR: Currency,
}

/// Container for the returned json.
///
/// Sample json response
/// ```json
/// {
///   "meta": { // We discard this as we do not needed it
///     "last_updated_at": "2022-03-12T23:59:59Z"
///   },
///   "data": {
///     "EUR": {
///       "code": "EUR",
///       "value": 0.134417
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    data: Data,
}

impl Currency {
    /// Create a new object of type Currency.
    ///
    /// Must provide kind of currency.
    pub fn new(kind: CurrencyTypes) -> Self {
        return Self {
            code: kind,
            value: 0.0,
        };
    }

    /// Async function to call `app.currencyapi.com` to get the `amount` of base currency converted into euro.
    ///
    /// Usage:
    /// ```no_run
    /// let kroner = Currency::new(CurrencyTypes::DKK);
    /// // The response must be awaited
    /// let response = kroner.convert(100.00).await;
    /// 
    /// assert_eq!(13.44, response);
    /// 
    /// ```
    pub async fn convert(self, amount: f64) -> f64 {
        dotenv().ok();
        // Build request url
        let request_url = format!("https://api.currencyapi.com/v3/latest?apikey={apikey}&base_currency={base_currency}&currencies={currencies}",
                                            apikey = env::var("APIKEY").unwrap_or("".to_string()),
                                            base_currency = self.code.to_string(),
                                            currencies = CurrencyTypes::EUR);
        let client = reqwest::Client::new();

        // Query the endpoint
        let response = client
            .get(request_url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        // Return the value IF the response status is 200 - OK
        // Panic if not
        match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                match response.json::<APIResponse>().await {
                    Ok(parsed) => {
                        return parsed.data.EUR.value * amount;
                    }
                    Err(e) => {
                        panic!("Hm, the response didn't match the shape we expected. {}", e);
                    }
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                {
                    panic!("Need to grab a new token");
                };
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        };
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kind: {:?}", self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_print() {}
}
