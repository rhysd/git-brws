use command;
use git;

pub enum Page {
    Open {
        branch: String,
    },
    Diff {
        lhs: String,
        rhs: String,
    },
    Commit {
        hash: String,
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
