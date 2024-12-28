use reqwest::{header::HeaderMap, Client, ClientBuilder, StatusCode};
use serde::Deserialize;

use crate::{Error, Result};

pub struct HttpClient {
    reqwest_client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            reqwest_client: reqwest::Client::new(),
        }
    }
}
impl From<Client> for HttpClient {
    fn from(reqwest_client: Client) -> Self {
        Self { reqwest_client }
    }
}

impl HttpClient {
    pub fn with_headers(headers: HeaderMap) -> Result<Self> {
        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(Error::Request)
            .map(HttpClient::from)
    }
}

impl HttpClient {
    pub async fn do_get<T: for<'a> Deserialize<'a>>(&self, url: &str) -> Result<HttpResponse<T>> {
        self.reqwest_client
            .get(url)
            .send()
            .await
            .map(HttpResponse::from_reqwest_response)?
            .await
    }

    pub async fn do_put<T: for<'a> Deserialize<'a>>(
        &self,
        url: &str,
        data: String,
    ) -> Result<HttpResponse<T>> {
        self.reqwest_client
            .put(url)
            .body(data)
            .send()
            .await
            .map(HttpResponse::from_reqwest_response)?
            .await
    }
}

#[allow(dead_code)]
pub struct HttpResponse<T> {
    pub body: T,
    pub status: StatusCode,
}

impl<T: for<'a> Deserialize<'a>> HttpResponse<T> {
    async fn from_reqwest_response(res: reqwest::Response) -> Result<Self> {
        let status = res.status();

        if !status.is_success() {
            let body = res.text().await?;
            return Err(Error::StatusCodeError { status, body });
        }

        let body = res.json::<T>().await?;

        Ok(Self { status, body })
    }
}
