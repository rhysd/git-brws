use crate::error::Error;
use crate::github_api::Client;
use crate::test::helper::https_proxy;

#[tokio::test]
async fn find_pr_url() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url("async-contextual-keyword", "rust-lang", "rust.vim", None)
        .await
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
        .await
        .unwrap();
    assert_eq!(
        url,
        Some("https://github.com/rust-lang/rust.vim/pull/290".to_string()),
    );
}

#[tokio::test]
async fn no_pr_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url(
            "branch-name-which-does-not-exist",
            "rust-lang",
            "rust.vim",
            Some("rhysd"),
        )
        .await
        .unwrap();
    assert_eq!(url, None);
}

#[tokio::test]
async fn find_parent() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let repo = client.repo("rhysd", "rust.vim").await.unwrap();
    let parent = repo.parent.unwrap();
    assert_eq!(parent.name, "rust.vim");
    assert_eq!(parent.owner.login, "rust-lang");
    assert_eq!(parent.default_branch, "master");
}

#[tokio::test]
async fn parent_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let parent = client.repo("rhysd", "git-brws").await.unwrap().parent;
    assert!(parent.is_none());
}

#[tokio::test]
async fn request_failure() {
    let client =
        Client::build("unknown.endpoint.example.com", None::<&str>, &None::<&str>).unwrap();
    match client.repo("rhysd", "git-brws").await {
        Ok(_) => assert!(false, "request succeeded"),
        Err(Error::HttpClientError(..)) => { /* ok */ }
        Err(e) => assert!(false, "unexpected error: {}", e),
    }
}

#[tokio::test]
async fn most_popular_repo_ok() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let repo = client
        .most_popular_repo_by_name("user:rhysd vim.wasm")
        .await
        .unwrap();
    assert_eq!(&repo.clone_url, "https://github.com/rhysd/vim.wasm.git");
}

#[tokio::test]
async fn most_popular_repo_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let err = client
        .most_popular_repo_by_name("user:rhysd this-repository-will-never-exist")
        .await
        .unwrap_err();
    match err {
        Error::NoSearchResult { .. } => { /* ok */ }
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[tokio::test]
async fn homepage() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let url = client.repo_homepage("rhysd", "git-brws").await.unwrap();
    match url {
        Some(url) => assert_eq!(&url, "https://rhysd.github.io/git-brws/"),
        url => assert!(false, "Unexpected url: {:?}", url),
    }
}

#[tokio::test]
async fn homepage_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let url = client
        .repo_homepage("rhysd", "filter-with-state")
        .await
        .unwrap();
    match url {
        None => { /* OK */ }
        url => assert!(false, "Unexpected url: {:?}", url),
    }
}

#[tokio::test]
async fn homepage_error_response() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    client
        .repo_homepage("rhysd", "this-repository-will-never-exist")
        .await
        .unwrap_err();
}
