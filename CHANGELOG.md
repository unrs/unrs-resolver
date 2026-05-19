# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.12.1](https://github.com/unrs/unrs-resolver/compare/v1.12.0...v1.12.1) - 2026-05-19

### <!-- 9 -->💼 Other

- Revert "perf: do not canonicalize the entry path ([#848](https://github.com/unrs/unrs-resolver/pull/848))" ([#204](https://github.com/unrs/unrs-resolver/pull/204)) (by @JounQin) - #204

### Contributors

* @JounQin

## [1.12.0](https://github.com/unrs/unrs-resolver/compare/v1.11.1...v1.12.0) - 2026-05-18

### <!-- 0 -->🚀 Features

- add `node_path` option to disable `NODE_PATH` env var behavior ([#1031](https://github.com/unrs/unrs-resolver/pull/1031)) (by @Boshen) - #202
- add DTS resolver matching TypeScript's bundler mode ([#997](https://github.com/unrs/unrs-resolver/pull/997)) (by @Boshen) - #202
- support NODE_PATH module lookup ([#1020](https://github.com/unrs/unrs-resolver/pull/1020)) (by @Boshen) - #202
- allow access to inner data in error related types ([#990](https://github.com/unrs/unrs-resolver/pull/990)) (by @sapphi-red) - #202
- allow subpath imports that start with #/ ([#907](https://github.com/unrs/unrs-resolver/pull/907)) (by @Boshen) - #202
- [**breaking**] disallow manually passing a list of references to `TsconfigOptions::references` ([#902](https://github.com/unrs/unrs-resolver/pull/902)) (by @Boshen) - #202
- support tsconfig `rootDirs` ([#885](https://github.com/unrs/unrs-resolver/pull/885)) (by @Boshen) - #202
- *(napi)* expose resolve_file API as resolveFileSync and resolveFileAsync ([#900](https://github.com/unrs/unrs-resolver/pull/900)) (by @Boshen)
- add `resolve_file` API for tsconfig auto discovery to work ([#860](https://github.com/unrs/unrs-resolver/pull/860)) (by @Boshen) - #202
- port tsconfck (find tsconfig files) ([#854](https://github.com/unrs/unrs-resolver/pull/854)) (by @Boshen) - #202
- add many.rs example for profiling resolver with many packages ([#836](https://github.com/unrs/unrs-resolver/pull/836)) (by @Boshen) - #202
- improve error message for empty package.json files ([#793](https://github.com/unrs/unrs-resolver/pull/793)) (by @Boshen) - #202
- improve PackagePathNotExported error message with condition names (by @Boshen) - #202
- add big-endian support for package.json parsing ([#768](https://github.com/unrs/unrs-resolver/pull/768)) (by @Boshen) - #202
- add tsconfig discovery ([#758](https://github.com/unrs/unrs-resolver/pull/758)) (by @Boshen) - #202
- add ESM file:// protocol support with comprehensive tests ([#746](https://github.com/unrs/unrs-resolver/pull/746)) (by @Boshen) - #202
- only resolve file:// protocol on windows ([#737](https://github.com/unrs/unrs-resolver/pull/737)) (by @Boshen) - #202
- add benchmark for package.json deserialization ([#698](https://github.com/unrs/unrs-resolver/pull/698)) (by @Boshen) - #202
- *(tsconfig)* support `files` / `include` / `exclude` ([#659](https://github.com/unrs/unrs-resolver/pull/659)) (by @shulaoda)
- feat(tsconfig) support `allowJs` in `compilerOptions` ([#658](https://github.com/unrs/unrs-resolver/pull/658)) (by @shulaoda) - #202
- *(tsconfig)* complete inheritance of `compilerOptions` fields ([#657](https://github.com/unrs/unrs-resolver/pull/657)) (by @shulaoda)
- support pass closure to restriction ([#604](https://github.com/unrs/unrs-resolver/pull/604)) (by @JounQin) - #202

### <!-- 1 -->🐛 Bug Fixes

- avoid panic in resolve_file for parentless paths ([#1053](https://github.com/unrs/unrs-resolver/pull/1053)) (by @Boshen) - #202
- *(dts)* strip ./ prefix from package entry when matching typesVersions ([#1051](https://github.com/unrs/unrs-resolver/pull/1051)) (by @Boshen)
- *(dts)* expand Declaration to TypeScript|Declaration for package entry resolution ([#1050](https://github.com/unrs/unrs-resolver/pull/1050)) (by @Boshen)
- *(dts)* prefer declaration extensions over JS in exports-resolved paths ([#1047](https://github.com/unrs/unrs-resolver/pull/1047)) (by @Boshen)
- avoid wasm/wasi dead-code lint in NodePath ([#1043](https://github.com/unrs/unrs-resolver/pull/1043)) (by @Boshen) - #202
- resolve normalized path on Windows even for `symlinks: false` ([#1036](https://github.com/unrs/unrs-resolver/pull/1036)) (by @sapphi-red) - #202
- skip unreadable tsconfig files during auto-discovery ([#1033](https://github.com/unrs/unrs-resolver/pull/1033)) (by @longlho) - #202
- remove invalid debug_assert for symlink + package.json ([#1032](https://github.com/unrs/unrs-resolver/pull/1032)) (by @Boshen) - #202
- resolve clippy unnecessary_unwrap warning ([#980](https://github.com/unrs/unrs-resolver/pull/980)) (by @Boshen) - #202
- prevent tsconfig cache pollution with separate raw and built caches ([#970](https://github.com/unrs/unrs-resolver/pull/970)) (by @Boshen) - #202
- use `/fixtures` path for WASI target (by @Boshen) - #202
- only recreate the cache when `yarn_pnp` is toggled ([#943](https://github.com/unrs/unrs-resolver/pull/943)) (by @Boshen) - #202
- resolve absolute path to package with trailing slash ([#942](https://github.com/unrs/unrs-resolver/pull/942)) (by @Boshen) - #202
- show yarn pnp errors as-is instead of NotFound error ([#939](https://github.com/unrs/unrs-resolver/pull/939)) (by @sapphi-red) - #202
- resolve solution tsconfig for auto discovered tsconfig ([#927](https://github.com/unrs/unrs-resolver/pull/927)) (by @Boshen) - #202
- fix `clone_with_options` + `yarn_pnp: true` not working ([#916](https://github.com/unrs/unrs-resolver/pull/916)) (by @sapphi-red) - #202
- fix resolution when resolving non-relative specifier on tsconfig baseUrl ([#903](https://github.com/unrs/unrs-resolver/pull/903)) (by @Boshen) - #202
- add fallback for statx NOSYS error on old kernels ([#901](https://github.com/unrs/unrs-resolver/pull/901)) (by @Boshen) - #202
- correct grammatical errors in documentation and comments ([#897](https://github.com/unrs/unrs-resolver/pull/897)) (by @Boshen) - #202
- resolve `node_modules/package/dir/foo.js` if `node_modules/package/dir/foo/` exists ([#896](https://github.com/unrs/unrs-resolver/pull/896)) (by @Boshen) - #202
- incorrect resolution when project reference extends a tsconfig without baseUrl ([#882](https://github.com/unrs/unrs-resolver/pull/882)) (by @Boshen) - #202
- store PathBuf with weak pointers to handle cache clearing ([#879](https://github.com/unrs/unrs-resolver/pull/879)) (by @Boshen) - #202
- apply `conditionNames: ['node', 'import']` when resolving tsconfig extends ([#869](https://github.com/unrs/unrs-resolver/pull/869)) (by @Boshen) - #202
- do not resolve to `node_modules/pacakge/index` ([#849](https://github.com/unrs/unrs-resolver/pull/849)) (by @Boshen) - #202
- use std::fs::canonicalize as a fallback when canonicalize fails ([#835](https://github.com/unrs/unrs-resolver/pull/835)) (by @Boshen) - #202
- remove AT_STATX_DONT_SYNC from statx calls ([#828](https://github.com/unrs/unrs-resolver/pull/828)) (by @Boshen) - #202
- *(package_json)* re-read file for serde_json fallback in simd implementation ([#808](https://github.com/unrs/unrs-resolver/pull/808)) (by @Boshen)
- revert system file cache optimization on Linux ([#810](https://github.com/unrs/unrs-resolver/pull/810)) (by @Brooooooklyn) - #202
- skip loading tsconfig.json from virtual module paths ([#809](https://github.com/unrs/unrs-resolver/pull/809)) (by @sapphi-red) - #202
- don't drop canonicalized path by cache clear ([#791](https://github.com/unrs/unrs-resolver/pull/791)) (by @sapphi-red) - #202
- derive Error for JSONError ([#779](https://github.com/unrs/unrs-resolver/pull/779)) (by @Boshen) - #202
- tsconfig paths should not be applied to paths inside node_modules ([#760](https://github.com/unrs/unrs-resolver/pull/760)) (by @Boshen) - #202
- ensure canonicalized paths remain accessible via strong references ([#733](https://github.com/unrs/unrs-resolver/pull/733)) (by @Boshen) - #202
- use `Weak` references for `CachedPath` to enable proper drop ([#727](https://github.com/unrs/unrs-resolver/pull/727)) (by @Boshen) - #202
- *(tsconfig)* respect Yarn PnP when resolving `extends` paths ([#656](https://github.com/unrs/unrs-resolver/pull/656)) (by @shulaoda)
- allow resolving `package?query#fragment` for packages with exports field ([#655](https://github.com/unrs/unrs-resolver/pull/655)) (by @sapphi-red) - #202
- support for resolving empty tsconfig file ([#602](https://github.com/unrs/unrs-resolver/pull/602)) (by @JounQin) - #202

### <!-- 2 -->🚜 Refactor

- remove clear_cache test that dynamically creates fixtures (by @Boshen) - #202
- move resolve and misc fixtures into fixtures/integration (by @Boshen) - #202
- replace ignored doctest with link to example (by @Boshen) - #202
- consolidate fixture directories for better test file mapping (by @Boshen) - #202
- replace `url` crate with `percent-encoding` ([#1065](https://github.com/unrs/unrs-resolver/pull/1065)) (by @Boshen) - #202
- clean up commented-out code in parse_package_specifier ([#1035](https://github.com/unrs/unrs-resolver/pull/1035)) (by @Boshen) - #202
- replace builtins.rs with nodejs-built-in-modules crate ([#940](https://github.com/unrs/unrs-resolver/pull/940)) (by @Boshen) - #202
- s/self.cache.as_ref()/&self.cache ([#910](https://github.com/unrs/unrs-resolver/pull/910)) (by @Boshen) - #202
- remove redundant PathBuf storage in CachedPath ([#891](https://github.com/unrs/unrs-resolver/pull/891)) (by @Boshen) - #202
- remove the redundant `inner_resolver` from `load_pnp` ([#862](https://github.com/unrs/unrs-resolver/pull/862)) (by @Boshen) - #202
- improve `Debug` and `Display` for `CachedPath` ([#861](https://github.com/unrs/unrs-resolver/pull/861)) (by @Boshen) - #202
- too_many_arguments = "allow" ([#863](https://github.com/unrs/unrs-resolver/pull/863)) (by @Boshen) - #202
- change Tsconfig::parse to accept owned string; add replace_bom_with_whitespace ([#859](https://github.com/unrs/unrs-resolver/pull/859)) (by @Boshen) - #202
- remove the useless getters and setters from `CompilerOptions` ([#858](https://github.com/unrs/unrs-resolver/pull/858)) (by @Boshen) - #202
- add `Tsconfig:references_resolved` ([#856](https://github.com/unrs/unrs-resolver/pull/856)) (by @Boshen) - #202
- move tsconfig resolution related code to its own file ([#855](https://github.com/unrs/unrs-resolver/pull/855)) (by @Boshen) - #202
- use RwLock<Vec<Arc<PackageJson>> for package.json storage ([#838](https://github.com/unrs/unrs-resolver/pull/838)) (by @Boshen) - #202
- do not store is_symlink in CachedPathImpl ([#850](https://github.com/unrs/unrs-resolver/pull/850)) (by @Boshen) - #202
- remove CachedPathImpl::canonicaling ([#834](https://github.com/unrs/unrs-resolver/pull/834)) (by @Boshen) - #202
- *(file_system)* deduplicate read methods and use Vec<u8> ([#816](https://github.com/unrs/unrs-resolver/pull/816)) (by @Boshen)
- use cfg_if and rustix in read_to_string_bypass_system_cache ([#802](https://github.com/unrs/unrs-resolver/pull/802)) (by @Boshen) - #202
- remove normalize-path dependency, use internal PathUtil ([#742](https://github.com/unrs/unrs-resolver/pull/742)) (by @Boshen) - #202
- remove a redundant path clone from PackageJson::parse ([#725](https://github.com/unrs/unrs-resolver/pull/725)) (by @Boshen) - #202
- split src/cache.rs into logical modules ([#714](https://github.com/unrs/unrs-resolver/pull/714)) (by @Boshen) - #202

### <!-- 3 -->📚 Documentation

- map npm package links to npmx.dev ([#1028](https://github.com/unrs/unrs-resolver/pull/1028)) (by @Boshen) - #202
- *(README.md)* update logo ([#968](https://github.com/unrs/unrs-resolver/pull/968)) (by @sapphi-red)

### <!-- 4 -->⚡ Performance

- remove unnecessary String clone in PnP resolution ([#1039](https://github.com/unrs/unrs-resolver/pull/1039)) (by @Boshen) - #202
- use byte operations instead of char iteration in specifier parser ([#1038](https://github.com/unrs/unrs-resolver/pull/1038)) (by @Boshen) - #202
- *(resolve)* reuse dot-prefixed subpath in exports/imports ([#1004](https://github.com/unrs/unrs-resolver/pull/1004)) (by @Boshen)
- *(cache)* remove package.json index arena indirection ([#1003](https://github.com/unrs/unrs-resolver/pull/1003)) (by @Boshen)
- *(tsconfig)* precompile wildcard path alias matcher ([#1001](https://github.com/unrs/unrs-resolver/pull/1001)) (by @Boshen)
- precompile alias match metadata in resolver hot path ([#999](https://github.com/unrs/unrs-resolver/pull/999)) (by @Boshen) - #202
- remove an allocation from `CachedPath::module_directory` ([#880](https://github.com/unrs/unrs-resolver/pull/880)) (by @Boshen) - #202
- skip searching for node_modules/@scope/package.json ([#876](https://github.com/unrs/unrs-resolver/pull/876)) (by @Boshen) - #202
- remove the redundant `node_modules/package/index` cache value ([#875](https://github.com/unrs/unrs-resolver/pull/875)) (by @Boshen) - #202
- cache all package.json resolutions for faster package.json lookup ([#853](https://github.com/unrs/unrs-resolver/pull/853)) (by @Boshen) - #202
- do not canonicalize the entry path ([#848](https://github.com/unrs/unrs-resolver/pull/848)) (by @Boshen) - #202
- remove Result from `CachedPathImpl::canonicalized` ([#847](https://github.com/unrs/unrs-resolver/pull/847)) (by @Boshen) - #202
- fast path for node_modules/package ([#839](https://github.com/unrs/unrs-resolver/pull/839)) (by @Boshen) - #202
- cache canonicalization results at every recursion level ([#843](https://github.com/unrs/unrs-resolver/pull/843)) (by @Boshen) - #202
- use IdentityHasher for visited set to avoid double hashing ([#837](https://github.com/unrs/unrs-resolver/pull/837)) (by @Boshen) - #202
- optimize FileSystem metadata operations with rustix ([#800](https://github.com/unrs/unrs-resolver/pull/800)) (by @Boshen) - #202
- use simd-json for package.json parsing ([#761](https://github.com/unrs/unrs-resolver/pull/761)) (by @Boshen) - #202
- make url crate optional for wasm32 targets (by @Boshen) - #202
- mark error path functions as #[cold] for better optimization ([#729](https://github.com/unrs/unrs-resolver/pull/729)) (by @Boshen) - #202
- bypass file system read cache if memory cache is available ([#707](https://github.com/unrs/unrs-resolver/pull/707)) (by @Brooooooklyn) - #202
- use `memmap` to speed up file reading ([#696](https://github.com/unrs/unrs-resolver/pull/696)) (by @Boshen) - #202
- use `GetFileAttributesExW` for symlink metadata lookup on Windows ([#691](https://github.com/unrs/unrs-resolver/pull/691)) (by @sapphi-red) - #202
- improve `pattern_key_compare` ([#639](https://github.com/unrs/unrs-resolver/pull/639)) (by @Boshen) - #202
- most specifiers don't have escaped characters ([#636](https://github.com/unrs/unrs-resolver/pull/636)) (by @Boshen) - #202

### <!-- 6 -->🧪 Testing

- add 28 tests to improve coverage (92% → 93%) ([#1082](https://github.com/unrs/unrs-resolver/pull/1082)) (by @Boshen) - #202
- use CARGO_MANIFEST_DIR instead `env::current_dir` (by @Boshen) - #202
- Add a test to ensure NODEJS_BUILTINS is alphabetized. ([#926](https://github.com/unrs/unrs-resolver/pull/926)) (by @connorshea) - #202
- add test cases for package imports starting with `#` or `#/` ([#905](https://github.com/unrs/unrs-resolver/pull/905)) (by @Boshen) - #202
- change all fixture directory names to dashed case ([#884](https://github.com/unrs/unrs-resolver/pull/884)) (by @Boshen) - #202
- add a tsconfig extend not found case ([#763](https://github.com/unrs/unrs-resolver/pull/763)) (by @Boshen) - #202
- improve test coverage for edge cases ([#740](https://github.com/unrs/unrs-resolver/pull/740)) (by @Boshen) - #202
- improve coverage for check_restrictions ([#739](https://github.com/unrs/unrs-resolver/pull/739)) (by @Boshen) - #202
- add memory leak test ([#726](https://github.com/unrs/unrs-resolver/pull/726)) (by @Boshen) - #202
- enable Windows global pnp case ([#703](https://github.com/unrs/unrs-resolver/pull/703)) (by @JounQin) - #202
- *(tsconfig)* tweak jsx `extends` tests ([#666](https://github.com/unrs/unrs-resolver/pull/666)) (by @shulaoda)
- make tests pass on Windows ([#654](https://github.com/unrs/unrs-resolver/pull/654)) (by @sapphi-red) - #202

### <!-- 9 -->💼 Other

- Merge remote-tracking branch 'upstream/main' into chore/merge_upstream (by @JounQin) - #202
- add tsconfig paths alias scalability benchmark ([#1002](https://github.com/unrs/unrs-resolver/pull/1002)) (by @Boshen) - #202
- Revert "perf: use `memmap` to speed up file reading" ([#701](https://github.com/unrs/unrs-resolver/pull/701)) (by @Boshen) - #202
- Add comprehensive tests for tsconfig extends functionality ([#660](https://github.com/unrs/unrs-resolver/pull/660)) (by @Copilot) - #202
- Expose the `ExtendsField` enum of TsConfig ([#607](https://github.com/unrs/unrs-resolver/pull/607)) (by @ostenbom) - #202

### Contributors

* @JounQin
* @renovate[bot]
* @Boshen
* @sapphi-red
* @longlho
* @leegeunhyeok
* @connorshea
* @Brooooooklyn
* @oxc-bot
* @shulaoda
* @Copilot
* @ostenbom

## [1.11.1](https://github.com/unrs/unrs-resolver/compare/v1.11.0...v1.11.1) - 2025-07-09

### <!-- 3 -->📚 Documentation

- add missing `descriptionFiles` option introduction (by @JounQin)

### Contributors

* @JounQin

## [1.11.0](https://github.com/unrs/unrs-resolver/compare/v1.10.1...v1.11.0) - 2025-07-06

### <!-- 0 -->🚀 Features

- return proper errors when failed to find or read yarn pnp manifest ([#590](https://github.com/unrs/unrs-resolver/pull/590)) (by @Boshen) - #178
- add `yarn_pnp` logic to `FileSystem` ([#589](https://github.com/unrs/unrs-resolver/pull/589)) (by @Boshen) - #178
- *(resolver)* rework yarn manifest file look up ([#586](https://github.com/unrs/unrs-resolver/pull/586)) (by @Boshen)

### <!-- 2 -->🚜 Refactor

- remove `fs_cache` feature flag ([#588](https://github.com/unrs/unrs-resolver/pull/588)) (by @Boshen) - #178

### <!-- 9 -->💼 Other

- Merge remote-tracking branch 'upstream/main' into chore/merge_upstream (by @JounQin) - #178

### Contributors

* @JounQin
* @oxc-bot
* @Boshen

## [1.10.1](https://github.com/unrs/unrs-resolver/compare/v1.10.0...v1.10.1) - 2025-07-02

### <!-- 1 -->🐛 Bug Fixes

- fix: incorrect package.json relative path ([#176](https://github.com/unrs/unrs-resolver/pull/176)) (by @JounQin) - #176

### Contributors

* @JounQin

## [1.10.0](https://github.com/unrs/unrs-resolver/compare/v1.9.2...v1.10.0) - 2025-07-02

### <!-- 1 -->🐛 Bug Fixes

- support resolving abnormal relative paths with `node_modules` ([#171](https://github.com/unrs/unrs-resolver/pull/171)) (by @JounQin) - #171
- *(deps)* update all dependencies ([#168](https://github.com/unrs/unrs-resolver/pull/168)) (by @renovate[bot])

### <!-- 9 -->💼 Other

- Merge remote-tracking branch 'upstream/main' into chore/merge_upstream (by @JounQin) - #174

### Contributors

* @JounQin
* @renovate[bot]

## [1.9.2](https://github.com/unrs/unrs-resolver/compare/v1.9.1...v1.9.2) - 2025-06-24

### <!-- 1 -->Bug Fixes

- requiring `.` or `./` should respect `mainFiles` option ([#163](https://github.com/unrs/unrs-resolver/pull/163)) (by @JounQin) - #163

### <!-- 3 -->Documentation

- update `alias` and `fallback` options type and description ([#161](https://github.com/unrs/unrs-resolver/pull/161)) (by @JounQin) - #161

### <!-- 7 -->Chore

- *(deps)* update all dependencies ([#164](https://github.com/unrs/unrs-resolver/pull/164)) (by @renovate[bot])

### Contributors

* @renovate[bot]
* @JounQin

## [1.9.1](https://github.com/unrs/unrs-resolver/compare/v1.9.0...v1.9.1) - 2025-06-20

### <!-- 1 -->Bug Fixes

- *(deps)* update all dependencies ([#157](https://github.com/unrs/unrs-resolver/pull/157)) (by @renovate[bot])

### Contributors

* @renovate[bot]

## [1.9.0](https://github.com/unrs/unrs-resolver/compare/v1.8.1...v1.9.0) - 2025-06-11

### <!-- 0 -->Features

- add `file:` protocol path support ([#151](https://github.com/unrs/unrs-resolver/pull/151)) (by @JounQin) - #151
- add two last android targets ([#154](https://github.com/unrs/unrs-resolver/pull/154)) (by @JounQin) - #154

### <!-- 1 -->Bug Fixes

- try different approach to support web container out of box ([#150](https://github.com/unrs/unrs-resolver/pull/150)) (by @JounQin) - #150

### Contributors

* @JounQin

## [1.8.1](https://github.com/unrs/unrs-resolver/compare/v1.8.0...v1.8.1) - 2025-06-11

### <!-- 1 -->Bug Fixes

- keep `napi` property in `package.json` for `napi-postinstall` ([#148](https://github.com/unrs/unrs-resolver/pull/148)) (by @JounQin)

### Contributors

* @JounQin

## [1.8.0](https://github.com/unrs/unrs-resolver/compare/v1.7.13...v1.8.0) - 2025-06-11

### <!-- 0 -->Features

- support runtime fallback for webcontainer ([#144](https://github.com/unrs/unrs-resolver/pull/144)) (by @JounQin) - #144
- merge from upstream `oxc-project/oxc-resolver` - 6th ([#146](https://github.com/unrs/unrs-resolver/pull/146)) (by @JounQin) - #146

### <!-- 7 -->Chore

- *(deps)* update all dependencies ([#141](https://github.com/unrs/unrs-resolver/pull/141)) (by @renovate[bot])

### Contributors

* @JounQin
* @renovate[bot]

## [1.7.13](https://github.com/unrs/unrs-resolver/compare/v1.7.12...v1.7.13) - 2025-06-10

### <!-- 1 -->Bug Fixes

- support extends in referenced project ([#142](https://github.com/unrs/unrs-resolver/pull/142))

## [1.7.12](https://github.com/unrs/unrs-resolver/compare/v1.7.11...v1.7.12) - 2025-06-09

### <!-- 1 -->Bug Fixes

- npm protocol alias with pnp should be supported ([#139](https://github.com/unrs/unrs-resolver/pull/139))

## [1.7.11](https://github.com/unrs/unrs-resolver/compare/v1.7.10...v1.7.11) - 2025-06-05

### <!-- 1 -->Bug Fixes

- remove invalid file cache due to extension alias ([#136](https://github.com/unrs/unrs-resolver/pull/136))

## [1.7.10](https://github.com/unrs/unrs-resolver/compare/v1.7.9...v1.7.10) - 2025-06-05

### <!-- 1 -->Bug Fixes

- Revert "fix: custom `condition_names` should take higher priority than target in package.json ([#115](https://github.com/unrs/unrs-resolver/pull/115))" ([#131](https://github.com/unrs/unrs-resolver/pull/131))

### Other

- chore: bump napi v3.0.0-beta.7 ([#133](https://github.com/unrs/unrs-resolver/pull/133))

## [1.7.9](https://github.com/unrs/unrs-resolver/compare/v1.7.8...v1.7.9) - 2025-06-03

### <!-- 1 -->Bug Fixes

- pnp global cache should be supported ([#129](https://github.com/unrs/unrs-resolver/pull/129))

  Windows global cache is still unsupported due to upstream

  see also <https://github.com/yarnpkg/pnp-rs/pull/10>

## [1.7.8](https://github.com/unrs/unrs-resolver/compare/v1.7.7...v1.7.8) - 2025-05-29

### <!-- 1 -->Bug Fixes

- resolve symlink with nested `node_modules` ([#125](https://github.com/unrs/unrs-resolver/pull/125))

## [1.7.7](https://github.com/unrs/unrs-resolver/compare/v1.7.6...v1.7.7) - 2025-05-29

### <!-- 1 -->Bug Fixes

- resolve dir index with dot specifier correctly ([#123](https://github.com/unrs/unrs-resolver/pull/123))

## [1.7.6](https://github.com/unrs/unrs-resolver/compare/v1.7.5...v1.7.6) - 2025-05-28

### <!-- 1 -->Bug Fixes

- prefer index over current file for `.` and `./` ([#121](https://github.com/unrs/unrs-resolver/pull/121))

## [1.7.5](https://github.com/unrs/unrs-resolver/compare/v1.7.4...v1.7.5) - 2025-05-28

### <!-- 1 -->Bug Fixes

- should try package exports first per spec ([#118](https://github.com/unrs/unrs-resolver/pull/118))

## [1.7.4](https://github.com/unrs/unrs-resolver/compare/v1.7.3...v1.7.4) - 2025-05-28

### <!-- 1 -->Bug Fixes

- prefer file over package dir in `node_modules` ([#116](https://github.com/unrs/unrs-resolver/pull/116))

## [1.7.3](https://github.com/unrs/unrs-resolver/compare/v1.7.2...v1.7.3) - 2025-05-28

### <!-- 1 -->Bug Fixes

- custom `condition_names` should take higher priority than target in package.json ([#115](https://github.com/unrs/unrs-resolver/pull/115))

## [1.7.2](https://github.com/unrs/unrs-resolver/compare/v1.7.1...v1.7.2) - 2025-04-27

### <!-- 1 -->Bug Fixes

- bump `napi-postinstall` to fix `yarn` pnp compatibility issue ([#106](https://github.com/unrs/unrs-resolver/pull/106))

## [1.7.1](https://github.com/unrs/unrs-resolver/compare/v1.7.0...v1.7.1) - 2025-04-26

### Chore

- bump `napi-postinstall` to support `yarn`/`pnpm` on `webcontainer` ([#103](https://github.com/unrs/unrs-resolver/pull/103))

### <!-- 6 -->Testing

- add case for #65 ([#100](https://github.com/unrs/unrs-resolver/pull/100))

## [1.7.0](https://github.com/unrs/unrs-resolver/compare/v1.6.6...v1.7.0) - 2025-04-24

### <!-- 0 -->Features

- enable `no_opt_arch` flag for `mimalloc-safe` on `linux-aarch64` ([#98](https://github.com/unrs/unrs-resolver/pull/98))

## [1.6.6](https://github.com/unrs/unrs-resolver/compare/v1.6.5...v1.6.6) - 2025-04-23

### <!-- 0 -->Features

- add new target `riscv64gc-unknown-linux-musl` ([#96](https://github.com/unrs/unrs-resolver/pull/96))

## [1.6.5](https://github.com/unrs/unrs-resolver/compare/v1.6.4...v1.6.5) - 2025-04-23

### <!-- 1 -->Bug Fixes

- rework on handling DOS device paths on Windows ([#84](https://github.com/unrs/unrs-resolver/pull/84))
- handle package.json and tsconfig.json with BOM ([#463](https://github.com/oxc-project/oxc-resolver/pull/463))

### <!-- 2 -->Performance

- avoid double call to `parse_package_specifier` ([#465](https://github.com/oxc-project/oxc-resolver/pull/465))

### <!-- 3 -->Documentation

- add more details about the changes in this fork ([#92](https://github.com/unrs/unrs-resolver/pull/92))

## [1.6.4](https://github.com/unrs/unrs-resolver/compare/v1.6.3...v1.6.4) - 2025-04-22

### <!-- 1 -->Bug Fixes

- properly handle DOS device paths in strip_windows_prefix ([#455](https://github.com/oxc-project/oxc-resolver/pull/455))

## [1.6.3](https://github.com/unrs/unrs-resolver/compare/v1.6.2...v1.6.3) - 2025-04-21

### <!-- 1 -->Bug Fixes

- support `load_as_directory` for pnp mode ([#75](https://github.com/unrs/unrs-resolver/pull/75))

### <!-- 6 -->Testing

- add case for import-js/eslint-import-resolver-typescript#429 ([#76](https://github.com/unrs/unrs-resolver/pull/76))

## [1.6.2](https://github.com/unrs/unrs-resolver/compare/v1.6.1...v1.6.2) - 2025-04-21

### <!-- 1 -->Bug Fixes

- resolve parent base url correctly by normalizing as absolute path ([#72](https://github.com/unrs/unrs-resolver/pull/72))

## [1.6.1](https://github.com/unrs/unrs-resolver/compare/v1.6.0...v1.6.1) - 2025-04-20

### <!-- 1 -->Bug Fixes

- disable `mimalloc` on linux with aarch64 ([#69](https://github.com/unrs/unrs-resolver/pull/69))

## [1.6.0](https://github.com/unrs/unrs-resolver/compare/v1.5.0...v1.6.0) - 2025-04-20

### <!-- 0 -->Features

- deserialize `preserve_value_imports` and `imports_not_used_as_values` from `compilerOptions` ([#457](https://github.com/oxc-project/oxc-resolver/pull/457))
- deserialize `target` from `compilerOptions` ([#456](https://github.com/oxc-project/oxc-resolver/pull/456))

### <!-- 1 -->Bug Fixes

- add `napi-postinstall` dep for workaround `npm`'s bug ([#66](https://github.com/unrs/unrs-resolver/pull/66))

## [1.5.0](https://github.com/unrs/unrs-resolver/compare/v1.4.1...v1.5.0) - 2025-04-11

### <!-- 1 -->Bug Fixes

- resolve `${configDir}` in tsconfig `compilerOptions.baseUrl` ([#450](https://github.com/oxc-project/oxc-resolver/pull/450))

## [1.4.1](https://github.com/unrs/unrs-resolver/compare/v1.4.0...v1.4.1) - 2025-04-07

### <!-- 4 -->Refactor

- remove unnecessary checks for query ([#53](https://github.com/unrs/unrs-resolver/pull/53))

## [1.4.0](https://github.com/unrs/unrs-resolver/compare/v1.3.3...v1.4.0) - 2025-04-06

### <!-- 0 -->Features

- handle query and fragment in pacakge.json `exports` and `imports` field ([#443](https://github.com/oxc-project/oxc-resolver/pull/443))
- resolve emitDecoratorMetadata in tsconfig ([#439](https://github.com/oxc-project/oxc-resolver/pull/439))
- _(napi)_ add mimalloc ([#423](https://github.com/oxc-project/oxc-resolver/pull/423))
- [**breaking**] Rust Edition 2024 ([#402](https://github.com/oxc-project/oxc-resolver/pull/402))
- deserialize `verbatim_module_syntax` from compilerOptions ([#411](https://github.com/oxc-project/oxc-resolver/pull/411))
- support wildcard `*` in alias plugin ([#388](https://github.com/oxc-project/oxc-resolver/pull/388))
- merge options from extends tsconfig.json ([#375](https://github.com/oxc-project/oxc-resolver/pull/375))
- add more fields in tsconfig#CompilerOptionsSerde ([#374](https://github.com/oxc-project/oxc-resolver/pull/374))
- [**breaking**] generic fs cache `type Resolver = ResolverGeneric<FsCache<FileSystemOs>>` ([#358](https://github.com/oxc-project/oxc-resolver/pull/358))
- [**breaking**] replace `FileSystem::canonicalize` with `FileSystem::read_link` ([#331](https://github.com/oxc-project/oxc-resolver/pull/331))
- faster and stable path hash for the cache ([#328](https://github.com/oxc-project/oxc-resolver/pull/328))
- add `Resolver::resolve_tsconfig` API ([#312](https://github.com/oxc-project/oxc-resolver/pull/312))
- [**breaking**] add `ResolveError::Builtin::prefixed_with_node_colon` ([#272](https://github.com/oxc-project/oxc-resolver/pull/272))
- [**breaking**] mark `ResolveError` #[non_exhaustive] ([#252](https://github.com/oxc-project/oxc-resolver/pull/252))
- show tried extension aliases in `ResolveError::ExtensionAlias` ([#251](https://github.com/oxc-project/oxc-resolver/pull/251))
- give a specific error for matched alias not found ([#238](https://github.com/oxc-project/oxc-resolver/pull/238))
- Yarn PnP ([#217](https://github.com/oxc-project/oxc-resolver/pull/217))

### <!-- 1 -->Bug Fixes

- handle query and fragment in package.json `exports` and `imports` field ([#443](https://github.com/oxc-project/oxc-resolver/pull/443))
- fix bench
- try browsers field and alias before resolving directory in node_modules ([#349](https://github.com/oxc-project/oxc-resolver/pull/349))
- special case for aliasing `@/` ([#348](https://github.com/oxc-project/oxc-resolver/pull/348))
- normalize resolved result on Windows for root ([#345](https://github.com/oxc-project/oxc-resolver/pull/345))
- don't panic when resolving `/` with `roots` ([#310](https://github.com/oxc-project/oxc-resolver/pull/310))
- use same UNC path normalization logic with libuv ([#306](https://github.com/oxc-project/oxc-resolver/pull/306))
- use `fs::canonicalize` to cover symlink edge cases ([#284](https://github.com/oxc-project/oxc-resolver/pull/284))
- extensionAlias cannot resolve mathjs ([#273](https://github.com/oxc-project/oxc-resolver/pull/273))
- resolve module `ipaddr.js` correctly when `extensionAlias` is provided ([#228](https://github.com/oxc-project/oxc-resolver/pull/228))
- _(napi)_ update buggy NAPI-RS versions ([#225](https://github.com/oxc-project/oxc-resolver/pull/225))
- remove `#[cfg(target_os = "windows")]` logic in `canonicalize` ([#221](https://github.com/oxc-project/oxc-resolver/pull/221))

### <!-- 2 -->Performance

- use papaya instead of dashmap ([#356](https://github.com/oxc-project/oxc-resolver/pull/356))
- try directory first in `node_modules` ([#340](https://github.com/oxc-project/oxc-resolver/pull/340))
- guard `load_alias` on hot path ([#339](https://github.com/oxc-project/oxc-resolver/pull/339))
- use `as_os_str` for `Hash` and `PartialEq` operations ([#338](https://github.com/oxc-project/oxc-resolver/pull/338))
- reduce hash while resolving package.json ([#319](https://github.com/oxc-project/oxc-resolver/pull/319))
- reduce memory allocation while normalizing package path ([#318](https://github.com/oxc-project/oxc-resolver/pull/318))
- reduce memory allocation while resolving package.json ([#317](https://github.com/oxc-project/oxc-resolver/pull/317))
- use `path.as_os_str().hash()` instead of `path.hash()` ([#316](https://github.com/oxc-project/oxc-resolver/pull/316))
- reduce memory allocation by using a thread_local path for path methods ([#315](https://github.com/oxc-project/oxc-resolver/pull/315))
- bring back the symlink optimization ([#298](https://github.com/oxc-project/oxc-resolver/pull/298))
- use simdutf8 to validate UTF-8 when reading files ([#237](https://github.com/oxc-project/oxc-resolver/pull/237))
- use custom canonicalize impl to avoid useless syscall ([#220](https://github.com/oxc-project/oxc-resolver/pull/220))

### <!-- 3 -->Documentation

- fix an incorrect comment on `Context::missing_dependencies`
- mention extension must start with a `.` in `with_extension` ([#313](https://github.com/oxc-project/oxc-resolver/pull/313))
- _(README)_ should be `new ResolverFactory`

### <!-- 4 -->Refactor

- remove papaya `.collector(seize::Collector::new())` call ([#393](https://github.com/oxc-project/oxc-resolver/pull/393))
- replace UnsafeCell with RefCell ([#346](https://github.com/oxc-project/oxc-resolver/pull/346))
- run clippy with `--all-targets` ([#333](https://github.com/oxc-project/oxc-resolver/pull/333))
- apply latest `cargo +nightly fmt` ([#281](https://github.com/oxc-project/oxc-resolver/pull/281))
- add more clippy fixes ([#279](https://github.com/oxc-project/oxc-resolver/pull/279))
- clean up elided lifetimes ([#277](https://github.com/oxc-project/oxc-resolver/pull/277))

### <!-- 6 -->Testing

- fix warning on Windows
- fix symlink test init on Windows ([#307](https://github.com/oxc-project/oxc-resolver/pull/307))

## [1.3.3](https://github.com/unrs/unrs-resolver/compare/v1.3.2...v1.3.3) - 2025-03-29

### Build

- build: remove `--strip` flag ([#44](https://github.com/unrs/unrs-resolver/pull/44))

### <!-- 6 -->Testing

- add nested package json case ([#40](https://github.com/unrs/unrs-resolver/pull/40))

## [1.3.2](https://github.com/unrs/unrs-resolver/compare/unrs_resolver-v1.3.1...v1.3.2) - 2025-03-26

### <!-- 1 -->Bug Fixes

- absolute path aliasing should not be skipped ([#37](https://github.com/unrs/unrs-resolver/pull/37))

## [1.3.1](https://github.com/unrs/unrs-resolver/compare/unrs_resolver-v1.3.0...unrs_resolver-v1.3.1) - 2025-03-26

### Other

- bump all (dev) deps ([#34](https://github.com/unrs/unrs-resolver/pull/34))

## [1.3.0](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.2.2...unrs_resolver-v1.3.0) - 2025-03-26

### <!-- 0 -->Features

- enable more targets ([#29](https://github.com/unrs/unrs-resolver/pull/29) and [#32](https://github.com/unrs/unrs-resolver/pull/32))

## [1.2.2](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.2.1...unrspack-resolver-v1.2.2) - 2025-03-19

### <!-- 1 -->Bug Fixes

- _(pnp)_ support `pnpapi` core module and package deep link ([#24](https://github.com/unrs/unrs-resolver/pull/24))

## [1.2.0](https://github.com/unrs/unrs-resolver/compare/unrspack-resolver-v1.1.2...unrspack-resolver-v2.0.0) - 2025-03-18

### <!-- 0 -->Features

- _(napi)_ add mimalloc ([#423](https://github.com/unrs/unrs-resolver/pull/423)) ([#18](https://github.com/unrs/unrs-resolver/pull/18))
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

## Old Changelog for `oxc-resolver`

[CHANGELOG_OLD](CHANGELOG_OLD.md)
