//
// main.rs
//
// MIT License
// Copyright 2024 iAchieved.it LLC
//

mod schwab;

use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::process;

#[derive(Deserialize)]
struct BuyLowConfig {
  maximum_amount: f64,
  equities: Vec<String>
}

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Equity {

  #[tabled(rename = "Equity")]
  symbol: String,
  
  #[tabled(rename = "Starting Price")]
  #[tabled(display_with = "display_dollars")]
  starting_price: f64,

  #[tabled(rename = "Ending Price")]
  #[tabled(display_with = "display_dollars")]
  ending_price:  f64,

  #[tabled(rename = "Percent Change")]
  #[tabled(display_with = "display_percent_change")]
  percent_change: f64
}

fn display_dollars(dollars: &f64) -> String {
  format!("${:.2}", dollars)
}

fn display_percent_change(percent_change: &f64) -> String {
  format!("{:.2}%", percent_change)
}

fn find_worst_performance(previous_prices: HashMap<&String, f64>,
                          current_prices: &HashMap<String, f64>) -> 
                          (String, Vec<Equity>) {

  let mut equities = Vec::new();
  let mut worst_equity:String = String::new();
  let mut worst_performance:f64 = 1000.0; // nice problem to have

  for (equity, previous_price) in previous_prices {
    let current_price = current_prices.get(equity).unwrap();
    let performance = (current_price - previous_price) / previous_price;

    //println!("{:>4}:  {:.2}%", equity, performance * 100.0);

    equities.push(Equity {
      symbol: equity.to_string(),
      starting_price: previous_price,
      ending_price: *current_price,
      percent_change: performance * 100.0
    });

    if performance < worst_performance {
      worst_equity = equity.to_string();
      worst_performance = performance;
    }
  }

  return (worst_equity, equities);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action)]
    live: bool
}


fn main() {

  let args = Args::parse();

  if !args.live {
    println!("Running in test mode, no orders will be placed.");
  }


  let path = std::path::Path::new("buy_low.toml");
  let file = std::fs::read_to_string(path).unwrap();
  let config:BuyLowConfig = toml::from_str(&file).unwrap();

  schwab::get_access_token();
  schwab::get_account_number();
  let cash_balance = schwab::get_cash_balance();

  let mut previous_prices = HashMap::new();
  for equity in &config.equities {
    previous_prices.insert(equity, schwab::get_price_history(&equity));
  }

  let current_prices = schwab::get_current_price(&config.equities);

  

  let (worst_equity, mut equities) = find_worst_performance(previous_prices,
                                                        &current_prices);

  equities.sort_by(|a, b| a.percent_change.partial_cmp(&b.percent_change).unwrap_or(Ordering::Equal));

  let table = Table::new(equities).to_string();

  println!("{}", table);

  print!("Worst performing equity: ");
  println!("{}", worst_equity);

  let max_amount = config.maximum_amount;
  let current_price = current_prices.get(&worst_equity).unwrap();
  let whole_shares = (max_amount / current_price) as u32;

  println!("Maximum amount to spend: ${}", max_amount);
  println!("Maximum whole shares of {} to purchase: {}", worst_equity,
           whole_shares);

  if max_amount > cash_balance {
    println!("Insufficient cash balance (${}) to make the purchase.",
             cash_balance);

    process::exit(-1);
  } else {
    println!("Current cash balance: ${}", cash_balance);
  }

  if args.live {
    schwab::create_order(&worst_equity, whole_shares);

    #[cfg(feature = "use_postgres")]
    schwab::insert_order();

  } else {
    println!("Test mode, otherwise {} shares of {} would be purchased.", 
    whole_shares, worst_equity);

    println!("If you're ready, run with --live");
  }

  println!("Done!");
}
