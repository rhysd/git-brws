use crate::config::{Config, EnvConfig};
use crate::error::Error;
use crate::page::{DiffOp, Line, Page};
use crate::service::build_page_url;
use crate::test::helper::{empty_env, https_proxy};
use std::fs;
use std::path::Path;

const OPEN: Page = Page::Open {
    website: false,
    pull_request: false,
};
const OPEN_WEBSITE: Page = Page::Open {
    website: true,
    pull_request: true,
};
const OPEN_PR: Page = Page::Open {
    website: false,
    pull_request: true,
};

fn config(repo: &str, branch: Option<&str>, env: Option<EnvConfig>) -> Config {
    let mut dir = std::env::current_dir().unwrap();
    dir.push(Path::new(".git"));
    let dir = fs::canonicalize(dir).unwrap();
    Config {
        repo: repo.to_string(),
        branch: branch.map(|s| s.to_string()),
        git_dir: Some(dir),
        args: vec![],
        stdout: false,
        pull_request: false,
        website: false,
        blame: false,
        env: env.unwrap_or_else(empty_env),
    }
}

fn config_for_pr(token: Option<String>, repo: &str, branch: Option<&str>) -> Config {
    let mut dir = std::env::current_dir().unwrap();
    dir.push(Path::new(".git"));
    let dir = fs::canonicalize(dir).unwrap();

    let mut env = empty_env();
    env.github_token = token;
    env.https_proxy = https_proxy();
    let env = env;

    Config {
        repo: repo.to_string(),
        branch: branch.map(|b| b.to_string()),
        git_dir: Some(dir),
        args: vec![],
        stdout: false,
        pull_request: true,
        website: false,
        blame: false,
        env,
    }
}

// Note:
// git@ -> ssh://git@ conversion is done in git.rs.
#[test]
fn convert_ssh_url() {
    for &(repo, expected) in &[
        (
            "ssh://git@github.com:22/user/repo.git",
            "https://github.com/user/repo",
        ),
        (
            "ssh://git@bitbucket.org:22/user/repo.git",
            "https://bitbucket.org/user/repo",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&OPEN, &c).unwrap(), expected);
    }
}

#[test]
fn open_page_url() {
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&OPEN, &c).unwrap(), expected);
    }
}

#[test]
fn open_branch_page_url() {
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/tree/dev",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/branch/dev",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/tree/dev",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/tree/dev",
        ),
        (
            "https://gitlab.somewhere.com/user/repo.git",
            "https://gitlab.somewhere.com/user/repo/tree/dev",
        ),
    ] {
        let c = config(repo, Some("dev"), None);
        assert_eq!(build_page_url(&OPEN, &c).unwrap(), expected);
    }
}

#[test]
fn commit_page_url() {
    let p = Page::Commit {
        hash: "90601f1037142605a32426f9ece0c07d479b9cc5".to_string(),
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/commit/90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/commits/90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/commit/90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/commit/90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&p, &c).unwrap(), expected);
    }
}

#[test]
fn diff_page_url() {
    for (ref op, ref opstr) in &[(DiffOp::TwoDots, ".."), (DiffOp::ThreeDots, "...")] {
        let p = Page::Diff {
            lhs: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
            rhs: "90601f1037142605a32426f9ece0c07d479b9cc5".to_string(),
            op: *op,
        };

        // github-like
        for &(repo, expected) in &[
            (
                "https://github.com/user/repo.git",
                format!("https://github.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3{}90601f1037142605a32426f9ece0c07d479b9cc5", opstr).as_str(),
            ),
            (
                "https://github.somewhere.com/user/repo.git",
                format!("https://github.somewhere.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3{}90601f1037142605a32426f9ece0c07d479b9cc5", opstr).as_str(),
            ),
        ] {
            let c = config(repo, None, None);
            assert_eq!(build_page_url(&p, &c).unwrap(), expected, "for {:?}", op);
        }
    }
}

#[test]
fn diff_page_for_gitlab_url() {
    fn page(op: DiffOp) -> Page {
        Page::Diff {
            lhs: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
            rhs: "90601f1037142605a32426f9ece0c07d479b9cc5".to_string(),
            op,
        }
    }

    let p = page(DiffOp::TwoDots);
    let u = "https://gitlab.com/user/repo.git";
    let c = config(u, None, None);
    assert!(
        build_page_url(&p, &c).is_err(),
        "GitLab does not support '..' but error did not occur"
    );

    let p = page(DiffOp::ThreeDots);
    let u = "https://gitlab.com/user/repo.git";
    let c = config(u, None, None);
    assert_eq!(
        build_page_url(&p, &c).unwrap(),
        "https://gitlab.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3...90601f1037142605a32426f9ece0c07d479b9cc5",
    );
}

#[test]
fn diff_page_for_bitbucket_url() {
    let p = Page::Diff {
        lhs: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        rhs: "90601f1037142605a32426f9ece0c07d479b9cc5".to_string(),
        op: DiffOp::ThreeDots,
    };
    let c = config("https://bitbucket.org/user/repo", None, None);
    assert!(
        build_page_url(&p, &c).is_err(),
        "bitbucket does not support diff page"
    );
}

#[test]
fn file_page_url() {
    let p = Page::FileOrDir {
        relative_path: Path::new("src")
            .join("main.rs")
            .to_string_lossy()
            .into_owned(),
        hash: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        line: None,
        blame: false,
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/src/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&p, &c).unwrap(), expected);
    }
}

#[test]
fn file_page_with_line_number_url() {
    let p = Page::FileOrDir {
        relative_path: Path::new("src")
            .join("main.rs")
            .to_string_lossy()
            .into_owned(),
        hash: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        line: Some(Line::At(12)),
        blame: false,
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L12",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/src/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#lines-12",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L12",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L12",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&p, &c).unwrap(), expected);
    }
}

#[test]
fn file_page_with_line_range_url() {
    let p = Page::FileOrDir {
        relative_path: Path::new("src")
            .join("main.rs")
            .to_string_lossy()
            .into_owned(),
        hash: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        line: Some(Line::Range(1, 2)),
        blame: false,
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L1-L2",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/src/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#lines-1:2",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L1-L2",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L1-L2",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&p, &c).unwrap(), expected);
    }
}

#[test]
fn invalid_repo_url() {
    for repo in &[
        "https://github.com.git",
        "https://github.com/user.git",
        "https://unknown.hosting_service.com/user/repo.git",
    ] {
        let c = config(repo, None, None);
        assert!(
            build_page_url(&OPEN, &c).is_err(),
            "{} must be invalid",
            repo
        );
    }
}

#[test]
fn customized_ssh_port() {
    let mut env = empty_env();
    env.ghe_ssh_port = Some(10022);
    env.gitlab_ssh_port = Some(10022);

    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com:10022/user/repo",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo",
        ),
        (
            "https://gitlab.somewhere.com/user/repo.git",
            "https://gitlab.somewhere.com:10022/user/repo",
        ),
    ] {
        let c = config(repo, None, Some(env.clone()));
        assert_eq!(build_page_url(&OPEN, &c).unwrap(), expected.to_string(),);
    }
}

#[test]
fn customized_ghe_host() {
    let mut env = empty_env();
    env.ghe_url_host = Some("my-original-ghe.org".to_string());

    for (port, expected) in &[
        (None, "https://my-original-ghe.org/user/repo"),
        (Some(10022), "https://my-original-ghe.org:10022/user/repo"),
    ] {
        env.ghe_ssh_port = port.clone();
        let c = config(
            "https://my-original-ghe.org/user/repo.git",
            None,
            Some(env.clone()),
        );
        assert_eq!(build_page_url(&OPEN, &c).unwrap(), expected.to_string(),);
    }
}

#[test]
fn broken_repo_url() {
    let env = &empty_env();
    for &url in &[
        "https://foo@/foo.bar", // empty host
        "https://foo bar",      // invalid domain character
    ] {
        let c = config(url, None, Some(env.clone()));
        match build_page_url(&OPEN, &c) {
            Err(Error::BrokenUrl { .. }) => { /* ok */ }
            v => assert!(false, "Unexpected error or success: {:?}", v),
        }
    }
}

#[test]
fn issue_number_url() {
    let p = Page::Issue { number: 123 };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/issues/123",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/issues/123",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/issues/123",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/issues/123",
        ),
    ] {
        let c = config(repo, None, None);
        assert_eq!(build_page_url(&p, &c).unwrap(), expected);
    }
}

#[test]
fn unknown_github_enterprise_url() {
    let mut env = empty_env();
    env.ghe_url_host = Some("github-yourcompany.com".to_string());
    let c = config(
        "https://github-othercompany.com/foo/bar.git",
        None,
        Some(env),
    );
    match build_page_url(&OPEN, &c).unwrap_err() {
        Error::UnknownHostingService { .. } => { /* OK */ }
        err => assert!(false, "Unexpected error: {}", err),
    }

    let mut c = config_for_pr(None, "https://github-othercompany.com/foo/bar.git", None);
    c.env.ghe_url_host = Some("github-yourcompany.com".to_string());
    match build_page_url(&OPEN_PR, &c).unwrap_err() {
        Error::UnknownHostingService { .. } => { /* OK */ }
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[test]
fn website_github_pages() {
    let mut env = empty_env();
    env.github_token = skip_if_no_token!();
    env.https_proxy = https_proxy();
    let env = env;
    let testcases = &[
        (
            "https://github.com/rhysd/git-brws.git", // With gh-pages
            "https://rhysd.github.io/git-brws/",
        ),
        (
            "https://github.com/rhysd/dogfiles", // Without gh-pages, but with homepage
            "https://rhysd.github.io",
        ),
        (
            "https://github.com/rhysd/filter-with-state", // Without homepage
            "https://rhysd.github.io/filter-with-state",
        ),
    ];
    for (url, expected) in testcases {
        let c = config(url, None, Some(env.clone()));
        let actual = build_page_url(&OPEN_WEBSITE, &c).unwrap();
        assert_eq!(*expected, &actual);
    }
}

#[test]
fn website_github_enterprise_pages() {
    let mut env = empty_env();
    env.ghe_url_host = Some("yourcompany-github.com".to_string());
    env.https_proxy = https_proxy();
    let env = env;

    // TODO: Tests for the case where domain isolation is enabled are missing
    // TODO: Tests with actual GitHub Enterprise instance
    let testcases = &[
        (
            "https://github.yourcompany.com/foo/bar.git",
            "https://github.yourcompany.com/pages/foo/bar",
        ),
        (
            "https://yourcompany-github.com/foo/bar.git",
            "https://yourcompany-github.com/pages/foo/bar",
        ),
    ];
    for (url, expected) in testcases {
        let c = config(url, None, Some(env.clone()));
        let actual = build_page_url(&OPEN_WEBSITE, &c).unwrap();
        assert_eq!(*expected, &actual);
    }
}

#[test]
fn website_gitlab_pages() {
    let env = empty_env();
    let testcases = &[
        (
            "https://gitlab.com/foo/bar.git",
            "https://foo.gitlab.io/bar",
        ),
        (
            "https://gitlab.example.com/foo/bar.git",
            "https://foo.gitlab.example.com/bar",
        ),
    ];
    for (url, expected) in testcases {
        let c = config(url, None, Some(env.clone()));
        let actual = build_page_url(&OPEN_WEBSITE, &c).unwrap();
        assert_eq!(*expected, &actual);
    }
}

#[test]
fn website_bitbucket_cloud() {
    let mut env = empty_env();
    env.https_proxy = https_proxy();
    let testcases = &[
        (
            "https://bitbucket.org/rhysd/bar.git", // Fall back to user page
            "https://rhysd.bitbucket.io",
        ),
        (
            "https://bitbucket.org/rhysd/bb-cloud-test.git",
            "https://rhysd.bitbucket.io/bb-cloud-test",
        ),
    ];
    for (url, expected) in testcases {
        let c = config(url, None, Some(env.clone()));
        let actual = build_page_url(&OPEN_WEBSITE, &c).unwrap();
        assert_eq!(*expected, &actual);
    }
}

#[test]
fn pull_request_page_url_with_branch() {
    let cfg = config_for_pr(
        skip_if_no_token!(),
        "https://github.com/rust-lang/rust.vim.git",
        Some("async-contextual-keyword"),
    );

    let url = build_page_url(&OPEN_PR, &cfg).unwrap();
    assert_eq!(&url, "https://github.com/rust-lang/rust.vim/pull/290");
}

#[test]
fn pull_request_create_page_url_at_own_repo() {
    let cfg = config_for_pr(
        skip_if_no_token!(),
        "https://github.com/rhysd/git-brws.git",
        Some("this-branch-never-existing"),
    );

    let url = build_page_url(&OPEN_PR, &cfg).unwrap();
    assert_eq!(
        &url,
        "https://github.com/rhysd/git-brws/compare/master...this-branch-never-existing"
    );
}

#[test]
fn pull_request_create_page_url_at_parent_repo() {
    let cfg = config_for_pr(
        skip_if_no_token!(),
        "https://github.com/rhysd/rust.vim.git",
        Some("this-branch-never-existing"),
    );

    let url = build_page_url(&OPEN_PR, &cfg).unwrap();
    assert_eq!(
        &url,
        "https://github.com/rust-lang/rust.vim/compare/master...this-branch-never-existing"
    );
}

#[test]
fn pull_request_page_url_retrieving_branch_from_git_dir() {
    let cfg = config_for_pr(
        skip_if_no_token!(),
        "https://github.com/rhysd/git-brws.git",
        None,
    );

    // Accept both error and page since current branch may be for pull request
    match build_page_url(&OPEN_PR, &cfg) {
        Ok(url) => {
            assert!(
                url.contains("git-brws"),
                "URL is not for git-brws repo: {}",
                url
            );
        }
        result => assert!(false, "Unexpected result: {:?}", result),
    }
}

#[test]
fn pull_request_page_url_without_branch_outside_git_repo() {
    let mut cfg = config_for_pr(None, "ssh://git@github.com:22/rhysd/git-brws.git", None);
    cfg.git_dir = None;
    match build_page_url(&OPEN_PR, &cfg).unwrap_err() {
        Error::NoLocalRepoFound { operation } => assert!(
            operation.contains("opening a pull request"),
            "Unexpected operation: {}",
            operation
        ),
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[test]
fn pull_request_unsupported_services() {
    let urls = &[
        "https://gitlab.com/foo/bar.git",
        "https://gitlab.yourcompany.com/foo/bar.git",
        "https://bitbucket.org/foo/bar.git",
        "ssh://git@gitlab.com:22/foo/bar.git",
        "ssh://git@gitlab.yourcompany.com:22/foo/bar.git",
        "ssh://git@bitbucket.org:22/foo/bar.git",
    ];
    for url in urls {
        let cfg = config_for_pr(None, url, None);
        match build_page_url(&OPEN_PR, &cfg).unwrap_err() {
            Error::PullReqNotSupported { .. } => { /* OK */ }
            err => assert!(false, "Unexpected error for URL {}: {}", url, err),
        }
    }
}

#[test]
fn pull_request_github_enterprise_with_no_token() {
    let cfg = config_for_pr(None, "https://github.yourcompany.com/foo/bar.git", None);
    match build_page_url(&OPEN_PR, &cfg).unwrap_err() {
        Error::GheTokenRequired => { /* OK */ }
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[test]
fn tab_page_for_github_and_gitlab() {
    let hosts = &[
        "github.com",
        "github.yourcompany.com",
        "gitlab.com",
        "gitlab.yourcompany.com",
    ];
    let page = Page::Tag {
        tagname: "tag".to_string(),
        commit: "01234cdef".to_string(),
    };
    for host in hosts {
        let expected = format!("https://{}/foo/bar/tree/tag", host);
        for url in &[
            format!("https://{}/foo/bar.git", host),
            format!("ssh://git@{}:22/foo/bar.git", host),
        ] {
            let c = config(url, None, None);
            let actual = build_page_url(&page, &c).unwrap();
            assert_eq!(actual, expected, "{}", url);
        }
    }
}

#[test]
fn tab_page_for_bitbucket() {
    let page = Page::Tag {
        tagname: "tag".to_string(),
        commit: "01234cdef".to_string(),
    };
    let expected = "https://bitbucket.org/user/repo/commits/01234cdef";
    for url in &[
        "https://bitbucket.org/user/repo",
        "ssh://git@bitbucket.org:22/user/repo.git",
    ] {
        let c = config(url, None, None);
        let actual = build_page_url(&page, &c).unwrap();
        assert_eq!(actual, expected, "{}", url);
    }
}
