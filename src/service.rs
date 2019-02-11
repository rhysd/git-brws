#![allow(clippy::too_many_arguments)]

extern crate path_slash;
extern crate reqwest;
extern crate url;

use crate::env::EnvConfig;
use crate::error::{Error, Result};
use crate::github_api::Client;
use crate::page::{DiffOp, Line, Page};
use std::borrow::Cow;
use std::mem;
use url::Url;

#[cfg(target_os = "windows")]
fn to_slash<S: AsRef<str>>(s: &S) -> String {
    use path_slash::PathExt;
    use std::path::Path;
    Path::new(s.as_ref()).to_slash_lossy()
}

// Do nothing on Windows
#[cfg(not(target_os = "windows"))]
fn to_slash<S: AsRef<str>>(s: &S) -> &str {
    s.as_ref()
}

fn first_available_url<T: AsRef<str>>(
    candidates: &mut [String],
    fallback: String,
    https_proxy: Option<T>,
) -> String {
    let mut builder = reqwest::Client::builder();
    if let Some(ref p) = https_proxy {
        if let Ok(p) = reqwest::Proxy::https(p.as_ref()) {
            builder = builder.proxy(p);
        } else {
            return fallback;
        }
    }
    if let Ok(client) = builder.build() {
        for mut candidate in candidates.iter_mut() {
            let req = client.head(candidate.as_str());
            if let Ok(res) = req.send() {
                let status = res.status();
                if status == reqwest::StatusCode::OK {
                    return mem::replace(&mut candidate, String::new());
                }
            }
        }
    }
    fallback
}

fn build_github_like_url<S, T, U>(
    host: &str,
    user: &str,
    repo: &str,
    branch: &Option<String>,
    api_endpoint: Option<S>,
    api_token: Option<T>,
    https_proxy: Option<U>,
    page: &Page,
) -> String
where
    S: AsRef<str>,
    T: ToString,
    U: AsRef<str>,
{
    #[allow(clippy::unneeded_field_pattern)]
    match page {
        Page::Open { website } if *website => {
            match host {
                "github.com" => {
                    if let Some(endpoint) = api_endpoint {
                        if let Ok(client) =
                            Client::build(endpoint.as_ref(), api_token, &https_proxy)
                        {
                            if let Ok(Some(homepage)) = client.repo_homepage(user, repo) {
                                return homepage;
                            }
                        }
                    }
                    let host = &host[0..host.len() - 4];
                    format!("https://{}.{}.io/{}", user, host, repo)
                }
                // For GitHub Enterprise:
                //   https://help.github.com/enterprise/user/articles/user-organization-and-project-pages/
                host => {
                    // Token is always required for GitHub Enterprise
                    if let (Some(endpoint), Some(token)) = (api_endpoint, api_token) {
                        if let Ok(client) =
                            Client::build(endpoint.as_ref(), Some(token), &https_proxy)
                        {
                            if let Ok(Some(homepage)) = client.repo_homepage(user, repo) {
                                return homepage;
                            }
                        }
                    }
                    let with_subdomain = format!("https://pages.{}/{}/{}", host, user, repo);
                    let without_subdomain = format!("https://{}/pages/{}/{}", host, user, repo);
                    first_available_url(&mut [with_subdomain], without_subdomain, https_proxy)
                }
            }
        }
        Page::Open { website: _ } => {
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
            host, user, repo, lhs, op, rhs,
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
            to_slash(relative_path),
        ),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(Line::At(line)),
        } => format!(
            "https://{}/{}/{}/blob/{}/{}#L{}",
            host,
            user,
            repo,
            hash,
            to_slash(relative_path),
            line,
        ),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(Line::Range(start, end)),
        } => format!(
            "https://{}/{}/{}/blob/{}/{}#L{}-L{}",
            host,
            user,
            repo,
            hash,
            to_slash(relative_path),
            start,
            end,
        ),
        Page::Issue { number } => format!("https://{}/{}/{}/issues/{}", host, user, repo, number),
        Page::PullRequest { url } => url.clone(),
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
    Ok(build_github_like_url::<&str, &str, &str>(
        host, user, repo, branch, None, None, None, page,
    ))
}

fn build_bitbucket_url<T: AsRef<str>>(
    user: &str,
    repo: &str,
    branch: &Option<String>,
    page: &Page,
    https_proxy: Option<T>,
) -> Result<String> {
    #[allow(clippy::unneeded_field_pattern)]
    match page {
        Page::Open { website } if *website => {
            // Build bitbucket cloud URL:
            //   https://confluence.atlassian.com/bitbucket/publishing-a-website-on-bitbucket-cloud-221449776.html
            let with_user = format!("https://{}.bitbucket.io/{}", user, repo);
            let without_user = format!("https://{}.bitbucket.io", user);
            Ok(first_available_url(
                &mut [with_user],
                without_user,
                https_proxy,
            ))
        }
        Page::Open { website: _ } => {
            if let Some(ref b) = branch {
                Ok(format!(
                    "https://bitbucket.org/{}/{}/branch/{}",
                    user, repo, b,
                ))
            } else {
                Ok(format!("https://bitbucket.org/{}/{}", user, repo))
            }
        }
        Page::Diff { .. } => Err(Error::BitbucketDiffNotSupported),
        Page::Commit { ref hash } => Ok(format!(
            "https://bitbucket.org/{}/{}/commits/{}",
            user, repo, hash,
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
            to_slash(relative_path),
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(Line::At(line)),
        } => Ok(format!(
            "https://bitbucket.org/{}/{}/src/{}/{}#lines-{}",
            user,
            repo,
            hash,
            to_slash(relative_path),
            line,
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: Some(Line::Range(start, end)),
        } => Ok(format!(
            "https://bitbucket.org/{}/{}/src/{}/{}#lines-{}:{}",
            user,
            repo,
            hash,
            to_slash(relative_path),
            start,
            end,
        )),
        Page::Issue { number } => Ok(format!(
            "https://bitbucket.org/{}/{}/issues/{}",
            user, repo, number,
        )),
        Page::PullRequest { .. } => unreachable!(),
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
    env: &EnvConfig,
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
        "github.com" => Ok(build_github_like_url(
            host,
            user,
            repo_name,
            branch,
            Some("api.github.com"),
            env.github_token.as_ref(),
            env.https_proxy.as_ref(),
            page,
        )),
        "gitlab.com" => build_gitlab_url(host, user, repo_name, branch, page),
        "bitbucket.org" => {
            build_bitbucket_url(user, repo_name, branch, page, env.https_proxy.as_ref())
        }
        _ => {
            let is_gitlab = host.starts_with("gitlab.");
            let port = if host.starts_with("github.") {
                env.ghe_ssh_port
            } else if is_gitlab {
                env.gitlab_ssh_port
            } else {
                match env.ghe_url_host {
                    Some(ref v) if v == host => env.ghe_ssh_port,
                    _ => {
                        return Err(Error::UnknownHostingService {
                            url: repo.to_string(),
                        });
                    }
                }
            };

            let host = match port {
                Some(port) => Cow::Owned(format!("{}:{}", host, port)),
                None => Cow::Borrowed(host),
            };

            if is_gitlab {
                build_gitlab_url(&host, user, repo_name, branch, page)
            } else {
                Ok(build_github_like_url(
                    &host,
                    user,
                    repo_name,
                    branch,
                    Some(format!("{}/api/v3", host)),
                    env.ghe_token.as_ref(),
                    env.https_proxy.as_ref(),
                    page,
                ))
            }
        }
    }
}
