extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate twitter_api;
extern crate oauth_client as oauth;

use std::time;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

mod crawler;


pub fn build_message (prices : &mut HashMap<&str, crawler::CurrencyPrice>) -> String {
    let mut s = String::from("");
    for (cur, value) in prices.iter() {
        let t = format!("#{}: ${} ${}€\n",cur, value.USD, value.EUR);
        s = s + &t[..];
    }

    return s;
}

fn main() {

    let crawler = crawler::Crawler::new();

    //let mut prices : HashMap<&str,crawler::CurrencyPrice>= HashMap::new();
    let mut prices = HashMap::new();

    
    for currency in ["BTC","LTC","ETH", "DASH", "ZEC", "XRP"] {
        // Get the price of the currency
        let c = &currency[..];
        let price = crawler.get_price(c);
        prices.insert(c, price);
    }
    
    let msg = build_message(&mut prices);

    println!(msg);
        
    
}