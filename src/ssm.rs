use aws_sdk_ssm::model::ParameterType;
use aws_sdk_ssm::{Client, Error, Region};

pub struct SSMService {
    client: Client,
}

impl SSMService {
    pub async fn new(region: String) -> Self {
        let config = aws_config::from_env()
            .region(Region::new(region))
            .load()
            .await;

        let client = Client::new(&config);
        SSMService { client }
    }

    pub async fn put_parameter(
        &self,
        name: &str,
        value: &str,
        overwrite: bool,
    ) -> Result<(), Error> {
        let resp = &self
            .client
            .put_parameter()
            .overwrite(overwrite)
            .r#type(ParameterType::SecureString)
            .name(name)
            .value(value)
            .send()
            .await?;

        let mut verb: String = "created".to_owned();
        if resp.version() > 1 {
            verb = "updated".to_owned();
        }
        println!(
            "[ok] parameter {} | Name: {} | version: {}",
            &verb,
            &name,
            &resp.version()
        );
        Ok(())
    }
}
