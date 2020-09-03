git brws
========
[![crates.io][crate-badge]][crate]
[![Linux and macOS CI][travis-badge]][travis-ci]
[![Windows CI][appveyor-badge]][appveyor]

`git brws` is a command line tool to open a repository, file, commit, diff, tag, blame, pull request,
issue or project's website in your web browser from command line. 'brws' is an abbreviation of 'browse'.

Features:

- Opens a page of
  - Repository ([e.g.](https://github.com/rhysd/git-brws))
  - File ([e.g.](https://github.com/rhysd/git-brws/blob/master/Cargo.toml))
  - Commit ([e.g.](https://github.com/rhysd/git-brws/commit/60024ab1280f9f10423b22bc708f3f6ef97db6b5))
  - Diff ([e.g.](https://github.com/rhysd/git-brws/compare/e3c18d0d50252112d37bde97061370204b3cdab7..60024ab1280f9f10423b22bc708f3f6ef97db6b5), [e.g.](https://github.com/rhysd/git-brws/compare/e3c18d0d50252112d37bde97061370204b3cdab7...60024ab1280f9f10423b22bc708f3f6ef97db6b5))
  - Tag ([e.g.](https://github.com/rhysd/git-brws/tree/0.10.0))
  - Pull request (only for GitHub and GitHub Enterprise) ([e.g.](https://github.com/rust-lang/rust.vim/pull/290))
  - Issue ([e.g.](https://github.com/rhysd/git-brws/issues/8))
  - Website ([e.g.](https://rhysd.github.io/git-brws/))
    - Homepage of the repository for GitHub or GitHub Enterprise, [GitLab pages][gitlab-pages], [Bitbucket Cloud][bitbucket-cloud]
  - Blame ([e.g.](https://github.com/rhysd/git-brws/blame/9ab093f6720c2c2fe8375408f7f3ab40a3d3337a/src/service.rs))
- Supports following services
  - [GitHub](https://github.com)
  - [Bitbucket](https://bitbucket.org)
  - [GitHub Enterprise](https://enterprise.github.com/home)
  - [GitLab](https://about.gitlab.com/)
  - [Azure DevOps](https://azure.microsoft.com/services/devops/)
- Prefers commit-specific page URL (permlink)
- Available on Linux, macOS and Windows

## Installation

`git brws` is available on Linux, macOS and Windows.

### With [Homebrew](https://brew.sh/)

```
$ brew tap "rhysd/git-brws" "https://github.com/rhysd/git-brws"
$ brew install git-brws
```

It installs `git-brws` to `/usr/local/bin` and `git-brws.1` to `/usr/local/share/man/man1`.
This is recommended way for installation on macOS since updating to the new version is easy.

### On [Arch Linux](https://www.archlinux.org/)

You can install `git-brws` via the [AUR package](https://aur.archlinux.org/packages/git-brws/):

```
git clone https://aur.archlinux.org/git-brws.git
cd git-brws
makepkg -si
```

### With [cargo](https://crates.io/)

```
$ cargo install git-brws
```

### As a single executable binary

Pre-built binary executables are available at [release page][] for macOS (64bit), Linux (64bit, 32bit)
and Windows (64bit, 32bit). Download and unarchive the binary then put the executable in `$PATH`.

Manpage for `man` command is available. Please find `git-brws.1` in the unarchived directory or download
it from [here](https://raw.githubusercontent.com/rhysd/git-brws/master/git-brws.1) and copy it to the
`man` directory in your system (e.g. `/usr/local/share/man/man1/`).

Note: `git brws --help` only works when it is installed.

## Usage

```
Usage: git brws [Options] {Args}

Options:
    -r, --repo REPO     Shorthand format (repo, user/repo, host/user/repo) or
                        Git URL you want to see. When only repo name is
                        specified, most popular repository will be searched
                        from GitHub
    -b, --branch BRANCH Branch name to browse
    -d, --dir PATH      Directory path to the repository. Default value is
                        current working directory
    -R, --remote REMOTE Remote name (e.g. origin). Default value is a remote
                        the current branch is tracking. If current branch
                        tracks no branch, it falls back to 'origin'
    -u, --url           Output URL to stdout instead of opening in browser
    -p, --pr            Open pull request page instead of repository page. If
                        not existing, open 'Create Pull Request' page
    -w, --website       Open website page instead of repository page (homepage
                        URL for GitHub, GitLab pages, Bitbucket Cloud)
    -B, --blame         Open blame page instead of repository page. File path
                        to blame must be passed also.
    -c, --current-branch
                        Open the current branch instead of default branch
    -h, --help          Print this help
    -v, --version       Show version
```

### Open a repository page

- Repository at current directory

```
$ git brws
```

- `develop` branch

```
$ git brws -b develop
```

- `origin` remote of `develop` branch

```
$ git brws -r origin -b develop
```

- @rhysd's 'Shiba' repository

```
$ git brws -r rhysd/Shiba
```

- Most popular `react` repository on GitHub

```
$ git brws -r react
```

It will open https://github.com/facebook/react.

Note: When only repository name is specified for `-r` option, `git-brws` searches GitHub with query
`{repo} in:name` and opens the best-matched repository page.

- Specify directory of repository

```
$ git brws -d ~/.go/src/github.com/rhysd/dot-github
```

### Open a file page

- File of current branch of current remote

```
$ git brws ./some/file.txt
```

- Line of the file

```
$ git brws ./some/file.txt#L123
```

Note: The `L` can be omit.

- Range from line to line of the file

```
$ git brws ./some/file.txt#L123-L126
```

Note: The `L` can be omit.

### Open a commit page

- `HEAD` page of current repository

```
$ git brws HEAD
```

### Open a tag page

- `0.10.0` tag page of current repository

```
$ git brws 0.10.0
```

### Open a diff page between commits

- Diff between `HEAD` and `HEAD~3`

```
$ git brws HEAD~3..HEAD
```

- Diff between `113079b` and `60024ab`

```
$ git brws 60024ab..113079b
```

### Open a diff page from specific commit and its merge base

In addition to `..`, diff with `...` is supported.

- Diff between `branchB` and the merge base commit from `branchB` into `branchA`

```
$ git brws branchA...branchB
```

If you don't know the difference between `..` and `...`, please read `git diff --help`.

Note: Only GitHub and GitHub Enterprise support `...`. For GitLab, only `...` is available.

### Open a pull request page

- Pull request page of current branch of current repository

```
$ git brws --pr
```

- Pull request page of specific branch of specific repository

```
# Specify my forked repository
$ git brws --pr --repo rhysd/rust.vim -b async-contextual-keyword

# Or specify original repository
$ git brws --pr --repo rust-lang/rust.vim -b async-contextual-keyword
```

Note: Currently only GitHub and GitHub Enterprise are supported.

Note: If you have created multiple pull requests at the same repository with the same branch name,
the command may not open a pull request page you want.

Note: When a pull request page for current branch is not existing yet, it opens 'Create Pull Request'
page instead.

### Open a website for the repository

```
# Website for current repository
$ git brws --website
```

With `--repo` option, arbitrary repository's website can be opened.

```
# Opens https://reactjs.org
$ git brws --website --repo react
```

It opens a website for the repository.

- For GitHub, URL for 'homepage' configuration of the repository if it's set. Otherwise
  `https://{user}.github.io/{repo}`
- For GitHub Enterprise, `https://pages.{host}/{user}/{repo}` or `https://{host}/pages/{user}/{repo}`
  depending on your GitHub Enterprise configuration of subdomain isolation
- For GitLab, [GitLab Pages][gitlab-pages]
- For Bitbucket, [Bitbucket Cloud][bitbucket-cloud]

### Open an issue page

- Issue #8

```
$ git brws '#8'
```

Note: `#` is usually used for a line comment in major shells. Please quote the argument

### Open a blame page

- Specific file

```
$ git brws --blame some/file.txt
```

- Specific line at file

```
$ git brws --blame some/file.txt#L5
```

- Specific range at file

```
$ git brws --blame some/file.txt#L5-L9
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

Some environment variables are available to customize behavior of `git-brws`.

| Variable | Description |
|----------|-------------|
| `$GIT_BRWS_GIT_COMMAND` | Git command to use. If not specified, `"git"` will be used. |
| `$GIT_BRWS_GHE_URL_HOST` | When you use your own GitHub Enterprise repository, you can specify its host to this variable. By default, `git brws` detects `^github\.` as GHE host. If your GHE repository host does not match it, please specify this variable. If your repository is `https://example-repo.org/user/repo`, `example-repo.org` should be set. |
| `$GIT_BRWS_GHE_SSH_PORT` | When you set a number to it, the number will be used for the ssh port for GitHub Enterprise URLs. |
| `$GIT_BRWS_GITLAB_URL_HOST` | When you use self-hosted GitLab instance, you can specify its host to this variable. By default, `git brws` detects host matching to `^gitlab\.` as GitLab. If your GitLab URL host does not match it, please specify this variable. If your repository is hosted at `https://your-code.net/user/repo`, `your-code.net` should be set. |
| `$GIT_BRWS_GITLAB_SSH_PORT` | When you set a number to it, the number will be used for the ssh port for self-hosted GitLab URLs. This is useful when your environment hosts GitLab to non-trivial ssh port URL. |
| `$GIT_BRWS_GITHUB_TOKEN` | This variable is used for `--pr` (or `-p`) only. API access token for github.com. They are optional, but useful for avoiding API rate limit and accessing to private repositories. Please generate a token from https://github.com/settings/tokens/new |
| `$GITHUB_TOKEN` | Ditto. When `GIT_BRWS_GITHUB_TOKEN` is not set, `GITHUB_TOKEN` is looked. |
| `$GIT_BRWS_GHE_TOKEN` | This variable is used for `--pr` (or `-p`) only. API access token for GitHub Enterprise instance. It is sometimes mandatory (depending on your GHE instance configuration). Please generate a token from `https://{YOUR GHE HOST}/settings/tokens/new`. |
| `$GIT_BRWS_BROWSE_COMMAND` | Command to open URL. If this value is specified, the command is executed with URL as first argument to browse the URL. |
| `$GIT_BRWS_SHORT_COMMIT_HASH` | Setting `true` will use 7-letters short commit hash like `78fbce6` for URLs. |
| `$https_proxy` | This variable is used for `--pr` (or `-p`) only. An HTTPS Proxy server URL if you use a web proxy. |

## Related Projects

- [hub (`hub browse`)](https://hub.github.com/)
- [git-open](https://github.com/paulirish/git-open)
- [open-browser-github.vim](https://github.com/tyru/open-browser-github.vim)
- [git-browse](https://github.com/albertyw/git-browse)

## TODOs

Please see [the project page](https://github.com/rhysd/git-brws/projects/1).

## Development

Please see [CONTRIBUTING.md](https://github.com/rhysd/git-brws/blob/master/CONTRIBUTING.md).

## License

Distributed under [the MIT license](LICENSE.txt).

[crate-badge]: https://img.shields.io/crates/v/git-brws.svg
[crate]: https://crates.io/crates/git-brws
[travis-ci]: https://travis-ci.org/rhysd/git-brws
[travis-badge]: https://travis-ci.org/rhysd/git-brws.svg?branch=master
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/q9gvpd30k1k5jsf0/branch/master?svg=true
[appveyor]: https://ci.appveyor.com/project/rhysd/git-brws/branch/master
[release page]: https://github.com/rhysd/git-brws/releases
[gitlab-pages]: https://docs.gitlab.com/ee/user/project/pages/getting_started_part_one.html#project-websites
[bitbucket-cloud]: https://confluence.atlassian.com/bitbucket/publishing-a-website-on-bitbucket-cloud-221449776.html
