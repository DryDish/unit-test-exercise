use super::{functions::round_to_two_decimals, units::CurrencyTypes};
use dotenv::dotenv;
use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
    Response,
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
        // Get response from API
        let response = query_api(self.code).await;

        // Process response and get the value of the currency
        let total = handle_result(response).await;

        // Multiply by the desired amount
        let total_multiplied = total * amount;

        // Return total, rounded to two decimals
        return round_to_two_decimals(total_multiplied);
    }
}

/// Function to query `app.currencyapi.com` and get the exchange rate of `CurrencyType`.
///
/// Returns a `Response` object
async fn query_api(code: CurrencyTypes) -> Response {
    dotenv().ok();
    // Build request url
    let request_url = format!("https://api.currencyapi.com/v3/latest?apikey={apikey}&base_currency={base_currency}&currencies={currencies}",
                                        apikey = env::var("APIKEY").unwrap_or("".to_string()),
                                        base_currency = code.to_string(),
                                        currencies = CurrencyTypes::EUR);

    // Client to make the request
    let client = reqwest::Client::new();

    // Query the endpoint
    let response_res = client
        .get(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await;

    // Unwrap the response
    // Panic if the response contains an error
    let response = match response_res {
        Ok(res) => res,
        Err(e) => panic!("Query failed: {}", e),
    };

    return response;
}

// Function designed to unwrap and parse a Response object
async fn handle_result(response: Response) -> f64 {
    let result = match response.status() {
        // Compare status codes, check for 200
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to the APIResponse struct
            let total = match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    // Returns the value of currency
                    parsed.data.EUR.value
                }
                Err(e) => {
                    panic!("Unable to parse Json to APIResponse. {}", e);
                }
            };
            total
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            {
                panic!("Need a new token");
            };
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    };
    return result;
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
