use crate::github::MergeMethod;
use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MergebroConfig {
    pub github: GithubConfig,

    #[serde(default)]
    pub merge: MergeConfig,

    #[serde(default)]
    pub poll: PollConfig,

    #[serde(default)]
    pub workflows: WorkflowsConfig,

    #[serde(default)]
    pub reviews: PullRequestReviewsConfig,
}

#[derive(Deserialize, Debug)]
pub struct PollConfig {
    pub delay_seconds: u8,
}

impl Default for PollConfig {
    fn default() -> PollConfig {
        PollConfig { delay_seconds: 30 }
    }
}

#[derive(Deserialize, Debug)]
pub struct MergeConfig {
    pub default_method: MergeMethod,
}

impl Default for MergeConfig {
    fn default() -> MergeConfig {
        MergeConfig {
            default_method: MergeMethod::Merge,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GithubConfig {
    pub username: String,
    pub token: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct WorkflowsConfig {
    pub circleci: Option<CircleCiConfig>,
}

#[derive(Deserialize, Debug)]
pub struct CircleCiConfig {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestReviewsConfig {
    pub default: ReviewsConfig,

    #[serde(default)]
    pub repos: Vec<RepoReviewsConfig>,
}

impl Default for PullRequestReviewsConfig {
    fn default() -> Self {
        Self {
            default: ReviewsConfig { approvals: 1 },
            repos: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RepoReviewsConfig {
    pub repo: String,

    #[serde(flatten)]
    pub config: ReviewsConfig,
}

#[derive(Deserialize, Debug)]
pub struct ReviewsConfig {
    pub approvals: u32,
}

impl MergebroConfig {
    pub fn new(config_file_path: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        let config_file_path = shellexpand::tilde(config_file_path);
        config.merge(File::with_name(&config_file_path).required(false))?;
        config.merge(Environment::with_prefix("mergebro").separator("_"))?;
        config.try_into()
    }
}
