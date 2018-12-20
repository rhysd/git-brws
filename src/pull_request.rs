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

    // Note: Search pull request URL in the case where the repository is an original, not a fork.
    // Author should not be set since original repository's owner may be different from current
    // user (e.g. organization name). And multiple branches which has the same name cannot exist
    // in one repository.
    if let Some(url) = client.find_pr_url(branch, author, repo, None)? {
        return Ok(url);
    }

    if let Some((owner, repo)) = client.parent_repo(author, repo)? {
        // Note: Search pull request URL in the case where the repository was forked from original.
        // Author should be set since other person may create another pull request with the same branch name.
        if let Some(url) =
            client.find_pr_url(branch, owner.as_str(), repo.as_str(), Some(author))?
        {
            return Ok(url);
        }
    }

    Err(format!(
        "No pull request authored by @{} at {}@{}",
        author, repo, branch
    ))
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
            "api.github.com",
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
                format!("{}:{}/api/v3", host, p)
            } else {
                format!("{}/api/v3", host)
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
