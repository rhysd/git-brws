extern crate reqwest;
extern crate serde;

use self::reqwest::{header, Proxy, StatusCode};
use crate::error::{Error, Result};

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
        let issues: Issues = res.json()?;

        if issues.items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(issues.items[0].html_url.clone()))
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
        let repo: Repo = res.json()?;

        match repo.parent {
            Some(p) => Ok(Some((p.owner.login, p.name))),
            None => Ok(None),
        }
    }
}
