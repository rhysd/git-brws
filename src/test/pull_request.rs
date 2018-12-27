use crate::error::Error;
use crate::pull_request::find_url;
use crate::test::helper;

macro_rules! env {
    () => {{
        let mut e = helper::empty_env();
        e.github_token = skip_if_no_token!();
        e.https_proxy = helper::https_proxy();
        e
    }};
}

#[test]
fn test_find_pr_within_orig_repo() {
    let env = env!();
    let url = find_url("https://github.com/rhysd/vim.wasm", "async-eventloop", &env).unwrap();
    assert_eq!(url.as_str(), "https://github.com/rhysd/vim.wasm/pull/10");
}

#[test]
fn test_find_pr_from_fork_repo_url() {
    let env = env!();
    let url = find_url(
        "https://github.com/rhysd/rust.vim",
        "async-contextual-keyword",
        &env,
    )
    .unwrap();
    assert_eq!(
        url.as_str(),
        "https://github.com/rust-lang/rust.vim/pull/290"
    );
}

#[test]
fn test_find_pr_from_original_repo_url() {
    let env = env!();
    let url = find_url(
        "https://github.com/rust-lang/rust.vim",
        "async-contextual-keyword",
        &env,
    )
    .unwrap();
    assert_eq!(
        url.as_str(),
        "https://github.com/rust-lang/rust.vim/pull/290"
    );
}

#[test]
fn test_not_supported_service() {
    let env = helper::empty_env();
    match find_url("https://gitlab.com/rhysd/foo", "some-branch", &env) {
        Err(Error::PullReqNotSupported { service }) => assert_eq!(service, "gitlab.com"),
        v => assert!(false, "Unexpected success or error: {:?}", v),
    }
}

#[test]
fn test_no_pr_found() {
    let env = env!();
    match find_url(
        "https://github.com/rhysd/git-brws",
        "unknown-branch-which-does-not-exist-for-test",
        &env,
    ) {
        Ok(v) => assert!(false, "Unexpected success: {}", v),
        Err(Error::GitHubPullReqNotFound {
            author,
            repo,
            branch,
        }) => {
            assert_eq!(author, "rhysd");
            assert_eq!(repo, "git-brws");
            assert_eq!(branch, "unknown-branch-which-does-not-exist-for-test");
        }
        v => assert!(false, "Unexpected success or error: {:?}", v),
    }
}

#[test]
fn test_unknown_github_enterprise_url() {
    let mut env = env!();
    env.ghe_url_host = Some("mygithub.example.com".to_string());
    match find_url(
        "https://mygithub.example.com/rhysd/foo",
        "some-branch",
        &env,
    ) {
        Err(Error::HttpClientError(..)) => { /* ok */ }
        v => assert!(false, "Unexpected success or error: {:?}", v),
    }
}

#[test]
fn test_invalid_url() {
    match find_url("https://", "some-branch", &helper::empty_env()) {
        Err(Error::BrokenUrl { msg, .. }) => assert!(msg.contains("empty host"), "{}", msg),
        v => assert!(false, "Unexpected success or error: {:?}", v),
    }
}
