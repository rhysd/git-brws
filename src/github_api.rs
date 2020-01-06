use crate::error::{Error, Result};
use reqwest::{header, Proxy, StatusCode};
use reqwest::{Client as ReqwestClient, RequestBuilder, Response};
use std::mem;

#[derive(Debug, Deserialize)]
pub struct ParentRepoOwner {
    pub login: String,
}
#[derive(Debug, Deserialize)]
pub struct ParentRepo {
    pub name: String,
    pub owner: ParentRepoOwner,
    pub default_branch: String,
}
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub parent: Option<ParentRepo>,
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
struct Issue {
    html_url: String,
}
#[derive(Debug, Deserialize)]
struct Issues {
    items: Vec<Issue>,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchedRepo {
    pub clone_url: String,
}
#[derive(Debug, Deserialize)]
struct SearchResults {
    items: Vec<SearchedRepo>,
}

#[derive(Debug, Deserialize)]
struct RepoForHomepage {
    homepage: Option<String>,
}

pub struct Client<'a> {
    client: ReqwestClient,
    token: Option<&'a str>,
    endpoint: &'a str,
}

impl<'a> Client<'a> {
    pub fn build<T: AsRef<str>, U: AsRef<str>>(endpoint: &'a str, token: &'a Option<T>, https_proxy: &Option<U>) -> Result<Self> {
        // GitHub API requires user agent in headers: https://developer.github.com/v3/#user-agent-required
        let mut b = ReqwestClient::builder().user_agent("git-brws");

        if let Some(ref p) = https_proxy {
            b = b.proxy(Proxy::https(p.as_ref())?);
        }

        Ok(Self {
            client: b.build()?,
            token: token.as_ref().map(AsRef::as_ref),
            endpoint,
        })
    }

    pub async fn send(&self, mut req: RequestBuilder) -> Result<Response> {
        req = req.header(header::ACCEPT, "application/vnd.github.v3+json");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let res = req.send().await?;

        let status = res.status();
        if status == StatusCode::OK {
            Ok(res)
        } else {
            Err(Error::GitHubStatusFailure {
                status,
                msg: res.text().await.unwrap(),
            })
        }
    }

    pub async fn find_pr_url(
        &self,
        branch: &str,
        owner: &str,
        repo: &str,
        pr_author: Option<&str>,
    ) -> Result<Option<String>> {
        let query = if let Some(author) = pr_author {
            format!(
                "type:pr head:{} author:{} repo:{}/{}",
                branch, author, owner, repo,
            )
        } else {
            format!("type:pr head:{} repo:{}/{}", branch, owner, repo)
        };
        let params = [("q", query.as_str()), ("sort", "updated")];
        let url = format!("https://{}/search/issues", self.endpoint);
        let req = self.client.get(url.as_str()).query(&params);
        let res = self.send(req).await?;
        let mut issues: Issues = res.json().await?;

        if issues.items.is_empty() {
            Ok(None)
        } else {
            let html_url = mem::replace(&mut issues.items[0].html_url, String::new());
            Ok(Some(html_url))
        }
    }

    pub async fn repo<S, T>(&self, author: S, repo: T) -> Result<Repo>
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let author = author.as_ref();
        let repo = repo.as_ref();
        let url = format!("https://{}/repos/{}/{}", self.endpoint, author, repo);
        let req = self.client.get(url.as_str());
        let res = self.send(req).await?;
        let repo: Repo = res.json().await?;
        Ok(repo)
    }

    pub async fn most_popular_repo_by_name<S: AsRef<str>>(&self, name: S) -> Result<SearchedRepo> {
        // XXX: No query syntax for exact matching to repository name. Use `in:name` instead though
        // it's matching to substrings.
        let query = format!("{} in:name", name.as_ref());
        let params = [("q", query.as_str()), ("per_page", "1")];
        let url = format!("https://{}/search/repositories", self.endpoint);
        let req = self.client.get(&url).query(&params);
        let res = self.send(req).await?;
        let mut results: SearchResults = res.json().await?;

        if results.items.is_empty() {
            Err(Error::NoSearchResult { query })
        } else {
            Ok(mem::replace(&mut results.items[0], SearchedRepo::default()))
        }
    }

    pub async fn repo_homepage<S: AsRef<str>, U: AsRef<str>>(
        &self,
        owner: S,
        repo: U,
    ) -> Result<Option<String>> {
        let owner = owner.as_ref();
        let repo = repo.as_ref();
        let url = format!("https://{}/repos/{}/{}", self.endpoint, owner, repo);
        let req = self.client.get(url.as_str());
        let res = self.send(req).await?;
        let repo: RepoForHomepage = res.json().await?;
        Ok(repo.homepage)
    }
}
