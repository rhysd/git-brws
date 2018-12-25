extern crate url;

use self::url::Url;
use crate::page::{DiffOp, Page};
use std::path::Path;

use crate::envvar;
use crate::errors::Result;

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
            host, user, repo, hash, relative_path
        ),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(line),
        } => format!(
            "https://{}/{}/{}/blob/{}/{}#L{}",
            host, user, repo, hash, relative_path, line
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
            return Err(
                "GitLab does not support '..' for comparing diff between commits. Please use '...'"
                    .to_string(),
            );
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
        Page::Open => if let Some(ref b) = branch {
            Ok(format!("https://bitbucket.org/{}/{}/branch/{}", user, repo, b))
        } else {
            Ok(format!("https://bitbucket.org/{}/{}", user, repo))
        },
        Page::Diff {..} => Err("BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)".to_string()),
        Page::Commit {ref hash} => Ok(format!("https://bitbucket.org/{}/{}/commits/{}", user, repo, hash)),
        Page::FileOrDir {ref relative_path, ref hash, line: None} => Ok(format!("https://bitbucket.org/{}/{}/src/{}/{}", user, repo, hash, relative_path)),
        Page::FileOrDir {ref relative_path, ref hash, line: Some(line)} => {
            let file = Path::new(relative_path)
                .file_name()
                .ok_or_else(|| format!("Cannot get file name from path: {}", relative_path))?
                .to_str()
                .ok_or_else(|| format!("Cannot convert path to UTF8 string: {}", relative_path))?;
            Ok(format!("https://bitbucket.org/{}/{}/src/{}/{}#{}-{}", user, repo, hash, relative_path, file, line))
        },
    }
}

// Note: Parse '/user/repo.git' or '/user/repo' or 'user/repo' into 'user' and 'repo'
pub fn slug_from_path<'a>(path: &'a str) -> Result<(&'a str, &'a str)> {
    let mut split = path.split('/').skip_while(|s| s.is_empty());
    let user = split
        .next()
        .ok_or_else(|| format!("Can't detect user name from path {}", path))?;
    let mut repo = split
        .next()
        .ok_or_else(|| format!("Can't detect repository name from path {}", path))?;
    if repo.ends_with(".git") {
        // Slice '.git' from 'repo.git'
        repo = &repo[0..repo.len() - 4];
    }
    Ok((user, repo))
}

// Known URL formats
//  1. https://hosting_service.com/user/repo.git
//  2. git@hosting_service.com:user/repo.git (-> ssh://git@hosting_service.com:22/user/repo.git)
pub fn parse_and_build_page_url(
    repo: &str,
    page: &Page,
    branch: &Option<String>,
    env: &envvar::Envvar,
) -> Result<String> {
    let url = Url::parse(repo).map_err(|e| format!("Failed to parse URL '{}': {}", repo, e))?;
    let path = url.path();
    let (user, repo_name) = slug_from_path(path)?;
    let host = url
        .host_str()
        .ok_or_else(|| format!("Failed to parse host from {}", repo))?;
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
                        return Err(format!("Unknown hosting service for URL {}. If you want to use custom URL for GitHub Enterprise, please set $GIT_BRWS_GHE_URL_HOST", repo));
                    }
                }
            };
            Ok(build_custom_github_like_url(
                host, user, repo_name, branch, page, port_env,
            ))
        }
    }
}
