use chrono::Datelike;
use chrono_tz::Tz;
use envconfig::Envconfig;
use octocrab::{models::issues::Issue, Octocrab};

use crate::{
    label::{determine_label, DeadlineLabel},
    parse::{parse_issue, ParseError},
};

#[derive(Debug, Envconfig)]
struct RawConfig {
    #[envconfig(from = "GITHUB_TOKEN")]
    token: String,
    #[envconfig(from = "GITHUB_REPOSITORY")]
    repository: String,
}

pub struct Config {
    pub owner: String,
    pub repository: String,
}

impl Config {
    pub fn initialize() -> Result<(Self, String), envconfig::Error> {
        let builder = RawConfig::init_from_env()?;

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

    /// Fetch open issues and add deadline labels based on a command in issue body.
    pub async fn update_labels(&self) -> octocrab::Result<()> {
        let issues = self
            .client
            .issues(&self.config.owner, &self.config.repository)
            .list()
            .send()
            .await?;

        for issue in issues {
            self.update_label_in_issue(issue).await?;
        }
        Ok(())
    }

    async fn update_label_in_issue(&self, issue: Issue) -> octocrab::Result<()> {
        if issue.body.is_none() {
            return Ok(());
        }

        let now = chrono::Local::now();
        let deadline = match parse_issue(issue.body.as_ref().unwrap(), Tz::Asia__Tokyo, now.year())
        {
            Ok(deadline) => deadline,
            Err(ParseError::Empty) => return Ok(()),
            Err(err) => {
                eprintln!("{}", err);
                return Ok(());
            }
        };
        let label = determine_label(deadline, now);

        if Self::is_label_already_added(&issue, &label) {
            return Ok(());
        }

        self.remove_existing_labels(&issue).await?;
        self.create_label(&label).await?;
        self.client
            .issues(&self.config.owner, &self.config.repository)
            .add_labels(issue.number as u64, &[label.to_string()])
            .await?;

        Ok(())
    }

    fn is_label_already_added(issue: &Issue, label: &DeadlineLabel) -> bool {
        issue
            .labels
            .iter()
            .map(|label| &label.name)
            .any(|name| name == &label.to_string())
    }

    async fn remove_existing_labels(&self, issue: &Issue) -> octocrab::Result<()> {
        let deadline_labels = issue
            .labels
            .iter()
            .filter(|label| label.name.starts_with(DeadlineLabel::LABEL_PREFIX));

        for label in deadline_labels {
            self.client
                .issues(&self.config.owner, &self.config.repository)
                .remove_label(issue.number as u64, &label.name)
                .await?;
        }

        Ok(())
    }

    pub async fn create_label(&self, label: &DeadlineLabel) -> octocrab::Result<()> {
        if self.check_label_existance(label).await? {
            return Ok(());
        }

        self.client
            .issues(&self.config.owner, &self.config.repository)
            .create_label(label.to_string(), "ff0000", "")
            .await?;
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
