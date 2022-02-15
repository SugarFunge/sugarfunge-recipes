use requests::*;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use structs::*;

mod requests;
mod structs;

async fn print_header(title: &str, path: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.trim())
        .unwrap();
    writeln!(file, "\nRequest: {}\n", title)?;
    println!("\nRequest: {}\n", title);
    Ok(())
}

async fn send_request(request: &Request, path: String) -> Result<String, Box<dyn Error>> {
    let res;
    match request.endpoint.as_str() {
        "account/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_create(request.endpoint.as_str(), body).await?;
            print_header("Account Create", path).await?;
        }
        "account/fund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_fund(request.endpoint.as_str(), body).await?;
            print_header("Account Fund", path).await?;
        }
        "account/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_balance(request.endpoint.as_str(), body).await?;
            print_header("Account Balance", path).await?;
        }
        "asset/create_class" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create_class(request.endpoint.as_str(), body).await?;
            print_header("Asset Create Class", path).await?;
        }
        "asset/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create(request.endpoint.as_str(), body).await?;
            print_header("Asset Create", path).await?;
        }
        "asset/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_mint(request.endpoint.as_str(), body).await?;
            print_header("Asset Mint", path).await?;
        }
        "asset/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_balance(request.endpoint.as_str(), body).await?;
            print_header("Asset Balance", path).await?;
        }
        "asset/transfer_from" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_transfer(request.endpoint.as_str(), body).await?;
            print_header("Asset Transfer From", path).await?;
        }
        "currency/issue" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issue(request.endpoint.as_str(), body).await?;
            print_header("Currency Issue", path).await?;
        }
        "currency/issuance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issuance(request.endpoint.as_str(), body).await?;
            print_header("Currency Issuance", path).await?;
        }
        "currency/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_mint(request.endpoint.as_str(), body).await?;
            print_header("Currency Mint", path).await?;
        }
        "currency/burn" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_burn(request.endpoint.as_str(), body).await?;
            print_header("Currency Burn", path).await?;
        }
        "currency/supply" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_supply(request.endpoint.as_str(), body).await?;
            print_header("Currency Supply", path).await?;
        }
        "escrow/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_create(request.endpoint.as_str(), body).await?;
            print_header("Escrow Create", path).await?;
        }
        "escrow/deposit" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_deposit(request.endpoint.as_str(), body).await?;
            print_header("Escrow Deposit", path).await?;
        }
        "escrow/refund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_refund(request.endpoint.as_str(), body).await?;
            print_header("Escrow Refund", path).await?;
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
    let _ = &file_name.pop();
    let path = "test/".to_owned() + &file_name;
    File::create(path.clone() + ".txt").unwrap();

    let file = File::open((path.clone() + ".json").trim())?;
    let requests: Vec<Request> = serde_json::from_reader(file)?;
    let iterator = requests.iter();

    for request in iterator {
        let res = send_request(request, path.clone() + ".txt").await?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open((path.clone() + ".txt").trim())
            .unwrap();

        writeln!(file, "Response: {}", res)?;
        println!("Response: {}", res);
    }
    Ok(())
}
