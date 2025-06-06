// See documentation at <https://docs.rs/unrs_resolver>

use std::{env, path::PathBuf};

use unrs_resolver::{AliasValue, ResolveOptions, Resolver};

fn main() {
    let path = PathBuf::from(env::args().nth(1).expect("path"));

    assert!(path.is_dir(), "{} must be a directory that will be resolved against.", path.display());
    assert!(path.is_absolute(), "{} must be an absolute path.", path.display());

    let specifier = env::args().nth(2).expect("specifier");

    println!("path: {}", path.to_string_lossy());
    println!("specifier: {specifier}");

    let options = ResolveOptions {
        alias_fields: vec![vec!["browser".into()]],
        alias: vec![("asdf".into(), vec![AliasValue::from("./test.js")])],
        extensions: vec![".js".into(), ".ts".into()],
        extension_alias: vec![(".js".into(), vec![".ts".into(), ".js".into()])],
        // ESM
        condition_names: vec!["node".into(), "import".into()],
        // CJS
        // condition_names: vec!["node".into(), "require".into()],
        ..ResolveOptions::default()
    };

    match Resolver::new(options).resolve(path, &specifier) {
        Err(error) => println!("Error: {error}"),
        Ok(resolution) => println!("Resolved: {}", resolution.full_path().display()),
    }
}
