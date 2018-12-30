extern crate path_slash;
extern crate url;

use self::url::Url;
use crate::env::Env;
use crate::error::{Error, Result};
use crate::page::{DiffOp, Page};

#[cfg(target_os = "windows")]
fn to_slash<S: AsRef<str>>(s: &S) -> String {
    use self::path_slash::PathExt;
    use std::path::Path;
    Path::new(s.as_ref()).to_slash_lossy()
}

// Do nothing on Windows
#[cfg(not(target_os = "windows"))]
fn to_slash<S: AsRef<str>>(s: &S) -> &str {
    s.as_ref()
}

fn build_github_like_url(
    host: &str,
    user: &str,
    repo: &str,
    branch: &Option<String>,
    page: &Page,
) -> String {
    match page {
        Page::Open => {
            if let Some(ref b) = branch {
                format!("https://{}/{}/{}/tree/{}", host, user, repo, b)
            } else {
                format!("https://{}/{}/{}", host, user, repo)
            }
        }
        Page::Diff {
            ref lhs,
            ref rhs,
            ref op,
        } => format!(
            "https://{}/{}/{}/compare/{}{}{}",
            host, user, repo, lhs, op, rhs
        ),
        Page::Commit { ref hash } => format!("https://{}/{}/{}/commit/{}", host, user, repo, hash),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: None,
        } => format!(
            "https://{}/{}/{}/blob/{}/{}",
            host,
            user,
            repo,
            hash,
            to_slash(relative_path)
        ),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(line),
        } => format!(
            "https://{}/{}/{}/blob/{}/{}#L{}",
            host,
            user,
            repo,
            hash,
            to_slash(relative_path),
            line
        ),
    }
}

fn build_custom_github_like_url(
    host: &str,
    user: &str,
    repo: &str,
    branch: &Option<String>,
    page: &Page,
    ssh_port_env: &Option<String>,
) -> String {
    match ssh_port_env {
        Some(ref v) if !v.is_empty() => build_github_like_url(
            &format!("{}:{}", host, v).as_str(),
            user,
            repo,
            branch,
            page,
        ),
        _ => build_github_like_url(host, user, repo, branch, page),
    }
}

fn build_gitlab_url(
    host: &str,
    user: &str,
    repo: &str,
    branch: &Option<String>,
    page: &Page,
) -> Result<String> {
    if let Page::Diff { op, .. } = page {
        if *op == DiffOp::TwoDots {
            return Err(Error::GitLabDiffNotSupported);
        }
    }
    Ok(build_github_like_url(host, user, repo, branch, page))
}

fn build_bitbucket_url(
    user: &str,
    repo: &str,
    branch: &Option<String>,
    page: &Page,
) -> Result<String> {
    match page {
        Page::Open => {
            if let Some(ref b) = branch {
                Ok(format!(
                    "https://bitbucket.org/{}/{}/branch/{}",
                    user, repo, b
                ))
            } else {
                Ok(format!("https://bitbucket.org/{}/{}", user, repo))
            }
        }
        Page::Diff { .. } => Err(Error::BitbucketDiffNotSupported),
        Page::Commit { ref hash } => Ok(format!(
            "https://bitbucket.org/{}/{}/commits/{}",
            user, repo, hash
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: None,
        } => Ok(format!(
            "https://bitbucket.org/{}/{}/src/{}/{}",
            user,
            repo,
            hash,
            to_slash(relative_path)
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(line),
        } => Ok(format!(
            "https://bitbucket.org/{}/{}/src/{}/{}#lines-{}",
            user,
            repo,
            hash,
            to_slash(relative_path),
            line
        )),
    }
}

// Note: Parse '/user/repo.git' or '/user/repo' or 'user/repo' into 'user' and 'repo'
pub fn slug_from_path<'a>(path: &'a str) -> Result<(&'a str, &'a str)> {
    let mut split = path.split('/').skip_while(|s| s.is_empty());
    let user = split.next().ok_or_else(|| Error::NoUserInPath {
        path: path.to_string(),
    })?;
    let mut repo = split.next().ok_or_else(|| Error::NoRepoInPath {
        path: path.to_string(),
    })?;
    if repo.ends_with(".git") {
        // Slice '.git' from 'repo.git'
        repo = &repo[0..repo.len() - 4];
    }
    Ok((user, repo))
}

// Known URL formats
//  1. https://hosting_service.com/user/repo.git
//  2. git@hosting_service.com:user/repo.git (-> ssh://git@hosting_service.com:22/user/repo.git)
pub fn build_page_url(
    repo: &str,
    page: &Page,
    branch: &Option<String>,
    env: &Env,
) -> Result<String> {
    let url = Url::parse(repo).map_err(|e| Error::BrokenUrl {
        url: repo.to_string(),
        msg: format!("{}", e),
    })?;
    let path = url.path();
    let (user, repo_name) = slug_from_path(path)?;
    let host = url.host_str().ok_or_else(|| Error::BrokenUrl {
        url: repo.to_string(),
        msg: "No host in URL".to_string(),
    })?;
    match host {
        "github.com" => Ok(build_github_like_url(host, user, repo_name, branch, page)),
        "gitlab.com" => build_gitlab_url(host, user, repo_name, branch, page),
        "bitbucket.org" => build_bitbucket_url(user, repo_name, branch, page),
        _ => {
            let port_env = if host.starts_with("github.") {
                &env.ghe_ssh_port
            } else if host.starts_with("gitlab.") {
                &env.gitlab_ssh_port
            } else {
                match env.ghe_url_host {
                    Some(ref v) if v == host => &env.ghe_ssh_port,
                    _ => {
                        return Err(Error::UnknownHostingService {
                            url: repo.to_string(),
                        });
                    }
                }
            };
            Ok(build_custom_github_like_url(
                host, user, repo_name, branch, page, port_env,
            ))
        }
    }
}
