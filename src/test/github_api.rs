use crate::error::ErrorKind;
use crate::github_api::Client;
use crate::test::helper::https_proxy;

#[tokio::test]
async fn find_pr_url() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
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
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
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
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let repo = client.repo("rhysd", "rust.vim").await.unwrap();
    let parent = repo.parent.unwrap();
    assert_eq!(parent.name, "rust.vim");
    assert_eq!(parent.owner.login, "rust-lang");
}

#[tokio::test]
async fn parent_not_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let parent = client.repo("rhysd", "git-brws").await.unwrap().parent;
    assert!(parent.is_none());
}

#[tokio::test]
async fn request_failure() {
    let client =
        Client::build("unknown.endpoint.example.com", &None::<&str>, &None::<&str>).unwrap();
    match client.repo("rhysd", "git-brws").await.unwrap_err().kind() {
        ErrorKind::HttpClientError(..) => { /* ok */ }
        e => panic!("unexpected error: {}", e),
    }
}

#[tokio::test]
async fn most_popular_repo_ok() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let repo = client
        .most_popular_repo_by_name("user:rhysd vim.wasm")
        .await
        .unwrap();
    assert_eq!(&repo.clone_url, "https://github.com/rhysd/vim.wasm.git");
}

#[tokio::test]
async fn most_popular_repo_not_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let err = client
        .most_popular_repo_by_name("user:rhysd this-repository-will-never-exist")
        .await
        .unwrap_err();
    match err.kind() {
        ErrorKind::NoSearchResult { .. } => { /* ok */ }
        err => panic!("Unexpected error: {}", err),
    }
}

#[tokio::test]
async fn homepage() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let url = client.repo_homepage("rhysd", "git-brws").await.unwrap();
    match url {
        Some(url) => assert_eq!(&url, "https://rhysd.github.io/git-brws/"),
        url => panic!("Unexpected url: {:?}", url),
    }
}

#[tokio::test]
async fn homepage_not_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    let url = client
        .repo_homepage("rhysd", "filter-with-state")
        .await
        .unwrap();
    match url {
        None => { /* OK */ }
        url => panic!("Unexpected url: {:?}", url),
    }
}

#[tokio::test]
async fn homepage_error_response() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", &token, &https_proxy()).unwrap();
    client
        .repo_homepage("rhysd", "this-repository-will-never-exist")
        .await
        .unwrap_err();
}
