use serde::{ Serialize, Deserialize };
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenIdConfig {
    pub registration_endpoint: String,
    pub request_object_signing_alg_values_supported: Vec<String>,
    pub request_parameter_supported: bool,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub token_endpoint: String,
    pub revocation_endpoint: String
}

impl OpenIdConfig {
    async fn from_well_known(uri: &str) -> Self {
        reqwest::get(uri)
            .await.expect("Failed openid config HTTP GET")
            .json::<OpenIdConfig>()
            .await.expect("Failed decoding openid config")
    }
}