use command;
use git;

pub enum Page {
    Open {
        branch: String,
        remote_url: String,
    },
    Diff {
        lhs: String,
        rhs: String,
        remote_url: String,
    },
    Commit {
        hash: String,
        remote_url: String,
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
        let len = self.opts.args.len();
        if len == 0 || len > 2 {
            return Err("  hash must be specified".to_string());
        }
        if self.opts.repo.is_some() {
            return Err("  --repo can't be specified for commit".to_string());
        }
        let hash = self.git.hash(&self.opts.args[0])?;
        let remote_url = if len == 2 {
            self.git.remote_url(&self.opts.args[1])?
        } else {
            self.git.tracking_remote()?.0
        };
        Ok(Page::Commit {
            hash: hash,
            remote_url: remote_url,
        })
    }
}

pub fn parse_operation(opts: command::Options) -> Result<Page, ErrorMsg> {
    let mut errors = vec!["Error on parsing command line arguments".to_string()];

    let parser = BrowsePageParser {
        opts: &opts,
        git: git::new(&opts.dir)?,
    };

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    };

    Err(errors.join("\n"))
}
