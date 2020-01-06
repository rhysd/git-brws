use crate::config::{Config, EnvConfig};
use crate::error::{Error, Result};
use crate::github_api;
use std::borrow::Cow;

#[derive(PartialEq, Debug)]
pub enum Page<'a, 'b> {
    Existing {
        url: String,
    },
    New {
        author: Cow<'a, str>,
        repo: Cow<'b, str>,
        default_branch: String,
        branch: String,
    },
    NewAtParent {
        author: Cow<'a, str>,
        repo: Cow<'b, str>,
        fork_author: &'a str,
        default_branch: String,
        branch: String,
    },
}

async fn find_github_pr_url_for_branch<'a, 'b, B: AsRef<str>>(
    branch: B,
    endpoint: &str,
    author: &'a str,
    repo: &'b str,
    env: &EnvConfig,
) -> Result<Page<'a, 'b>> {
    let branch = branch.as_ref();
    let token = if endpoint == "api.github.com" {
        &env.github_token
    } else {
        if env.ghe_token.is_none() {
            return Err(Error::GheTokenRequired);
        }
        &env.ghe_token
    };
    let client = github_api::Client::build(endpoint, token.as_ref(), &env.https_proxy)?;

    let (pr_url, fetched_repo) = futures::join!(
        // Note: Search pull request URL in the case where the repository is an original, not a
        // fork. Author should not be set since original repository's owner may be different from
        // current user (e.g. organization name). And multiple branches which has the same name
        // cannot exist in one repository.
        client.find_pr_url(branch, author, repo, None),
        // Note: Send requests for fetching request and getting repository information at the same
        // time for speed up.
        client.repo(author, repo),
    );

    if let Some(url) = pr_url? {
        return Ok(Page::Existing { url });
    }

    let fetched_repo = fetched_repo?;
    if let Some(parent) = fetched_repo.parent {
        let owner = parent.owner.login;
        let repo = parent.name;

        // Note: Search pull request URL in the case where the repository was forked from original.
        // Author should be set since other person may create another pull request with the same branch name.
        if let Some(url) = client
            .find_pr_url(branch, owner.as_str(), repo.as_str(), Some(author))
            .await?
        {
            Ok(Page::Existing { url })
        } else {
            Ok(Page::NewAtParent {
                author: Cow::Owned(owner),
                repo: Cow::Owned(repo),
                fork_author: author,
                default_branch: parent.default_branch,
                branch: branch.to_string(),
            })
        }
    } else {
        Ok(Page::New {
            author: Cow::Borrowed(author),
            repo: Cow::Borrowed(repo),
            default_branch: fetched_repo.default_branch,
            branch: branch.to_string(),
        })
    }
}

pub async fn find_page<'a, 'b>(
    endpoint: &str,
    author: &'a str,
    repo: &'b str,
    cfg: &Config,
) -> Result<Page<'a, 'b>> {
    match cfg.branch {
        Some(ref b) => find_github_pr_url_for_branch(b, endpoint, author, repo, &cfg.env).await,
        None => {
            if let Some(git) = cfg.git() {
                find_github_pr_url_for_branch(
                    git.current_branch()?,
                    endpoint,
                    author,
                    repo,
                    &cfg.env,
                )
                .await
            } else {
                Err(Error::NoLocalRepoFound {
                    operation: "opening a pull request without specifying branch".to_string(),
                })
            }
        }
    }
}
