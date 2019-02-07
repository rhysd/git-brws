extern crate envy;
extern crate serde;

use crate::error::Result;

#[inline]
fn default_git_command() -> String {
    "git".to_string()
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct EnvConfig {
    #[serde(default = "default_git_command")]
    pub git_command: String,
    pub ghe_ssh_port: Option<u16>,
    pub ghe_url_host: Option<String>,
    pub gitlab_ssh_port: Option<u16>,
    pub github_token: Option<String>,
    pub ghe_token: Option<String>,
    pub https_proxy: Option<String>,
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
