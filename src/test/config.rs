use crate::config::EnvConfig;
use crate::error::ErrorKind;
use crate::test::helper::empty_env;
use std::env;

#[test]
fn invalid_env_value() {
    let vars = vec![("GIT_BRWS_GHE_SSH_PORT".to_string(), "hello".to_string())];
    match EnvConfig::from_iter(vars.into_iter()).unwrap_err().kind() {
        ErrorKind::EnvLoadError(e) => {
            let msg = format!("{}", e);
            assert!(msg.contains("invalid digit found in string"), "{}", msg);
        }
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[test]
fn with_global_env() {
    let iter = [
        ("GIT_BRWS_GHE_TOKEN", "token for ghe"),
        ("GIT_BRWS_GITHUB_TOKEN", "token for github"),
        ("GIT_BRWS_SHORT_COMMIT_HASH", "true"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()));
    let env = EnvConfig::from_iter(iter).unwrap().with_global_env();

    assert_eq!(env.git_command, "git");
    assert_eq!(env.gitlab_ssh_port, None);
    assert_eq!(env.ghe_token.unwrap(), "token for ghe");
    assert!(env.short_commit_hash);

    // $GITHUB_TOKEN is never used since $GIT_BRWS_GITHUB_TOKEN is prioritized
    //
    // XXX: This test is risky since when it does not work the token is exposed though the token
    // does not have any permission except for boosting API rate limit.
    assert_eq!(env.github_token.unwrap(), "token for github");

    let https_proxy = env::var("GIT_BRWS_HTTPS_PROXY")
        .or_else(|_| env::var("https_proxy"))
        .or_else(|_| env::var("HTTPS_PROXY"))
        .ok();
    assert_eq!(env.https_proxy, https_proxy);
}

#[test]
fn with_global_env_updates_none_values() {
    let env = empty_env().with_global_env();
    let https_proxy = env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .ok();
    let github_token = env::var("GITHUB_TOKEN").ok();
    assert_eq!(env.https_proxy, https_proxy);
    assert_eq!(env.github_token, github_token);
}
