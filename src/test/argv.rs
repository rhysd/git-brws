use std::env;
use std::path::Path;
use argv::*;

fn args(strs: Vec<&str>) -> Vec<String> {
    let mut v = vec!["git-brws".to_string()];
    for s in strs {
        v.push(s.to_string());
    }
    v
}

#[test]
fn no_option() {
    match parse_options(args(vec![])).unwrap() {
        ParsedArgv::Parsed(o, false) => {
            assert!(
                vec![
                    "https://github.com/rhysd/git-brws.git",
                    "ssh://git@github.com:22/rhysd/git-brws.git",
                ]
                .iter()
                .any(|u| o.repo == u.to_string())
            );
            assert_eq!(o.branch, None);
            assert!(o.git_dir.ends_with(".git"));
            assert!(o.args.is_empty());
        },
        _ => assert!(false),
    };

    match parse_options(args(vec!["foo", "bar"])).unwrap() {
        ParsedArgv::Parsed(o, false) => {
            assert_eq!(o.args.len(), 2);
        },
        _ => assert!(false),
    };
}

#[test]
fn with_options() {
    match parse_options(args(vec!["foo", "-u", "-r", "foo/bar", "--dir", ".", "bar", "-b", "dev"])).unwrap() {
        ParsedArgv::Parsed(o, true) => {
            assert_eq!(o.repo, "https://github.com/foo/bar.git");
            assert_eq!(o.branch, Some("dev".to_string()));
            assert!(o.git_dir.ends_with(".git"));
            assert_eq!(o.args.len(), 2);
        },
        _ => assert!(false),
    };
}

#[test]
fn repo_formatting() {
    let p = |r| parse_options(args(vec!["-r", r])).unwrap();
    match p("bitbucket.org/foo/bar") {
        ParsedArgv::Parsed(o, false) => assert_eq!(o.repo, "https://bitbucket.org/foo/bar.git"),
        _ => assert!(false),
    }
    match p("https://gitlab.com/foo/bar") {
        ParsedArgv::Parsed(o, false) => assert_eq!(o.repo, "https://gitlab.com/foo/bar.git"),
        _ => assert!(false),
    }
}

#[test]
fn help_option() {
    match parse_options(args(vec!["-h"])).unwrap() {
        ParsedArgv::Help(s) => {
            assert!(s.starts_with("Usage:"));
        },
        _ => assert!(false),
    }
}

#[test]
fn version_option() {
    match parse_options(args(vec!["-v"])).unwrap() {
        ParsedArgv::Version(s) => {
            assert!(!s.is_empty());
        },
        _ => assert!(false),
    }
}

#[test]
fn unknown_options() {
    assert!(parse_options(args(vec!["--unknown"])).is_err());
}

#[test]
fn detect_git_dir() {
    let mut p = env::current_dir().unwrap();
    p.push(Path::new("src/test/assets/test1/dir1"));
    match parse_options(args(vec!["-d", p.to_str().unwrap()])).unwrap() {
        ParsedArgv::Parsed(o, false) => {
            assert!(o.git_dir.ends_with("src/test/assets/test1/.git"));
        },
        _ => assert!(false),
    }
}
