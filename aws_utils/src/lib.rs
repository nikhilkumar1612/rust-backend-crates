use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client;

struct AwsUtils {
    client: Client,
}

impl AwsUtils {
    pub async fn init() -> Self {
        let config = aws_config::load_defaults(
            BehaviorVersion::latest()
        ).await;

        let client = Client::new(&config);

        Self {client}
    }

    pub async fn fetch_secret(
        &self,
        secret_name: &str
    ) -> Result<String, aws_sdk_secretsmanager::Error> {
        let resp = self.client.get_secret_value()
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
        self.client
            .create_secret()
            .name(secret_name)
            .secret_string(secret_value)
            .send()
            .await?;

        Ok(())
    }
}
