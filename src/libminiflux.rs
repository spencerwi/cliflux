extern crate reqwest;
extern crate serde;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub site_url: String,
    pub feed_url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FeedEntry {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub feed: Feed,
}

#[derive(Deserialize, Serialize)]
struct FeedEntriesResponse {
    pub total: i32,
    pub entries: Vec<FeedEntry>,
}

#[derive(Deserialize, Serialize)]
struct UpdateEntriesRequest {
    pub status: String,
    pub entry_ids: Vec<i32>,
}

enum ReadStatus {
    Read,
    Unread,
}
impl ToString for ReadStatus {
    fn to_string(&self) -> String {
        return match self {
            ReadStatus::Read => "read".to_string(),
            ReadStatus::Unread => "unread".to_string(),
        };
    }
}

pub struct Client {
    base_url: String,
    api_key: String,
    http_client: reqwest::Client,
}
impl Client {
    pub fn new(base_url: String, api_key: &String) -> Client {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            HeaderName::from_bytes(b"X-Auth-Token").unwrap(),
            HeaderValue::from_str(&api_key).unwrap(),
        );
        let http_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap(); // TODO: better error handling

        return Client {
            base_url,
            api_key: api_key.to_string(),
            http_client,
        };
    }

    pub async fn get_unread_entries(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<FeedEntry>, reqwest::Error> {
        let response: FeedEntriesResponse = self
            .http_client
            .get(
                &format!(
                    "{}/v1/entries?status=unread&limit={}&offset={}",
                    self.base_url, limit, offset
                )
                .to_string(),
            )
            .send()
            .await?
            .json::<FeedEntriesResponse>()
            .await?;

        return Ok(response.entries);
    }

    pub async fn mark_entry_as_read(&self, entry_id: i32) -> Result<(), reqwest::Error> {
        return self
            .change_entry_read_status(entry_id, ReadStatus::Unread)
            .await;
    }

    pub async fn mark_entry_as_unread(&self, entry_id: i32) -> Result<(), reqwest::Error> {
        return self
            .change_entry_read_status(entry_id, ReadStatus::Read)
            .await;
    }

    async fn change_entry_read_status(
        &self,
        entry_id: i32,
        status: ReadStatus,
    ) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .put(format!("{}/v1/entries", self.base_url))
            .json(&UpdateEntriesRequest {
                status: status.to_string(),
                entry_ids: vec![entry_id],
            })
            .send()
            .await?;
        return Ok(());
    }
}
