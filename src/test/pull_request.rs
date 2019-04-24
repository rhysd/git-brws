use crate::config::{Config, EnvConfig};
use crate::pull_request::{find_page, Page};
use crate::test::helper;
use std::fs;
use std::path::Path;

macro_rules! env {
    () => {{
        let mut e = helper::empty_env();
        e.github_token = skip_if_no_token!();
        e.https_proxy = helper::https_proxy();
        e
    }};
}

fn config(branch: Option<&str>, env: EnvConfig) -> Config {
    let mut dir = std::env::current_dir().unwrap();
    dir.push(Path::new(".git"));
    let dir = fs::canonicalize(dir).unwrap();
    Config {
        repo: "dummy url not used".to_string(),
        branch: branch.map(|s| s.to_string()),
        git_dir: Some(dir),
        args: vec![],        // Unused
        stdout: false,       // Unused
        pull_request: false, // Unused
        website: false,      // Unused
        blame: false,      // Unused
        env,
    }
}

#[test]
fn test_find_pr_within_orig_repo() {
    let cfg = config(Some("async-eventloop"), env!());
    let page = find_page("api.github.com", "rhysd", "vim.wasm", &cfg).unwrap();
    assert_eq!(
        page,
        Page::Existing {
            url: "https://github.com/rhysd/vim.wasm/pull/10".to_string(),
        },
    );
}

#[test]
fn test_find_pr_from_fork_repo_url() {
    let cfg = config(Some("async-contextual-keyword"), env!());
    let page = find_page("api.github.com", "rhysd", "rust.vim", &cfg).unwrap();
    assert_eq!(
        page,
        Page::Existing {
            url: "https://github.com/rust-lang/rust.vim/pull/290".to_string(),
        },
    );
}

#[test]
fn test_find_pr_from_original_repo_url() {
    let cfg = config(Some("async-contextual-keyword"), env!());
    let page = find_page("api.github.com", "rust-lang", "rust.vim", &cfg).unwrap();
    assert_eq!(
        page,
        Page::Existing {
            url: "https://github.com/rust-lang/rust.vim/pull/290".to_string(),
        },
    );
}

#[test]
fn test_no_pr_found_at_own_repo() {
    let cfg = config(Some("unknown-branch-which-does-not-exist-for-test"), env!());
    match find_page("api.github.com", "rhysd", "git-brws", &cfg).unwrap() {
        Page::New {
            author,
            repo,
            default_branch,
            branch,
        } => {
            assert_eq!(author, "rhysd");
            assert_eq!(repo, "git-brws");
            assert_eq!(default_branch, "master");
            assert_eq!(branch, "unknown-branch-which-does-not-exist-for-test")
        }
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn test_no_pr_found_at_parent_repo() {
    let cfg = config(Some("unknown-branch-which-does-not-exist-for-test"), env!());
    match find_page("api.github.com", "rhysd", "rust.vim", &cfg).unwrap() {
        Page::New {
            author,
            repo,
            default_branch,
            branch,
        } => {
            assert_eq!(author, "rust-lang");
            assert_eq!(repo, "rust.vim");
            assert_eq!(default_branch, "master");
            assert_eq!(branch, "unknown-branch-which-does-not-exist-for-test")
        }
        p => assert!(false, "{:?}", p),
    }
}
