extern crate url;

use page::Page;
use self::url::Url;

use util;

fn build_github_url(user: &str, repo: &str, branch: &Option<String>, page: &Page) -> String {
    match page {
        &Page::Open => if let &Some(ref b) = branch {
            format!("https://github.com/{}/{}/tree/{}", user, repo, b)
        } else {
            format!("https://github.com/{}/{}", user, repo)
        },
        &Page::Diff {ref lhs, ref rhs} => format!("https://github.com/{}/{}/compare/{}...{}", user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://github.com/{}/{}/commit/{}", user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://github.com/{}/{}/blob/{}/{}", user, repo, hash, relative_path),
    }
}

fn build_bitbucket_url(user: &str, repo: &str, branch: &Option<String>, page: &Page) -> util::Result<String> {
    match page {
        &Page::Open => if let &Some(ref b) = branch {
            Ok(format!("https://bitbucket.org/{}/{}/branch/{}", user, repo, b))
        } else {
            Ok(format!("https://bitbucket.org/{}/{}", user, repo))
        },
        &Page::Diff {..} => Err("BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)".to_string()),
        &Page::Commit {ref hash} => Ok(format!("https://bitbucket.org/{}/{}/commits/{}", user, repo, hash)),
        &Page::FileOrDir {ref relative_path, ref hash} => Ok(format!("https://bitbucket.org/{}/{}/src/{}/{}", user, repo, hash, relative_path)),
    }
}

fn build_github_enterprise_url(host: &str, user: &str, repo: &str, branch: &Option<String>, page: &Page) -> String {
    match page {
        &Page::Open => if let &Some(ref b) = branch {
            format!("https://{}/{}/{}/tree/{}", host, user, repo, b)
        } else {
            format!("https://{}/{}/{}", host, user, repo)
        },
        &Page::Diff {ref lhs, ref rhs} => format!("https://{}/{}/{}/compare/{}...{}", host, user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://{}/{}/{}/commit/{}", host, user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://{}/{}/{}/blob/{}/{}", host, user, repo, hash, relative_path),
    }
}

fn build_gitlab_url(user: &str, repo: &str, branch: &Option<String>, page: &Page) -> String {
    match page {
        &Page::Open => if let &Some(ref b) = branch {
            format!("https://gitlab.com/{}/{}/tree/{}", user, repo, b)
        } else {
            format!("https://gitlab.com/{}/{}", user, repo)
        },
        &Page::Diff {ref lhs, ref rhs} => format!("https://gitlab.com/{}/{}/compare/{}...{}", user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://gitlab.com/{}/{}/commit/{}", user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://gitlab.com/{}/{}/blob/{}/{}", user, repo, hash, relative_path),
    }
}

// Note: Parse '/user/repo.git' or '/user/repo' or 'user/repo' into 'user' and 'repo'
fn user_and_repo_from_path<'a>(path: &'a str) -> util::Result<(&'a str, &'a str)> {
    let mut split = path.split('/').skip_while(|s| s.is_empty());
    let user = split.next().ok_or(format!("Can't detect user name from path {}", path))?;
    let mut repo = split.next().ok_or(format!("Can't detect repository name from path {}", path))?;
    if repo.ends_with(".git") {
        // Slice '.git' from 'repo.git'
        repo = &repo[0 .. repo.len() - 4];
    }
    Ok((user, repo))
}

// Known URL formats
//  https://hosting_service.com/user/repo.git
//  git@hosting_service.com:user/repo.git (-> ssh://git@hosting_service.com:22/user/repo.git)
pub fn parse_and_build_page_url(repo: &String, page: &Page, branch: &Option<String>) -> util::Result<String> {
    let url = Url::parse(repo).map_err(|e| format!("{}", e))?;
    let path = url.path();
    let (user, repo_name) = user_and_repo_from_path(path)?;
    let host = url.host_str().ok_or(format!("Failed to parse host from {}", repo))?;
    match host {
        "github.com" => Ok(build_github_url(user, repo_name, branch, page)),
        "bitbucket.org" => build_bitbucket_url(user, repo_name, branch, page),
        "gitlab.com" => Ok(build_gitlab_url(user, repo_name, branch, page)),
        host => if host.starts_with("github.") {
            Ok(build_github_enterprise_url(host, user, repo_name, branch, page))
        } else {
            Err(format!("Unknown hosting service for URL {}", repo))
        },
    }
}
