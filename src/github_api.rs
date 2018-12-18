extern crate reqwest;
extern crate serde;

use self::reqwest::{header, Proxy, StatusCode};
use crate::errors::Result;

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
struct Repo {
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

pub struct Client {
    client: reqwest::Client,
    token: Option<String>,
    host: String,
}

impl Client {
    pub fn build<T, U, V>(host: &T, token: Option<U>, https_proxy: &Option<V>) -> Result<Self>
    where
        T: ToString + ?Sized,
        U: ToString,
        V: AsRef<str>,
    {
        let mut b = reqwest::Client::builder();

        if let Some(ref p) = https_proxy {
            b = b.proxy(
                Proxy::https(p.as_ref())
                    .map_err(|e| format!("Cannot setup HTTPS proxy {}: {}", p.as_ref(), e))?,
            );
        }

        let client = b
            .build()
            .map_err(|e| format!("Cannot setup HTTP client: {}", e))?;

        Ok(Self {
            client,
            token: token.map(|s| s.to_string()),
            host: host.to_string(),
        })
    }

    pub fn find_pr_url<S, T, U, V>(&self, branch: S, author: T, owner: U, repo: V) -> Result<String>
    where
        S: AsRef<str>,
        T: AsRef<str>,
        U: AsRef<str>,
        V: AsRef<str>,
    {
        let params = [(
            "q",
            format!(
                "type:pr head:{} author:{} repo:{}/{}",
                branch.as_ref(),
                author.as_ref(),
                owner.as_ref(),
                repo.as_ref()
            ),
        )];

        let url = format!("https://api.{}/search/issues", self.host);
        let mut req = self
            .client
            .get(url.as_str())
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .query(&params);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let mut res = req
            .send()
            .map_err(|err| format!("Cannot send request to {}: {}", url, err))?;

        let status = res.status();
        if status != StatusCode::OK {
            return Err(format!(
                "API response status {}: {}",
                status,
                res.text().unwrap()
            ));
        }

        let issues: Issues = res
            .json()
            .map_err(|err| format!("Cannot deserialize JSON from {}: {}", url, err,))?;

        if issues.items.is_empty() {
            return Err(format!(
                "No result found for {}/{} authored by {} at branch {}",
                owner.as_ref(),
                repo.as_ref(),
                author.as_ref(),
                branch.as_ref(),
            ));
        }

        Ok(issues.items[0].html_url.clone())
    }

    pub fn parent_repo<S, T>(&self, author: S, repo: T) -> Result<Option<(String, String)>>
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        let author = author.as_ref();
        let repo = repo.as_ref();
        let url = format!("https://api.{}/repos/{}/{}", self.host, author, repo);
        let mut req = self
            .client
            .get(url.as_str())
            .header(header::ACCEPT, "application/vnd.github.v3+json");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let mut res = req
            .send()
            .map_err(|e| format!("Cannot send request to {}: {}", url, e))?;

        let status = res.status();
        if status != StatusCode::OK {
            return Err(format!(
                "API response status {}: {}",
                status,
                res.text().unwrap()
            ));
        }

        let repo: Repo = res
            .json()
            .map_err(|e| format!("Cannot deserialize JSON from {}: {}", url, e))?;

        match repo.parent {
            Some(p) => Ok(Some((p.owner.login, p.name))),
            None => Ok(None),
        }
    }
}
