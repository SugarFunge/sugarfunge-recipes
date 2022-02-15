use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub endpoint: String,
    pub body: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCreateResponse {
    pub seed: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundResponse {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundBody {
    pub seed: String,
    pub amount: u128,
    pub to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceBody {
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceResponse {
    pub balance: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub userdata: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateClassBody {
    pub seed: String,
    pub metadata: MetaData,
    pub class_id: u16,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateClassResponse {
    pub class_id: u16,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateBody {
    pub seed: String,
    pub account: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub metadata: MetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateResponse {
    pub class_id: u32,
    pub asset_id: u32,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenMintBody {
    pub seed: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenMintResponse {
    pub to: String,
    pub class_id: u32,
    pub asset_id: u32,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBalanceBody {
    pub account: String,
    pub class_id: u16,
    pub asset_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBalanceResponse {
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenTransferBody {
    pub seed: String,
    pub from: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenTransferResponse {
    pub from: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub class_id: u16,
    pub asset_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssueBody {
    pub seed: String,
    pub to: String,
    pub currency: Currency,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssueResponse {
    pub currency: Currency,
    pub who: String,
    pub amount: u128,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssuanceBody {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssuanceResponse {
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyMintBody {
    pub amount: u128,
    pub currency: Currency,
    pub seed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyMintResponse {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyBurnBody {
    pub amount: u128,
    pub currency: Currency,
    pub seed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyBurnResponse {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencySupplyBody {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencySupplyResponse {
    pub total_supply: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowCreateBody {
    pub seed: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowCreateResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowDepositBody {
    pub seed: String,
    pub escrow: String,
    pub class_id: u16,
    pub asset_ids: Vec<u16>,
    pub amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowDepositResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowRefundBody {
    pub seed: String,
    pub escrow: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowRefundResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}
