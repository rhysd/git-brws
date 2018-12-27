use crate::error::Error;
use crate::github_api::Client;
use crate::test::helper::https_proxy;

#[test]
fn test_find_pr_url() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url("async-contextual-keyword", "rust-lang", "rust.vim", None)
        .unwrap();
    assert_eq!(
        url,
        Some("https://github.com/rust-lang/rust.vim/pull/290".to_string()),
    );
    let url = client
        .find_pr_url(
            "async-contextual-keyword",
            "rust-lang",
            "rust.vim",
            Some("rhysd"),
        )
        .unwrap();
    assert_eq!(
        url,
        Some("https://github.com/rust-lang/rust.vim/pull/290".to_string()),
    );
}

#[test]
fn test_no_pr_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url(
            "branch-name-which-does-not-exist",
            "rust-lang",
            "rust.vim",
            Some("rhysd"),
        )
        .unwrap();
    assert_eq!(url, None);
}

#[test]
fn test_find_parent() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let parent = client.parent_repo("rhysd", "rust.vim").unwrap();
    assert_eq!(
        parent,
        Some(("rust-lang".to_string(), "rust.vim".to_string())),
    );
}

#[test]
fn test_parent_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let parent = client.parent_repo("rhysd", "git-brws").unwrap();
    assert_eq!(parent, None);
}

#[test]
fn test_request_failure() {
    let client =
        Client::build("unknown.endpoint.example.com", None::<&str>, &None::<&str>).unwrap();
    match client.parent_repo("rhysd", "git-brws") {
        Ok(_) => assert!(false, "request succeeded"),
        Err(Error::HttpClientError(..)) => { /* ok */ }
        Err(e) => assert!(false, "unexpected error: {}", e),
    }
}
