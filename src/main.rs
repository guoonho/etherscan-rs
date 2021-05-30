use std::env;
use std::process;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

struct Options {
    command: String,
    wallet: String,
    apikey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let options = validate(&args);

    match options.command.as_str() {
        "balance" => {
            balance(options.wallet, options.apikey).await?;
        }
        "price" => {
            price(options.apikey).await?;
        }
        _ => {
            println!("Command not found. Exiting...");
            process::exit(1);
        }
    }

    Ok(())
}

async fn balance(wallet: String, apikey: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.etherscan.io/api?module=account&action=balance&address={}&tag=latest&apikey={}", wallet, apikey);
    let body = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    if body["message"].as_str() != "OK" {
        println!("Could not get balance.");
    } else {
        let bal = body["result"].parse::<f64>().unwrap() / 1000000000000000000.0;
        println!("{}", bal);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceResult {
    status: String,
    message: String,
    result: HashMap<String, String>,
}

async fn price(apikey: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.etherscan.io/api?module=stats&action=ethprice&apikey={}", apikey);
    let result: PriceResult = reqwest::get(url)
        .await?
        .json()
        .await?;

    println!("{}", result.result["ethusd"]);

    Ok(())
}


fn validate(args: &[String]) -> Options {
    let mut apikey = String::new();
    let mut wallet = String::new();

    if args.len() < 2 {
        println!("You need to pass in a command: balance or price.");
        process::exit(1);
    }

    if args.len() < 4 {
        apikey = match env::var("ETHERSCAN_APIKEY") {
            Ok(val) => val,
            Err(_e) => String::from("none"),
        };
    
        wallet = match env::var("ETHERSCAN_WALLET") {
            Ok(val) => val,
            Err(_e) => String::from("none"),
        };
    } else {
        apikey = args[2].clone();
        wallet = args[3].clone();
    }

    let command = args[1].clone();

    if apikey == "none" {
        println!("ETHERSCAN_APIKEY env var not found or api key not given. Exiting...");
        process::exit(1);
    }

    if wallet == "none" {
        println!("ETHERSCAN_WALLET env var not found or api key not given. Exiting...");
        process::exit(1);
    }

    Options { command, wallet, apikey } 
}

