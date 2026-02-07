use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;

struct SmConfig {
    sm_client: SecretsManagerClient
}

struct S3Config {
    s3_client: S3Client,
    bucket: String
}

pub struct AwsUtils {
    sm_config: SmConfig,
    s3_config: S3Config
}

impl AwsUtils {
    pub async fn init(bucket: String) -> Self {
        let config = aws_config::load_defaults(
            BehaviorVersion::latest()
        ).await;

        let sm_client = SecretsManagerClient::new(&config);
        let s3_client = S3Client::new(&config);

        let sm_config = SmConfig {sm_client};
        let s3_config = S3Config {s3_client, bucket};

        Self {sm_config, s3_config}
    }

    pub async fn fetch_secret(
        &self,
        secret_name: &str
    ) -> Result<String, aws_sdk_secretsmanager::Error> {
        let resp = self.sm_config.sm_client.get_secret_value()
            .secret_id(secret_name)
            .send()
            .await?;
        Ok(resp.secret_string.unwrap())
    }

    pub async fn create_secret(
        &self,
        secret_name: &str,
        secret_value: &str,
    ) -> Result<(), aws_sdk_secretsmanager::Error> {
        self.sm_config
            .sm_client
            .create_secret()
            .name(secret_name)
            .secret_string(secret_value)
            .send()
            .await?;

        Ok(())
    }

    pub async fn upload_media(
        &self,
        key: &str,
        data: Vec<u8>
    ) -> Result<(), aws_sdk_s3::Error> {
        self.s3_config
            .s3_client
            .put_object()
            .bucket(&self.s3_config.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await?;

        Ok(())
    }
}
