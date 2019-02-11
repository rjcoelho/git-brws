extern crate url;

use crate::config::{Config, EnvConfig};
use crate::error::{Error, Result};
use crate::github_api;

fn find_github_pr_url_for_branch<B: AsRef<str>>(
    branch: B,
    endpoint: &str,
    author: &str,
    repo: &str,
    env: &EnvConfig,
) -> Result<String> {
    let branch = branch.as_ref();
    let token = if endpoint == "api.github.com" {
        &env.github_token
    } else {
        if env.ghe_token.is_none() {
            return Err(Error::GheTokenRequired);
        }
        &env.ghe_token
    };
    let client = github_api::Client::build(endpoint, token.as_ref(), &env.https_proxy)?;

    // Note: Search pull request URL in the case where the repository is an original, not a fork.
    // Author should not be set since original repository's owner may be different from current
    // user (e.g. organization name). And multiple branches which has the same name cannot exist
    // in one repository.
    if let Some(url) = client.find_pr_url(branch, author, repo, None)? {
        return Ok(url);
    }

    if let Some((owner, repo)) = client.parent_repo(author, repo)? {
        // Note: Search pull request URL in the case where the repository was forked from original.
        // Author should be set since other person may create another pull request with the same branch name.
        if let Some(url) =
            client.find_pr_url(branch, owner.as_str(), repo.as_str(), Some(author))?
        {
            return Ok(url);
        }
    }

    Err(Error::GitHubPullReqNotFound {
        author: author.to_string(),
        repo: repo.to_string(),
        branch: branch.to_string(),
    })
}

pub fn find_url(endpoint: &str, author: &str, repo: &str, cfg: &Config) -> Result<String> {
    match cfg.branch {
        Some(ref b) => find_github_pr_url_for_branch(b, endpoint, author, repo, &cfg.env),
        None => {
            if let Some(git) = cfg.git() {
                find_github_pr_url_for_branch(
                    git.current_branch()?,
                    endpoint,
                    author,
                    repo,
                    &cfg.env,
                )
            } else {
                Err(Error::NoLocalRepoFound {
                    operation: "opening a pull request without specifying branch".to_string(),
                })
            }
        }
    }
}
