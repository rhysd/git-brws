# Development

Repository for `git-brws` is [hosted at GitHub][github-repo].


## Run tests

To watch file changes and run linter/tests automatically:

```sh
cargo install cargo-watch
cargo watch -x clippy -x test
```

Some tests require GitHub API access token. To run full tests:

```sh
export GITHUB_TOKEN=xxxxxxxxxxx
cargo test
```

Note: Without `GITHUB_TOKEN` environment variable, `cargo test` runs test cases partially though it
says every test case was run. This is because `cargo test` does not provide a way to skip test cases
dynamically in source.

`cargo test` and `cargo clippy` are automatically run on pushing to remote by [cargo-husky][].
But some tests fail when the remote tracking branch does not exist. When you create a new branch,
please use `--no-verify`. Please do not use `--no-verify` otherwise.

```sh
git checkout -b new-branch
git push -u origin new-branch --no-verify
```


## Prepare manpage

To update manpage file `git-brws.1` and `docs/index.html`, please edit `git-brws.1.ronn` and generate
`git-brws.1` automatically with `docs/gen.sh` script. [ronn][] is necessary as a dependency.

```sh
gem install ronn
./docs/gen.sh
```


## Update Homebrew formula

To update [Homebrew][homebrew] formula file, please use `HomebrewFormula/update.sh`.

For example, when updating to 0.9.2:

```sh
./HomebrewFormula/update.sh 0.9.2
```


## Update changelog

To update changelog, please run [changelog-from-release][] after describing release note at GitHub
repository. It regenerates `CHANGELOG.md` from releases on GitHub.

```sh
go get github.com/rhysd/changelog-from-release
changelog-from-release
```


## Release process

1. Run `export GITHUB_TOKEN=...`
2. Run `cargo release` which will create a new tag and push it to remote
3. CI services will prepare binaries
4. Write release note at https://github.com/rhysd/git-brws/releases
5. Update changelog
6. Update Homebrew formula

[github-repo]: https://github.com/rhysd/git-brws
[cargo-husky]: https://github.com/rhysd/cargo-husky
[ronn]: https://github.com/rtomayko/ronn
[changelog-from-release]: https://github.com/rhysd/changelog-from-release
[homebrew]: https://brew.sh/
