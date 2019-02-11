use crate::error::Error;
use crate::github_api::Client;
use crate::test::helper::https_proxy;

#[test]
fn find_pr_url() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url("async-contextual-keyword", "rust-lang", "rust.vim", None)
        .unwrap();
    assert_eq!(
        url,
        Some("https://github.com/rust-lang/rust.vim/pull/290".to_string()),
    );
    let url = client
        .find_pr_url(
            "async-contextual-keyword",
            "rust-lang",
            "rust.vim",
            Some("rhysd"),
        )
        .unwrap();
    assert_eq!(
        url,
        Some("https://github.com/rust-lang/rust.vim/pull/290".to_string()),
    );
}

#[test]
fn no_pr_found() {
    let token = skip_if_no_token!();
    let client = Client::build("api.github.com", token, &https_proxy()).unwrap();
    let url = client
        .find_pr_url(
            "branch-name-which-does-not-exist",
            "rust-lang",
            "rust.vim",
            Some("rhysd"),
        )
        .unwrap();
    assert_eq!(url, None);
}

#[test]
fn find_parent() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let parent = client.parent_repo("rhysd", "rust.vim").unwrap();
    assert_eq!(
        parent,
        Some(("rust-lang".to_string(), "rust.vim".to_string())),
    );
}

#[test]
fn parent_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let parent = client.parent_repo("rhysd", "git-brws").unwrap();
    assert_eq!(parent, None);
}

#[test]
fn request_failure() {
    let client =
        Client::build("unknown.endpoint.example.com", None::<&str>, &None::<&str>).unwrap();
    match client.parent_repo("rhysd", "git-brws") {
        Ok(_) => assert!(false, "request succeeded"),
        Err(Error::HttpClientError(..)) => { /* ok */ }
        Err(e) => assert!(false, "unexpected error: {}", e),
    }
}

#[test]
fn most_popular_repo_ok() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let repo = client
        .most_popular_repo_by_name("user:rhysd vim.wasm")
        .unwrap();
    assert_eq!(&repo.clone_url, "https://github.com/rhysd/vim.wasm.git");
}

#[test]
fn most_popular_repo_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let err = client
        .most_popular_repo_by_name("user:rhysd this-repository-will-never-exist")
        .unwrap_err();
    match err {
        Error::NoSearchResult { .. } => { /* ok */ }
        err => assert!(false, "Unexpected error: {}", err),
    }
}

#[test]
fn homepage() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let url = client.repo_homepage("rhysd", "git-brws").unwrap();
    match url {
        Some(url) => assert_eq!(&url, "https://rhysd.github.io/git-brws/"),
        url => assert!(false, "Unexpected url: {:?}", url),
    }
}

#[test]
fn homepage_not_found() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    let url = client.repo_homepage("rhysd", "filter-with-state").unwrap();
    match url {
        None => { /* OK */ }
        url => assert!(false, "Unexpected url: {:?}", url),
    }
}

#[test]
fn homepage_error_response() {
    let client = Client::build("api.github.com", skip_if_no_token!(), &https_proxy()).unwrap();
    client
        .repo_homepage("rhysd", "this-repository-will-never-exist")
        .unwrap_err();
}
