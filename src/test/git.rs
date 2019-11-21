use crate::error::Error;
use crate::git::{git_dir, Git};
use std::env;

#[test]
fn get_git_dir() {
    let current = env::current_dir().unwrap();
    for (ref d, ref c) in &[
        (None, ""),
        (Some(&current), ""),
        (None, "git"),
        (Some(&current), "git"),
    ] {
        let dir = git_dir(d.clone(), c).unwrap();
        assert!(dir.ends_with(".git"), "{:?} with {:?}, {:?}", dir, d, c);
    }
}

#[test]
fn git_get_current_branch() {
    let g = Git::new(".git".as_ref(), "git");
    let b = g.current_branch();
    assert!(b.is_ok(), "{:?}", b);
}

#[test]
fn git_get_commit_hash() {
    let g = Git::new(".git".as_ref(), "git");
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
    let g = Git::new(".git".as_ref(), "git");
    match g.hash("HEAD~114514") {
        Err(Error::GitObjectNotFound { kind, .. }) => assert_eq!(kind, "commit"),
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}

#[test]
fn git_get_tag_hash() {
    let g = Git::new(".git".as_ref(), "git");
    let h = g.tag_hash("0.10.1").unwrap();
    assert_eq!(h, "601e6c33bb760d8e7d5684a75eec1f0257b8ff22");
}

#[test]
fn git_get_invalid_tag_hash() {
    let g = Git::new(".git".as_ref(), "git");
    match g.tag_hash("this-tag-is-not-existing") {
        Err(Error::GitObjectNotFound { kind, .. }) => assert_eq!(kind, "tag name"),
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}

#[test]
fn git_get_remote_url() {
    let g = Git::new(".git".as_ref(), "git");
    let u = g.remote_url("origin").unwrap();
    assert!(
        u.starts_with("git@") || u.starts_with("https://"),
        "invalid url: {:?}",
        u
    );
    assert!(u.contains("git-brws"), "invalid url: {:?}", u);
    assert!(u.ends_with(".git"), "invalid url: {:?}", u);
}

#[test]
fn git_get_invalid_remote_url() {
    let g = Git::new(".git".as_ref(), "git");
    match g.remote_url("this-remote-is-not-existing") {
        Err(Error::GitObjectNotFound { kind, .. }) => assert_eq!(kind, "remote"),
        r => assert!(false, "Unexpected result: {:?}", r),
    }
}

#[test]
fn git_root_dir() {
    let g = Git::new(".git".as_ref(), "git");
    let p = g.root_dir().unwrap();
    assert!(p.join(".git").exists(), "{:?}", p);
}

#[test]
fn git_check_remote_contains() {
    let g = Git::new(".git".as_ref(), "git");
    let h = g.tag_hash("0.10.1").unwrap();
    let b = g.remote_contains(&h, "origin/master").unwrap();
    assert!(b);
}
