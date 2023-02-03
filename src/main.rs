use std::{fmt::Error, process};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
const POLYGON_ENDPOINT: &str = "https://polygon.api.0x.org/swap/v1/quote?";
const POLYGON_ADDRESS: &str = "0xdef1c0ded9bec7f1a1670819833240f027b25eff";

#[derive(Serialize, Deserialize, Debug)]
struct OxSwap {
    sell_token: String,
    buy_token: String,
    sell_amount: String,
    query: String,
}

#[derive(Debug)]
struct OxQuote {
    price: String,
    calldata: String,
    to: String,
}

#[derive(Debug)]
enum OxQuoteError {
    ApiResponseError(reqwest::Error),
    ApiStatusError(StatusCode),
    JsonParsingError(reqwest::Error),
    MissingField,
}

impl OxSwap {
    pub fn new(sell_token: &str, buy_token: &str, sell_amount: &str) -> OxSwap {
        return OxSwap {
            sell_token: sell_token.to_string(),
            buy_token: buy_token.to_string(),
            sell_amount: sell_amount.to_string(),
            query: format!("sellToken={sell_token}&buyToken={buy_token}&sellAmount={sell_amount}"),
        };
    }

    pub fn quote(&self) -> Result<OxQuote, OxQuoteError> {
        let query = format!("{POLYGON_ENDPOINT}{}", self.query);
        let res = match reqwest::blocking::get(query) {
            Ok(res) => res,
            Err(error) => return Err(OxQuoteError::ApiResponseError(error)),
        };

        if res.status() != StatusCode::OK {
            return Err(OxQuoteError::ApiStatusError(res.status()));
        }

        let res: Value = match res.json() {
            Ok(res) => res,
            Err(error) => return Err(OxQuoteError::JsonParsingError(error)),
        };

        let price = match res["price"].as_str() {
            Some(price) => price,
            None => return Err(OxQuoteError::MissingField),
        };
        let calldata = match res["data"].as_str() {
            Some(data) => data,
            None => return Err(OxQuoteError::MissingField),
        };
        let to = match res["to"].as_str() {
            Some(to) => to,
            None => return Err(OxQuoteError::MissingField),
        };

        Ok(OxQuote {
            price: price.to_string(),
            calldata: calldata.to_string(),
            to: to.to_string(),
        })
    }
}

fn main() {
    let params = OxSwap::new("DAIII", "WETH", "1000000000000000000");

    let quote = match params.quote() {
        Ok(quote) => quote,
        Err(error) => {
            eprintln!("Error while quoting 0x API: {:?}", error);
            process::exit(1);
        }
    };

    println!("{:?}", quote);
}
