use crate::config::Config;
use crate::error::ErrorKind;
use crate::page::{parse_page, DiffOp, Line, Page};
use crate::test::helper::{empty_env, get_root_dir};
use std::env;
use std::path::{Path, PathBuf};

fn config(repo: &str, branch: Option<&str>, args: Vec<&str>) -> Config {
    Config {
        repo_url: repo.to_string(),
        branch: branch.map(|s| s.to_string()),
        cwd: env::current_dir().unwrap(),
        args: args.into_iter().map(String::from).collect(),
        stdout: false,
        pull_request: false,
        website: false,
        blame: false,
        remote: None,
        env: empty_env(),
    }
}

#[test]
fn parse_empty_args() {
    let mut c = config("https://github.com/user/repo.git", None, vec![]);
    match parse_page(&c).unwrap() {
        Page::Open {
            website: false,
            pull_request: false,
        } => { /* OK */ }
        p => panic!("{:?}", p),
    }

    // It still works even if .git was not found (#9)
    c.cwd = get_root_dir();
    match parse_page(&c).unwrap() {
        Page::Open {
            website: false,
            pull_request: false,
        } => { /* OK */ }
        p => panic!("{:?}", p),
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
        // Prefix '..'
        (
            Path::new("..")
                .join(std::env::current_dir().unwrap().file_name().unwrap())
                .join("src")
                .as_path(),
            Path::new("src"),
        ),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![entry.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FilePath {
                relative_path,
                hash,
                line: None,
                blame,
                is_dir,
            } => {
                assert_eq!(relative_path, relative.to_str().unwrap());
                assert!(!hash.is_empty());
                assert!(!blame);
                assert_eq!(is_dir, relative.is_dir());
            }
            p => panic!("Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_file_or_dir_for_blame() {
    let entry = Path::new(".").join("README.md");

    let mut c = config(
        "https://github.com/user/repo.git",
        None,
        vec![entry.to_str().unwrap()],
    );
    c.blame = true;
    let c = c;

    match parse_page(&c).unwrap() {
        Page::FilePath { blame, .. } => assert!(blame),
        p => panic!("Unexpected result: {:?}", p),
    }
}

#[test]
fn parse_file_single_line() {
    for (file, expected) in &[
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
            vec![file.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FilePath { line, .. } => assert_eq!(&line, expected, "input: {:?}", file),
            p => panic!("Unexpected result: {:?}, input: {:?}", p, file),
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
        vec!["README.md", "dbb66be9b78ecddef734d2f9cf8c2c7a2836145b"],
    );
    let p2 = parse_page(&c).unwrap();
    assert_ne!(p, p2);
}

#[test]
fn parse_commit_ref() {
    for &cm in &["HEAD", "HEAD~1", "HEAD^", "HEAD^^"] {
        let c = config("https://github.com/user/repo.git", None, vec![cm]);
        match parse_page(&c).unwrap() {
            Page::Commit { hash } => assert!(!hash.is_empty(), "{} for {}", hash, cm),
            p => panic!("Unexpected result: {:?} for {}", p, cm),
        }
    }
}

#[test]
fn parse_commit_branch_ref() {
    for spec in &["master", "master@{1month}"] {
        let c = config("https://github.com/user/repo.git", None, vec![spec]);
        match parse_page(&c).unwrap() {
            Page::Commit { hash } => assert!(!hash.is_empty(), "{} for {}", hash, spec),
            p => panic!("Unexpected result: {:?} for {}", p, spec),
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
            p => panic!("Unexpected result: {:?}", p),
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
            p => panic!("Unexpected result: {:?}", p),
        }
    }
}

#[test]
fn parse_diff_branch_spec() {
    for &(arg, expected_op) in &[
        ("master...HEAD", DiffOp::ThreeDots),
        ("master@{1month}..master@{1day}", DiffOp::TwoDots),
    ] {
        let c = config("https://github.com/user/repo.git", None, vec![arg]);
        match parse_page(&c).unwrap() {
            Page::Diff { lhs, rhs, op } => {
                assert!(!lhs.is_empty());
                assert!(!rhs.is_empty());
                assert_eq!(op, expected_op, "arg is {}", arg);
            }
            p => panic!("Unexpected result: {:?}", p),
        }
    }
}

#[test]
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
            p => panic!("Unexpected result: {:?}", p),
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
    match parse_page(&c).unwrap_err().kind() {
        ErrorKind::PageParseError { attempts, .. } => {
            assert!(!attempts.is_empty());
            for (_, err) in attempts {
                match err.kind() {
                    ErrorKind::WrongNumberOfArgs { actual, .. } => assert_eq!(*actual, 4),
                    err => panic!("Unexpected error: {}", err),
                }
            }
        }
        e => panic!("Unexpected error: {:?}", e),
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
    for path in &[
        Path::new("..").join("foo.txt"),
        Path::new("foo").join(".."),
        PathBuf::from(".."),
    ] {
        let c = config(
            "https://github.com/user/repo.git",
            None,
            vec![path.to_str().unwrap()],
        );
        if let Ok(p @ Page::Diff { .. }) = parse_page(&c) {
            panic!("Unexpectedly parsed as diff: {:?}", p);
        }
    }
}

#[test]
fn issue_number() {
    let c = config("https://github.com/user/repo.git", None, vec!["#123"]);
    match parse_page(&c) {
        Ok(Page::Issue { number }) => assert_eq!(number, 123),
        v => panic!("Unexpected result {:?}", v),
    }
}

#[test]
fn line_cannot_be_set_to_dir() {
    for arg in &["src#123", "src#12-23"] {
        let c = config("https://github.com/user/repo.git", None, vec![arg]);
        match parse_page(&c).unwrap_err().kind() {
            ErrorKind::PageParseError { attempts, .. } => assert!(
                attempts.iter().any(|(_, err)| match err.kind() {
                    ErrorKind::LineSpecifiedForDir(path) => format!("{:?}", path).contains("src"),
                    _ => false,
                }),
                "{:?} for {}",
                attempts,
                arg,
            ),
            e => panic!("Unexpected error {:?} for {}", e, arg),
        }
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
            vec![file.to_str().unwrap()],
        );
        match parse_page(&c).unwrap() {
            Page::FilePath { line, .. } => {
                assert_eq!(line, Some(Line::Range(1, 2)), "input: {:?}", file)
            }
            p => panic!("Unexpected result: {:?}, input: {:?}", p, file),
        }
    }
}

#[test]
fn setting_website_returns_open_always() {
    for args in &[vec![], vec!["HEAD"], vec!["-r", "foo/bar"]] {
        let mut c = config("https://github.com/user/repo.git", None, args.clone());
        c.website = true;
        match parse_page(&c).unwrap() {
            Page::Open {
                website: true,
                pull_request: false,
            } => { /* OK */ }
            page => panic!("Unexpected parse result: {:?}", page),
        }
    }
}

#[test]
fn setting_pull_request_returns_open_always() {
    for args in &[vec![], vec!["HEAD"], vec!["-r", "foo/bar"]] {
        let mut c = config("https://github.com/user/repo.git", None, args.clone());
        c.pull_request = true;
        match parse_page(&c).unwrap() {
            Page::Open {
                website: false,
                pull_request: true,
            } => { /* OK */ }
            page => panic!("Unexpected parse result: {:?}", page),
        }
    }
}

#[test]
fn parse_tag_ref() {
    let c = config(
        "https://github.com/rhysd/git-brws.git",
        None,
        vec!["0.10.0"],
    );
    match parse_page(&c).unwrap() {
        Page::Tag { tagname, commit } => {
            assert_eq!(&tagname, "0.10.0");
            assert_eq!(&commit, "0b412dc7b223dd3a7fc16b6406e7b2cc866e3ed3");
        }
        page => panic!("Unexpected parse result: {:?}", page),
    }
}

#[test]
fn parse_blame_without_file_path() {
    for args in &[vec![], vec!["0.10.0"]] {
        let mut c = config("https://github.com/user/repo.git", None, args.clone());
        c.blame = true;
        let c = c;

        match parse_page(&c).unwrap_err().kind() {
            ErrorKind::BlameWithoutFilePath => { /* ok */ }
            e => panic!("Unexpected error: {:?}", e),
        }
    }
}

#[test]
fn parse_blame_directory() {
    let mut c = config("https://github.com/rhysd/git-brws.git", None, vec!["src"]);
    c.blame = true;
    let c = c;

    match parse_page(&c).unwrap_err().kind() {
        ErrorKind::CannotBlameDirectory { dir } => {
            assert!(dir.ends_with("src"), "{:?}", dir);
        }
        e => panic!("Unexpected parse error: {:?}", e),
    }
}

#[test]
fn shorten_commit_hash() {
    fn cfg(args: Vec<&str>) -> Config {
        let mut c = config("https://github.com/rhysd/git-brws.git", None, args);
        c.env.short_commit_hash = true;
        c
    }

    // hash
    let c = cfg(vec!["dbb66be9b78ecddef734d2f9cf8c2c7a2836145b"]);
    let p = parse_page(&c).unwrap();
    assert!(
        matches!(&p, Page::Commit{ hash } if hash == "dbb66be"),
        "{:?}",
        p,
    );

    // diff
    let c = cfg(vec!["499edbb...bc869a1"]);
    match parse_page(&c).unwrap() {
        Page::Diff { lhs, rhs, .. } => {
            assert_eq!(lhs, "499edbb");
            assert_eq!(rhs, "bc869a1");
        }
        p => panic!("did not match: {:?}", p),
    }

    // tag
    let c = cfg(vec!["0.10.0"]);
    let p = parse_page(&c).unwrap();
    assert!(
        matches!(&p, Page::Tag{ commit, .. } if commit == "0b412dc"),
        "{:?}",
        p,
    );

    // path
    let c = cfg(vec![
        "README.md",
        "dbb66be9b78ecddef734d2f9cf8c2c7a2836145b",
    ]);
    let p = parse_page(&c).unwrap();
    assert!(
        matches!(&p, Page::FilePath{ hash, .. } if hash == "dbb66be"),
        "{:?}",
        p,
    );
}
