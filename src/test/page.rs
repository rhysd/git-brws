use crate::command::Config;
use crate::error::Error;
use crate::page::{parse_page, DiffOp, Line, Page};
use crate::test::helper::empty_env;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn config(repo: &str, branch: Option<&str>, args: Vec<&str>) -> Config {
    let mut dir = env::current_dir().unwrap();
    dir.push(Path::new(".git"));
    let dir = fs::canonicalize(dir).unwrap();
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
        pull_request: false,
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
        (
            Path::new(".").join("README.md").as_path(),
            Path::new("README.md"),
        ),
        (Path::new("src"), Path::new("src")),
        (
            Path::new(".").join("src").join("main.rs").as_path(),
            Path::new("src").join("main.rs").as_path(),
        ),
        // Contains '..'
        (
            Path::new(".")
                .join("src")
                .join("..")
                .join("README.md")
                .as_path(),
            Path::new("README.md"),
        ),
        // Suffix '..'
        (
            Path::new(".").join("src").join("test").join("..").as_path(),
            Path::new("src"),
        ),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![&entry.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FileOrDir {
                relative_path,
                hash,
                line: None,
            } => {
                assert_eq!(relative_path, relative.to_str().unwrap());
                assert!(!hash.is_empty());
            }
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_file_single_line() {
    for &(ref file, ref expected) in &[
        (Path::new(".").join("README.md#21"), Some(Line::At(21))),
        (
            Path::new(".").join("src").join("main.rs#10"),
            Some(Line::At(10)),
        ),
        (PathBuf::from("LICENSE.txt"), None),
        (
            Path::new(".").join("src").join("..").join("README.md#21"),
            Some(Line::At(21)),
        ),
        (Path::new(".").join("README.md#L21"), Some(Line::At(21))),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![&file.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FileOrDir { ref line, .. } => assert_eq!(line, expected, "input: {:?}", file),
            p => assert!(false, "Unexpected result: {:?}, input: {:?}", p, file),
        }
    }
}

#[test]
fn not_existing_file() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["not/existing/file.txt"],
    );
    assert!(parse_page(&c).is_err());
}

#[test]
fn file_at_specific_commit() {
    let c = config("https://github.com/user/repo.git", None, vec!["README.md"]);
    let p = parse_page(&c).unwrap();
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["README.md", "HEAD^"],
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
#[cfg_attr(feature = "on-ci", ignore)]
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
    for &(arg, expected_op) in &[
        ("HEAD^..HEAD", DiffOp::TwoDots),
        ("HEAD^...HEAD", DiffOp::ThreeDots),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![arg]);
        match parse_page(&c).unwrap() {
            Page::Diff { lhs, rhs, op } => {
                assert!(!lhs.is_empty());
                assert!(!rhs.is_empty());
                assert_eq!(op, expected_op, "arg is {}", arg);
            }
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
#[cfg_attr(feature = "on-ci", ignore)]
fn parse_diff_revisions() {
    for &(arg, expected_op) in &[
        ("499edbb..bc869a1", DiffOp::TwoDots),
        ("499edbb...bc869a1", DiffOp::ThreeDots),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![arg]);
        match parse_page(&c).unwrap() {
            Page::Diff { lhs, rhs, op } => {
                assert_eq!(lhs, "499edbbbad4d8054e4a47e12944e5fb4a2ef7ec5");
                assert_eq!(rhs, "bc869a14617a131fefe8fa1a3dcdeba0745880d5");
                assert_eq!(op, expected_op, "arg is {}", arg);
            }
            p => assert!(false, "Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn wrong_num_of_args() {
    let c = config(
        "https://github.com/user/repo.git",
        None,
        vec!["foo", "bar", "piyo", "blah"],
    );
    match parse_page(&c) {
        Err(Error::PageParseError { attempts, .. }) => {
            assert!(!attempts.is_empty());
            for err in attempts {
                match err {
                    Error::WrongNumberOfArgs { actual, .. } => assert_eq!(actual, 4),
                    err => assert!(false, "Unexpected error: {}", err),
                }
            }
        }
        v => assert!(false, "Unexpected success or error: {:?}", v),
    }
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

#[test]
fn diff_lhs_or_rhs_empty() {
    for ref path in &[
        Path::new("..").join("foo.txt"),
        Path::new("foo").join(".."),
        PathBuf::from(".."),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![path.to_str().unwrap()],
        );
        match parse_page(&c) {
            Ok(p @ Page::Diff { .. }) => assert!(false, "Unexpectedly parsed as diff: {:?}", p),
            _ => { /*ok*/ }
        }
    }
}

#[test]
fn issue_number() {
    let c = config("https://github.com/user/repo.git", None, vec!["#123"]);
    match parse_page(&c) {
        Ok(Page::Issue { number }) => assert_eq!(number, 123),
        v => assert!(false, "Unexpected result {:?}", v),
    }
}

#[test]
fn line_cannot_be_set_to_dir() {
    let c = config("https://github.com/user/repo.git", None, vec!["src#123"]);
    match parse_page(&c) {
        Err(Error::PageParseError { attempts, .. }) => assert!(
            attempts.iter().any(|err| match err {
                Error::LineSpecifiedForDir(path) => format!("{:?}", path).contains("src"),
                _ => false,
            }),
            "{:?}",
            attempts,
        ),
        v => assert!(false, "Unexpected result {:?}", v),
    }
}

#[test]
fn parse_file_line_range() {
    for file in &[
        Path::new("README.md#1-2"),
        Path::new("README.md#L1-2"),
        Path::new("README.md#1-L2"),
        Path::new("README.md#L1-L2"),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![&file.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FileOrDir { line, .. } => {
                assert_eq!(line, Some(Line::Range(1, 2)), "input: {:?}", file)
            }
            p => assert!(false, "Unexpected result: {:?}, input: {:?}", p, file),
        }
    }
}
