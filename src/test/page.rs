use crate::command::Config;
use crate::page::{parse_page, Page};
use crate::test::helper::empty_env;
use std::env;
use std::path::Path;

fn config(repo: &str, branch: Option<&str>, args: Vec<&str>) -> Config {
    let mut dir = env::current_dir().unwrap();
    dir.push(Path::new(".git"));
    let mut a = Vec::new();
    for arg in args {
        a.push(arg.to_string());
    }
    Config {
        repo: repo.to_string(),
        branch: branch.map(|s| s.to_string()),
        git_dir: dir,
        args: a,
        stdout: false,
        env: empty_env(),
    }
}

#[test]
fn parse_empty_args() {
    let c = config("https://github.com/user/repo.git", None, vec![]);
    match parse_page(&c).unwrap() {
        Page::Open => { /* OK */ }
        _ => assert!(false),
    }
}

#[test]
fn parse_file_or_dir() {
    for &(entry, relative) in &[
        ("./README.md", "README.md"),
        ("src", "src"),
        ("./src/main.rs", "src/main.rs"),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![&entry]);
        match parse_page(&c).unwrap() {
            Page::FileOrDir {
                relative_path,
                hash,
                line: None,
            } => {
                assert_eq!(relative_path, relative);
                assert!(!hash.is_empty());
            }
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_file_line() {
    for &(file, expected) in &[
        ("./README.md#21", Some(21)),
        ("./src/main.rs#10", Some(10)),
        ("LICENSE.txt", None),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![&file]);
        match parse_page(&c).unwrap() {
            Page::FileOrDir {
                relative_path: _,
                hash: _,
                line,
            } => {
                assert_eq!(line, expected);
            }
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn not_exsiting_file() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["not/existing/file.txt"],
    );
    assert!(parse_page(&c).is_err());
}

#[test]
fn file_at_specific_commit() {
    let c = config("https://github.com/user/repo.git", None, vec![&"README.md"]);
    let p = parse_page(&c).unwrap();
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec![&"README.md", "HEAD^"],
    );
    let p2 = parse_page(&c).unwrap();
    assert!(p != p2, format!("{:?} v.s. {:?}", p, p2));
}

#[test]
fn parse_commit_ref() {
    for &cm in &["HEAD", "HEAD~1", "HEAD^"] {
        let c = config("https://github.com/user/repo.git", None, vec![cm]);
        match parse_page(&c).unwrap() {
            Page::Commit { hash } => assert!(!hash.is_empty()),
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_short_commit_hash() {
    for &(cm, expected) in &[
        ("499edbb", "499edbbbad4d8054e4a47e12944e5fb4a2ef7ec5"),
        (
            "bc869a14617a131fefe8fa1a3dcdeba0745880d5",
            "bc869a14617a131fefe8fa1a3dcdeba0745880d5",
        ),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![cm]);
        match parse_page(&c).unwrap() {
            Page::Commit { hash } => assert_eq!(hash, expected),
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_diff_ref_name() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["HEAD^..HEAD"],
    );
    match parse_page(&c).unwrap() {
        Page::Diff { lhs, rhs } => {
            assert!(!lhs.is_empty());
            assert!(!rhs.is_empty());
        }
        p => assert!(false, "Unexpected result: {:?}", p),
    }
}

#[test]
fn parse_diff() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["499edbb..bc869a1"],
    );
    match parse_page(&c).unwrap() {
        Page::Diff { lhs, rhs } => {
            assert_eq!(lhs, "499edbbbad4d8054e4a47e12944e5fb4a2ef7ec5");
            assert_eq!(rhs, "bc869a14617a131fefe8fa1a3dcdeba0745880d5");
        }
        p => assert!(false, "Unexpected result: {:?}", p),
    }
}

#[test]
fn wrong_num_of_args() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["foo", "bar", "piyo"],
    );
    assert!(parse_page(&c).is_err());
}

#[test]
fn unknown_diff() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["HEAD~114514..HEAD~114513"],
    );
    assert!(parse_page(&c).is_err());
}

#[test]
fn file_for_unknown_commit() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["README.md", "HEAD~114514"],
    );
    assert!(parse_page(&c).is_err());
}
