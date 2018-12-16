use crate::envvar;
use std::env;

pub fn empty_env() -> envvar::Envvar {
    envvar::Envvar {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
    }
}

pub fn on_travis_ci() -> bool {
    env::var("TRAVIS").is_ok()
}
