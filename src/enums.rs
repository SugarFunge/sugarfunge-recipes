use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub userdata: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub class_id: u16,
    pub asset_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Body {
    AccountCreateBody(String),
    AccountFundBody {
        seed: String,
        amount: u128,
        to: String,
    },
    AccountBalanceBody {
        account: String,
    },
    TokenCreateClassBody {
        seed: String,
        metadata: MetaData,
        class_id: u16,
        owner: String,
    },
    TokenCreateBody {
        seed: String,
        account: String,
        class_id: u16,
        asset_id: u16,
        metadata: MetaData,
    },
    TokenMintBody {
        seed: String,
        to: String,
        class_id: u16,
        asset_id: u16,
        amount: u128,
    },
    TokenBalanceBody {
        account: String,
        class_id: u16,
        asset_id: u16,
    },
    TokenTransferBody {
        seed: String,
        from: String,
        to: String,
        class_id: u16,
        asset_id: u16,
        amount: u128,
    },
    CurrencyIssueBody {
        seed: String,
        to: String,
        currency: Currency,
        amount: u128,
    },
    CurrencyIssuanceBody {
        currency: Currency,
    },
    CurrencyMintBody {
        amount: u128,
        currency: Currency,
        seed: String,
    },
    CurrencyBurnBody {
        amount: u128,
        currency: Currency,
        seed: String,
    },
    CurrencySupplyBody {
        currency: Currency,
    },
    EscrowCreateBody {
        seed: String,
        owner: String,
    },
    EscrowDepositBody {
        seed: String,
        escrow: String,
        class_id: u16,
        asset_ids: Vec<u16>,
        amounts: Vec<u128>,
    },
    EscrowRefundBody {
        seed: String,
        escrow: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    AccountCreateResponse {
        seed: String,
        account: String,
    },
    AccountFundResponse {
        from: String,
        to: String,
        amount: u128,
    },
    AccountBalanceResponse {
        balance: u128,
    },
    TokenCreateClassResponse {
        class_id: u16,
        who: String,
    },
    TokenCreateResponse {
        class_id: u32,
        asset_id: u32,
        who: String,
    },
    TokenMintResponse {
        to: String,
        class_id: u32,
        asset_id: u32,
        amount: u128,
        who: String,
    },
    TokenBalanceResponse {
        amount: u128,
    },
    TokenTransferResponse {
        from: String,
        to: String,
        class_id: u16,
        asset_id: u16,
        amount: u128,
        who: String,
    },
    CurrencyIssueResponse {
        currency: Currency,
        who: String,
        amount: u128,
    },
    CurrencyIssuanceResponse {
        amount: u128,
    },
    CurrencyMintResponse {
        currency: Currency,
        amount: u128,
        who: String,
    },
    CurrencyBurnResponse {
        currency: Currency,
        amount: u128,
        who: String,
    },
    CurrencySupplyResponse {
        total_supply: u128,
    },
    EscrowCreateResponse {
        escrow: String,
        operator: String,
        owner: String,
    },
    EscrowDepositResponse {
        escrow: String,
        operator: String,
        owner: String,
    },
    EscrowRefundResponse {
        escrow: String,
        operator: String,
        owner: String,
    },
}
