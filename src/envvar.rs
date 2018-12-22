use std::env;

#[derive(Debug, PartialEq)]
pub struct Envvar {
    pub git_command: String,
    pub ghe_ssh_port: Option<String>,
    pub ghe_url_host: Option<String>,
    pub gitlab_ssh_port: Option<String>,
    pub github_token: Option<String>,
    pub ghe_token: Option<String>,
    pub https_proxy: Option<String>,
}

pub fn new() -> Envvar {
    Envvar {
        git_command: env::var("GIT_BRWS_GIT_COMMAND").unwrap_or_else(|_| "git".to_string()),
        ghe_ssh_port: env::var("GIT_BRWS_GHE_SSH_PORT").ok(),
        ghe_url_host: env::var("GIT_BRWS_GHE_URL_HOST").ok(),
        gitlab_ssh_port: env::var("GIT_BRWS_GITLAB_SSH_PORT").ok(),
        github_token: env::var("GIT_BRWS_GITHUB_TOKEN")
            .or_else(|_| env::var("GITHUB_TOKEN"))
            .ok(),
        ghe_token: env::var("GIT_BRWS_GHE_TOKEN").ok(),
        https_proxy: env::var("https_proxy")
            .or_else(|_| env::var("HTTPS_PROXY"))
            .ok(),
    }
}
