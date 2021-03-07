use crate::config::{Config, EnvConfig};
use crate::error::ErrorKind;
use crate::test::helper::empty_env;
use crate::url;
use std::env;

#[cfg(not(target_os = "windows"))]
fn executable_path(cmd: &str) -> String {
    cmd.to_string()
}

#[cfg(target_os = "windows")]
fn executable_path(cmd: &str) -> String {
    // Note: Do not use .canonicalize() since the path converts a file path into extended length
    // path form like '\\?\D:\...'. But the form is not available to exec a command (it says command
    // not found even the \\? path exists). This error only occurs on x86_64 Windows CI on Appveyor
    // or GitHub Actions (seems related to some OS configuration).
    use std::path::PathBuf;
    let mut p = PathBuf::from(file!());
    p.pop(); // git-brws/src/test
    p.push("testdata"); // git-brws/src/test/testdata
    p.push(format!("{}.exe", cmd)); // git-brws/src/test/testdata/true.exe
    p.to_str().unwrap().to_string()
}

fn browse_env_config(cmd: String) -> EnvConfig {
    let mut env = empty_env();
    env.browse_command = Some(cmd);
    env
}

#[test]
fn smoke() {
    let c = Config {
        repo_url: "ssh://git@github.com:22/rhysd/git-brws.git".to_string(),
        branch: None,
        cwd: env::current_dir().unwrap(),
        args: vec![],
        stdout: false,
        pull_request: false,
        website: false,
        blame: false,
        remote: None,
        env: empty_env(),
    };
    match url::build_url(&c) {
        Ok(u) => assert_eq!(
            u, "https://github.com/rhysd/git-brws",
            "Unexpected URL: {}",
            u
        ),
        Err(e) => panic!("url::build_url() was not processed properly: {}", e),
    }
}

#[test]
fn browse_url_with_user_command() {
    let exe = executable_path("true");
    let env = browse_env_config(exe);
    url::browse("https://example.com", &env).unwrap();
}

#[test]
fn fail_to_browse_url_with_user_command() {
    let exe = executable_path("false");
    let env = browse_env_config(exe.clone());
    match url::browse("https://example.com", &env).unwrap_err().kind() {
        ErrorKind::UserBrowseCommandFailed { cmd, url, .. } => {
            assert_eq!(cmd, &exe);
            assert_eq!(url, "https://example.com");
        }
        e => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn browse_command_is_not_found() {
    let env = browse_env_config("this-command-is-not-existing-yeah".to_string());
    match url::browse("https://example.com", &env).unwrap_err().kind() {
        ErrorKind::IoError { .. } => { /* ok */ }
        e => panic!("Unexpected error: {:?}", e),
    }
}
