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

async fn exchange_public_token(public_token: &str, client_id: &str, secret: &str) -> Result<PlaidTokenResponse, reqwest::Error> {
    let params = [
        ("client_id", client_id),
        ("secret", secret),
        ("public_token", public_token),
    ];

    let client = Client::new();
    let response = client.post(&format!("{}/item/public_token/exchange", PLAID_BASE_URL))
        .header(header::CONTENT_TYPE, "application/json")
        .form(&params)
        .send()
        .await?
        .json::<PlaidTokenResponse>()
        .await?;

    Ok(response)
}

async fn main() {
    let public_token = "your_public_token";
    let client_id = "your_client_id";
    let secret = "your_secret";

    match exchange_public_token(public_token, client_id, secret).await {
        Ok(response) => {
            if let Some(error_message) = response.error_message {
                println!("Plaid token exchange error: {}", error_message);
            } else {
                println!("Access Token: {}", response.access_token);
                println!("Item ID: {}", response.item_id);
                // Perform further operations with the access token
                // such as retrieving account details, transactions, etc.
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
