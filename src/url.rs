extern crate open;

use crate::config::{Config, EnvConfig};
use crate::error::{Error, Result};
use crate::page::parse_page;
use crate::service;
use std::process::Command;

pub fn build_url(cfg: &Config) -> Result<String> {
    let page = parse_page(&cfg)?;
    service::build_page_url(&page, &cfg)
}

pub fn browse(url: &str, env: &EnvConfig) -> Result<()> {
    if let Some(ref cmd) = env.browse_command {
        let out = Command::new(cmd).arg(url).output()?;
        if !out.status.success() {
            return Err(Error::UserBrowseCommandFailed {
                cmd: cmd.to_string(),
                url: url.to_string(),
                msg: String::from_utf8_lossy(&out.stderr)
                    .trim()
                    .replace('\n', " "),
            });
        }
        return Ok(());
    }

    match open::that(url) {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            let url = url.to_string();
            let msg = if let Some(code) = status.code() {
                format!("Command exited with non-zero status {}", code)
            } else {
                "Error on opening URL {}: Command terminated by signal".to_string()
            };
            Err(Error::OpenUrlFailure { url, msg })
        }
        Err(e) => Err(Error::OpenUrlFailure {
            url: url.to_string(),
            msg: format!("{}", e),
        }),
    }
}
