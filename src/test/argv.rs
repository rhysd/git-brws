use crate::argv::*;
use std::env;
use std::path::Path;

#[test]
fn no_option() {
    match parse_options(&["git-brws"]).unwrap() {
        ParsedArgv::Parsed(o) => {
            println!("FOO! {:?}", o);
            assert!(
                vec![
                    "https://github.com/rhysd/git-brws.git",
                    "ssh://git@github.com:22/rhysd/git-brws.git",
                    "git@github.com:rhysd/git-brws.git",
                ].iter()
                .any(|u| o.repo == u.to_string())
            );
            assert_eq!(o.branch, None);
            assert!(o.git_dir.ends_with(".git"));
            assert!(o.args.is_empty());
            assert!(!o.stdout);
        }
        r => assert!(false, "Failed to parse args with no option: {:?}", r),
    };

    match parse_options(&["git-brws", "foo", "bar"]).unwrap() {
        ParsedArgv::Parsed(o) => {
            assert_eq!(o.args.len(), 2);
        }
        _ => assert!(false),
    };
}

#[test]
fn with_options() {
    match parse_options(&[
        "git-brws", "foo", "-u", "-r", "foo/bar", "--dir", ".", "bar", "-b", "dev",
    ]).unwrap()
    {
        ParsedArgv::Parsed(o) => {
            assert_eq!(o.repo, "https://github.com/foo/bar.git");
            assert_eq!(o.branch, Some("dev".to_string()));
            assert!(o.git_dir.ends_with(".git"));
            assert_eq!(o.args.len(), 2);
            assert!(o.stdout);
        }
        _ => assert!(false),
    };
}

#[test]
fn ssh_conversion_with_option() {
    match parse_options(&["git-brws", "-r", "git@github.com:user/repo.git"]).unwrap() {
        ParsedArgv::Parsed(o) => {
            assert_eq!(o.repo, "ssh://git@github.com:22/user/repo.git");
        }
        p => assert!(
            false,
            "Parse must be succeeded but actually results in {:?}",
            p
        ),
    };
}

#[test]
fn repo_formatting() {
    let p = |r| parse_options(&["git-brws", "-r", r]).unwrap();
    match p("bitbucket.org/foo/bar") {
        ParsedArgv::Parsed(o) => assert_eq!(o.repo, "https://bitbucket.org/foo/bar.git"),
        _ => assert!(false),
    }
    match p("https://gitlab.com/foo/bar") {
        ParsedArgv::Parsed(o) => assert_eq!(o.repo, "https://gitlab.com/foo/bar.git"),
        _ => assert!(false),
    }
}

#[test]
fn help_option() {
    match parse_options(&["git-brws", "-h"]).unwrap() {
        ParsedArgv::Help(s) => {
            assert!(s.starts_with("Usage:"));
        }
        _ => assert!(false),
    }
}

#[test]
fn version_option() {
    match parse_options(&["git-brws", "-v"]).unwrap() {
        ParsedArgv::Version(s) => {
            assert!(!s.is_empty());
        }
        _ => assert!(false),
    }
}

#[test]
fn unknown_options() {
    assert!(parse_options(&["git-brws", "--unknown"]).is_err());
}

#[test]
fn detect_git_dir() {
    let current = env::current_dir().unwrap();
    let mut p = current.clone();
    p.push(Path::new("src/test/"));
    match parse_options(&["git-brws", "-d", p.to_str().unwrap()]).unwrap() {
        ParsedArgv::Parsed(o) => {
            let mut expected = current.clone();
            expected.push(".git");
            assert_eq!(o.git_dir, expected);
        }
        _ => assert!(false),
    }
}
