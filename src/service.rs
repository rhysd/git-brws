use crate::config::Config;
use crate::error::{Error, Result};
use crate::github_api::Client;
use crate::page::{DiffOp, Line, Page};
use crate::pull_request;
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

// TODO: Omit fallback and return Result<String>
fn first_available_url<T: AsRef<str>>(
    candidates: &mut [String],
    fallback: String,
    https_proxy: &Option<T>,
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

fn build_github_like_url<S: AsRef<str>>(
    host: &str,
    user: &str,
    repo: &str,
    api_endpoint: Option<S>,
    cfg: &Config,
    page: &Page,
) -> Result<String> {
    match page {
        Page::Open { website: true, .. } => {
            match host {
                "github.com" => {
                    if let Some(endpoint) = api_endpoint {
                        if let Ok(client) = Client::build(
                            endpoint.as_ref(),
                            cfg.env.github_token.as_ref(),
                            &cfg.env.https_proxy,
                        ) {
                            if let Ok(Some(homepage)) = client.repo_homepage(user, repo) {
                                return Ok(homepage);
                            }
                        }
                    }
                    let host = &host[0..host.len() - 4];
                    Ok(format!("https://{}.{}.io/{}", user, host, repo))
                }
                "gitlab.com" => Ok(format!("https://{}.gitlab.io/{}", user, repo)),
                host if host.starts_with("gitlab.") => {
                    Ok(format!("https://{}.{}/{}", user, host, repo))
                }
                // For GitHub Enterprise:
                //   https://help.github.com/enterprise/user/articles/user-organization-and-project-pages/
                host => {
                    // Token is always required for GitHub Enterprise
                    if let (Some(ref endpoint), Some(ref token)) =
                        (api_endpoint, &cfg.env.ghe_token)
                    {
                        if let Ok(client) =
                            Client::build(endpoint.as_ref(), Some(token), &cfg.env.https_proxy)
                        {
                            if let Ok(Some(homepage)) = client.repo_homepage(user, repo) {
                                return Ok(homepage);
                            }
                        }
                    }
                    let with_subdomain = format!("https://pages.{}/{}/{}", host, user, repo);
                    let without_subdomain = format!("https://{}/pages/{}/{}", host, user, repo);
                    Ok(first_available_url(
                        &mut [with_subdomain],
                        without_subdomain,
                        &cfg.env.https_proxy,
                    ))
                }
            }
        }
        Page::Open {
            pull_request: true, ..
        } => {
            if let Some(endpoint) = api_endpoint {
                match pull_request::find_page(endpoint.as_ref(), user, repo, cfg)? {
                    pull_request::Page::Existing { url } => Ok(url),
                    pull_request::Page::New {
                        author,
                        repo,
                        default_branch,
                        branch,
                    } => Ok(format!(
                        "https://{}/{}/{}/compare/{}...{}",
                        host, author, repo, default_branch, branch,
                    )),
                    pull_request::Page::NewAtParent {
                        author,
                        repo,
                        fork_author,
                        default_branch,
                        branch,
                    } => Ok(format!(
                        "https://{}/{}/{}/compare/{}...{}:{}",
                        host, author, repo, default_branch, fork_author, branch,
                    )),
                }
            } else {
                Err(Error::PullReqNotSupported {
                    service: host.to_string(),
                })
            }
        }
        Page::Open { .. } => {
            if let Some(ref b) = cfg.branch {
                Ok(format!("https://{}/{}/{}/tree/{}", host, user, repo, b))
            } else {
                Ok(format!("https://{}/{}/{}", host, user, repo))
            }
        }
        Page::Tag { ref tagname, .. } => Ok(format!(
            "https://{}/{}/{}/tree/{}",
            host, user, repo, tagname,
        )),
        Page::Diff {
            ref lhs,
            ref rhs,
            ref op,
        } => Ok(format!(
            "https://{}/{}/{}/compare/{}{}{}",
            host, user, repo, lhs, op, rhs,
        )),
        Page::Commit { ref hash } => Ok(format!(
            "https://{}/{}/{}/commit/{}",
            host, user, repo, hash
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line,
            blame,
        } => Ok(format!(
            "https://{host}/{user}/{repo}/{feat}/{hash}/{path}{anchor}",
            host = host,
            user = user,
            repo = repo,
            feat = if *blame { "blame" } else { "blob" },
            hash = hash,
            path = to_slash(relative_path),
            anchor = match line {
                None => "".to_string(),
                Some(Line::At(line)) => format!("#L{}", line),
                Some(Line::Range(start, end)) => format!("#L{}-L{}", start, end),
            },
        )),
        Page::Issue { number } => Ok(format!(
            "https://{}/{}/{}/issues/{}",
            host, user, repo, number
        )),
    }
}

fn build_gitlab_url(
    host: &str,
    user: &str,
    repo: &str,
    cfg: &Config,
    page: &Page,
) -> Result<String> {
    if let Page::Diff { op, .. } = page {
        if *op == DiffOp::TwoDots {
            return Err(Error::GitLabDiffNotSupported);
        }
    }
    build_github_like_url::<&str>(host, user, repo, None, cfg, page)
}

fn build_bitbucket_url(user: &str, repo: &str, cfg: &Config, page: &Page) -> Result<String> {
    match page {
        Page::Open { website: true, .. } => {
            // Build bitbucket cloud URL:
            //   https://confluence.atlassian.com/bitbucket/publishing-a-website-on-bitbucket-cloud-221449776.html
            let with_user = format!("https://{}.bitbucket.io/{}", user, repo);
            let without_user = format!("https://{}.bitbucket.io", user);
            Ok(first_available_url(
                &mut [with_user],
                without_user,
                &cfg.env.https_proxy,
            ))
        }
        Page::Open {
            pull_request: true, ..
        } => Err(Error::PullReqNotSupported {
            service: "bitbucket.org".to_string(),
        }),
        Page::Open { .. } => {
            if let Some(ref b) = cfg.branch {
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
        // On Bitbucket, there is no tag-specific page. However, unlike GitHub, bitbucket supports
        // tag commit. Open the tag commit page instead.
        Page::Tag { ref commit, .. } => Ok(format!(
            "https://bitbucket.org/{}/{}/commits/{}",
            user, repo, commit,
        )),
        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line,
            blame,
        } => Ok(format!(
            "https://bitbucket.org/{user}/{repo}/{feat}/{hash}/{path}{anchor}",
            user = user,
            repo = repo,
            feat = if *blame { "annotate" } else { "src" },
            hash = hash,
            path = to_slash(relative_path),
            anchor = match line {
                None => "".to_string(),
                Some(Line::At(line)) => format!("#lines-{}", line),
                Some(Line::Range(start, end)) => format!("#lines-{}:{}", start, end),
            },
        )),
        Page::Issue { number } => Ok(format!(
            "https://bitbucket.org/{}/{}/issues/{}",
            user, repo, number,
        )),
    }
}

fn build_azdevops_url(team: &str, repo: &str, cfg: &Config, page: &Page) -> Result<String> {
    match page {
        Page::Open {
            pull_request: true, ..
        } => {
            if let Some(ref b) = cfg.branch {
                Ok(format!("https://dev.azure.com/{}/_git/{}/pullrequestcreate?sourceRef={}&targetRef=master", team, repo, b))
            } else {
                Err(Error::NoLocalRepoFound {
                    operation: "opening a pull request without specifying branch".to_string(),
                })
            }
        }

        Page::Open { .. } => {
            if let Some(ref b) = cfg.branch {
                Ok(format!(
                    "https://dev.azure.com/{}/_git/{}?version=GB{}",
                    team, repo, b
                ))
            } else {
                Ok(format!("https://dev.azure.com/{}/{}", team, repo))
            }
        }

        Page::Commit { ref hash } => Ok(format!(
            "https://dev.azure.com/{}/_git/{}/commit/{}",
            team, repo, hash
        )),

        Page::Tag { ref tagname, .. } => Ok(format!(
            "https://dev.azure.com/{}/_git/{}?version=GT{}",
            team, repo, tagname
        )),

        Page::FileOrDir {
            ref relative_path,
            ref hash,
            line: None,
            blame,
        } => Ok(format!(
            "https://dev.azure.com/{}/_git/{}/commit/{}?path={}{}",
            team,
            repo,
            hash,
            to_slash(relative_path),
            if *blame { "?_a=annotate" } else { "" },
        )),

        Page::Issue { number } => Ok(format!(
            "https://dev.azure.com/{}/{}/_workitems/edit/{}",
            team, repo, number
        )),

        _ => Err(Error::AzureDevOpsNotSupported),
    }
}

fn is_azdevops(host: &str) -> bool {
    match host {
        "visualstudio.com" => true,
        "vs-ssh.visualstudio.com" => true,
        "dev.azure.com" => true,
        "ssh.dev.azure.com" => true,
        _ => false,
    }
}

// Note: Parse '/team/_git/repo' or '/team/repo' into 'team' and 'repo'
pub fn azdevops_slug_from_path<'a>(path: &'a str) -> Result<(&'a str, &'a str)> {
    let mut split = path.split('/').skip_while(|s| s.is_empty());

    let mut team = split.next().ok_or_else(|| Error::NoUserInPath {
        path: path.to_string(),
    })?;

    // Strip off v3 from Azure DevOps ssh:// paths.
    // See: preprocess_repo_to_url
    //
    // Example: ssh://git@ssh.dev.azure.com:v3/team/repo/repo
    //
    if team == "v3" {
        team = split.next().ok_or_else(|| Error::NoRepoInPath {
            path: path.to_string(),
        })?;
    }

    let mut repo = split.next().ok_or_else(|| Error::NoRepoInPath {
        path: path.to_string(),
    })?;

    if repo.ends_with("_git") {
        repo = split.next().ok_or_else(|| Error::NoRepoInPath {
            path: path.to_string(),
        })?;
    }
    Ok((team, repo))
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

fn preprocess_repo_to_url(repo: &str) -> Result<Url> {
    // Workaround Url::parse not being able to parse the SSH urls for AzureDevOps
    // as they don't specify a port number, but use the colon syntax. It seems like
    // the URL's don't adhere to the RFC? So we force a port number to the default
    // SSH port so the Url will parse correctly.
    //
    // Example: ssh://git@ssh.dev.azure.com:v3/team/repo/repo
    //
    let processed_repo = if repo.contains("visualstudio.com:v3") || repo.contains("azure.com:v3") {
        repo.replace(":v3/", ":22/v3/")
    } else {
        repo.to_string()
    };

    Url::parse(&processed_repo).map_err(|e| Error::BrokenUrl {
        url: processed_repo,
        msg: format!("{}", e),
    })
}

// Known URL formats
//  1. https://hosting_service.com/user/repo.git
//  2. git@hosting_service.com:user/repo.git (-> ssh://git@hosting_service.com:22/user/repo.git)
pub fn build_page_url(page: &Page, cfg: &Config) -> Result<String> {
    let repo_url = &cfg.repo;
    let url = preprocess_repo_to_url(&repo_url)?;
    let env = &cfg.env;

    let path = url.path();
    let host = url.host_str().ok_or_else(|| Error::BrokenUrl {
        url: repo_url.to_string(),
        msg: "No host in URL".to_string(),
    })?;

    let (user, repo_name) = if is_azdevops(host) {
        azdevops_slug_from_path(path)?
    } else {
        slug_from_path(path)?
    };

    match host {
        "github.com" => {
            build_github_like_url(host, user, repo_name, Some("api.github.com"), cfg, page)
        }
        "gitlab.com" => build_gitlab_url(host, user, repo_name, cfg, page),
        "bitbucket.org" => build_bitbucket_url(user, repo_name, cfg, page),
        "visualstudio.com" => build_azdevops_url(user, repo_name, cfg, page),
        "vs-ssh.visualstudio.com" => build_azdevops_url(user, repo_name, cfg, page),
        "dev.azure.com" => build_azdevops_url(user, repo_name, cfg, page),
        "ssh.dev.azure.com" => build_azdevops_url(user, repo_name, cfg, page),
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
                            url: repo_url.to_string(),
                        });
                    }
                }
            };

            let host = match port {
                Some(port) => Cow::Owned(format!("{}:{}", host, port)),
                None => Cow::Borrowed(host),
            };

            if is_gitlab {
                build_gitlab_url(&host, user, repo_name, cfg, page)
            } else {
                build_github_like_url(
                    &host,
                    user,
                    repo_name,
                    Some(format!("{}/api/v3", host)),
                    cfg,
                    page,
                )
            }
        }
    }
}
