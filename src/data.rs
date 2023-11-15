use quick_xml::Reader;
use quick_xml::events::Event;
use std::str;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use log::error;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use chrono::Utc;

pub struct DataStore {
    http_client: Client,
    bucket     : String,
    region     : String,
    access_key : String,
    secret_key : String,
    endpoint   : String,
}

#[derive(Debug, Deserialize)]
struct ListBucketResult {
    #[serde(rename = "Contents", default)]
    contents: Vec<S3Object>,
}

#[derive(Debug, Deserialize)]
struct S3Object {
    Key: String,
    LastModified: String,
    ETag: String,
    Size: u64,
    StorageClass: String,
    Owner: S3ObjectOwner,
}

#[derive(Debug, Deserialize)]
struct S3ObjectOwner {
    ID: String,
    DisplayName: String,
}

// Define the hash_payload function.
fn hash_payload(payload: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    format!("{:x}", hasher.finalize())
}

impl DataStore {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let bucket     = std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set");
        let region     = std::env::var("AWS_REGION").expect("AWS_REGION must be set");
        let access_key = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
        let endpoint   = format!("https://{}.{}.cdn.digitaloceanspaces.com", bucket, region);

        let http_client = Client::new();
        Ok(DataStore { http_client, bucket, region, access_key, secret_key, endpoint })
    }

    pub async fn list_all_objects(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let date = Utc::now();
        let date_str = date.format("%Y%m%dT%H%M%SZ").to_string();
        let date_short = date.format("%Y%m%d").to_string();
        let service = "s3";
        let request_type = "aws4_request";

        let canonical_uri = format!("/{}/", self.bucket);
        let canonical_querystring = "";

        let canonical_headers = format!("host:{}\nx-amz-date:{}\n", self.endpoint, date_str);
        let signed_headers = "host;x-amz-date";
        let payload_hash = hash_payload(b"");
        let canonical_request = format!("GET\n{}\n{}\n{}\n{}\n{}", canonical_uri, canonical_querystring, canonical_headers, signed_headers, payload_hash);

        let string_to_sign = format!("AWS4-HMAC-SHA256\n{}\n{}/{}/{}/{}\n{}", date_str, date_short, self.region, service, request_type, hex::encode(Sha256::digest(canonical_request.as_bytes())));

        let signing_key = derive_signing_key(&self.secret_key, &date_short, &self.region, service);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let authorization_header = format!("AWS4-HMAC-SHA256 Credential={}/{}/{}/{}/{}, SignedHeaders={}, Signature={}", self.access_key, date_short, self.region, service, request_type, signed_headers, signature);

        let endpoint = format!("https://{}.{}.cdn.digitaloceanspaces.com", self.bucket, self.region);
        let response = self.http_client.get(&self.endpoint)
            .header("Authorization", authorization_header)
            .header("x-amz-date", date_str)
            .send().await?;

        if !response.status().is_success() {
            error!("Error listing objects in bucket {}: {:?}", self.bucket, response.status());
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to list objects")));
        }

        let response_body = response.bytes().await?;
        let mut reader = Reader::from_reader(response_body.as_ref());
        reader.trim_text(true);
        let mut buf = Vec::<u8>::new();
        let mut keys = Vec::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"Key" => {
                    if let Ok(Event::Text(e)) = reader.read_event() {
                        let key_str = e.unescape().unwrap_or_default().into_owned();
                        keys.push(key_str);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Box::new(e)),
                _ => (),
            }
            buf.clear();
        }

        Ok(keys)
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

// Helper functions for signature generation
fn derive_signing_key(secret: &str, date: &str, region: &str, service: &str) -> Vec<u8> {
    let key = format!("AWS4{}", secret);
    let date_key = hmac_sha256(key.as_bytes(), date.as_bytes());
    let date_region_key = hmac_sha256(&date_key, region.as_bytes());
    let date_region_service_key = hmac_sha256(&date_region_key, service.as_bytes());
    hmac_sha256(&date_region_service_key, "aws4_request".as_bytes())
}

type HmacSha256 = Hmac<Sha256>;

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
