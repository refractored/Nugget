use crate::{get_config, Error};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde_json::Value;
use std::sync::OnceLock;

static URL: &str = "https://api.polymart.org/v1/";
static SERVICE: &str = "RefractoredDiscordVerification";
static NONCE: OnceLock<String> = OnceLock::new();

fn get_nonce() -> &'static String {
    NONCE.get_or_init(||{
        rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    })
}

pub struct GenerateUrlResponse {
    pub success: bool,
    pub url: Option<String>,
}

pub async fn generate_polymart_verify_url() -> Result<GenerateUrlResponse, Error> {
    let client = reqwest::Client::new();
    
    let params = [
        ("service", SERVICE),
        ("nonce", get_nonce())
    ];

    let response = client.get(URL.to_owned() + "generateUserVerifyURL")
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;
    let values: Value = serde_json::from_str(&body)?;

    let success: bool = values["response"]["success"].as_bool().unwrap_or(false);

    let url = values["response"]["result"]["url"].as_str().map(|s| s.to_owned());

    let data = GenerateUrlResponse { success, url};

    Ok(data)
}

pub struct VerifyUserResponse{
    pub success: bool,
    pub message: Option<String>,
    pub user: Option<String>,
}

pub async fn verify_user(token: &str) -> Result<VerifyUserResponse, Error> {
    let client = reqwest::Client::new();

    let params = [
        ("service", SERVICE),
        ("token", token),
        ("nonce", get_nonce())
    ];

    let response = client.get(URL.to_owned() + "verifyUser")
        .query(&params)
        .send()
        .await?;
    
    let body = response.text().await?;
    let value: Value = serde_json::from_str(&body)?;

    let data = VerifyUserResponse {
        success: value["response"]["success"].as_bool().unwrap_or_else(|| false),
        message: value["response"]["message"].as_str().map(|s| s.to_owned()),
        user: value["response"]["result"]["user"]["id"].as_str().map(|s| s.to_owned()),
    };

    Ok(data)
}

pub struct ResourceDataResponse {
    pub success: bool,
    pub purchase_valid: Option<bool>,
    pub purchase_status: Option<String>,
}

pub async fn get_resource_data(resource_id: &str, user_id: &str) -> Result<ResourceDataResponse, Error> {
    let client = reqwest::Client::new();

    let params = [
        ("service", SERVICE),
        ("token", get_config().config.api_key.as_str()),
        ("resource_id", resource_id),
        ("user_id", user_id),
        ("nonce", get_nonce())
    ];

    let response = client.get(URL.to_owned() + "getResourceUserData")
        .query(&params)
        .send()
        .await?;
    let body = response.text().await?;
    let value: Value = serde_json::from_str(&body)?;

    let data = ResourceDataResponse {
        success: value["response"]["success"].as_bool().unwrap_or_else(|| false),
        purchase_valid: value["response"]["resource"]["purchaseValid"].as_bool(),
        purchase_status: value["response"]["resource"]["purchaseStatus"].as_str().map(|s| s.to_owned()),
    };

    Ok(data)
}