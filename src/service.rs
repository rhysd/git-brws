use page::Page;

type ErrorMsg = String;

pub struct GitHub<'a> {
    repo: &'a String,
    branch: &'a String,
}

enum Service<'a> {
    GitHub(GitHub<'a>),
}

impl<'a> GitHub<'a> {
    fn page_url(&self, page: Page) -> Result<String, ErrorMsg> {
        match page {
            Page::Open => Ok("".to_string()),
            Page::Diff {lhs, rhs} => Ok("".to_string()),
            Page::Commit {hash} => Ok("".to_string()),
            Page::FileOrDir {relative_path} => Ok("".to_string()),
        }
    }
}

fn new<'a>(repo: &'a String, branch: &'a String) -> Service<'a> {
    Service::GitHub(GitHub { repo: repo, branch: branch })
}
