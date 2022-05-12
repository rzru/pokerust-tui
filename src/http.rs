use std::{fmt::Debug, sync::Arc};

use hyper::{
    body::{to_bytes, Bytes},
    client::HttpConnector,
    Client, StatusCode,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio::sync::mpsc::{self, Sender};

pub struct Http {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Http {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self { client }
    }

    async fn get(&self, uri: &str) -> Option<Bytes> {
        if let Ok(uri) = uri.parse() {
            let resp = self.client.get(uri).await.unwrap();

            if resp.status() == StatusCode::OK {
                if let Ok(result) = to_bytes(resp.into_body()).await {
                    return Some(result);
                }
            }
        }

        None
    }

    pub async fn get_as_object<T>(&self, uri: &str) -> Option<T>
    where
        T: DeserializeOwned + Debug,
    {
        let bytes = self.get(uri).await;

        if let Some(bytes) = bytes {
            let result: Result<T, serde_json::Error> = serde_json::from_slice(&bytes);

            if let Ok(result) = result {
                return Some(result);
            }
        }

        None
    }
}

pub async fn fetch_external<T, R: 'static, F>(data: &[T], fetch_url_extractor: F) -> Vec<R>
where
    R: DeserializeOwned + Send + Debug,
    F: Fn(&T) -> String,
{
    let mut res = vec![];
    let (tx, mut rx) = mpsc::channel(32);
    let http = Arc::new(Http::new());

    for item in data {
        let http = Arc::clone(&http);
        let url = fetch_url_extractor(item);
        let tx = tx.clone();
        spawn_fetcher(http, url, tx).await;
    }

    drop(tx);

    while let Some(message) = rx.recv().await {
        res.push(message)
    }

    res
}

async fn spawn_fetcher<T: 'static>(http: Arc<Http>, url: String, tx: Sender<T>)
where
    T: DeserializeOwned + Send + Debug,
{
    tokio::spawn(async move {
        let data = http.get(&url).await;

        if let Some(bytes) = data {
            let fetched = serde_json::from_slice(&bytes).unwrap();

            tx.send(fetched).await.unwrap();
        }
    });
}
