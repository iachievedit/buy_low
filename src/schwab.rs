//
// schwab.rs
//
// MIT License
// Copyright 2024 iAchieved.it LLC
//
use dotenv::dotenv;
use std::env;

use std::collections::HashMap;
use reqwest::header;
use reqwest::blocking::Client;
use serde::Deserialize;

use serde_json::{Value};



#[derive(Deserialize)]
struct SchwabClient {
  access_token: String
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SchwabAccount {
  hashValue: String
}

#[cfg(feature = "use_postgres")]
use postgres::{Client as PostgresClient, NoTls};
#[cfg(feature = "use_postgres")]
use postgres_types::{ToSql, FromSql};

#[cfg(feature = "use_postgres")]
#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "asset_type")]
enum AssetType {
  #[postgres(name = "EQUITY")]
  Equity,
}

#[cfg(feature = "use_postgres")]
#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "instruction")]
enum Instruction {
  #[postgres(name = "BUY")]
  Buy,
  #[postgres(name = "SELL")]
  Sell
}

static mut ACCESS_TOKEN:String = String::new();
static mut ACCOUNT_HASH:String = String::new();

pub fn get_access_token() {
  dotenv().ok();

  let refresh_token = env::var("SCHWAB_REFRESH_TOKEN").unwrap();
  let app_id = env::var("SCHWAB_APP_KEY").unwrap();
  let app_secret = env::var("SCHWAB_APP_SECRET").unwrap();

  let mut headers = header::HeaderMap::new();
  headers.insert("Content-Type",
  header::HeaderValue::from_static("application/x-www-form-urlencoded"));

  let mut params = HashMap::new();
  params.insert("grant_type", "refresh_token");
  params.insert("refresh_token", refresh_token.as_str());

  let client = Client::new();

  let res = client.post("https://api.schwabapi.com/v1/oauth/token")
  .headers(headers)
  .basic_auth(app_id, Some(app_secret))
  .form(&params)
  .send();

  if let Ok(response) = res {
    if response.status() == reqwest::StatusCode::OK {
      let schwab:SchwabClient = response.json().unwrap();
      unsafe {
         ACCESS_TOKEN = schwab.access_token;
      }
    } else {
      panic!("Error getting access token");
    }
  } else {
    panic!("Error getting access token");
  }

}

pub fn get_account_number() {
  let mut headers = header::HeaderMap::new();
  unsafe {
    headers.insert("Authorization",
    header::HeaderValue::from_str(format!("Bearer {}", ACCESS_TOKEN).as_str()).unwrap());
  }

  let client = Client::new();

  let res = client.get("https://api.schwabapi.com/trader/v1/accounts/accountNumbers")
  .headers(headers)
  .send();

  if let Ok(response) = res {
    if response.status() == reqwest::StatusCode::OK {
      let schwab:Vec<SchwabAccount> = response.json().unwrap();
  
      unsafe {
        ACCOUNT_HASH = schwab[0].hashValue.clone();
      }


    } else {
      panic!("Error getting account number");
    }
  } else {
    panic!("Error getting account number");
  }

}

pub fn get_cash_balance() -> f64 {
  let mut headers = header::HeaderMap::new();
  unsafe {
    headers.insert("Authorization",
    header::HeaderValue::from_str(format!("Bearer {}", ACCESS_TOKEN).as_str()).unwrap());
  }

  let client = Client::new();

  unsafe {
  let account_url = format!("https://api.schwabapi.com/trader/v1/accounts/{}", ACCOUNT_HASH);

  let res = client.get(account_url.as_str())
  .headers(headers)
  .send();
  
  if let Ok(response) = res {
    let json:Value = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let cash_balance = json["securitiesAccount"]["currentBalances"]["cashBalance"].clone();

    return cash_balance.as_f64().unwrap();
  } else {
    panic!("Error getting cash balance");
  }
}

}

pub fn get_price_history(symbol:&str) -> f64 {

  let mut headers = header::HeaderMap::new();
  unsafe {
    headers.insert("Authorization",
    header::HeaderValue::from_str(format!("Bearer {}", ACCESS_TOKEN).as_str()).unwrap());
  }

  let mut params = HashMap::new();
  params.insert("symbol", symbol);
  params.insert("periodType", "month");
  params.insert("frequencyType", "daily");

  let client = Client::new();

  let res = client.get("https://api.schwabapi.com/marketdata/v1/pricehistory")
  .headers(headers)
  .query(&params)
  .send();

  if let Ok(response) = res {

    let json:Value = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let month_ago = json["candles"][0]["close"].clone();

    return month_ago.as_f64().unwrap();
  } else {
    panic!("Error getting price history");
  }
}

pub fn get_current_price(symbols:&Vec<String>) -> HashMap<String, f64> {

  let mut prices = HashMap::new();

  let mut headers = header::HeaderMap::new();
  unsafe {
    headers.insert("Authorization",
    header::HeaderValue::from_str(format!("Bearer {}", ACCESS_TOKEN).as_str()).unwrap());
  }

  let mut params = HashMap::new();

  let symbols_str = symbols.join(",");
  params.insert("symbols", symbols_str);

  let client = Client::new();

  let res = client.get("https://api.schwabapi.com/marketdata/v1/quotes")
  .headers(headers)
  .query(&params)
  .send();

  if let Ok(response) = res {
    let json:Value = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    for (key, value) in json.as_object().unwrap() {
      prices.insert(key.clone(), value["quote"]["mark"].as_f64().unwrap());
    }

    return prices;
  } else {
    panic!("Error getting current price");
  }
}

pub fn create_order(symbol:&str, quantity:u32) {

  println!("Creating order for {} with quantity {}", symbol, quantity);

  let mut headers = header::HeaderMap::new();
  unsafe {
    headers.insert("Authorization",
    header::HeaderValue::from_str(format!("Bearer {}", ACCESS_TOKEN).as_str()).unwrap());
  }
  headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));


  let client = Client::new();

  unsafe {
    let account_url = 
    format!("https://api.schwabapi.com/trader/v1/accounts/{}/orders",
    ACCOUNT_HASH);
    
    println!("{}", account_url);

    let order_body = serde_json::json!({
      "orderType": "MARKET",
      "session": "NORMAL",
      "duration": "DAY",
      "orderStrategyType": "SINGLE",
      "orderLegCollection": [
        {
          "instruction": "BUY",
          "quantity": quantity,
          "instrument": {
            "symbol": symbol,
            "assetType": "EQUITY"
          }
        }
      ]
    });

    let res = client.post(account_url.as_str())
    .headers(headers)
    .body(order_body.to_string())
    .send();
  
  if let Ok(response) = res {
    println!("{:?}", response);
  } else {
    panic!("Error getting cash balance");
  }
}

}

// Postgres features
#[cfg(feature = "use_postgres")]
pub fn insert_order() {

  dotenv().ok();

  let postgres_conn_string = env::var("POSTGRES_CONN_STRING").unwrap();

  let mut client = PostgresClient::connect(postgres_conn_string.as_str(), NoTls).unwrap();

  client.execute(
    r#"INSERT INTO orders
     (session, duration, order_type, order_strategy_type, symbol, asset_type, instruction, quantity)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
    &[&"NORMAL", &"DAY", &"MARKET", &"SINGLE", &"QQQ", &AssetType::Equity, &Instruction::Buy, &2],
  ).unwrap();

}