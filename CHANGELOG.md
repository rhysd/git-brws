<a name="0.11.2"></a>
# [0.11.2](https://github.com/rhysd/git-brws/releases/tag/0.11.2) - 19 May 2019

- **Fix:** URL is wrong when opening 'New Pull Request' page of parent repository (#13)
- **Improve:** Reduce binary size (about 30% smaller) by tweaking build configuration

[Changes][0.11.2]


<a name="0.11.1"></a>
# [0.11.1](https://github.com/rhysd/git-brws/releases/tag/0.11.1) - 30 Apr 2019

**Fix:** Cause an error when attempting to blame directory path

[Changes][0.11.1]


<a name="0.11.0"></a>
# [0.11.0](https://github.com/rhysd/git-brws/releases/tag/0.11.0) - 29 Apr 2019

- **Change:** Open 'Create Pull Request' page when pull request is not created yet for the branch. Previous behavior caused an error in the case.
- **New:** Support 'Blame' page for file path with `--blame` or `-B` option. Not only file path, with line such as `/path#L123` and with range such as `/path#L1-2` are supported. This feature is available for all services; GitHub, GitHub:Enterprise, GitLab and bitbucket. #11
- **New:** `$GIT_BRWS_BROWSE_COMMAND` environment variable was introduced to customize how to open the URL. The command specified with the environment variable is invoked with URL as the first argument for opening the URL.
- **Fix:** When opening file path, fallback into branch name instead of commit hash if the latest commit is not contained in remote branch. Since the commit page where the commit is not contained in remote is 404. #12

[Changes][0.11.0]


<a name="0.10.1"></a>
# [0.10.1](https://github.com/rhysd/git-brws/releases/tag/0.10.1) - 12 Feb 2019

- **New:** Argument now accepts [project's tag page](https://github.com/rhysd/git-brws/tree/0.10.0). GitHub, GitHub Enterprise, GitLab and Bitbucket are supported
- **Fix:** `$https_proxy` and `$GITHUB_TOKEN` were not referred
- Improve error messages when argument cannot be handled and Git command cannot run

[Changes][0.10.1]


<a name="0.10.0"></a>
# [0.10.0](https://github.com/rhysd/git-brws/releases/tag/0.10.0) - 11 Feb 2019

- **New:** `--website` option which opens the repository's website
  - Homepage of the repository for GitHub or GitHub Enterprise, [GitLab pages](https://docs.gitlab.com/ee/user/project/pages/getting_started_part_one.html#project-websites), [Bitbucket Cloud](https://confluence.atlassian.com/bitbucket/publishing-a-website-on-bitbucket-cloud-221449776.html)
- **New:** `--repo` now accepts only repository name. It opens the most popular repository searched with the repository name.
  - For example, `--repo react` opens https://github.com/facebook/react
- **Breaking:** `--repo` no longer accepts remote name. Instead, `--remote` (or `-R`) is available. This is for avoiding confusing behavior that repo name is intended but remote name is accidentally matched.
- **Improve:** Error handlings. Now invalid combination of options and arguments are reported as an error
- Many refactorings to clean up internal structure

This is a demo that `-w -r react` can open https://reactjs.org:

![tmp](https://user-images.githubusercontent.com/823277/52643570-abd78d00-2f20-11e9-870b-489580bb756a.gif)


[Changes][0.10.0]


<a name="0.9.2"></a>
# [0.9.2](https://github.com/rhysd/git-brws/releases/tag/0.9.2) - 09 Feb 2019

- Allow to open page outside Git repository directory with `-r` (#9)
- Improve error messages
- Use Rust 2018 for compiling
- Improve `-h` help document and `man` document with more examples
- Some internal refactorings

[Changes][0.9.2]


<a name="0.9.1"></a>
# [0.9.1](https://github.com/rhysd/git-brws/releases/tag/0.9.1) - 04 Jan 2019

Tiny updates for documents:

- Add more examples in manpage
- Fix links in README
- Add categories metadata to crate package

[Changes][0.9.1]


<a name="0.9.0"></a>
# [0.9.0](https://github.com/rhysd/git-brws/releases/tag/0.9.0) - 31 Dec 2018

A happy new year!!

- Add issue support by `#{number}` argument. For example, `-r rhysd/git-brws '#8'` opens [this issue](https://github.com/rhysd/git-brws/issues/8). This is useful when you see issue number in source file and want to open it directly.
- Fix path separator is wrong on Windows when specifying file path in repository
- Add [manpage `git-brws.1`](https://github.com/rhysd/git-brws/blob/master/git-brws.1) for supporting `git brws --help` and `man git-brws`
- Add [Homebrew formula](https://github.com/rhysd/git-brws/blob/master/HomebrewFormula/git-brws.rb). This is recommended way for installation on macOS since it automatically setup `git-brws.1` and is easy to update.
- Improve check when a directory is specified with line number
- Refactor remaining old error handlings

[Changes][0.9.0]


<a name="0.8.1"></a>
# [0.8.1](https://github.com/rhysd/git-brws/releases/tag/0.8.1) - 28 Dec 2018

- Correct canonicalizing a path (Fix #7)
- Many refactorings

[Changes][0.8.1]


<a name="0.8.0"></a>
# [0.8.0](https://github.com/rhysd/git-brws/releases/tag/0.8.0) - 26 Dec 2018

- **New:** Add Windows support (both 64bit and 32bit). Please find release binaries from release page
- **New:** Support `A...B` to see page for diff between `B` and merge base commit for branch `A`
- **Fix:** `A..B` did not open correct page. Now it opens `..` URL. Note that `..` page is only supported by GitHub or GitHub Enterprise
- **Fix:** File path argument such as `../foo.txt`, `foo/..`, `..` were wrongly treated as diff
- **New:** CI on Windows using Appveyor

[Changes][0.8.0]


<a name="0.7.1"></a>
# [0.7.1](https://github.com/rhysd/git-brws/releases/tag/0.7.1) - 24 Dec 2018

Fix x86 build was broken

[Changes][0.7.1]


<a name="0.7.0"></a>
# [0.7.0](https://github.com/rhysd/git-brws/releases/tag/0.7.0) - 22 Dec 2018

![screenshot](https://user-images.githubusercontent.com/823277/50382987-8aabaa80-06ee-11e9-8b94-11a6a9cb1bb8.gif)


[Changes][0.7.0]


<a name="0.6.3"></a>
# [0.6.3](https://github.com/rhysd/git-brws/releases/tag/0.6.3) - 20 Dec 2018



[Changes][0.6.3]


<a name="0.6.2"></a>
# [0.6.2](https://github.com/rhysd/git-brws/releases/tag/0.6.2) - 18 Dec 2018



[Changes][0.6.2]


<a name="v0.6.1"></a>
# [v0.6.1](https://github.com/rhysd/git-brws/releases/tag/v0.6.1) - 31 Jan 2018

- Update all dependencies
- Use rustc v1.23.0 to build

[Changes][v0.6.1]


<a name="v0.5.0"></a>
# [v0.5.0](https://github.com/rhysd/git-brws/releases/tag/v0.5.0) - 01 Jan 2017



[Changes][v0.5.0]


<a name="v0.4.3"></a>
# [v0.4.3](https://github.com/rhysd/git-brws/releases/tag/v0.4.3) - 01 Jan 2017

- Add aarch64 to release targets.


[Changes][v0.4.3]


<a name="v0.4.2"></a>
# [version 0.4.2 (v0.4.2)](https://github.com/rhysd/git-brws/releases/tag/v0.4.2) - 31 Dec 2016

First binaries release for Linux (32bit/64bit) and macOS (64bit) using Travis CI.
- Fix converting `git@` protocol when user specifies it directly via `-r` option
- Add default behavior when `HEAD` is not attached and remote tracking branch not found. Fall back into `origin`.


[Changes][v0.4.2]


[0.11.2]: https://github.com/rhysd/git-brws/compare/0.11.1...0.11.2
[0.11.1]: https://github.com/rhysd/git-brws/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/rhysd/git-brws/compare/0.10.1...0.11.0
[0.10.1]: https://github.com/rhysd/git-brws/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/rhysd/git-brws/compare/0.9.2...0.10.0
[0.9.2]: https://github.com/rhysd/git-brws/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/rhysd/git-brws/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/rhysd/git-brws/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/rhysd/git-brws/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/rhysd/git-brws/compare/0.7.1...0.8.0
[0.7.1]: https://github.com/rhysd/git-brws/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/rhysd/git-brws/compare/0.6.3...0.7.0
[0.6.3]: https://github.com/rhysd/git-brws/compare/0.6.2...0.6.3
[0.6.2]: https://github.com/rhysd/git-brws/compare/v0.6.1...0.6.2
[v0.6.1]: https://github.com/rhysd/git-brws/compare/v0.5.0...v0.6.1
[v0.5.0]: https://github.com/rhysd/git-brws/compare/v0.4.3...v0.5.0
[v0.4.3]: https://github.com/rhysd/git-brws/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/rhysd/git-brws/tree/v0.4.2

 <!-- Generated by changelog-from-release -->
