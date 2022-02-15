use requests::*;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use structs::*;

mod requests;
mod structs;

async fn print_header(title: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("test/results.txt")
        .unwrap();
    writeln!(file, "\nRequest: {}\n", title)?;
    println!("\nRequest: {}\n", title);
    Ok(())
}

async fn send_request(request: &Request) -> Result<String, Box<dyn Error>> {
    let res;
    match request.endpoint.as_str() {
        "account/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_create(request.endpoint.as_str(), body).await?;
            print_header("Account Create").await?;
        }
        "account/fund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_fund(request.endpoint.as_str(), body).await?;
            print_header("Account Fund").await?;
        }
        "account/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_balance(request.endpoint.as_str(), body).await?;
            print_header("Account Balance").await?;
        }
        "asset/create_class" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create_class(request.endpoint.as_str(), body).await?;
            print_header("Asset Create Class").await?;
        }
        "asset/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create(request.endpoint.as_str(), body).await?;
            print_header("Asset Create").await?;
        }
        "asset/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_mint(request.endpoint.as_str(), body).await?;
            print_header("Asset Mint").await?;
        }
        "asset/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_balance(request.endpoint.as_str(), body).await?;
            print_header("Asset Balance").await?;
        }
        "asset/transfer_from" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_transfer(request.endpoint.as_str(), body).await?;
            print_header("Asset Transfer From").await?;
        }
        "currency/issue" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issue(request.endpoint.as_str(), body).await?;
            print_header("Currency Issue").await?;
        }
        "currency/issuance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issuance(request.endpoint.as_str(), body).await?;
            print_header("Currency Issuance").await?;
        }
        "currency/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_mint(request.endpoint.as_str(), body).await?;
            print_header("Currency Mint").await?;
        }
        "currency/burn" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_burn(request.endpoint.as_str(), body).await?;
            print_header("Currency Burn").await?;
        }
        "currency/supply" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_supply(request.endpoint.as_str(), body).await?;
            print_header("Currency Supply").await?;
        }
        "escrow/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_create(request.endpoint.as_str(), body).await?;
            print_header("Escrow Create").await?;
        }
        "escrow/deposit" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_deposit(request.endpoint.as_str(), body).await?;
            print_header("Escrow Deposit").await?;
        }
        "escrow/refund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_refund(request.endpoint.as_str(), body).await?;
            print_header("Escrow Refund").await?;
        }
        _ => res = "Este endpoint no existe".to_string(),
    }
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Introduce el nombre del archivo: ");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read file name");

    File::create("test/results.txt").unwrap();

    let file = File::open(("test/".to_owned() + &file_name).trim())?;
    let requests: Vec<Request> = serde_json::from_reader(file)?;
    let iterator = requests.iter();

    for request in iterator {
        let res = send_request(request).await?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("test/results.txt")
            .unwrap();

        writeln!(file, "Response: {}", res)?;
        println!("Response: {}", res);
    }
    Ok(())
}
