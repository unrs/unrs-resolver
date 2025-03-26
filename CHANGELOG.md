# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0](https://github.com/unrs/unrs-resolver/compare/unrs_resolver-v1.3.0...unrs_resolver-v2.0.0) - 2025-03-26

### <!-- 0 -->Features

- *(napi)* add mimalloc ([#423](https://github.com/unrs/unrs-resolver/pull/423)) ([#18](https://github.com/unrs/unrs-resolver/pull/18))
- merge from upstream `oxc-project/oxc-resolver` ([#15](https://github.com/unrs/unrs-resolver/pull/15))
- support resolving path with extra query ([#7](https://github.com/unrs/unrs-resolver/pull/7))
- *(pnp)* support link ([#49](https://github.com/unrs/unrs-resolver/pull/49))
- Revert  vfs logic ([#44](https://github.com/unrs/unrs-resolver/pull/44))
- expose PnpFileSystem ([#43](https://github.com/unrs/unrs-resolver/pull/43))
- pub read api ([#41](https://github.com/unrs/unrs-resolver/pull/41))
- rebase and refine extension-alias error format ([#30](https://github.com/unrs/unrs-resolver/pull/30))
- pub all FileMetadata field ([#27](https://github.com/unrs/unrs-resolver/pull/27))
- remove send + sync constraint ([#25](https://github.com/unrs/unrs-resolver/pull/25))
- rebase latest oxc-resolver and support pnp ([#12](https://github.com/unrs/unrs-resolver/pull/12))
- *(napi)* add tracing via `OXC_LOG:DEBUG` ([#202](https://github.com/unrs/unrs-resolver/pull/202))
- strip symbols and enable LTO ([#197](https://github.com/unrs/unrs-resolver/pull/197))
- export package.json `type` and `sideEffects` field by default for bundlers ([#196](https://github.com/unrs/unrs-resolver/pull/196))
- *(napi)* add async API ([#191](https://github.com/unrs/unrs-resolver/pull/191))
- [**breaking**] remove the constraint on packages exports `default` must be the last one ([#171](https://github.com/unrs/unrs-resolver/pull/171))
- [**breaking**] return `ResolveError:Builtin("node:{specifier}")` from package imports and exports ([#165](https://github.com/unrs/unrs-resolver/pull/165))
- add `imports_fields` option ([#138](https://github.com/unrs/unrs-resolver/pull/138))
- substitute path that starts with `${configDir}/` in tsconfig.compilerOptions.paths ([#136](https://github.com/unrs/unrs-resolver/pull/136))
- allow `Resolver<Box<dyn FileSystem>>` by removing unnecessary `Default` constraint ([#116](https://github.com/unrs/unrs-resolver/pull/116))
- add more builder functions for options ([#110](https://github.com/unrs/unrs-resolver/pull/110))
- add feature `package_json_raw_json_api` for returning package's raw json ([#104](https://github.com/unrs/unrs-resolver/pull/104))
- support tsconfig#extends array ([#102](https://github.com/unrs/unrs-resolver/pull/102))
- more builder pattern options ([#84](https://github.com/unrs/unrs-resolver/pull/84))
- functions to add more options using builder pattern ([#81](https://github.com/unrs/unrs-resolver/pull/81))
- *(napi)* support wasi target ([#31](https://github.com/unrs/unrs-resolver/pull/31))
- add file_dependencies and missing_dependencies API ([#50](https://github.com/unrs/unrs-resolver/pull/50))
- add context to PackageImportNotDefined error
- improve errors by adding more contexts
- *(napi)* expose cloneWithOptions and clearCache methods ([#40](https://github.com/unrs/unrs-resolver/pull/40))
- clean up the error message todos ([#38](https://github.com/unrs/unrs-resolver/pull/38))
- return not found when recursing non-existent file ([#36](https://github.com/unrs/unrs-resolver/pull/36))
- *(napi)* update the doc and type for tsconfig references ([#24](https://github.com/unrs/unrs-resolver/pull/24))
- *(napi)* add options ([#19](https://github.com/unrs/unrs-resolver/pull/19))
- *(resolver)* add a `realpath` to package.json ([#1634](https://github.com/unrs/unrs-resolver/pull/1634))
- *(resovler)* impl Into for IOError ([#1223](https://github.com/unrs/unrs-resolver/pull/1223))
- *(resolver)* strip trailling commas from tsconfig.json ([#1198](https://github.com/unrs/unrs-resolver/pull/1198))
- *(resolver)* configurable tsconfig project references ([#965](https://github.com/unrs/unrs-resolver/pull/965))
- *(resolver)* add more tracing events to resolver ([#907](https://github.com/unrs/unrs-resolver/pull/907))
- *(resolver)* add TsconfigNotFound error ([#905](https://github.com/unrs/unrs-resolver/pull/905))
- *(resolver)* add tracing-subscriber feature ([#904](https://github.com/unrs/unrs-resolver/pull/904))
- *(resolver)* tsconfig project references ([#862](https://github.com/unrs/unrs-resolver/pull/862))
- *(resolver)* add thiserror ([#847](https://github.com/unrs/unrs-resolver/pull/847))
- *(resolver)* add tracing example
- *(resolver)* add an option to turn off builtin_modules ([#833](https://github.com/unrs/unrs-resolver/pull/833))
- *(resolver)* check for node.js core modules ([#794](https://github.com/unrs/unrs-resolver/pull/794))
- *(resolver)* implement nested alias field ([#795](https://github.com/unrs/unrs-resolver/pull/795))
- *(resolver)* implement tsconfig-paths ([#750](https://github.com/unrs/unrs-resolver/pull/750))
- *(resolver)* handle path alias with `#` ([#739](https://github.com/unrs/unrs-resolver/pull/739))
- *(resolver)* expose raw package_json value; improve print debug ([#738](https://github.com/unrs/unrs-resolver/pull/738))
- *(resolver)* implement configurable `exports_fields` option ([#733](https://github.com/unrs/unrs-resolver/pull/733))
- *(resolver)* resolve `#` as path instead of a fragment ([#727](https://github.com/unrs/unrs-resolver/pull/727))
- *(resolver)* pass on query string from alias fields
- *(resolver)* complete browser_field implementation
- *(resolver)* check for infinite recursion ([#714](https://github.com/unrs/unrs-resolver/pull/714))
- *(resolver)* implement `main_fields`
- *(resolver)* add `exports_fields` and `main_fields` for logging purposes.
- *(resolver)* add tracing ([#710](https://github.com/unrs/unrs-resolver/pull/710))
- *(resolver)* implement recursive alias, file as alias and exports field with query / fragment ([#695](https://github.com/unrs/unrs-resolver/pull/695))
- *(resolver)* implement resolveToContext ([#694](https://github.com/unrs/unrs-resolver/pull/694))
- *(resolver)* implement restrictions (path only) ([#693](https://github.com/unrs/unrs-resolver/pull/693))
- *(resolver)* implement the basics of ESM ([#691](https://github.com/unrs/unrs-resolver/pull/691))
- *(resolver)* implement fully specified ([#687](https://github.com/unrs/unrs-resolver/pull/687))
- *(resolver)* imports field ([#681](https://github.com/unrs/unrs-resolver/pull/681))
- *(resolver)* finish most of exports field ([#674](https://github.com/unrs/unrs-resolver/pull/674))
- *(resolver)* port the rest of the exports field tests ([#659](https://github.com/unrs/unrs-resolver/pull/659))
- *(resolver)* implement more of exports field ([#648](https://github.com/unrs/unrs-resolver/pull/648))
- *(resolver)* initialize implementation of package.json exports field ([#630](https://github.com/unrs/unrs-resolver/pull/630))
- *(resolver)* check for directory before loading a directory ([#590](https://github.com/unrs/unrs-resolver/pull/590))
- *(resolver)* implement symlinks ([#582](https://github.com/unrs/unrs-resolver/pull/582))
- *(resolver)* complete query and fragment parsing ([#579](https://github.com/unrs/unrs-resolver/pull/579))
- *(resolver)* add preferRelative and preferAbsolute ([#577](https://github.com/unrs/unrs-resolver/pull/577))
- *(resolver)* implement roots ([#576](https://github.com/unrs/unrs-resolver/pull/576))
- *(resolver)* implement fallback ([#572](https://github.com/unrs/unrs-resolver/pull/572))
- *(resolver)* implement enforceExtension ([#567](https://github.com/unrs/unrs-resolver/pull/567))
- *(resolver)* implement enforceExtension ([#566](https://github.com/unrs/unrs-resolver/pull/566))
- *(resolver)* implement descriptionFiles option ([#565](https://github.com/unrs/unrs-resolver/pull/565))
- *(resolver)* implement the basics of path alias ([#564](https://github.com/unrs/unrs-resolver/pull/564))
- *(resolver)* accept different file system implementations ([#562](https://github.com/unrs/unrs-resolver/pull/562))
- *(resolver)* implement browser field ([#561](https://github.com/unrs/unrs-resolver/pull/561))
- *(resolver)* implement scoped packages ([#558](https://github.com/unrs/unrs-resolver/pull/558))
- *(resolver)* port incorrect description file test ([#557](https://github.com/unrs/unrs-resolver/pull/557))
- *(resolver)* implement extension_alias ([#556](https://github.com/unrs/unrs-resolver/pull/556))
- *(resolver)* port resolve tests ([#555](https://github.com/unrs/unrs-resolver/pull/555))
- *(resolver)* resolve extensions ([#549](https://github.com/unrs/unrs-resolver/pull/549))
- *(resolver)* resolve as module ([#544](https://github.com/unrs/unrs-resolver/pull/544))
- *(resolver)* resolve js file ([#543](https://github.com/unrs/unrs-resolver/pull/543))
- *(resolver)* add resolver test fixtures ([#542](https://github.com/unrs/unrs-resolver/pull/542))

### <!-- 1 -->Bug Fixes

- *(pnp)* support `pnpapi` core module and package deep link ([#24](https://github.com/unrs/unrs-resolver/pull/24))
- references should take higher priority ([#13](https://github.com/unrs/unrs-resolver/pull/13))
- takes paths and references into account at the same time
- should always try resolve_path_alias
- abs path in main fields ([#52](https://github.com/unrs/unrs-resolver/pull/52))
- 🐛 pnp feat respect options.enable_pnp ([#47](https://github.com/unrs/unrs-resolver/pull/47))
- alias match request end with slash ([#35](https://github.com/unrs/unrs-resolver/pull/35))
- resolve mathjs error when using `extensionAlias` ([#31](https://github.com/unrs/unrs-resolver/pull/31))
- fix symbol link support in pnpm workspace ([#21](https://github.com/unrs/unrs-resolver/pull/21))
- fallback to next main field when resolve failed ([#17](https://github.com/unrs/unrs-resolver/pull/17))
- tsconfig project reference it self should throw error ([#211](https://github.com/unrs/unrs-resolver/pull/211))
- comment ([#179](https://github.com/unrs/unrs-resolver/pull/179))
- alias value should try fragment as path ([#172](https://github.com/unrs/unrs-resolver/pull/172))
- alias not found should return error ([#168](https://github.com/unrs/unrs-resolver/pull/168))
- RootsPlugin debug_assert on windows ([#145](https://github.com/unrs/unrs-resolver/pull/145))
- RootsPlugin should fall through if it fails to resolve the roots ([#144](https://github.com/unrs/unrs-resolver/pull/144))
- lazily read package.json.exports for shared resolvers ([#137](https://github.com/unrs/unrs-resolver/pull/137))
- incorrect resolution when using shared resolvers with different `main_fields` ([#134](https://github.com/unrs/unrs-resolver/pull/134))
- canonicalize is not supported on wasi target ([#124](https://github.com/unrs/unrs-resolver/pull/124))
- missing `Debug` for `Specifier`
- windows path with C:// like prefixes ([#92](https://github.com/unrs/unrs-resolver/pull/92))
- extending tsconfig paths with baseUrl in original tsconfig file ([#91](https://github.com/unrs/unrs-resolver/pull/91))
- specifier with multiple `?` ([#83](https://github.com/unrs/unrs-resolver/pull/83))
- tsconfig#extends must be a string ([#80](https://github.com/unrs/unrs-resolver/pull/80))
- normalize aliased path ([#78](https://github.com/unrs/unrs-resolver/pull/78))
- panic when `?` is passed in ([#70](https://github.com/unrs/unrs-resolver/pull/70))
- resolve "browser" field when "exports" is present ([#59](https://github.com/unrs/unrs-resolver/pull/59))
- returning broken missing dependencies when alias and extensions are provided ([#54](https://github.com/unrs/unrs-resolver/pull/54))
- change ResolveError::NotFound(PathBuf) to report specifier
- make raw_json return `&Arc<serde_json::Value>`
- browser field resolving relative to path to itself ([#34](https://github.com/unrs/unrs-resolver/pull/34))
- *(justfile)* fix example command
- *(resolver)* resolve query and fragments with unicode filenames ([#1591](https://github.com/unrs/unrs-resolver/pull/1591))
- *(resolver)* make sure package.json path is inside the resolved path ([#1481](https://github.com/unrs/unrs-resolver/pull/1481))
- *(resolver)* resolve tsconfig extend that are extensionless ([#971](https://github.com/unrs/unrs-resolver/pull/971))
- *(resolver)* log error as debug so it does not print the error by default
- *(resolver)* fix tsconfig lookup when a directory is provided ([#900](https://github.com/unrs/unrs-resolver/pull/900))
- *(resolver)* fix collision on hash entries ([#850](https://github.com/unrs/unrs-resolver/pull/850))
- *(resolver)* fix a case where ignored package has a fallback ([#837](https://github.com/unrs/unrs-resolver/pull/837))
- *(resolver)* fix a case where an alias is part of a dashed package name ([#836](https://github.com/unrs/unrs-resolver/pull/836))
- *(resolver)* fix cases with conflicting node_modules path ([#835](https://github.com/unrs/unrs-resolver/pull/835))
- *(resolver)* add test case for resolve_to_context ([#834](https://github.com/unrs/unrs-resolver/pull/834))
- *(resolver)* resolve exports field that are directories ([#820](https://github.com/unrs/unrs-resolver/pull/820))
- *(resolver)* fix resolving package_self with the correct subpath
- *(resolver)* correct behavior for enforceExtension
- *(resolver)* do not resolve browser field that are strings ([#816](https://github.com/unrs/unrs-resolver/pull/816))
- *(resolver)* make sure package name is valid when loading package self ([#810](https://github.com/unrs/unrs-resolver/pull/810))
- *(resolver)* fix a case where package name and specifier is the wrong order
- *(resolver)* add a case with multi-dot filename
- *(resolver)* add `derive` to serde
- *(resolver)* fix a case with multi-dot file extensions ([#704](https://github.com/unrs/unrs-resolver/pull/704))
- *(resolver)* fix resolver benchmark

### <!-- 2 -->Performance

- skip resolving extension alias when `options.extension_alias` is empty ([#203](https://github.com/unrs/unrs-resolver/pull/203))
- improve call to `Path::ends_with` ([#199](https://github.com/unrs/unrs-resolver/pull/199))
- skip searching for package.json when `alias_fields` is not provided ([#194](https://github.com/unrs/unrs-resolver/pull/194))
- *(resolver)* remove extra large fields from raw package json ([#23](https://github.com/unrs/unrs-resolver/pull/23))
- *(resolver)* do not search for package.json inside non-existing directories ([#1480](https://github.com/unrs/unrs-resolver/pull/1480))
- *(resolver)* use system canonicalize to reduce total number of path hashes ([#902](https://github.com/unrs/unrs-resolver/pull/902))
- *(resolver)* used cached node_modules in `package_resolve` ([#901](https://github.com/unrs/unrs-resolver/pull/901))
- *(resolver)* do not search inside non-existent directories for scoped packages ([#899](https://github.com/unrs/unrs-resolver/pull/899))
- *(resolver)* avoid double hashing by memoizing the hash ([#871](https://github.com/unrs/unrs-resolver/pull/871))
- *(resolver)* optimize canonicalize ([#870](https://github.com/unrs/unrs-resolver/pull/870))
- *(resolver)* cache `node_modules` lookup ([#869](https://github.com/unrs/unrs-resolver/pull/869))
- *(resolver)* stop descending into node_modules when possible ([#821](https://github.com/unrs/unrs-resolver/pull/821))
- *(resolver)* reduce memory allocation when resolving node_modules ([#608](https://github.com/unrs/unrs-resolver/pull/608))
- *(resolver)* hash once for the `get` + `insert` case ([#606](https://github.com/unrs/unrs-resolver/pull/606))
- *(resolver)* allocate less when resolving extensions ([#603](https://github.com/unrs/unrs-resolver/pull/603))
- *(resolver)* reduce the total number of hashes by passing the cached value around ([#602](https://github.com/unrs/unrs-resolver/pull/602))
- *(resolver)* do not read package_json of a file ([#601](https://github.com/unrs/unrs-resolver/pull/601))
- *(resolver)* improve cache hit for package.json ([#585](https://github.com/unrs/unrs-resolver/pull/585))
- *(resolver)* cache canonicalized path ([#584](https://github.com/unrs/unrs-resolver/pull/584))
- *(resolver)* use `fs::symlink_metadata`, which doesn't traverse symlinks ([#581](https://github.com/unrs/unrs-resolver/pull/581))
- *(resolver)* cache all package.json queries ([#569](https://github.com/unrs/unrs-resolver/pull/569))
- *(resolver)* use rustc_hash::FxHasher for DashMap ([#568](https://github.com/unrs/unrs-resolver/pull/568))
- *(resolver)* add file system cache ([#547](https://github.com/unrs/unrs-resolver/pull/547))

### <!-- 3 -->Documentation

- document feature flags
- improve terminology and clarify contexts ([#118](https://github.com/unrs/unrs-resolver/pull/118))
- *(README)* add errors section
- document `specifier` and `path` for `resolve`
- remove some periods

### <!-- 4 -->Refactor

- add test cases for resolve alias value with fragment ([#170](https://github.com/unrs/unrs-resolver/pull/170))
- improve `normalize_with` function ([#153](https://github.com/unrs/unrs-resolver/pull/153))
- remove `PartialEq` and `Eq` from `Specifier` ([#148](https://github.com/unrs/unrs-resolver/pull/148))
- avoid an extra allocation in `load_extensions`
- remove the browser field lookup in `resolve_esm_match` ([#141](https://github.com/unrs/unrs-resolver/pull/141))
- remove the extra `condition_names` from `package_exports_resolve`
- deserialize less values in tsconfig ([#109](https://github.com/unrs/unrs-resolver/pull/109))
- selectively parse package_json fields instead of parsing everything ([#103](https://github.com/unrs/unrs-resolver/pull/103))
- increase some codecov
- use the same const for slash start '/' and '\\'
- improve code code coverage ([#67](https://github.com/unrs/unrs-resolver/pull/67))
- s/ResolveContext/Ctx for inner usage
- add a `resolve_tracing` method
- move ResolveContext to its own file
- s/ResolveState/ResolveResult
- use FxHashMap instead of FxIndexMap for BrowserField ([#33](https://github.com/unrs/unrs-resolver/pull/33))
- tweak the load_browser_field API
- remove some unnecessary trace information
- *(resolver)* do not search for package.json inside non-existing directories ([#1482](https://github.com/unrs/unrs-resolver/pull/1482))
- *(rust)* move to workspace lint table ([#1444](https://github.com/unrs/unrs-resolver/pull/1444))
- *(clippy)* allow clippy::too_many_lines
- *(clippy)* allow struct_excessive_bools
- *(resolver)* move tests folder to fixtures ([#964](https://github.com/unrs/unrs-resolver/pull/964))
- *(resolver)* clean up `load_alias` ([#875](https://github.com/unrs/unrs-resolver/pull/875))
- *(resolver)* remove unnecessary `RefCell` ([#849](https://github.com/unrs/unrs-resolver/pull/849))
- *(benchmark)* use codspeed for all benchmarks ([#839](https://github.com/unrs/unrs-resolver/pull/839))
- *(resolver)* remove nodejs_resolver comparison
- improve code coverage a little bit
- improve code coverage in various places ([#721](https://github.com/unrs/unrs-resolver/pull/721))
- *(resolver)* remove the leading dot trim on extensions
- *(resolver)* clean up some code and tests
- *(resolver)* clean up the tests a little bit
- *(resolver)* remove the identity-hash crate
- *(resolver)* add a EnforceExtension tri state
- *(resolver)* make Resolution::full_path not owned
- *(resolver)* return package json error immediately instead of saving it ([#702](https://github.com/unrs/unrs-resolver/pull/702))
- *(resolver)* improve code by looking at the code coverage ([#697](https://github.com/unrs/unrs-resolver/pull/697))
- *(resolver)* clean some code ([#692](https://github.com/unrs/unrs-resolver/pull/692))
- *(resolver)* change internal funcs to non-pub by moving to unit tests ([#682](https://github.com/unrs/unrs-resolver/pull/682))
- *(resolver)* use DashSet for the cache ([#605](https://github.com/unrs/unrs-resolver/pull/605))
- *(resolver)* make the global cache hold less memory ([#593](https://github.com/unrs/unrs-resolver/pull/593))
- *(resolver)* improve browser_field lookup ([#592](https://github.com/unrs/unrs-resolver/pull/592))
- *(resolver)* s/request_str/request
- *(resolver)* improve how browser field is resolved ([#589](https://github.com/unrs/unrs-resolver/pull/589))
- *(resolver)* check against Result for better assertion message ([#573](https://github.com/unrs/unrs-resolver/pull/573))
- *(resolver)* add our own path util for normalization
- *(resolver)* add oxc_resolver crate

## [1.3.0](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.2.2...unrspack-resolver-v1.3.0) - 2025-03-21

### Features

- enable more targets ([#29](https://github.com/unrs/unrs-resolver/pull/29))

## [1.2.2](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.2.1...unrspack-resolver-v1.2.2) - 2025-03-19

### <!-- 1 -->Bug Fixes

- *(pnp)* support `pnpapi` core module and package deep link ([#24](https://github.com/unrs/unrs-resolver/pull/24))

## [1.2.0](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.1.2...unrspack-resolver-v2.0.0) - 2025-03-18

### Features

- *(napi)* add mimalloc ([#423](https://github.com/unrs/unrs-resolver/pull/423)) ([#18](https://github.com/unrs/unrs-resolver/pull/18))
- merge from upstream `oxc-project/oxc-resolver` ([#15](https://github.com/unrs/unrs-resolver/pull/15))

## [1.1.2](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.1.1...unrspack-resolver-v1.1.2) - 2025-03-16

### Fixed

- references should take higher priority ([#13](https://github.com/unrs/unrs-resolver/pull/13))
- takes paths and references into account at the same time
- should always try resolve_path_alias

## [1.1.1](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.1.0...unrspack-resolver-v1.1.1) - 2025-03-16

### Other

- bump all (dev) deps
- bump to edition 2024

## [1.1.0](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.0.0...unrspack-resolver-v1.1.0) - 2025-03-15

### Added

- support resolving path with extra query ([#7](https://github.com/unrs/unrs-resolver/pull/7))

## [1.0.0](https://github.com/unrs/unrs-resolver/releases/tag/unrspack-resolver-v1.0.0) - 2025-03-15

## [5.0.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v4.2.0...oxc_resolver-v5.0.0) - 2025-03-07

### <!-- 0 -->Features

- [**breaking**] Rust Edition 2024 ([#402](https://github.com/oxc-project/oxc-resolver/pull/402))
- deserialize `verbatim_module_syntax` from compilerOptions ([#411](https://github.com/oxc-project/oxc-resolver/pull/411))

### <!-- 4 -->Refactor

- remove papaya `.collector(seize::Collector::new())` call ([#393](https://github.com/oxc-project/oxc-resolver/pull/393))

## [4.2.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v4.1.0...oxc_resolver-v4.2.0) - 2025-02-19

### <!-- 0 -->Features

- support wildcard `*` in alias plugin (#388)

## [4.1.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v4.0.1...oxc_resolver-v4.1.0) - 2025-02-14

### <!-- 0 -->Features

- merge options from extends tsconfig.json (#375)
- add more fields in tsconfig#CompilerOptionsSerde (#374)

### <!-- 1 -->Bug Fixes

- fix bench

## [4.0.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v3.0.3...oxc_resolver-v4.0.0) - 2025-01-20

### <!-- 0 -->Features

- [**breaking**] generic fs cache `type Resolver = ResolverGeneric<FsCache<FileSystemOs>>` (#358)
- [**breaking**] `PackageJson` and `TsConfig` traits (#360)

### <!-- 2 -->Performance

- use papaya instead of dashmap (#356)

## [3.0.3](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v3.0.2...oxc_resolver-v3.0.3) - 2024-12-14

### <!-- 1 -->Bug Fixes

- try browsers field and alias before resolving directory in node_modules (#349)

## [3.0.2](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v3.0.1...oxc_resolver-v3.0.2) - 2024-12-13

### <!-- 1 -->Bug Fixes

- special case for aliasing `@/` (#348)
- normalize resolved result on Windows for root (#345)

### <!-- 4 -->Refactor

- replace UnsafeCell with RefCell (#346)

## [3.0.1](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v3.0.0...oxc_resolver-v3.0.1) - 2024-12-12

### <!-- 2 -->Performance

- try directory first in `node_modules` (#340)

## [3.0.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v2.1.1...oxc_resolver-v3.0.0) - 2024-12-11

### Added

- [**breaking**] replace `FileSystem::canonicalize` with `FileSystem::read_link` (#331)

### Other

- guard `load_alias` on hot path (#339)

## [2.1.1](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v2.1.0...oxc_resolver-v2.1.1) - 2024-11-22

### Performance

- reduce hash while resolving package.json ([#319](https://github.com/oxc-project/oxc-resolver/pull/319))
- reduce memory allocation while normalizing package path ([#318](https://github.com/oxc-project/oxc-resolver/pull/318))
- reduce memory allocation while resolving package.json ([#317](https://github.com/oxc-project/oxc-resolver/pull/317))
- use `path.as_os_str().hash()` instead of `path.hash()` ([#316](https://github.com/oxc-project/oxc-resolver/pull/316))
- reduce memory allocation by using a thread_local path for path methods ([#315](https://github.com/oxc-project/oxc-resolver/pull/315))

### Other

- remove the deprecated simdutf8 aarch64_neon feature
- mention extension must start with a `.` in `with_extension` ([#313](https://github.com/oxc-project/oxc-resolver/pull/313))

## [2.1.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v2.0.1...oxc_resolver-v2.1.0) - 2024-11-20

### Added

- add `Resolver::resolve_tsconfig` API ([#312](https://github.com/oxc-project/oxc-resolver/pull/312))

### Fixed

- don't panic when resolving `/` with `roots` ([#310](https://github.com/oxc-project/oxc-resolver/pull/310))
- use same UNC path normalization logic with libuv ([#306](https://github.com/oxc-project/oxc-resolver/pull/306))

### Other

- *(deps)* update rust crates to v1.0.215
- fix symlink test init on windows ([#307](https://github.com/oxc-project/oxc-resolver/pull/307))

## [2.0.1](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v2.0.0...oxc_resolver-v2.0.1) - 2024-11-08

### Other

- `cargo upgrade` && `pnpm upgrade`
- bring back the symlink optimization ([#298](https://github.com/oxc-project/oxc-resolver/pull/298))
- *(deps)* update rust crate criterion2 to v2

## [2.0.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v1.12.0...oxc_resolver-v2.0.0) - 2024-10-22

### Added

- [**breaking**] add `add ResolveError::Builtin::is_runtime_module` ([#272](https://github.com/oxc-project/oxc-resolver/pull/272))

### Fixed

- use `fs::canonicalize` to cover symlink edge cases ([#284](https://github.com/oxc-project/oxc-resolver/pull/284))
- extensionAlias cannot resolve mathjs ([#273](https://github.com/oxc-project/oxc-resolver/pull/273))

## [0.5.2](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.5.1...rspack_resolver-v0.5.2) - 2025-02-28

### Added

- *(pnp)* support link (#49)

### Other

- bump pnp 0.9.1 (#50)

## [0.5.1](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.5.0...rspack_resolver-v0.5.1) - 2025-02-11

### Fixed

- 🐛 pnp feat respect options.enable_pnp (#47)

## [0.4.0](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.3.6...rspack_resolver-v0.4.0) - 2024-12-26

### Feat

- Implements the PnP manifest lookup within the resolver ([#39](https://github.com/web-infra-dev/rspack-resolver/pull/39))

## [0.3.6](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.3.5...rspack_resolver-v0.3.6) - 2024-12-13

### Fixed

- alias match request end with slash (#35)

## [0.3.5](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.3.4...rspack_resolver-v0.3.5) - 2024-10-21

### Fixed

- resolve mathjs error when using `extensionAlias` ([#31](https://github.com/web-infra-dev/rspack-resolver/pull/31))

## [0.3.4](https://github.com/web-infra-dev/rspack-resolver/compare/rspack_resolver-v0.3.3...rspack_resolver-v0.3.4) - 2024-10-21

### Added

- rebase and refine extension-alias error format ([#30](https://github.com/web-infra-dev/rspack-resolver/pull/30))

## [1.12.0](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.11.0...oxc_resolver-v1.12.0) - 2024-09-25

### Added

- [**breaking**] mark `ResolveError` #[non_exhaustive] ([#252](https://github.com/oxc-project/oxc_resolver/pull/252))
- show tried extension aliases in `ResolveError::ExtensionAlias` ([#251](https://github.com/oxc-project/oxc_resolver/pull/251))
- give a specific error for matched alias not found ([#238](https://github.com/oxc-project/oxc_resolver/pull/238))

## [1.11.0](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.10.2...oxc_resolver-v1.11.0) - 2024-08-26

### Added
- use simdutf8 to validate UTF-8 when reading files ([#237](https://github.com/oxc-project/oxc_resolver/pull/237))
- Yarn PnP (behind a feature flag) ([#217](https://github.com/oxc-project/oxc_resolver/pull/217))

## [1.10.2](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.10.1...oxc_resolver-v1.10.2) - 2024-07-16

### Chore
- Release FreeBSD

## [1.10.1](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.10.0...oxc_resolver-v1.10.1) - 2024-07-15

### Fixed
- resolve module `ipaddr.js` correctly when `extensionAlias` is provided ([#228](https://github.com/oxc-project/oxc_resolver/pull/228))

## [1.10.0](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.9.4...oxc_resolver-v1.10.0) - 2024-07-11

### Added
- *(napi)* expose module type info in ResolveResult ([#223](https://github.com/oxc-project/oxc_resolver/pull/223))

### Fixed
- remove `#[cfg(target_os = "windows")]` logic in `canonicalize` ([#221](https://github.com/oxc-project/oxc_resolver/pull/221))

### Other
- update `cargo deny` ([#222](https://github.com/oxc-project/oxc_resolver/pull/222))
- pin crate-ci/typos version

## [1.9.4](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.9.3...oxc_resolver-v1.9.4) - 2024-07-10

### Other
- use custom canonicalize impl to avoid useless syscall ([#220](https://github.com/oxc-project/oxc_resolver/pull/220))
- add symlink fixtures ([#219](https://github.com/oxc-project/oxc_resolver/pull/219))

## [1.9.3](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v1.9.2...oxc_resolver-v1.9.3) - 2024-07-03

### Fixed
- tsconfig project reference it self should throw error ([#211](https://github.com/oxc-project/oxc-resolver/pull/211))

### Other
- *(napi)* make napi binary smaller with minimal tracing features ([#213](https://github.com/oxc-project/oxc-resolver/pull/213))
- *(napi)* remove tokio ([#212](https://github.com/oxc-project/oxc-resolver/pull/212))
- *(deps)* update rust crate dashmap to v6 ([#209](https://github.com/oxc-project/oxc-resolver/pull/209))

## [1.9.2](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.9.1...oxc_resolver-v1.9.2) - 2024-06-30

### Added
- *(napi)* add tracing via `OXC_LOG:DEBUG` ([#202](https://github.com/oxc-project/oxc_resolver/pull/202))

### Other
- document directory is an absolute path for `resolve(directory, specifier)` ([#206](https://github.com/oxc-project/oxc_resolver/pull/206))
- add a broken tsconfig test ([#205](https://github.com/oxc-project/oxc_resolver/pull/205))
- improve code coverage for src/error.rs ([#204](https://github.com/oxc-project/oxc_resolver/pull/204))
- skip resolving extension alias when `options.extension_alias` is empty ([#203](https://github.com/oxc-project/oxc_resolver/pull/203))
- add npm badge to crates.io

## [1.9.1](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.9.0...oxc_resolver-v1.9.1) - 2024-06-29

### Added
- strip symbols and enable LTO ([#197](https://github.com/oxc-project/oxc_resolver/pull/197))

### Other
- improve call to `Path::ends_with` ([#199](https://github.com/oxc-project/oxc_resolver/pull/199))
- list [profile.release] explicitly ([#198](https://github.com/oxc-project/oxc_resolver/pull/198))

## [1.9.0](https://github.com/oxc-project/oxc-resolver/compare/oxc_resolver-v1.8.4...oxc_resolver-v1.9.0) - 2024-06-28

### Added
- export package.json `type` and `sideEffects` field by default for bundlers ([#196](https://github.com/oxc-project/oxc-resolver/pull/196))

## [1.8.4](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.8.3...oxc_resolver-v1.8.4) - 2024-06-27

### Other
- skip searching for package.json when `alias_fields` is not provided ([#194](https://github.com/oxc-project/oxc_resolver/pull/194))

## [1.8.3](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.8.3...oxc_resolver-v1.8.2) - 2024-06-26

* *(napi*) release wasi build

## [1.8.2](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.8.2...oxc_resolver-v1.8.1) - 2024-06-24

### Added
- *(napi)* add async API ([#191](https://github.com/oxc-project/oxc_resolver/pull/191))

## [1.8.1](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.8.0...oxc_resolver-v1.8.1) - 2024-05-31

### Fixed
- alias value should try fragment as path ([#172](https://github.com/oxc-project/oxc_resolver/pull/172))

## [1.8.0](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.7.0...oxc_resolver-v1.8.0) - 2024-05-27

### Added
- [**breaking**] remove the constraint on packages exports `default` must be the last one ([#171](https://github.com/oxc-project/oxc_resolver/pull/171))
- [**breaking**] return `ResolveError:Builtin("node:{specifier}")` from package imports and exports ([#165](https://github.com/oxc-project/oxc_resolver/pull/165))

### Fixed
- alias not found should return error ([#168](https://github.com/oxc-project/oxc_resolver/pull/168))

### Other
- add panic test for extensions without a leading dot ([#150](https://github.com/oxc-project/oxc_resolver/pull/150))
- add test case for empty alias fields ([#149](https://github.com/oxc-project/oxc_resolver/pull/149))

## [1.7.0](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.6.7...oxc_resolver-v1.7.0) - 2024-04-24

### Added
- add `imports_fields` option ([#138](https://github.com/oxc-project/oxc_resolver/pull/138))
- substitute path that starts with `${configDir}/` in tsconfig.compilerOptions.paths ([#136](https://github.com/oxc-project/oxc_resolver/pull/136))

### Fixed
- RootsPlugin debug_assert on windows ([#145](https://github.com/oxc-project/oxc_resolver/pull/145))
- RootsPlugin should fall through if it fails to resolve the roots ([#144](https://github.com/oxc-project/oxc_resolver/pull/144))
- lazily read package.json.exports for shared resolvers ([#137](https://github.com/oxc-project/oxc_resolver/pull/137))

### Other
- remove `PartialEq` and `Eq` from `Specifier` ([#148](https://github.com/oxc-project/oxc_resolver/pull/148))
- add test case for tsconfig paths alias fall through ([#147](https://github.com/oxc-project/oxc_resolver/pull/147))
- use `cargo shear`
- fix test not failing the jobs property ([#146](https://github.com/oxc-project/oxc_resolver/pull/146))
- lazily read package.json.browser_fields for shared resolvers ([#142](https://github.com/oxc-project/oxc_resolver/pull/142))
- avoid an extra allocation in `load_extensions`
- ignore code coverage for `Display` on `ResolveOptions` ([#140](https://github.com/oxc-project/oxc_resolver/pull/140))
- remove the browser field lookup in `resolve_esm_match` ([#141](https://github.com/oxc-project/oxc_resolver/pull/141))
- remove the extra `condition_names` from `package_exports_resolve`

## [1.6.7](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.6.6...oxc_resolver-v1.6.7) - 2024-04-22

### Fixed
- incorrect resolution when using shared resolvers with different `main_fields` ([#134](https://github.com/oxc-project/oxc_resolver/pull/134))

## [1.6.6](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.6.5...oxc_resolver-v1.6.6) - 2024-04-22

### Other
- print resolve options while debug tracing ([#133](https://github.com/oxc-project/oxc_resolver/pull/133))

## [1.6.5](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.6.4...oxc_resolver-v1.6.5) - 2024-04-10

### Fixed
- canonicalize is not supported on wasi target ([#124](https://github.com/oxc-project/oxc_resolver/pull/124))

### Other
- document feature flags

## [1.6.4](https://github.com/oxc-project/oxc_resolver/compare/oxc_resolver-v1.6.3...oxc_resolver-v1.6.4) - 2024-03-29

### Docs

* improve terminology and clarify contexts
