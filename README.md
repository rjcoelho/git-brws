git brws
========
[![Crate Badge][]][GitHub Project]
[![Build Status][]][CI Results]

`git brws` is a command line tool to open a repository, file, commit, diff or pull request in your web
browser from command line. 'brws' is an abbreviation of 'browse'.

Features:

- Opens a page of
  - Repository ([e.g.](https://github.com/rhysd/git-brws))
  - File ([e.g.](https://github.com/rhysd/git-brws/blob/master/Cargo.toml))
  - Commit ([e.g.](https://github.com/rhysd/git-brws/commit/60024ab1280f9f10423b22bc708f3f6ef97db6b5))
  - Diff (`..`: [e.g.](https://github.com/rhysd/git-brws/compare/e3c18d0d50252112d37bde97061370204b3cdab7..60024ab1280f9f10423b22bc708f3f6ef97db6b5), `...`: [e.g.](https://github.com/rhysd/git-brws/compare/e3c18d0d50252112d37bde97061370204b3cdab7...60024ab1280f9f10423b22bc708f3f6ef97db6b5))
  - Pull request (Only GitHub and GitHub Enterprise) ([e.g.](https://github.com/rust-lang/rust.vim/pull/290))
- Supports below services
  - [GitHub](https://github.com)
  - [Bitbucket](https://bitbucket.org)
  - [GitHub Enterprise](https://enterprise.github.com/home)
  - [GitLab](https://about.gitlab.com/)
- Prefers commit-specific page URL
- Available on Linux, macOS and Windows

## Installation

`git brws` currently supports Linux, macOS and Windows.

- With [cargo](https://crates.io/)

```
$ cargo install git-brws
```

- As a single binary

You can download a binary executable from [release page][] for macOS, Linux (x86\_64, i686) and Windows
(64bit, 32bit). Unarchive downloaded file and put the binary in a directory in `$PATH`.

## Usage

```
Usage: git brws [Options] {Args}

Options:
    -r, --repo REPO     Shorthand format (user/repo, service/user/repo) or
                        remote name (e.g. origin) or Git URL you want to see
    -b, --branch BRANCH Branch name to browse
    -d, --dir PATH      Directory path to the repository
    -u, --url           Output URL to stdout instead of opening in browser
    -p, --pr            Open pull request page instead of repository page
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

### Show a diff page between commits

- Show diff between `HEAD` and `HEAD~3`

```
$ git brws HEAD~3..HEAD
```

- Show diff between `113079b` and `60024ab`

```
$ git brws 60024ab..113079b
```

Note: Only GitHub and GitHub Enterprise support `...`. For GitLab, only `...` is available.

### Show a diff page from specific commit and its merge base

Not only `..`, `...` is supported.

- Show diff between `branchB` and the merge base commit from `branchB` into `branchA`

```
$ git brws branchA...branchB
```

If you don't know the difference between `..` and `...`, please read `git diff --help`.

### Open a pull request page

- Show pull request page of current branch of current repository

```
$ git brws --pr
```

- Show pull request page of specific branch of specific repository

```
# Specify my forked repository
$ git brws --pr --repo rhysd/rust.vim -b async-contextual-keyword

# Or specify original repository
$ git brws --pr --repo rust-lang/rust.vim -b async-contextual-keyword
```

Note: Currently only GitHub and GitHub Enterprise are supported.

Note: If you have created multiple pull requests at the same repository with the same branch name,
the command may not open a pull request page you want.

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

You can customize behavior of this command with environment variables.

| Variable | Description |
|----------|-------------|
| `$GIT_BRWS_GIT_COMMAND` | Git command to use. If not specified, `"git"` will be used. |
| `$GIT_BRWS_GHE_URL_HOST` | When you use your own GitHub Enterprise repository, you can specify its host to this variable. By default, `git brws` detects `^github\.` as GHE host. If your GHE repository host does not match it, please specify this variable. If your repository is `https://example-repo.org/user/repo`, `example-repo.org` should be set. |
| `$GIT_BRWS_GHE_SSH_PORT` | When you set a number to it, the number will be used for the ssh port for GitHub Enterprise URLs. |
| `$GIT_BRWS_GITLAB_SSH_PORT` | When you set a number to it, the number will be used for the ssh port for self-hosted GitLab URLs. This is useful when your environment hosts GitLab to non-trivial ssh port URL. |
| `$GIT_BRWS_GITHUB_TOKEN` | This variable is used for `--pr` (or `-p`) only. API access token for github.com. They are optional, but useful for avoiding API rate limit and accessing to private repositories. Please generate a token from https://github.com/settings/tokens/new |
| `$GITHUB_TOKEN` | Ditto. When `GIT_BRWS_GITHUB_TOKEN` is not set, `GITHUB_TOKEN` is looked. |
| `$GIT_BRWS_GHE_TOKEN` | This variable is used for `--pr` (or `-p`) only. API access token for GitHub Enterprise instance. It is sometimes mandatory (depending on your GHE instance configuration). Please generate a token from `https://{YOUR GHE HOST}/settings/tokens/new`. |
| `$https_proxy` | This variable is used for `--pr` (or `-p`) only. A HTTPS Proxy server URL if you use a web proxy. |

## Related Projects

- [hub browse](https://hub.github.com/)
- [git open](https://github.com/paulirish/git-open)
- [open-browser-github.vim](https://github.com/tyru/open-browser-github.vim)
- [git browse](https://github.com/albertyw/git-browse)

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
