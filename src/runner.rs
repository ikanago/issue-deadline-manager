use envconfig::Envconfig;
use octocrab::Octocrab;

use crate::label::DeadlineLabel;

#[derive(Debug, Envconfig)]
struct ConfigBuilder {
    #[envconfig(from = "GITHUB_TOKEN")]
    token: String,
    #[envconfig(from = "GITHUB_REPOSITORY")]
    repository: String,
}

pub struct Config {
    pub owner: String,
    pub repository: String,
}

impl Config{
    pub fn initialize() -> Result<(Self, String), envconfig::Error> {
        let builder = ConfigBuilder::init_from_env()?;

        let mut owner_and_repo = builder.repository.split('/');
        let owner = owner_and_repo.next().ok_or(envconfig::Error::ParseError {
            name: "GITHUB_REPOSITORY is empty",
        })?;
        let repository = owner_and_repo.next().ok_or(envconfig::Error::ParseError {
            name: "Repository name in GITHUB_REPOSITORY is empty",
        })?;

        let config = Config {
            owner: owner.to_string(),
            repository: repository.to_string(),
        };
        Ok((config, builder.token))
    }
}

pub struct Runner {
    client: Octocrab,
    config: Config,
}

impl Runner {
    pub fn new(token: String, config: Config) -> Self {
        let client = octocrab::OctocrabBuilder::new()
            .personal_token(token)
            .build()
            .unwrap();
        Self { client, config }
    }

    pub async fn register_labels(&self, labels: &[DeadlineLabel]) -> octocrab::Result<()> {
        for label in labels {
            if !self.check_label_existance(label).await? {
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
