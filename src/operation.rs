use command;
use git;

pub enum Page {
    Open,
    Diff {
        lhs: String,
        rhs: String,
    },
    Commit {
        hash: String,
    },
}

pub struct Operation {
    remote: Option<String>,
    page: Page,
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
        Ok(Page::Commit {hash: hash})
    }
}

pub fn parse_operation(opts: command::Options) -> Result<Operation, ErrorMsg> {
    let mut errors = vec!["Error on parsing command line arguments".to_string()];

    let parser = BrowsePageParser {
        opts: &opts,
        git: git::new(&opts.dir),
    };

    match parser.try_parse_commit() {
        Ok(p) => return Ok(p),
        Err(msg) => errors.push(msg),
    };

    Err(errors.join("\n"))
}
