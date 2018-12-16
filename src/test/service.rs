use crate::page::Page;
use crate::service::parse_and_build_page_url;
use crate::test::helper::empty_env;

// Note:
// git@ -> ssh://git@ conversion is done in git.rs.
#[test]
fn convert_ssh_url() {
    let p = Page::Open;
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
        assert_eq!(
            parse_and_build_page_url(&repo.to_string(), &p, &None, &empty_env()).unwrap(),
            expected
        );
    }
}

#[test]
fn parse_and_build_open_page() {
    let p = Page::Open;
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
        assert_eq!(
            parse_and_build_page_url(&repo.to_string(), &p, &None, &empty_env()).unwrap(),
            expected
        );
    }
}

#[test]
fn parse_and_build_open_branch_page() {
    let p = Page::Open;
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
        assert_eq!(
            parse_and_build_page_url(
                &repo.to_string(),
                &p,
                &Some("dev".to_string()),
                &empty_env()
            )
            .unwrap(),
            expected
        );
    }
}

#[test]
fn parse_and_build_commit_page() {
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
        assert_eq!(parse_and_build_page_url(repo, &p, &None, &empty_env()).unwrap(), expected);
    }
}

#[test]
fn parse_and_build_diff_page() {
    let p = Page::Diff {
        lhs: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        rhs: "90601f1037142605a32426f9ece0c07d479b9cc5".to_string(),
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3...90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
        (
            "https://github.somewhere.com/user/repo.git",
            "https://github.somewhere.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3...90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
        (
            "https://gitlab.com/user/repo.git",
            "https://gitlab.com/user/repo/compare/561848bad7164d7568658456088b107ec9efd9f3...90601f1037142605a32426f9ece0c07d479b9cc5",
        ),
    ] {
        assert_eq!(parse_and_build_page_url(&repo, &p, &None, &empty_env()).unwrap(), expected);
    }
    assert!(
        parse_and_build_page_url(&"https://bitbucket.org/user/repo", &p, &None, &empty_env())
            .is_err(),
        "bitbucket does not support diff page"
    );
}

#[test]
fn parse_and_build_file_page() {
    let p = Page::FileOrDir {
        relative_path: "src/main.rs".to_string(),
        hash: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        line: None,
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
        assert_eq!(parse_and_build_page_url(&repo, &p, &None, &empty_env()).unwrap(), expected);
    }
}

#[test]
fn parse_and_build_file_page_with_line_number() {
    let p = Page::FileOrDir {
        relative_path: "src/main.rs".to_string(),
        hash: "561848bad7164d7568658456088b107ec9efd9f3".to_string(),
        line: Some(12),
    };
    for &(repo, expected) in &[
        (
            "https://github.com/user/repo.git",
            "https://github.com/user/repo/blob/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#L12",
        ),
        (
            "https://bitbucket.org/user/repo.git",
            "https://bitbucket.org/user/repo/src/561848bad7164d7568658456088b107ec9efd9f3/src/main.rs#main.rs-12",
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
        assert_eq!(parse_and_build_page_url(&repo, &p, &None, &empty_env()).unwrap(), expected);
    }
}

#[test]
fn invalid_repo_url() {
    for repo in &[
        "https://github.com.git",
        "https://github.com/user.git",
        "https://unknown.hosting_service.com/user/repo.git",
    ] {
        assert!(
            parse_and_build_page_url(&repo, &Page::Open, &None, &empty_env()).is_err(),
            "{} must be invalid",
            repo
        );
    }
}

#[test]
fn customized_ssh_port() {
    let mut envs = empty_env();
    envs.ghe_ssh_port = Some("10022".to_string());
    envs.gitlab_ssh_port = Some("10022".to_string());

    let p = Page::Open;
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
        assert_eq!(
            parse_and_build_page_url(&repo, &p, &None, &envs),
            Ok(expected.to_string())
        );
    }
}

#[test]
fn customized_ghe_host() {
    let mut envs = empty_env();
    envs.ghe_url_host = Some("my-original-ghe.org".to_string());

    for (port, expected) in &[
        (None, "https://my-original-ghe.org/user/repo"),
        (
            Some("10022".to_string()),
            "https://my-original-ghe.org:10022/user/repo",
        ),
    ] {
        envs.ghe_ssh_port = port.clone();
        assert_eq!(
            parse_and_build_page_url(
                &"https://my-original-ghe.org/user/repo.git",
                &Page::Open,
                &None,
                &envs,
            ),
            Ok(expected.to_string()),
        );
    }
}
