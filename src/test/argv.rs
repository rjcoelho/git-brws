use crate::argv::*;
use std::env;
use std::fs;

#[test]
fn no_option() {
    match Parsed::from_iter(&["git-brws"]).unwrap() {
        Parsed::OpenPage(o) => {
            assert!(vec![
                "https://github.com/rhysd/git-brws.git",
                "ssh://git@github.com:22/rhysd/git-brws.git",
                "git@github.com:rhysd/git-brws.git",
            ]
            .iter()
            .any(|u| &o.repo == u));
            assert_eq!(o.branch, None);
            match o.git_dir {
                Some(ref d) => assert!(d.ends_with(".git"), "{:?}", d),
                None => assert!(false, ".git was not found"),
            }
            assert!(o.args.is_empty());
            assert!(!o.stdout);
        }
        r => assert!(false, "Failed to parse args with no option: {:?}", r),
    };

    match Parsed::from_iter(&["git-brws", "foo", "bar"]).unwrap() {
        Parsed::OpenPage(o) => {
            assert_eq!(o.args.len(), 2);
        }
        _ => assert!(false),
    };
}

#[test]
fn with_options() {
    match Parsed::from_iter(&[
        "git-brws", "foo", "-u", "-r", "foo/bar", "--dir", ".", "bar", "-b", "dev",
    ])
    .unwrap()
    {
        Parsed::OpenPage(o) => {
            assert_eq!(o.repo, "https://github.com/foo/bar.git");
            assert_eq!(o.branch, Some("dev".to_string()));
            match o.git_dir {
                Some(ref d) => assert!(d.ends_with(".git"), "{:?}", d),
                None => assert!(false, ".git was not found"),
            }
            assert_eq!(o.args.len(), 2);
            assert!(o.stdout);
        }
        _ => assert!(false),
    };
}

#[test]
fn ssh_conversion_with_option() {
    match Parsed::from_iter(&["git-brws", "-r", "git@github.com:user/repo.git"]).unwrap() {
        Parsed::OpenPage(o) => {
            assert_eq!(o.repo, "ssh://git@github.com:22/user/repo.git");
        }
        p => assert!(
            false,
            "Parse must be succeeded but actually results in {:?}",
            p
        ),
    };
}

#[test]
fn repo_formatting() {
    let p = |r| Parsed::from_iter(&["git-brws", "-r", r]).unwrap();
    match p("bitbucket.org/foo/bar") {
        Parsed::OpenPage(o) => assert_eq!(o.repo, "https://bitbucket.org/foo/bar.git"),
        _ => assert!(false),
    }
    match p("https://gitlab.com/foo/bar") {
        Parsed::OpenPage(o) => assert_eq!(o.repo, "https://gitlab.com/foo/bar.git"),
        _ => assert!(false),
    }
}

#[test]
fn help_option() {
    match Parsed::from_iter(&["git-brws", "-h"]).unwrap() {
        Parsed::Help(s) => {
            assert!(s.starts_with("Usage:"));
        }
        _ => assert!(false),
    }
}

#[test]
fn version_option() {
    match Parsed::from_iter(&["git-brws", "-v"]).unwrap() {
        Parsed::Version(s) => {
            assert!(!s.is_empty());
        }
        _ => assert!(false),
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
        Parsed::OpenPage(o) => {
            let expected = Some(current.join(".git"));
            assert_eq!(o.git_dir, expected);
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
        Parsed::OpenPage(o) => {
            assert_eq!(o.git_dir, None);
            assert_eq!(&o.repo, "https://github.com/foo/bar.git");
        }
        p => assert!(false, "{:?}", p),
    }
}
