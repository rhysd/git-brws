use std::env;

#[derive(Debug, PartialEq)]
pub struct Envvar {
    pub git_command: String,
    pub ghe_ssh_port: Option<String>,
    pub ghe_url_host: Option<String>,
    pub gitlab_ssh_port: Option<String>,
}

pub fn new() -> Envvar {
    Envvar {
        git_command: env::var("GIT_BRWS_GIT_COMMAND").unwrap_or_else(|_| "git".to_string()),
        ghe_ssh_port: env::var("GIT_BRWS_GHE_SSH_PORT").ok(),
        ghe_url_host: env::var("GIT_BRWS_GHE_URL_HOST").ok(),
        gitlab_ssh_port: env::var("GIT_BRWS_GITLAB_SSH_PORT").ok(),
    }
}
