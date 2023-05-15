use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

const PLAID_BASE_URL: &str = "https://api.plaid.com";

#[derive(Debug, Serialize, Deserialize)]
struct PlaidTokenResponse {
    access_token: String,
    item_id: String,
    #[serde(rename = "error")]
    error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: String,
    amount: f64,
    merchant: String,
    // Additional transaction fields
}

async fn exchange_public_token(public_token: &str, client_id: &str, secret: &str) -> Result<PlaidTokenResponse, reqwest::Error> {
    let params = [
        ("client_id", client_id),
        ("secret", secret),
        ("public_token", public_token),
    ];

    let client = Client::new();
    let response = client.post(&format!("{}/item/public_token/exchange", PLAID_BASE_URL))
        .header(header::CONTENT_TYPE,
