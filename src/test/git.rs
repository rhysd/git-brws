use crate::error::ErrorKind;
use crate::git::Git;
use std::env;

#[test]
fn git_get_current_branch() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let b = g.current_branch();
    assert!(b.is_ok(), "{:?}", b);
}

#[test]
fn git_get_commit_hash() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let b = g.current_branch().unwrap();

    for spec in &["HEAD", &b] {
        match g.hash(spec) {
            Ok(h) => {
                for c in h.chars() {
                    assert!(c.is_ascii_hexdigit(), "{:?}", h);
                }
            }
            Err(err) => assert!(false, "{:?}", err),
        };
    }
}

#[test]
fn git_get_invalid_hash() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    match g.hash("HEAD~114514").unwrap_err().kind() {
        ErrorKind::GitObjectNotFound { kind, .. } => assert_eq!(*kind, "commit"),
        e => assert!(false, "Unexpected error: {:?}", e),
    }
}

#[test]
fn git_get_tag_hash() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let h = g.tag_hash("0.10.1").unwrap();
    assert_eq!(h, "601e6c33bb760d8e7d5684a75eec1f0257b8ff22");
}

#[test]
fn git_get_invalid_tag_hash() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    match g.tag_hash("this-tag-is-not-existing").unwrap_err().kind() {
        ErrorKind::GitObjectNotFound { kind, .. } => assert_eq!(*kind, "tag name"),
        e => assert!(false, "Unexpected error: {:?}", e),
    }
}

#[test]
fn git_get_remote_url() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let u = g.remote_url("origin").unwrap();
    assert!(
        u.starts_with("git@") || u.starts_with("https://"),
        "invalid url: {:?}",
        u
    );
    assert!(u.contains("git-brws"), "invalid url: {:?}", u);
    // .git may not exist because GitHub allows omitting .git suffix in https Git URL
}

#[test]
fn git_get_invalid_remote_url() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    match g
        .remote_url("this-remote-is-not-existing")
        .unwrap_err()
        .kind()
    {
        ErrorKind::GitObjectNotFound { kind, .. } => assert_eq!(*kind, "remote"),
        e => assert!(false, "Unexpected error: {:?}", e),
    }
}

#[test]
fn git_root_dir() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let p = g.root_dir().unwrap();
    assert!(p.join(".git").exists(), "{:?}", p);
}

#[test]
fn git_check_remote_contains() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let h = g.tag_hash("0.10.1").unwrap();
    let b = g.remote_contains(&h, "origin/master").unwrap();
    assert!(
        b,
        "Hash {} is not included in remote branch origin/master",
        &h
    );
}

#[test]
fn git_get_tracking_remote() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let (url, _remote_name) = g.tracking_remote_url(&Some("master")).unwrap();
    assert!(
        url.starts_with("https://") || url.starts_with("git@") || url.starts_with("ssh://"),
        "{:?}",
        url
    );
}

#[test]
fn unknown_branch_for_tracking_remote() {
    let cwd = env::current_dir().unwrap();
    let g = Git::new(&cwd, "git");
    let err = g
        .tracking_remote_url(&Some("unknown-branch-this-is-not-existing"))
        .unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::GitCommandError{ .. }));
}
