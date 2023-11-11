use rusoto_core::Region::Custom;
use rusoto_s3::{S3Client, S3, GetObjectRequest};
use tokio::io::AsyncReadExt;

pub struct DataStore {
    s3_client: S3Client,
    bucket: String,
    key: String,
}

impl DataStore {
    pub fn new(bucket: String, key: String) -> Self {
        let region = Custom {
            name: "sfo3".to_owned(),
            endpoint: "https://sfo3.digitaloceanspaces.com".to_owned(),
        };
        DataStore { 
            s3_client: S3Client::new(region), 
            bucket, 
            key 
        }
    }

    pub async fn get_nav_items(&self) -> Result<Vec<NavItem>> {
        let mut request = GetObjectRequest::default();
        request.bucket = self.bucket.clone();
        request.key = self.key.clone();

        let result = self.s3_client.get_object(request).await.expect("Failed to get object");
        let stream = result.body.expect("No stream found");
        let mut body = String::new();
        stream.into_async_read().read_to_string(&mut body).await.expect("Failed to read string");

        let nav_items: Vec<NavItem> = serde_json::from_str(&body)?;

        Ok(nav_items)
    }
}