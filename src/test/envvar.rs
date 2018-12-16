use crate::envvar::*;
use crate::test::helper::empty_env;
use std::collections::HashMap;
use std::env;

#[test]
fn new_envvars() {
    assert_eq!(new(), empty_env());

    let mut envs = HashMap::new();
    envs.insert("GIT_BRWS_GIT_COMMAND", "/path/to/git");
    envs.insert("GIT_BRWS_GHE_SSH_PORT", "10022");
    envs.insert("GIT_BRWS_GITLAB_SSH_PORT", "10023");
    envs.insert("GIT_BRWS_GHE_URL_HOST", "my-original-ghe.org");
    let envs = envs;

    for (key, val) in &envs {
        env::set_var(key, val);
    }

    let e = new();

    for (key, _) in &envs {
        env::remove_var(key);
    }

    assert_eq!(
        e,
        Envvar {
            git_command: "/path/to/git".to_string(),
            ghe_ssh_port: Some("10022".to_string()),
            ghe_url_host: Some("my-original-ghe.org".to_string()),
            gitlab_ssh_port: Some("10023".to_string()),
        }
    );
}
