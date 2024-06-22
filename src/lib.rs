use std::env;

use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use thiserror::Error;

pub mod api;
pub mod models;

#[derive(Debug, Error)]
pub enum NomadError {
    #[error(transparent)]
    InvalidRequest(serde_json::error::Error),
    #[error(transparent)]
    RequestError(reqwest::Error),
    #[error(transparent)]
    ResponseError(reqwest::Error),
    #[error(transparent)]
    InvalidResponse(reqwest::Error),
    #[error("Status code: {0}, body: {1}")]
    UnexpectedResponseCode(reqwest::StatusCode, String),
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub token: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        let addr = env::var("NOMAD_ADDR").unwrap_or_else(|_| "http://127.0.0.1:4646".to_string());
        let token = env::var("NOMAD_TOKEN").unwrap_or_default();

        Config {
            address: addr,
            token: Some(token),
        }
    }
}

pub struct Nomad {
    client: Client,
    config: Config,
}

impl Nomad {
    pub fn new(config: Config) -> Self {
        let builder = Client::builder().use_rustls_tls();
        let client = builder.build().expect("HTTP client should be buildable");

        Nomad { client, config }
    }

    pub fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/v1/{}", self.config.address, endpoint)
    }

    pub fn request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        self.client.request(method, self.get_endpoint(endpoint))
    }

    async fn send<TResponse: DeserializeOwned>(
        &self,
        mut builder: RequestBuilder,
    ) -> Result<TResponse, NomadError> {
        if let Some(token) = self.config.token.as_ref() {
            builder = builder.header("X-Nomad-Token", token);
        }

        let request = builder.build().map_err(NomadError::RequestError)?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(NomadError::ResponseError)?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| NomadError::UnexpectedResponseCode(status, e.to_string()))?;
            return Err(NomadError::UnexpectedResponseCode(status, body));
        }

        match response.json::<TResponse>().await {
            Ok(body) => Ok(body),
            Err(e) => Err(NomadError::InvalidResponse(e)),
        }
    }
}

impl Default for Nomad {
    fn default() -> Self {
        Nomad::new(Config::from_env())
    }
}
