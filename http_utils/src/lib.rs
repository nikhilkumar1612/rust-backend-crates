use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
}

fn client() -> Client {
    Client::new()
}

pub async fn get<T>(url: &str) -> Result<T, HttpError>
where
    T: serde::de::DeserializeOwned,
{
    let response = client().get(url).send().await.unwrap();
    Ok(
        response.error_for_status()?.json::<T>().await?
    )
}

pub async fn post<B, R>(url: &str, body: &B) -> Result<R, HttpError>
where
    B: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    Ok(client()
        .post(url)
        .json(body)
        .send()
        .await?
        .error_for_status()?
        .json::<R>()
        .await?
    )
}
