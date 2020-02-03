use crate::argv::*;
use crate::error::Error;
use std::env;
use std::fs;

// TODO: This test only consider the repository name 'rhysd/git-brws.git'

#[test]
fn args_with_no_option() {
    match Parsed::from_iter(&["git-brws"]).unwrap() {
        Parsed::OpenPage(c) => {
            assert!(
                &[
                    "https://github.com/rhysd/git-brws.git",
                    "ssh://git@github.com:22/rhysd/git-brws.git",
                    "git@github.com:rhysd/git-brws.git",
                    // On GitHub, omitting '.git' is ok for Git URLs
                    "https://github.com/rhysd/git-brws",
                ]
                .contains(&c.repo_url.as_str()),
                "{:?}",
                c.repo_url,
            );
            assert_eq!(c.branch, None);
            match c.git_dir {
                Some(ref d) => assert!(d.ends_with(".git"), "{:?}", d),
                None => assert!(false, ".git was not found"),
            }
            assert!(c.args.is_empty());
            assert!(!c.stdout);
            assert!(!c.website);
            assert!(!c.blame);
        }
        r => assert!(false, "Failed to parse args with no option: {:?}", r),
    };

    match Parsed::from_iter(&["git-brws", "foo", "bar"]).unwrap() {
        Parsed::OpenPage(c) => {
            assert_eq!(c.args.len(), 2);
        }
        p => assert!(false, "{:?}", p),
    };
}

#[test]
fn multiple_options() {
    match Parsed::from_iter(&[
        "git-brws", "-u", "-r", "foo/bar", "--dir", ".", "-b", "dev", "-w", "--blame",
    ])
    .unwrap()
    {
        Parsed::OpenPage(c) => {
            assert_eq!(c.repo_url, "https://github.com/foo/bar.git");
            assert_eq!(c.branch, Some("dev".to_string()));
            match c.git_dir {
                Some(ref d) => assert!(d.ends_with(".git"), "{:?}", d),
                None => assert!(false, ".git was not found"),
            }
            assert_eq!(c.args.len(), 0);
            assert!(c.stdout);
            assert!(c.website);
            assert!(c.blame);
        }
        p => assert!(false, "{:?}", p),
    };
}

#[test]
fn fix_ssh_repo_url() {
    for (url, expected) in &[
        // GitHub SSH protocols with SCP form.
        // .git file extension and ssh:// and port number can be omitted
        (
            "git@github.com:user/repo.git",
            "ssh://git@github.com:22/user/repo.git",
        ),
        (
            "ssh://git@github.com:user/repo.git",
            "ssh://git@github.com:22/user/repo.git",
        ),
        (
            "ssh://git@github.com:22/user/repo.git",
            "ssh://git@github.com:22/user/repo.git",
        ),
        (
            "ssh://git@github.com:user/repo",
            "ssh://git@github.com:22/user/repo.git",
        ),
        // Azure DevOps URLs
        (
            "ssh://team@vs-ssh.visualstudio.com:v3/team/repo/repo",
            "ssh://team@vs-ssh.visualstudio.com:22/v3/team/repo/repo.git",
        ),
        (
            "ssh://git@ssh.dev.azure.com:v3/team/repo/repo",
            "ssh://git@ssh.dev.azure.com:22/v3/team/repo/repo.git",
        ),
        // Port number is not omitted
        (
            "git@github.somewhere.com:123/user/repo.git",
            "ssh://git@github.somewhere.com:123/user/repo.git",
        ),
    ] {
        match Parsed::from_iter(&["git-brws", "-r", url]).unwrap() {
            Parsed::OpenPage(c) => {
                assert_eq!(c.repo_url, *expected);
            }
            p => assert!(
                false,
                "Parse must be succeeded but actually results in {:?}",
                p
            ),
        }
    }
}

#[test]
fn repo_formatting() {
    let p = |r| Parsed::from_iter(&["git-brws", "-r", r]).unwrap();
    match p("bitbucket.org/foo/bar") {
        Parsed::OpenPage(c) => assert_eq!(c.repo_url, "https://bitbucket.org/foo/bar.git"),
        p => assert!(false, "{:?}", p),
    }
    match p("https://gitlab.com/foo/bar") {
        Parsed::OpenPage(c) => assert_eq!(c.repo_url, "https://gitlab.com/foo/bar.git"),
        p => assert!(false, "{:?}", p),
    }
    match p("foo/bar") {
        Parsed::OpenPage(c) => assert_eq!(c.repo_url, "https://github.com/foo/bar.git"),
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn valid_remote_name() {
    match Parsed::from_iter(&["git-brws", "-R", "origin"]).unwrap() {
        Parsed::OpenPage(c) => assert!(
            [
                "https://github.com/rhysd/git-brws.git",
                // On GitHub, omitting '.git' is ok for Git URLs
                "https://github.com/rhysd/git-brws",
                "ssh://git@github.com:22/rhysd/git-brws.git"
            ]
            .contains(&c.repo_url.as_str()),
            "Unexpected remote URL for 'origin' remote: {}. For pull request, please ignore this test is failing",
            c.repo_url,
        ),
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn invalid_remote_name() {
    match Parsed::from_iter(&["git-brws", "-R", "this-remote-is-never-existing"]).unwrap_err() {
        Error::GitObjectNotFound { kind, object, .. } => {
            assert_eq!(kind, "remote");
            assert_eq!(&object, "this-remote-is-never-existing");
        }
        e => assert!(false, "Unexpected error: {}", e),
    }
}

#[test]
fn help_option() {
    match Parsed::from_iter(&["git-brws", "-h"]).unwrap() {
        Parsed::Help(s) => {
            assert!(s.starts_with("Usage:"));
        }
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn version_option() {
    match Parsed::from_iter(&["git-brws", "-v"]).unwrap() {
        Parsed::Version(s) => {
            assert!(!s.is_empty());
        }
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn unknown_options() {
    assert!(Parsed::from_iter(&["git-brws", "--unknown"]).is_err());
}

#[test]
fn detect_git_dir() {
    let current = fs::canonicalize(env::current_dir().unwrap()).unwrap();
    let p = current.join("src").join("test");
    match Parsed::from_iter(&["git-brws", "-d", p.to_str().unwrap()]).unwrap() {
        Parsed::OpenPage(c) => {
            let expected = Some(current.join(".git"));
            assert_eq!(c.git_dir, expected);
        }
        p => assert!(false, "{:?}", p),
    }
}

// For checking #9
#[test]
fn no_git_dir() {
    let mut root = fs::canonicalize(env::current_dir().unwrap())
        .unwrap()
        .clone();
    loop {
        let prev = root.clone();
        root.pop();
        if prev == root {
            break;
        }
    }
    let root = root;

    let git_dir = root.join(".git");
    assert!(
        !git_dir.exists(),
        "{:?} should not exist as precondition of this test case",
        git_dir
    );

    match Parsed::from_iter(&["git-brws", "-d", root.to_str().unwrap(), "-r", "foo/bar"]).unwrap() {
        Parsed::OpenPage(c) => {
            assert_eq!(c.git_dir, None);
            assert_eq!(&c.repo_url, "https://github.com/foo/bar.git");
        }
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn search_repo_from_github_by_name() {
    skip_if_no_token!();

    // Add user:rhysd to ensure to get expected result. But actually repository name is usually
    // passed like `-r react` as use case.
    let parsed = Parsed::from_iter(&["git-brws", "-r", "user:rhysd vim.wasm"]).unwrap();
    match parsed {
        Parsed::OpenPage(c) => {
            assert_eq!(&c.repo_url, "https://github.com/rhysd/vim.wasm.git");
        }
        p => assert!(false, "{:?}", p),
    }
}

#[test]
fn repo_specified_but_argument_is_not_empty() {
    let err = Parsed::from_iter(&["git-brws", "-r", "foo", "HEAD"]).unwrap_err();
    match err {
        Error::ArgsNotAllowed { ref args, .. } => {
            assert!(format!("{}", err).contains("\"HEAD\""), "{:?}", args);
        }
        e => assert!(false, "Unexpected error: {}", e),
    }
}
