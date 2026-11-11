//! <https://github.com/webpack/enhanced-resolve/blob/main/test/restrictions.test.js>

use std::sync::Arc;

use regress::Regex;

use crate::{ResolveError, ResolveOptions, Resolver, Restriction};

#[test]
fn should_respect_regexp_restriction() {
    let f = super::fixture().join("restrictions");

    let re = Regex::new(r"\.(sass|scss|css)$").unwrap();
    let resolver1 = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            path.as_os_str().to_str().is_some_and(|s| re.find(s).is_some())
        }))],
        ..ResolveOptions::default()
    });

    let resolution = resolver1.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Err(ResolveError::NotFound("pck1".to_string())));
}

#[test]
fn should_try_to_find_alternative_1() {
    let f = super::fixture().join("restrictions");

    let re = Regex::new(r"\.(sass|scss|css)$").unwrap();
    let resolver1 = Resolver::new(ResolveOptions {
        extensions: vec![".js".into(), ".css".into()],
        main_files: vec!["index".into()],
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            path.as_os_str().to_str().is_some_and(|s| re.find(s).is_some())
        }))],
        ..ResolveOptions::default()
    });

    let resolution = resolver1.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck1/index.css")));
}

#[test]
fn should_respect_string_restriction() {
    let fixture = super::fixture();
    let f = fixture.join("restrictions");

    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        restrictions: vec![Restriction::Path(f.clone())],
        ..ResolveOptions::default()
    });

    let resolution = resolver.resolve(&f, "pck2");
    assert_eq!(resolution, Err(ResolveError::NotFound("pck2".to_string())));
}

#[test]
fn should_try_to_find_alternative_2() {
    let f = super::fixture().join("restrictions");

    let re = Regex::new(r"\.(sass|scss|css)$").unwrap();
    let resolver1 = Resolver::new(ResolveOptions {
        extensions: vec![".js".into(), ".css".into()],
        main_fields: vec!["main".into(), "style".into()],
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            path.as_os_str().to_str().is_some_and(|s| re.find(s).is_some())
        }))],
        ..ResolveOptions::default()
    });

    let resolution = resolver1.resolve(&f, "pck2").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck2/index.css")));
}

#[test]
fn should_try_to_find_alternative_3() {
    let f = super::fixture().join("restrictions");

    let re = Regex::new(r"\.(sass|scss|css)$").unwrap();
    let resolver1 = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        main_fields: vec!["main".into(), "module".into(), "style".into()],
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            path.as_os_str().to_str().is_some_and(|s| re.find(s).is_some())
        }))],
        ..ResolveOptions::default()
    });

    let resolution = resolver1.resolve(&f, "pck2").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck2/index.css")));
}

// Test coverage for check_restrictions at line 783 in load_index()
#[test]
fn should_check_restrictions_in_load_index_with_enforce_extension_disabled() {
    let f = super::fixture().join("restrictions");

    let re = Regex::new(r"\.(css)$").unwrap();
    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into(), ".css".into()],
        main_files: vec!["index".into()],
        enforce_extension: crate::EnforceExtension::Disabled,
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            path.as_os_str().to_str().is_some_and(|s| re.find(s).is_some())
        }))],
        ..ResolveOptions::default()
    });

    // Should find index.css instead of index.js due to restriction
    let resolution = resolver.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck1/index.css")));
}

// Test coverage for check_restrictions at line 831 in load_alias_or_file()
#[test]
fn should_check_restrictions_in_load_alias_or_file() {
    let f = super::fixture().join("restrictions");

    // Restrict to only files outside the restrictions directory
    let restrictions_path = f.clone();
    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        restrictions: vec![Restriction::Fn(Arc::new(move |path| {
            !path.starts_with(&restrictions_path)
        }))],
        ..ResolveOptions::default()
    });

    // Direct file access should fail due to restriction
    let resolution = resolver.resolve(&f, "./node_modules/pck1/index.js");
    resolution.unwrap_err();
}

// Test coverage for check_restrictions at line 1148 in browser field/alias resolution
#[test]
fn should_check_restrictions_in_browser_field_alias() {
    let f = super::fixture().join("browser-module");

    let resolver = Resolver::new(ResolveOptions {
        alias_fields: vec![vec!["browser".into()]],
        restrictions: vec![Restriction::Fn(Arc::new(|path| {
            // Restrict files containing "browser" in their path
            !path.to_str().is_some_and(|s| s.contains("browser"))
        }))],
        ..ResolveOptions::default()
    });

    // Should fail to resolve due to restriction on browser field
    let resolution = resolver.resolve(&f, "./lib/self.js");
    resolution.unwrap_err();
}

// Test coverage for check_restrictions at line 1326 in load_extension_alias()
#[test]
fn should_check_restrictions_in_extension_alias() {
    let f = super::fixture().join("extension-alias");

    let resolver = Resolver::new(ResolveOptions {
        extension_alias: vec![
            (".js".into(), vec![".ts".into(), ".js".into()]),
            (".mjs".into(), vec![".mts".into(), ".mjs".into()]),
        ],
        restrictions: vec![Restriction::Fn(Arc::new(|path| {
            // Only allow .js files, not .ts files
            path.extension().and_then(|e| e.to_str()) == Some("js")
        }))],
        ..ResolveOptions::default()
    });

    // Should resolve to .js file even though .ts exists, due to restriction
    let resolution = resolver.resolve(&f, "./index.js").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("index.js")));
}

// Test coverage for check_restrictions at line 1570 in package main field resolution
#[test]
fn should_check_restrictions_in_package_main_fields() {
    let f = super::fixture().join("restrictions");

    let resolver = Resolver::new(ResolveOptions {
        main_fields: vec!["module".into(), "main".into()],
        restrictions: vec![Restriction::Fn(Arc::new(|path| {
            // Restrict .js files
            path.extension().and_then(|e| e.to_str()) != Some("js")
        }))],
        ..ResolveOptions::default()
    });

    // Should skip module.js and main field due to restriction
    let resolution = resolver.resolve(&f, "pck2");
    assert_eq!(resolution, Err(ResolveError::NotFound("pck2".to_string())));
}

// Test multiple restrictions together (Path + Fn)
#[test]
fn should_apply_multiple_restrictions() {
    let f = super::fixture().join("restrictions");

    // Use two function restrictions to test that both are applied
    let re_css = Regex::new(r"\.(css)$").unwrap();
    let re_no_js = Regex::new(r"\.(js)$").unwrap();
    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into(), ".css".into()],
        main_files: vec!["index".into()],
        restrictions: vec![
            Restriction::Fn(Arc::new(move |path| {
                path.as_os_str().to_str().is_some_and(|s| re_css.find(s).is_some())
            })),
            Restriction::Fn(Arc::new(move |path| {
                // Reject .js files
                path.as_os_str().to_str().is_some_and(|s| re_no_js.find(s).is_none())
            })),
        ],
        ..ResolveOptions::default()
    });

    // Should pass both restrictions and resolve to CSS file
    let resolution = resolver.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck1/index.css")));
}

// Test that all restrictions must pass
#[test]
fn should_fail_if_any_restriction_fails() {
    let f = super::fixture().join("restrictions");

    // Use two function restrictions where one will fail
    let re_css = Regex::new(r"\.(css)$").unwrap();
    let re_no_css = Regex::new(r"\.(css)$").unwrap();
    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into(), ".css".into()],
        main_files: vec!["index".into()],
        restrictions: vec![
            Restriction::Fn(Arc::new(move |path| {
                // First restriction: must be CSS
                path.as_os_str().to_str().is_some_and(|s| re_css.find(s).is_some())
            })),
            Restriction::Fn(Arc::new(move |path| {
                // Second restriction: must NOT be CSS (contradicts first)
                path.as_os_str().to_str().is_some_and(|s| re_no_css.find(s).is_none())
            })),
        ],
        ..ResolveOptions::default()
    });

    // Should fail because restrictions contradict each other
    let resolution = resolver.resolve(&f, "pck1");
    assert_eq!(resolution, Err(ResolveError::NotFound("pck1".to_string())));
}

// A restriction on a file admits exactly that file.
#[test]
fn should_allow_exact_path_in_restriction() {
    let f = super::fixture().join("restrictions");
    let exact_file = f.join("node_modules/pck1/index.css");

    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".css".into()],
        main_files: vec!["index".into()],
        restrictions: vec![Restriction::Path(exact_file.clone())],
        ..ResolveOptions::default()
    });

    let resolution = resolver.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Ok(exact_file));
}

// A restriction on a directory admits every file under it.
#[test]
fn should_resolve_inside_a_directory_restriction() {
    let f = super::fixture().join("restrictions");

    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        restrictions: vec![Restriction::Path(f.join("node_modules/pck1"))],
        ..ResolveOptions::default()
    });

    let resolution = resolver.resolve(&f, "pck1").map(|r| r.full_path());
    assert_eq!(resolution, Ok(f.join("node_modules/pck1/index.js")));
}

#[test]
fn should_respect_parent_directory_restriction() {
    let f = super::fixture().join("restrictions");

    let resolver = Resolver::new(ResolveOptions {
        extensions: vec![".js".into()],
        restrictions: vec![Restriction::Path(f.clone())],
        ..ResolveOptions::default()
    });

    // `pck2`'s main field points at `../../../c.js`, one level above the restricted
    // directory, so the package entry itself must be rejected.
    let resolution = resolver.resolve(&f, "pck2");
    assert_eq!(resolution, Err(ResolveError::NotFound("pck2".to_string())));
}

/// Path-component semantics of [`Restriction::Path`].
///
/// Ported from enhanced-resolve `restrictions` (`d8693b6`) and
/// <https://github.com/web-infra-dev/rspack-resolver/pull/288>.
#[cfg(not(target_os = "windows"))]
mod path_boundaries {
    use std::path::Path;

    use crate::{ResolveOptions, Resolver, Restriction};

    /// Builds a resolver (which normalizes the restriction) and asks whether `path` satisfies it.
    fn is_inside(restriction: &str, path: &str) -> bool {
        let resolver = Resolver::new(ResolveOptions {
            restrictions: vec![Restriction::Path(restriction.into())],
            ..ResolveOptions::default()
        });
        resolver.check_restrictions(Path::new(path))
    }

    #[test]
    fn file_inside_a_restriction() {
        assert!(is_inside("/a/b/c", "/a/b/c/index.js"));
    }

    #[test]
    fn the_restricted_directory_itself() {
        assert!(is_inside("/a/b/c", "/a/b/c"));
    }

    #[test]
    fn sibling_of_a_restriction() {
        assert!(!is_inside("/a/b/c", "/a/b/c-other.js"));
    }

    #[test]
    fn file_inside_a_restriction_ending_with_a_separator() {
        assert!(is_inside("/a/b/c/", "/a/b/c/index.js"));
    }

    #[test]
    fn file_inside_a_non_normalized_restriction() {
        assert!(is_inside("/a/x/../b/c", "/a/b/c/index.js"));
    }

    #[test]
    fn file_inside_the_root_restriction() {
        assert!(is_inside("/", "/a/index.js"));
    }

    #[test]
    fn empty_restriction_matches_every_path() {
        assert!(is_inside("", "/a/b/c/index.js"));
    }

    #[test]
    fn relative_restriction_matches_no_absolute_path() {
        assert!(!is_inside(".", "/a/b/c/index.js"));
        assert!(!is_inside("..", "/a/b/c/index.js"));
        assert!(!is_inside("foo/..", "/a/b/c/index.js"));
    }

    #[test]
    fn file_inside_a_restriction_differing_in_case() {
        assert!(!is_inside("/A/B/C", "/a/b/c/index.js"));
    }
}

/// Windows path forms cannot travel through a resolver on a posix host, so the check is driven
/// directly, the way upstream drives its plugin.
#[cfg(target_os = "windows")]
mod windows_path_boundaries {
    use std::path::Path;

    use crate::{ResolveOptions, Resolver, Restriction};

    fn is_inside(restriction: &str, path: &str) -> bool {
        let resolver = Resolver::new(ResolveOptions {
            restrictions: vec![Restriction::Path(restriction.into())],
            ..ResolveOptions::default()
        });
        resolver.check_restrictions(Path::new(path))
    }

    #[test]
    fn path_using_slashes_under_a_backslash_restriction() {
        assert!(is_inside(r"C:\a\b\c", "C:/a/b/c/index.js"));
    }

    #[test]
    fn path_mixing_separators_under_a_slash_restriction() {
        assert!(is_inside("C:/a/b/c", r"C:\a\b/c/index.js"));
    }

    #[test]
    fn path_under_a_restriction_ending_with_a_slash() {
        assert!(is_inside(r"C:\a\b\c/", r"C:\a\b\c\index.js"));
    }

    #[test]
    fn the_restricted_directory_itself() {
        assert!(is_inside(r"C:\a\b\c", r"C:\a\b\c"));
    }

    #[test]
    fn path_inside_a_unc_restriction() {
        assert!(is_inside(r"\\server\share\a", r"\\server\share\a\index.js"));
    }

    #[test]
    fn path_inside_a_dos_device_restriction() {
        assert!(is_inside(r"\\?\C:\a", r"\\?\C:\a\index.js"));
    }

    #[test]
    fn path_on_another_share_than_the_unc_restriction() {
        assert!(!is_inside(r"\\server\share\a", r"\\server\other\a\index.js"));
    }

    #[test]
    fn sibling_of_a_restriction_written_with_slashes() {
        assert!(!is_inside(r"C:\a\b\c", "C:/a/b/c-other.js"));
    }
}

/// A restriction has to hold for the path the resolver returns, not for the spelling a candidate
/// happened to have while it was being selected.
///
/// Ported from enhanced-resolve `restrictions > with symlinks`,
/// <https://github.com/webpack/enhanced-resolve/pull/595>.
///
/// These need real symlinks, which wasm does not have.
#[cfg(not(target_family = "wasm"))]
mod escaping_the_restriction {
    use std::{
        fs, io,
        ops::Deref,
        path::{Path, PathBuf},
    };

    use crate::{ResolveError, ResolveOptions, Resolver, Restriction};

    /// `\\?\`, spelled so that no literal ends with a backslash.
    const VERBATIM_PREFIX: &str = concat!('\\', '\\', '?', '\\');

    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
        #[cfg(target_family = "unix")]
        {
            std::os::unix::fs::symlink(original, link)
        }
        #[cfg(target_os = "windows")]
        {
            std::os::windows::fs::symlink_file(original, link)
        }
    }

    /// A scratch directory under `fixtures/`, removed when dropped.
    struct Scratch(PathBuf);

    impl Deref for Scratch {
        type Target = Path;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            _ = fs::remove_dir_all(&self.0);
            // Succeeds for the last test to finish; fails harmlessly while a sibling subdirectory
            // is still in use.
            if let Some(parent) = self.0.parent() {
                _ = fs::remove_dir(parent);
            }
        }
    }

    /// `allowed/{link,rel-link}.js` point at `outside/secret.js`.
    fn build(root: &Path) -> io::Result<()> {
        fs::create_dir_all(root.join("allowed"))?;
        fs::create_dir_all(root.join("outside"))?;
        fs::write(root.join("outside/secret.js"), "")?;
        fs::write(root.join("allowed/real.js"), "")?;
        symlink(root.join("outside/secret.js"), root.join("allowed/link.js"))?;
        symlink(Path::new("../outside/secret.js"), root.join("allowed/rel-link.js"))
    }

    impl Scratch {
        /// `None` when the platform refuses to create symlinks.
        fn new(name: &str) -> Option<Self> {
            let root = super::super::fixture_root().join("restrictions-scratch").join(name);
            _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).ok()?;
            // The restriction has to name the real directory, and Windows answers with a verbatim
            // (`\\?\`) path where `join` collapses the `..` these tests are about.
            let canonical = root.canonicalize().ok()?;
            let canonical = canonical.to_str()?;
            let root = PathBuf::from(canonical.strip_prefix(VERBATIM_PREFIX).unwrap_or(canonical));
            build(&root).ok()?;
            Some(Self(root))
        }
    }

    fn resolve_with(root: &Path, specifier: &str, symlinks: bool) -> Result<PathBuf, ResolveError> {
        let allowed = root.join("allowed");
        Resolver::new(ResolveOptions {
            extensions: vec![".js".into()],
            restrictions: vec![Restriction::Path(allowed.clone())],
            symlinks,
            ..ResolveOptions::default()
        })
        .resolve(&allowed, specifier)
        .map(|r| r.full_path())
    }

    #[test]
    fn in_root_symlink_to_an_outside_target() {
        let Some(root) = Scratch::new("symlink") else {
            return;
        };
        assert_eq!(
            resolve_with(&root, "./link.js", true),
            Err(ResolveError::NotFound("./link.js".to_string()))
        );
    }

    #[test]
    fn in_root_relative_symlink_to_an_outside_target() {
        let Some(root) = Scratch::new("rel-symlink") else {
            return;
        };
        assert_eq!(
            resolve_with(&root, "./rel-link.js", true),
            Err(ResolveError::NotFound("./rel-link.js".to_string()))
        );
    }

    #[test]
    fn parent_dir_traversal_in_an_absolute_specifier() {
        let Some(root) = Scratch::new("traversal") else {
            return;
        };
        let escaping = root.join("allowed/../outside/secret.js");
        let escaping = escaping.to_str().expect("path should be UTF-8");
        assert_eq!(
            resolve_with(&root, escaping, true),
            Err(ResolveError::NotFound(escaping.to_string()))
        );
    }

    #[test]
    fn parent_dir_traversal_without_symlink_resolution() {
        let Some(root) = Scratch::new("traversal-nosym") else {
            return;
        };
        let escaping = root.join("allowed/../outside/secret.js");
        let escaping = escaping.to_str().expect("path should be UTF-8");
        assert_eq!(
            resolve_with(&root, escaping, false),
            Err(ResolveError::NotFound(escaping.to_string()))
        );
    }

    #[test]
    fn real_in_root_file_still_resolves() {
        let Some(root) = Scratch::new("real") else {
            return;
        };
        assert_eq!(resolve_with(&root, "./real.js", true), Ok(root.join("allowed/real.js")));
    }
}
