git brws
========
[![Crate Badge][]][GitHub Project]
[![Build Status][]][CI Results]

`git brws` is a command line tool to open a repository, file, commit or diff in your web browser from command line.

Features:

- Opens a page of
  - Repository ([e.g.](https://github.com/rhysd/git-brws))
  - File ([e.g.](https://github.com/rhysd/git-brws/blob/master/Cargo.toml))
  - Commit ([e.g.](https://github.com/rhysd/git-brws/commit/60024ab1280f9f10423b22bc708f3f6ef97db6b5))
  - Diff ([e.g.](https://github.com/rhysd/git-brws/compare/e3c18d0d50252112d37bde97061370204b3cdab7...60024ab1280f9f10423b22bc708f3f6ef97db6b5))
- Supports below services
  - [GitHub](https://github.com)
  - [Bitbucket](https://bitbucket.org)
  - [GitHub Enterprise](https://enterprise.github.com/home)
  - [GitLab](https://about.gitlab.com/)
- Prefers commit-specific page URL

## Installation

`git brws` currently supports Linux (x86\_64, i686, aarch64) and macOS.

- With [cargo](https://crates.io/)

```
$ cargo install git-brws
```

- As single binary

You can download a binary executable from [release page][].
Unarchive downloaded file and put the binary in your `bin` directory. 

## Usage

```
Usage: git brws [Options] {Args}

Options:
    -r, --repo REPO     Shorthand format (user/repo, service/user/repo) or
                        remote name (e.g. origin) or Git URL you want to see
    -b, --branch BRANCH Branch name of the repository
    -d, --dir PATH      Directory path to your repository
    -u, --url           Output URL to STDOUT instead of opening in browser
    -h, --help          Print this help
    -v, --version       Show version
```

## Usage Examples

### Open a repository page

- Open current repository page

```
$ git brws
```

- Open 'develop' branch

```
$ git brws -b develop
```

- Open 'origin' remote of 'develop' branch

```
$ git brws -r origin -b develop
```

- Open @rhysd's 'Shiba' repository

```
$ git brws -r rhysd/Shiba
```

- Open a repository specified by directory

```
$ git brws -d ~/.go/src/github.com/rhysd/dot-github
```

### Open specific file

- Open specific file of current branch of current remote

```
$ git brws ./some/file.txt
```

- Open specific line of the file

```
$ git brws ./some/file.txt#L123
```

### Open a specific commit page

- Open `HEAD` page of current repository

```
$ git brws HEAD
```

### Show a specific diff page

- Show diff between `HEAD` and `HEAD~3`

```
$ git brws HEAD~3..HEAD
```

- Show diff between `113079b` and `60024ab`

```
$ git brws 60024ab..113079b
```

### Cooperate with other tools

With `-u` option, `git brws` outputs URL to stdout.

For example, in Vim, you can write your repository URL to your text instantly.

```
:r!git brws -u
```

And below can open editing file in your browser.

```
:!git brws %
```

## Customization

You can customize behavior of this command with environment varibles.

- `$GIT_BRWS_GIT_COMMAND`: Git command to use. If not specified, `"git"` will be used.
- `$GIT_BRWS_GITHUB_URL_HOST`: When you use your own GitHub:Enterprise repository, you can specify its host to this variable.
  By default, `git brws` detects `^github\.` as GH:E host. If your GH:E repository host does not match it, please specify
  this variable. If your repository is `https://example-repo.org/user/repo`, `example-repo.org` should be set.
- `GIT_BRWS_GITHUB_SSH_PORT`: When you set a number to it, the number will be used for the ssh port for GITHUB:Enterprise URLs.
- `GIT_BRWS_GITLAB_SSH_PORT`: When you set a number to it, the number will be used for the ssh port for self-hosted GitLab URLs.
  This is useful when your environment hosts GitLab to non-trivial ssh port URL.

## Related Projects

- [hub browse](https://hub.github.com/)
- [git open](https://github.com/paulirish/git-open)
- [open-browser-github.vim](https://github.com/tyru/open-browser-github.vim)

## License

Distributed under [the MIT license](LICENSE.txt).

## TODOs

Please see [the project page](https://github.com/rhysd/git-brws/projects/1).

## Development

```sh
cargo install cargo-watch
```

```sh
# Watch and build sources/tests automatically
cargo watch
```

[GitHub Project]: https://github.com/rhysd/git-brws
[Crate Badge]: https://img.shields.io/crates/v/git-brws.svg
[CI Results]: https://travis-ci.org/rhysd/git-brws
[Build Status]: https://travis-ci.org/rhysd/git-brws.svg?branch=master
[release page]: https://github.com/rhysd/git-brws/releases
