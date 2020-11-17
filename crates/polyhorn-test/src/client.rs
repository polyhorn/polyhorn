use serde::Serialize;

#[derive(Serialize)]
pub enum Message {
    Snapshot {
        test_name: &'static str,
        snapshot_name: String,
    },
    OpenURL(String),
}

pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String) -> Client {
        Client {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send(&self, message: &Message) {
        self.client
            .post(&self.url)
            .json(message)
            .send()
            .await
            .unwrap();
    }
}
