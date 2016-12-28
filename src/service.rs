extern crate url;

use page::Page;
use self::url::Url;

type ErrorMsg = String;
type UrlResult = Result<String, ErrorMsg>;

pub struct Service {
    user: String,
    repo: String,
    host: String,
}

impl Service {
    fn github_url(&self, page: &Page, branch: &Option<String>) -> String {
        match page {
            &Page::Open => if let &Some(ref b) = branch {
                format!("https://github.com/{}/{}/tree/{}", self.user, self.repo, b)
            } else {
                format!("https://github.com/{}/{}", self.user, self.repo)
            },
            &Page::Diff {ref lhs, ref rhs} => format!("https://github.com/{}/{}/compare/{}...{}", self.user, self.repo, lhs, rhs),
            &Page::Commit {ref hash} => format!("https://github.com/{}/{}/commit/{}", self.user, self.repo, hash),
            &Page::FileOrDir {ref relative_path, ref hash} => format!("https://github.com/{}/{}/blob/{}/{}", self.user, self.repo, hash, relative_path),
        }
    }

    fn bitbucket_url(&self, page: &Page, branch: &Option<String>) -> UrlResult {
        match page {
            &Page::Open => if let &Some(ref b) = branch {
                Ok(format!("https://bitbucket.org/{}/{}/branch/{}", self.user, self.repo, b))
            } else {
                Ok(format!("https://bitbucket.org/{}/{}", self.user, self.repo))
            },
            &Page::Diff {..} => Err("BitBucket does not support diff between commits (see https://bitbucket.org/site/master/issues/4779/ability-to-diff-between-any-two-commits)".to_string()),
            &Page::Commit {ref hash} => Ok(format!("https://bitbucket.org/{}/{}/commits/{}", self.user, self.repo, hash)),
            &Page::FileOrDir {ref relative_path, ref hash} => Ok(format!("https://bitbucket.org/{}/{}/src/{}/{}", self.user, self.repo, hash, relative_path)),
        }
    }

    fn github_enterprise_url(&self, page: &Page, branch: &Option<String>) -> String {
        match page {
            &Page::Open => if let &Some(ref b) = branch {
                format!("https://{}/{}/{}/tree/{}", self.host, self.user, self.repo, b)
            } else {
                format!("https://{}/{}/{}", self.host, self.user, self.repo)
            },
            &Page::Diff {ref lhs, ref rhs} => format!("https://{}/{}/{}/compare/{}...{}", self.host, self.user, self.repo, lhs, rhs),
            &Page::Commit {ref hash} => format!("https://{}/{}/{}/commit/{}", self.host, self.user, self.repo, hash),
            &Page::FileOrDir {ref relative_path, ref hash} => format!("https://{}/{}/{}/blob/{}/{}", self.host, self.user, self.repo, hash, relative_path),
        }
    }

    fn gitlab_url(&self, page: &Page, branch: &Option<String>) -> String {
        match page {
            &Page::Open => if let &Some(ref b) = branch {
                format!("https://gitlab.com/{}/{}/tree/{}", self.user, self.repo, b)
            } else {
                format!("https://gitlab.com/{}/{}", self.user, self.repo)
            },
            &Page::Diff {ref lhs, ref rhs} => format!("https://gitlab.com/{}/{}/compare/{}...{}", self.user, self.repo, lhs, rhs),
            &Page::Commit {ref hash} => format!("https://gitlab.com/{}/{}/commit/{}", self.user, self.repo, hash),
            &Page::FileOrDir {ref relative_path, ref hash} => format!("https://gitlab.com/{}/{}/blob/{}/{}", self.user, self.repo, hash, relative_path),
        }
    }

    pub fn page_url(&self, page: &Page, branch: &Option<String>) -> UrlResult {
        match self.host.as_str() {
            "github.com" => Ok(self.github_url(page, branch)),
            "bitbucket.org" => self.bitbucket_url(page, branch),
            "gitlab.com" => Ok(self.gitlab_url(page, branch)),
            host => if host.starts_with("github.") {
                Ok(self.github_enterprise_url(page, branch))
            } else {
                Err(format!("Unknown hosting service for URL {}", self.repo))
            },
        }
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

pub fn parse_service(repo: &String) -> Result<Service, ErrorMsg> {
    let url = Url::parse(repo).map_err(|e| format!("{}", e))?;
    let path = url.path();
    let (user, repo_name) = user_and_repo_from_path(path)?;
    let host = url.host_str().ok_or(format!("Failed to parse host from {}", repo))?;
    Ok(Service {
        user: user.to_string(),
        repo: repo_name.to_string(),
        host: host.to_string(),
    })
}
