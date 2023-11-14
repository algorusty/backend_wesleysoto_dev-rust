use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::error::Error;
use log::error;

pub struct DataStore {
    http_client: Client,
    bucket: String,
    region: String,
    access_key: String,
    secret_key: String,
}

#[derive(Deserialize)]
struct ListObjectsResponse {
    Contents: Vec<S3Object>,
}

#[derive(Deserialize)]
struct S3Object {
    Key: String,
}

impl DataStore {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let bucket = std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set");
        let region = std::env::var("AWS_REGION").expect("AWS_REGION must be set");
        let access_key = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");

        let http_client = Client::new();
        Ok(DataStore { http_client, bucket, region, access_key, secret_key })
    }

    pub async fn list_all_objects(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let endpoint = format!("https://{}.{}.digitaloceanspaces.com", self.bucket, self.region);
        let response = self.http_client.get(&endpoint)
            .header("Authorization", format!("{}:{}", self.access_key, self.secret_key))
            .send().await?;

        if !response.status().is_success() {
            error!("Error listing objects in bucket {}: {:?}", self.bucket, response.status());
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to list objects")));
        }

        let list_objects_response: ListObjectsResponse = response.json().await?;
        Ok(list_objects_response.Contents.into_iter().map(|obj| obj.Key).collect())
    }

    pub async fn objects(&self) -> Vec<String> {
        self.list_all_objects().await.unwrap_or_else(|_| vec![])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavItem {
    pub icon: String,
    pub text: String,
}


impl From<String> for NavItem {
    fn from(item: String) -> Self {
        let parts: Vec<&str> = item.split(':').collect();
        NavItem {
            icon: parts[0].to_string(),
            text: parts[1].to_string(),
        }
    }
}