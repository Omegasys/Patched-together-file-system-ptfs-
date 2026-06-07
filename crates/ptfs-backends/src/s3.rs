use crate::trait_::StorageBackend;

/// Cloud object storage backend (S3-compatible)
pub struct S3Backend {
    pub bucket: String,
    pub region: String,
}

impl S3Backend {
    pub fn new(bucket: String, region: String) -> Self {
        Self { bucket, region }
    }
}

impl StorageBackend for S3Backend {
    fn name(&self) -> &str {
        "s3"
    }

    fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()> {
        println!(
            "[PTFS-S3] upload {} bytes → bucket {}/{}",
            data.len(),
            self.bucket,
            key
        );

        // Real system:
        // - AWS SDK or minio client
        // - PUT object request

        Ok(())
    }

    fn download(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        println!(
            "[PTFS-S3] download {} from bucket {}",
            key,
            self.bucket
        );

        Ok(vec![7, 8, 9])
    }

    fn health_check(&self) -> bool {
        !self.bucket.is_empty()
    }
}
