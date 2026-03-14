use dotenv;
use serde::Deserialize;

use clap::{Command, arg};

#[derive(Deserialize)]
struct Stock {
    ticker: String,
    price: String,
    change_amount: String,
    change_percentage: String,
}

#[derive(Deserialize)]
struct TopGainersLosers {
    top_gainers: Vec<Stock>,
    top_losers: Vec<Stock>
}

#[derive(Deserialize)]
struct TickerInfo {
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[derive(Deserialize)]
struct Ticker {
    #[serde(rename = "Global Quote")]
    global_quote: TickerInfo,
}

fn main() {

    let matches = Command::new("Stocks CLI")
        .version("1.0")
        .arg(arg!(-t --top).required(false))
        .arg(arg!([ticker]))
        .get_matches();

    if matches.get_flag("top") {
        gainers_losers();
    } else if matches.get_one::<String>("ticker").is_some() {
        ticker_lookup(matches.get_one::<String>("ticker").expect("Invalid Ticker").to_string());
    }
}

fn gainers_losers() {
    println!("Top 10 Stock Gainers\n");

    let client = reqwest::blocking::Client::new();

    let top_gainers_losers: TopGainersLosers = client
        .get(format!("https://www.alphavantage.co/query?function=TOP_GAINERS_LOSERS&apikey={}", dotenv::var("ALPHAVANTAGEAPIKEY").unwrap()))
        .send()
        .expect("Failed to fetch top gainers & losers")
        .json()
        .expect("Failed to parse stocks");

    for (_i, gainer) in top_gainers_losers.top_gainers.iter().take(10).enumerate() {
        println!("{} - ${} (+${} / +{})\n", gainer.ticker, gainer.price, gainer.change_amount, gainer.change_percentage);
    }

    println!("\nTop 10 Stock Losers\n");

    for (_i, loser) in top_gainers_losers.top_losers.iter().take(10).enumerate() {
        println!("{} - ${} (-${} / {})\n", loser.ticker, loser.price, loser.change_amount.parse::<f32>().unwrap().abs().to_string(), loser.change_percentage);
    }
}

fn ticker_lookup(ticker:String) {
    let client = reqwest::blocking::Client::new();

    let ticker_lookup: Ticker = client
        .get(format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", ticker, dotenv::var("ALPHAVANTAGEAPIKEY").unwrap()))
        .send()
        .expect("Failed to fetch ticker")
        .json()
        .expect("Failed to parse ticker");

    println!("{} - ${} ({})", ticker, ticker_lookup.global_quote.price, ticker_lookup.global_quote.change_percent);
}