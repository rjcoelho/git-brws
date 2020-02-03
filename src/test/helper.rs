use crate::config::EnvConfig;
use std::env;

pub fn empty_env() -> EnvConfig {
    EnvConfig {
        git_command: "git".to_string(),
        ghe_ssh_port: None,
        ghe_url_host: None,
        gitlab_ssh_port: None,
        github_token: None,
        ghe_token: None,
        https_proxy: None,
        browse_command: None,
    }
}

pub fn https_proxy() -> Option<String> {
    env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .ok()
}

macro_rules! skip_if_no_token {
    () => {{
        // XXX: On CI, run tests for calling GitHub API only on Linux. This is because git-brws uses
        // 'GET /search/*' APIs but they have special rate limit 30/min. Running jobs parallelly on
        // CI hits the rate limit and tests fails. Even if running the jobs sequentially, it
        // sometimes hits the limit.
        match (
            ::std::env::var("GITHUB_ACTIONS"),
            ::std::env::var("RUNNER_OS"),
        ) {
            (Ok(ref v), Ok(ref os)) if v == "true" && os != "Linux" => return,
            _ => {}
        }
        match ::std::env::var("GIT_BRWS_GITHUB_TOKEN").or_else(|_| ::std::env::var("GITHUB_TOKEN"))
        {
            Ok(ref v) if v == "" => return,
            Ok(v) => Some(v),
            Err(_) => return,
        }
    }};
}
