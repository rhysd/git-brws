use crate::error::Result;
use crate::git::Git;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub repo: String,
    pub branch: Option<String>,
    pub git_dir: Option<PathBuf>,
    pub args: Vec<String>,
    pub stdout: bool,
    pub pull_request: bool,
    pub website: bool,
    pub blame: bool,
    pub env: EnvConfig,
}

impl Config {
    pub fn git(&self) -> Option<Git<'_>> {
        self.git_dir
            .as_ref()
            .map(|git_dir| Git::new(git_dir, &self.env.git_command))
    }
}

#[inline]
fn default_git_command() -> String {
    "git".to_string()
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct EnvConfig {
    #[serde(default = "default_git_command")]
    pub git_command: String,
    pub ghe_ssh_port: Option<u16>,
    pub ghe_url_host: Option<String>,
    pub gitlab_ssh_port: Option<u16>,
    pub github_token: Option<String>,
    pub ghe_token: Option<String>,
    pub https_proxy: Option<String>,
    pub browse_command: Option<String>,
}

impl EnvConfig {
    pub fn with_global_env(mut self) -> Self {
        if self.https_proxy.is_none() {
            self.https_proxy = env::var("https_proxy")
                .or_else(|_| env::var("HTTPS_PROXY"))
                .ok();
        }
        if self.github_token.is_none() {
            self.github_token = env::var("GITHUB_TOKEN").ok();
        }
        self
    }
}

impl EnvConfig {
    // Note: Using `from_env` is not good in terms of testing.
    pub fn from_iter<I>(iter: I) -> Result<EnvConfig>
    where
        I: IntoIterator<Item = (String, String)>,
    {
        Ok(envy::prefixed("GIT_BRWS_").from_iter(iter)?)
    }
}
