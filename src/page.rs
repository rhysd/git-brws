use std::fs;

use crate::command;
use crate::errors::Result;
use crate::git;

#[derive(Debug, PartialEq)]
pub enum Page {
    Open,
    Diff {
        lhs: String,
        rhs: String,
    },
    Commit {
        hash: String,
    },
    FileOrDir {
        relative_path: String,
        hash: String,
        line: Option<usize>,
    },
}

struct BrowsePageParser<'a> {
    cfg: &'a command::Config,
    git: git::Git<'a>,
}

impl<'a> BrowsePageParser<'a> {
    fn try_parse_commit(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return Err("  Invalid number of arguments for commit (1 is expected)".to_string());
        }
        let hash = self.git.hash(&self.cfg.args[0])?;
        Ok(Page::Commit { hash })
    }

    fn try_parse_diff(&self) -> Result<Page> {
        if self.cfg.args.len() != 1 {
            return Err("  Invalid number of arguments for diff (1 is expected)".to_string());
        }

        let mut split = self.cfg.args[0].splitn(2, "..");
        let lhs = split.next().unwrap();
        let rhs = split.next().ok_or_else(|| {
            format!(
                "  Diff format must be specified as LHS..RHS but found {}",
                self.cfg.args[0]
            )
        })?;

        Ok(Page::Diff {
            lhs: self.git.hash(&lhs)?,
            rhs: self.git.hash(&rhs)?,
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
            return Err(
                "  Invalid number of arguments for file or directory (1..2 is expected)"
                    .to_string(),
            );
        }

        let (path, line) = self.parse_path_and_line();

        let entry = fs::canonicalize(path)
            .map_err(|e| format!("  Unable to locate file '{}': {}", path, e))?;
        let relative_path = entry
            .strip_prefix(&self.git.root_dir()?)
            .map_err(|e| format!("  Unable to locate the file in repository: {}", e))?
            .to_str()
            .ok_or("  Failed to convert path into UTF-8 string")?
            .to_string();

        let hash = if len == 2 {
            self.git.hash(&self.cfg.args[1].as_str())?
        } else {
            self.git.hash(&"HEAD")?
        };
        Ok(Page::FileOrDir {
            relative_path,
            hash,
            line,
        })
    }
}

pub fn parse_page(cfg: &command::Config) -> Result<Page> {
    let mut errors = vec!["Error on parsing command line arguments".to_string()];

    let parser = BrowsePageParser {
        cfg,
        git: git::new(&cfg.git_dir)?,
    };

    if cfg.args.is_empty() {
        return Ok(Page::Open);
    }

    match parser.try_parse_file_or_dir() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    match parser.try_parse_diff() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    Err(errors.join("\n"))
}
