use command;
use git;

use std::fs;

#[derive(Debug)]
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
    },
}

type ErrorMsg = String;
type ParseResult = Result<Page, ErrorMsg>;

struct BrowsePageParser<'a> {
    opts: &'a command::Options,
    git: git::Git<'a>,
}

impl<'a> BrowsePageParser<'a> {
    fn try_parse_commit(&self) -> ParseResult {
        if self.opts.args.len() != 1 {
            return Err("  Invalid number of arguments for commit (1 is expected)".to_string());
        }
        let hash = self.git.hash(&self.opts.args[0])?;
        Ok(Page::Commit {
            hash: hash,
        })
    }

    fn try_parse_diff(&self) -> ParseResult {
        if self.opts.args.len() != 1 {
            return Err("  Invalid number of arguments for diff (1 is expected)".to_string());
        }

        let mut split = self.opts.args[0].splitn(2, "..");
        let lhs = split.next().unwrap();
        let rhs = split.next().ok_or(format!("  Diff format must be specified as LHS..RHS but found {}", self.opts.args[0]))?;

        Ok(Page::Diff {
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
        })
    }

    fn try_parse_file_or_dir(&self) -> ParseResult {
        if self.opts.args.len() != 1 {
            return Err("  Invalid number of arguments for file or directory (1 is expected)".to_string());
        }

        let path = &self.opts.args[0];

        let entry = fs::canonicalize(path).map_err(|e| format!("  Unable to locate file '{}': {}", path, e))?;
        let relative_path = entry
            .strip_prefix(&self.git.root_dir()?)
            .map_err(|e| format!("  Unable to locate the file in repository: {}", e))?
            .to_str()
            .ok_or("  Failed to convert path into UTF-8 string")?
            .to_string();

        Ok(Page::FileOrDir {
            relative_path: relative_path,
        })
    }
}

pub fn parse_page(opts: &command::Options) -> Result<Page, ErrorMsg> {
    let mut errors = vec!["Error on parsing command line arguments".to_string()];

    let parser = BrowsePageParser {
        opts: opts,
        git: git::new(&opts.git_dir)?,
    };

    if opts.args.is_empty() {
        return Ok(Page::Open)
    }

    match parser.try_parse_file_or_dir() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    match parser.try_parse_diff() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    }

    Err(errors.join("\n"))
}
