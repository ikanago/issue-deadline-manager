use octocrab::Octocrab;

use crate::label::DeadlineLabel;

pub struct RunnerConfig {
    pub owner: String,
    pub repository: String,
}

pub struct Runner {
    client: Octocrab,
    config: RunnerConfig,
}

impl Runner {
    pub fn new(token: String, config: RunnerConfig) -> Self {
        let client = octocrab::OctocrabBuilder::new()
            .personal_token(token)
            .build()
            .unwrap();
        Self { client, config }
    }

    pub async fn register_labels(&self, labels: &[DeadlineLabel]) -> octocrab::Result<()> {
        for label in labels {
            if !self.check_label_existance(&label).await? {
                self.client
                    .issues(&self.config.owner, &self.config.repository)
                    .create_label(label.to_string(), "ff0000", "")
                    .await?;
            }
        }
        Ok(())
    }

    async fn check_label_existance(&self, label: &DeadlineLabel) -> octocrab::Result<bool> {
        match self
            .client
            .issues(&self.config.owner, &self.config.repository)
            .get_label(label.to_string())
            .await
        {
            Ok(_) => Ok(true),
            Err(octocrab::Error::GitHub { .. }) => Ok(false),
            Err(err) => Err(err),
        }
    }
}
