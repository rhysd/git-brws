extern crate reqwest;
extern crate serde;

use crate::error::{Error, Result};
use reqwest::{header, Proxy, StatusCode};
use std::mem;

#[derive(Debug, Deserialize)]
struct ParentRepoOwner {
    login: String,
}
#[derive(Debug, Deserialize)]
struct ParentRepo {
    name: String,
    owner: ParentRepoOwner,
}
#[derive(Debug, Deserialize)]
struct RepoForParent {
    parent: Option<ParentRepo>,
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

pub struct Client {
    client: reqwest::Client,
    token: Option<String>,
    endpoint: String,
}

impl Client {
    pub fn build<T, U, V>(endpoint: &T, token: Option<U>, https_proxy: &Option<V>) -> Result<Self>
    where
        T: ToString + ?Sized,
        U: ToString,
        V: AsRef<str>,
    {
        let mut b = reqwest::Client::builder();

        if let Some(ref p) = https_proxy {
            b = b.proxy(Proxy::https(p.as_ref())?);
        }

        Ok(Self {
            client: b.build()?,
            token: token.map(|s| s.to_string()),
            endpoint: endpoint.to_string(),
        })
    }

    pub fn send(&self, mut req: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        req = req.header(header::ACCEPT, "application/vnd.github.v3+json");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let mut res = req.send()?;

        let status = res.status();
        if status == StatusCode::OK {
            Ok(res)
        } else {
            Err(Error::GitHubStatusFailure {
                status,
                msg: res.text().unwrap(),
            })
        }
    }

    pub fn find_pr_url(
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
        let mut res = self.send(req)?;
        let mut issues: Issues = res.json()?;

        if issues.items.is_empty() {
            Ok(None)
        } else {
            let html_url = mem::replace(&mut issues.items[0].html_url, String::new());
            Ok(Some(html_url))
        }
    }

    pub fn parent_repo<S, T>(&self, author: S, repo: T) -> Result<Option<(String, String)>>
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let author = author.as_ref();
        let repo = repo.as_ref();
        let url = format!("https://{}/repos/{}/{}", self.endpoint, author, repo);
        let req = self.client.get(url.as_str());
        let mut res = self.send(req)?;
        let repo: RepoForParent = res.json()?;

        match repo.parent {
            Some(p) => Ok(Some((p.owner.login, p.name))),
            None => Ok(None),
        }
    }

    pub fn most_popular_repo_by_name<S: AsRef<str>>(&self, name: S) -> Result<SearchedRepo> {
        // XXX: No query syntax for exact matching to repository name. Use `in:name` instead though
        // it's matching to substrings.
        let query = format!("{} in:name", name.as_ref());
        let params = [("q", query.as_str()), ("per_page", "1")];
        let url = format!("https://{}/search/repositories", self.endpoint);
        let req = self.client.get(&url).query(&params);
        let mut res = self.send(req)?;
        let mut results: SearchResults = res.json()?;

        if results.items.is_empty() {
            Err(Error::NoSearchResult { query })
        } else {
            Ok(mem::replace(&mut results.items[0], SearchedRepo::default()))
        }
    }

    pub fn repo_homepage<S: AsRef<str>, U: AsRef<str>>(
        &self,
        owner: S,
        repo: U,
    ) -> Result<Option<String>> {
        let owner = owner.as_ref();
        let repo = repo.as_ref();
        let url = format!("https://{}/repos/{}/{}", self.endpoint, owner, repo);
        let req = self.client.get(url.as_str());
        let mut res = self.send(req)?;
        let repo: RepoForHomepage = res.json()?;
        Ok(repo.homepage)
    }
}
