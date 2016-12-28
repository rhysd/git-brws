extern crate url;

use page::Page;
use self::url::Url;

type ErrorMsg = String;
type UrlResult = Result<String, ErrorMsg>;

fn parse_github_url(user: &str, repo: &str, branch: &String, page: &Page) -> String {
    match page {
        &Page::Open => format!("https://github.com/{}/{}/tree/{}", user, repo, branch),
        &Page::Diff {ref lhs, ref rhs} => format!("https://github.com/{}/{}/compare/{}...{}", user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://github.com/{}/{}/commit/{}", user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://github.com/{}/{}/blob/{}/{}", user, repo, hash, relative_path),
    }
}

fn parse_bitbucket_url(user: &str, repo: &str, branch: &String, page: &Page) -> UrlResult {
    match page {
        &Page::Open => Ok(format!("https://bitbucket.org/{}/{}/branch/{}", user, repo, branch)),
        &Page::Diff {..} => Err("BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)".to_string()),
        &Page::Commit {ref hash} => Ok(format!("https://bitbucket.org/{}/{}/commits/{}", user, repo, hash)),
        &Page::FileOrDir {ref relative_path, ref hash} => Ok(format!("https://bitbucket.org/{}/{}/src/{}/{}", user, repo, hash, relative_path)),
    }
}

fn parse_github_enterprise_url(host: &str, user: &str, repo: &str, branch: &String, page: &Page) -> String {
    match page {
        &Page::Open => format!("https://{}/{}/{}/tree/{}", host, user, repo, branch),
        &Page::Diff {ref lhs, ref rhs} => format!("https://{}/{}/{}/compare/{}...{}", host, user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://{}/{}/{}/commit/{}", host, user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://{}/{}/{}/blob/{}/{}", host, user, repo, hash, relative_path),
    }
}

fn parse_gitlab_url(user: &str, repo: &str, branch: &String, page: &Page) -> String {
    match page {
        &Page::Open => format!("https://gitlab.com/{}/{}/tree/{}", user, repo, branch),
        &Page::Diff {ref lhs, ref rhs} => format!("https://gitlab.com/{}/{}/compare/{}...{}", user, repo, lhs, rhs),
        &Page::Commit {ref hash} => format!("https://gitlab.com/{}/{}/commit/{}", user, repo, hash),
        &Page::FileOrDir {ref relative_path, ref hash} => format!("https://gitlab.com/{}/{}/blob/{}/{}", user, repo, hash, relative_path),
    }
}

// Note: Parse '/user/repo.git' or '/user/repo' or 'user/repo' into 'user' and 'repo'
fn user_and_repo_from_path<'a>(path: &'a str) -> Result<(&'a str, &'a str), ErrorMsg> {
    let mut split = path.split('/').skip_while(|s| s.is_empty());
    let user = split.next().ok_or(format!("Can't detect user name from path {}", path))?;
    let mut repo = split.next().ok_or(format!("Can't detect repository name from path {}", path))?;
    if repo.ends_with(".git") {
        // Slice '.git' from 'repo.git'
        repo = &repo[0 .. repo.len() - 4];
    }
    Ok((user, repo))
}

// Known URLs
//
// GitHub:
//  https://github.com/user/repo.git
//  git@github.com:user/repo.git (-> ssh://git@github.com:22/user/repo.git)
pub fn parse_url(repo: &String, branch: &String, page: &Page) -> UrlResult {
    let url = Url::parse(repo).map_err(|e| format!("{}", e))?;
    let path = url.path();
    let (user, repo_name) = user_and_repo_from_path(path)?;
    let host = url.host_str().ok_or(format!("Failed to parse host from {}", repo))?;
    match host {
        "github.com" => Ok(parse_github_url(user, repo_name, branch, page)),
        "bitbucket.org" => parse_bitbucket_url(user, repo_name, branch, page),
        "gitlab.com" => Ok(parse_gitlab_url(user, repo_name, branch, page)),
        host => if host.starts_with("github.") {
            Ok(parse_github_enterprise_url(host, user, repo_name, branch, page))
        } else {
            Err(format!("Unknown hosting service for URL {}", repo))
        },
    }
}
