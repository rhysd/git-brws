extern crate url;

use self::url::Url;
use crate::envvar::Envvar;
use crate::errors::Result;
use crate::github_api;
use crate::service::slug_from_path;

fn find_github_pr_url(
    author: &str,
    repo: &str,
    branch: &str,
    host: &str,
    token: &Option<String>,
    https_proxy: &Option<String>,
) -> Result<String> {
    let client = github_api::Client::build(host, token.clone(), https_proxy)?;
    if let Ok(url) = client.find_pr_url(branch, author, author, repo) {
        return Ok(url);
    }

    let parent = client.parent_repo(author, repo)?;
    if parent == None {
        return Err(format!(
            "No PR for {}/{} authored by @{} at branch {}",
            author, repo, author, branch
        ));
    }

    let (owner, repo) = match parent {
        Some(pair) => pair,
        None => {
            return Err(format!(
                "No PR for {}/{} authored by @{} at branch {}",
                author, repo, author, branch
            ))
        }
    };

    client.find_pr_url(branch, author, owner, repo)
}

pub fn find_url<S: AsRef<str>>(repo_url: S, branch: S, env: &Envvar) -> Result<String> {
    let repo_url = Url::parse(repo_url.as_ref())
        .map_err(|e| format!("Failed to parse URL '{}': {}", repo_url.as_ref(), e))?;
    let path = repo_url.path();
    let (author, repo) = slug_from_path(path)?;
    match repo_url
        .host_str()
        .ok_or_else(|| format!("Failed to parse host from {}", repo_url))?
    {
        "github.com" => find_github_pr_url(
            author,
            repo,
            branch.as_ref(),
            match env.ghe_url_host {
                Some(ref h) => h.as_str(),
                None => "github.com",
            },
            &env.github_token,
            &env.https_proxy,
        ),
        host => {
            let port = if host.starts_with("github.") {
                &env.ghe_ssh_port
            } else {
                match env.ghe_url_host {
                    Some(ref h) if host == h => &env.ghe_ssh_port,
                    _ => return Err(format!("--pr or -p is not supported for service {}", host)),
                }
            };

            let host = if let Some(ref p) = port {
                format!("{}:{}", host, p)
            } else {
                host.to_string()
            };

            find_github_pr_url(
                author,
                repo,
                branch.as_ref(),
                host.as_str(),
                &env.ghe_token,
                &env.https_proxy,
            )
        }
    }
}
