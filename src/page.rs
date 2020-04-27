use crate::config::Config;
use crate::error::{Error, ErrorKind, ExpectedNumberOfArgs, Result};
use crate::git::Git;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DiffOp {
    TwoDots,   // '..'
    ThreeDots, // '...'
}

impl fmt::Display for DiffOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffOp::TwoDots => write!(f, ".."),
            DiffOp::ThreeDots => write!(f, "..."),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    fn shorten_hash(&self, mut hash: String) -> String {
        if self.cfg.env.short_commit_hash {
            hash.truncate(7);
        }
        hash
    }

    fn wrong_number_of_args(&self, expected: ExpectedNumberOfArgs, kind: &str) -> Result<Page> {
        Error::err(ErrorKind::WrongNumberOfArgs {
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
            Ok(Page::Commit {
                hash: self.shorten_hash(hash),
            })
        }
    }

    fn try_parse_tag(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            self.wrong_number_of_args(ExpectedNumberOfArgs::Single(1), "tag name")
        } else {
            let hash = self.git.tag_hash(&self.cfg.args[0])?;
            Ok(Page::Tag {
                tagname: self.cfg.args[0].clone(),
                commit: self.shorten_hash(hash),
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
            return Error::err(ErrorKind::DiffDotsNotFound);
        };

        let mut split = arg.splitn(2, dots);
        let lhs = split.next().unwrap();
        let rhs = split.next().unwrap();

        if lhs.is_empty() || rhs.is_empty() {
            return Error::err(ErrorKind::DiffHandIsEmpty {
                input: arg.to_string(),
            });
        }

        let lhs = self.git.hash(&lhs)?;
        let rhs = self.git.hash(&rhs)?;
        Ok(Page::Diff {
            lhs: self.shorten_hash(lhs),
            rhs: self.shorten_hash(rhs),
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

        if path.is_dir() {
            if self.cfg.blame {
                return Error::err(ErrorKind::CannotBlameDirectory {
                    dir: path.to_string_lossy().into(),
                });
            }
            if line.is_some() {
                return Error::err(ErrorKind::LineSpecifiedForDir(path));
            }
        }

        let repo_root = self.git.root_dir()?;
        let relative_path = path
            .strip_prefix(&repo_root)
            .map_err(|_| {
                Error::new(ErrorKind::FileDirNotInRepo {
                    repo_root: repo_root.to_owned(),
                    path: path.clone(),
                })
            })?
            .to_str()
            .expect("Failed to convert path into UTF-8 string")
            .to_string();

        let mut hash = if len == 2 {
            self.git.hash(self.cfg.args[1].as_str())?
        } else {
            self.git.hash("HEAD")?
        };

        // Fall back into branch name when the commit is not existing in remote branch (#12)
        //
        // Ignore this check when the local branch does not point to any remote branch
        let remote_contains_hash = match self.git.remote_branch(&self.cfg.remote, &self.cfg.branch)
        {
            Ok(remote_branch) => self.git.remote_contains(&hash, &remote_branch)?,
            Err(_) => true, // Ignore check
        };
        if !remote_contains_hash {
            hash = match &self.cfg.branch {
                Some(b) => b.clone(),
                None => self.git.current_branch()?,
            };
        } else {
            // Use full-length hash for Git::remote_contains()
            hash = self.shorten_hash(hash);
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
            return Error::err(ErrorKind::InvalidIssueNumberFormat);
        }
        let arg = &arg[1..];
        let number: usize = arg
            .parse()
            .map_err(|_| Error::new(ErrorKind::InvalidIssueNumberFormat))?;
        Ok(Page::Issue { number })
    }
}

pub fn parse_page(cfg: &Config) -> Result<Page> {
    let mut attempts = Vec::with_capacity(5);

    // Note: Ignore any arguments when opening a website
    if cfg.args.is_empty() || cfg.website || cfg.pull_request {
        if cfg.blame {
            return Error::err(ErrorKind::BlameWithoutFilePath);
        }

        return Ok(Page::Open {
            website: cfg.website,
            pull_request: cfg.pull_request,
        });
    }

    let parser = BrowsePageParser {
        cfg,
        git: cfg.git(),
    };

    match parser.try_parse_issue_number() {
        Ok(p) => return Ok(p),
        Err(e) => attempts.push(("Issue number", *e)),
    }

    // Note: Early return for --blame
    match parser.try_parse_file_or_dir() {
        Ok(p) => return Ok(p),
        Err(err) => match err.kind() {
            ErrorKind::CannotBlameDirectory { .. } => return Err(err),
            _ if cfg.blame => return Error::err(ErrorKind::BlameWithoutFilePath),
            _ => attempts.push(("File or dir", *err)),
        },
    }

    match parser.try_parse_diff() {
        Ok(p) => return Ok(p),
        Err(e) => attempts.push(("Diff", *e)),
    }

    match parser.try_parse_tag() {
        Ok(p) => return Ok(p),
        Err(e) => attempts.push(("Tag", *e)),
    }

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(e) => attempts.push(("Commit", *e)),
    }

    Error::err(ErrorKind::PageParseError {
        args: cfg.args.clone(),
        attempts,
    })
}
