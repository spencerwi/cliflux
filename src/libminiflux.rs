extern crate reqwest;
extern crate serde;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub site_url: String,
    pub feed_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FeedEntry {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub feed: Feed,
    pub status: ReadStatus,
    pub starred: bool,
	#[serde(default)]
	pub original_content : Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
struct FeedEntriesResponse {
    pub total: i32,
    pub entries: Vec<FeedEntry>,
}

#[derive(Deserialize, Serialize, PartialEq)]
struct UpdateEntriesRequest {
    pub status: String,
    pub entry_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, PartialEq)]
struct OriginalContentResponse {
	pub content: String
}

#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReadStatus {
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
impl ReadStatus {
    pub fn toggle(&self) -> ReadStatus {
        match &self {
            ReadStatus::Read => ReadStatus::Unread,
            ReadStatus::Unread => ReadStatus::Read,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    base_url: String,
    http_client: reqwest::Client,
}
impl Client {
    pub fn new(config: &Config) -> Client {
        let api_key = &config.api_key;
        let base_url = config.server_url.clone();
        let invalid_certs = config.allow_invalid_certs;

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            HeaderName::from_bytes(b"X-Auth-Token").unwrap(),
            HeaderValue::from_str(&api_key).unwrap(),
        );
        let http_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(invalid_certs)
            .default_headers(default_headers)
            .build()
            .unwrap(); // TODO: better error handling

        return Client {
            base_url,
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
                    "{}/v1/entries?status=unread&order=published_at&direction=desc&limit={}&offset={}",
                    self.base_url, limit, offset
                )
                .to_string(),
            )
            .send()
            .await?
			.error_for_status()?
            .json::<FeedEntriesResponse>()
            .await?;

        return Ok(response.entries);
    }

    pub async fn get_starred_entries(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<FeedEntry>, reqwest::Error> {
        let response: FeedEntriesResponse = self
            .http_client
            .get(
                &format!(
                    "{}/v1/entries?starred=true&order=published_at&direction=desc&limit={}&offset={}",
                    self.base_url, limit, offset
                )
                .to_string(),
            )
            .send()
            .await?
			.error_for_status()?
            .json::<FeedEntriesResponse>()
            .await?;

        return Ok(response.entries);
    }

    pub async fn change_entry_read_status(
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
            .await?
			.error_for_status()?;
        return Ok(());
    }

    pub async fn toggle_starred(&self, entry_id: i32) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .put(format!(
                "{}/v1/entries/{}/bookmark",
                self.base_url, entry_id
            ))
            .send()
            .await?
			.error_for_status()?;
        return Ok(());
    }

    pub(crate) async fn save_entry(&self, entry_id: i32) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .post(format!(
                "{}/v1/entries/{}/save",
                self.base_url, entry_id
            ))
            .send()
            .await?
			.error_for_status()?;
        return Ok(());
    }

    pub(crate) async fn mark_all_as_read(&self, entry_ids: Vec<i32>) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .put(format!("{}/v1/entries", self.base_url))
            .json(&UpdateEntriesRequest {
                status: "read".to_string(),
                entry_ids,
            })
            .send()
            .await?
			.error_for_status()?;
        return Ok(());
    }

    pub(crate) async fn refresh_all_feeds(&self) -> Result<(), reqwest::Error> {
        let _ = self
            .http_client
            .put(format!("{}/v1/feeds/refresh", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        return Ok(());
    }

	pub(crate) async fn fetch_original_content(&self, entry_id: i32) -> Result<String, reqwest::Error> {
		let response : OriginalContentResponse = self
			.http_client
			.get(format!("{}/v1/entries/{}/fetch-content", self.base_url, entry_id))
			.send()
			.await?
			.error_for_status()?
            .json::<OriginalContentResponse>()
			.await?;
		return Ok(response.content);
	}
}
