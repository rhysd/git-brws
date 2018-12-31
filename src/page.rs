use crate::command;
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
pub enum Page {
    Open,
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
        line: Option<usize>,
    },
    Issue {
        number: usize,
    },
}

struct BrowsePageParser<'a> {
    cfg: &'a command::Config,
    git: Git<'a>,
}

impl<'a> BrowsePageParser<'a> {
    fn try_parse_commit(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return Err(Error::WrongNumberOfArgs {
                expected: ExpectedNumberOfArgs::Single(1),
                actual: self.cfg.args.len(),
                kind: "commit".to_string(),
            });
        }
        let hash = self.git.hash(&self.cfg.args[0])?;
        Ok(Page::Commit { hash })
    }

    fn try_parse_diff(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return Err(Error::WrongNumberOfArgs {
                expected: ExpectedNumberOfArgs::Single(1),
                actual: self.cfg.args.len(),
                kind: "diff".to_string(),
            });
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

    fn parse_path_and_line(&self) -> (&str, Option<usize>) {
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
        let line = (&arg[idx + 1..]).parse().ok();
        (&arg[..line_start], line)
    }

    fn try_parse_file_or_dir(&self) -> Result<Page> {
        let len = self.cfg.args.len();
        if len != 1 && len != 2 {
            return Err(Error::WrongNumberOfArgs {
                expected: ExpectedNumberOfArgs::Range(1, 2),
                actual: len,
                kind: "file or directory".to_string(),
            });
        }

        let (path, line) = self.parse_path_and_line();
        let path = fs::canonicalize(path)?;

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
        })
    }

    fn try_parse_issue_number(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return Err(Error::WrongNumberOfArgs {
                expected: ExpectedNumberOfArgs::Single(1),
                actual: self.cfg.args.len(),
                kind: "issue number".to_string(),
            });
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

pub fn parse_page(cfg: &command::Config) -> Result<Page> {
    let mut attempts = Vec::with_capacity(4);

    let parser = BrowsePageParser {
        cfg,
        git: Git::new(&cfg.git_dir, cfg.env.git_command.as_str()),
    };

    if cfg.args.is_empty() {
        return Ok(Page::Open);
    }

    match parser.try_parse_issue_number() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(msg),
    }

    match parser.try_parse_file_or_dir() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(msg),
    }

    match parser.try_parse_diff() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(msg),
    }

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => attempts.push(msg),
    }

    Err(Error::PageParseError {
        args: cfg.args.clone(),
        attempts,
    })
}
