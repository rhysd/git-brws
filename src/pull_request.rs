extern crate url;

use self::url::Url;
use crate::env::Env;
use crate::error::{Error, Result};
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

    Err(Error::GitHubPullReqNotFound {
        author: author.to_string(),
        repo: repo.to_string(),
        branch: branch.to_string(),
    })
}

pub fn find_url<U: AsRef<str>, B: AsRef<str>>(repo_url: U, branch: B, env: &Env) -> Result<String> {
    let url = Url::parse(repo_url.as_ref()).map_err(|e| Error::BrokenUrl {
        url: repo_url.as_ref().to_string(),
        msg: format!("{}", e),
    })?;
    // .map_err(|e| format!("Failed to parse URL '{}': {}", repo_url.as_ref(), e))?;
    let path = url.path();
    let (author, repo) = slug_from_path(path)?;
    match url.host_str().ok_or_else(|| Error::BrokenUrl {
        url: repo_url.as_ref().to_string(),
        msg: "No host in URL".to_string(),
    })? {
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
                    _ => {
                        return Err(Error::PullReqNotSupported {
                            service: host.to_string(),
                        });
                    }
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
