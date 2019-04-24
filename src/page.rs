use crate::config::Config;
use crate::error::{Error, ExpectedNumberOfArgs, Result};
use crate::git::Git;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DiffOp {
    TwoDots,   // '..'
    ThreeDots, // '...'
}

impl fmt::Display for DiffOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiffOp::TwoDots => write!(f, ".."),
            DiffOp::ThreeDots => write!(f, "..."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Line {
    At(usize),
    Range(usize, usize), // start and end
}

#[derive(Debug, PartialEq)]
pub enum Page {
    Open {
        website: bool,
        pull_request: bool,
    },
    Diff {
        lhs: String,
        rhs: String,
        op: DiffOp, // '...' or '..'
    },
    Commit {
        hash: String,
    },
    FileOrDir {
        relative_path: String,
        hash: String,
        line: Option<Line>,
        blame: bool,
    },
    Issue {
        number: usize,
    },
    Tag {
        tagname: String,
        commit: String,
    },
}

struct BrowsePageParser<'a> {
    cfg: &'a Config,
    git: Git<'a>,
}

impl<'a> BrowsePageParser<'a> {
    fn wrong_number_of_args(&self, expected: ExpectedNumberOfArgs, kind: &str) -> Result<Page> {
        Err(Error::WrongNumberOfArgs {
            expected,
            actual: self.cfg.args.len(),
            kind: kind.to_string(),
        })
    }

    fn try_parse_commit(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            self.wrong_number_of_args(ExpectedNumberOfArgs::Single(1), "commit")
        } else {
            let hash = self.git.hash(&self.cfg.args[0])?;
            Ok(Page::Commit { hash })
        }
    }

    fn try_parse_tag(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            self.wrong_number_of_args(ExpectedNumberOfArgs::Single(1), "tag name")
        } else {
            let hash = self.git.tag_hash(&self.cfg.args[0])?;
            Ok(Page::Tag {
                tagname: self.cfg.args[0].clone(),
                commit: hash,
            })
        }
    }

    fn try_parse_diff(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return self.wrong_number_of_args(ExpectedNumberOfArgs::Single(1), "diff");
        }

        let arg = &self.cfg.args[0];
        let dots = if arg.contains("...") {
            "..."
        } else if arg.contains("..") {
            ".."
        } else {
            return Err(Error::DiffDotsNotFound);
        };

        let mut split = arg.splitn(2, dots);
        let lhs = split.next().unwrap();
        let rhs = split.next().unwrap();

        if lhs.is_empty() || rhs.is_empty() {
            return Err(Error::DiffHandIsEmpty {
                input: arg.to_string(),
            });
        }

        Ok(Page::Diff {
            lhs: self.git.hash(&lhs)?,
            rhs: self.git.hash(&rhs)?,
            op: if dots == ".." {
                DiffOp::TwoDots
            } else {
                DiffOp::ThreeDots
            },
        })
    }

    fn parse_path_and_line(&self) -> (&str, Option<Line>) {
        let arg = &self.cfg.args[0];

        let line_start = match arg.find('#') {
            Some(i) => i,
            None => return (arg.as_str(), None),
        };

        let mut idx = line_start;
        if arg.chars().nth(idx + 1) == Some('L') {
            // Skip 'L' of file#L123
            idx += 1;
        }

        let path = &arg[..line_start];
        let line_spec = &arg[idx + 1..];
        if let Some(mut dash_idx) = line_spec.find('-') {
            let start = (&line_spec[..dash_idx]).parse().ok();

            if line_spec.chars().nth(dash_idx + 1) == Some('L') {
                // Skip second 'L' of file#L123-L345
                dash_idx += 1;
            }
            let end = (&line_spec[dash_idx + 1..]).parse().ok();

            (
                path,
                start.map(|s| match end {
                    Some(e) => Line::Range(s, e),
                    None => Line::At(s),
                }),
            )
        } else {
            let line = line_spec.parse().ok();
            (path, line.map(Line::At))
        }
    }

    fn try_parse_file_or_dir(&self) -> Result<Page> {
        let len = self.cfg.args.len();
        if len != 1 && len != 2 {
            return self
                .wrong_number_of_args(ExpectedNumberOfArgs::Range(1, 2), "file or directory");
        }

        let (path, line) = self.parse_path_and_line();
        let path = fs::canonicalize(path)?;

        if line.is_some() && path.is_dir() {
            return Err(Error::LineSpecifiedForDir(path));
        }

        let repo_root = self.git.root_dir()?;
        let relative_path = path
            .strip_prefix(&repo_root)
            .map_err(|_| Error::FileDirNotInRepo {
                repo_root: repo_root.to_owned(),
                path: path.clone(),
            })?
            .to_str()
            .expect("Failed to convert path into UTF-8 string")
            .to_string();

        let hash = if len == 2 {
            self.git.hash(self.cfg.args[1].as_str())?
        } else {
            self.git.hash("HEAD")?
        };

        Ok(Page::FileOrDir {
            relative_path,
            hash,
            line,
            blame: self.cfg.blame,
        })
    }

    fn try_parse_issue_number(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return self.wrong_number_of_args(ExpectedNumberOfArgs::Single(1), "issue number");
        }

        let arg = &self.cfg.args[0];
        if !arg.starts_with('#') {
            return Err(Error::InvalidIssueNumberFormat);
        }
        let arg = &arg[1..];
        let number: usize = arg.parse().map_err(|_| Error::InvalidIssueNumberFormat)?;
        Ok(Page::Issue { number })
    }
}

pub fn parse_page(cfg: &Config) -> Result<Page> {
    let mut attempts = Vec::with_capacity(5);

    // Note: Ignore any arguments when opening a website
    if cfg.args.is_empty() || cfg.website || cfg.pull_request {
        return Ok(Page::Open {
            website: cfg.website,
            pull_request: cfg.pull_request,
        });
    }

    let git = cfg.git().ok_or_else(|| Error::NoLocalRepoFound {
        operation: format!("opening URL with options {:?}", cfg.args),
    })?;

    let parser = BrowsePageParser { cfg, git };

    match parser.try_parse_issue_number() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(("Issue number", msg)),
    }

    match parser.try_parse_file_or_dir() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(("File or dir", msg)),
    }

    if cfg.blame {
        return Err(Error::BlameWithoutFilePath);
    }

    match parser.try_parse_diff() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(("Diff", msg)),
    }

    match parser.try_parse_tag() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(("Tag", msg)),
    }

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(("Commit", msg)),
    }

    Err(Error::PageParseError {
        args: cfg.args.clone(),
        attempts,
    })
}
