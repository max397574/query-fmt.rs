# Changelog

## [1.6.0](https://github.com/max397574/query-fmt.rs/compare/v1.5.1...v1.6.0) (2023-02-22)

### Features

- **args:** add option to not print filename ([c1a0465](https://github.com/max397574/query-fmt.rs/commit/c1a0465a35c968de8eb4d3b63c0089f6048abcaa))
- extract config into `Config` struct ([daf94fb](https://github.com/max397574/query-fmt.rs/commit/daf94fbb71ccbfda61af6db0a0aecfb21374c69e))

## [1.5.1](https://github.com/max397574/query-fmt.rs/compare/v1.5.0...v1.5.1) (2023-02-13)

### Bug Fixes

- **format:** don't add empty lines after commands and small refactors ([3b90028](https://github.com/max397574/query-fmt.rs/commit/3b90028160426636b0459a5ec365196f8fb3dd17))

## [1.5.0](https://github.com/max397574/query-fmt.rs/compare/v1.4.1...v1.5.0) (2023-02-13)

### Features

- add config values for (list) indent ([f0dfb10](https://github.com/max397574/query-fmt.rs/commit/f0dfb10727865503155bd10080a653c8b4237b68))

### Bug Fixes

- remove not needed references ([eaaa6ab](https://github.com/max397574/query-fmt.rs/commit/eaaa6abc98baf7d682f8ce442fee845f612cb1c2))

## [1.4.1](https://github.com/max397574/query-fmt.rs/compare/v1.4.0...v1.4.1) (2023-02-03)

### Bug Fixes

- fix lists with named nodes ([2c2ab5c](https://github.com/max397574/query-fmt.rs/commit/2c2ab5c18afcceb7b2a6bc8aa4492d2dfff8f699))
- make it work with escape sequences ([b577bb2](https://github.com/max397574/query-fmt.rs/commit/b577bb25c822d397b5d243641b108ed51b24b7e9))

## [1.4.0](https://github.com/max397574/query-fmt.rs/compare/v1.3.0...v1.4.0) (2023-01-31)

### Features

- actually modify the file ([6ada11b](https://github.com/max397574/query-fmt.rs/commit/6ada11b0e35af1df9e42aa24a04de167d43025d4))
- added preview option ([a821e3f](https://github.com/max397574/query-fmt.rs/commit/a821e3f2dbfb11b0ea328724a226173245241718))
- allow whole directories to be formatted ([daa8f4a](https://github.com/max397574/query-fmt.rs/commit/daa8f4a0ea00ddfccf4dd8c83d37a051be233a90))
- only indent lists with one space ([988c2e3](https://github.com/max397574/query-fmt.rs/commit/988c2e3634a321236e3f03fc2523339bfb9e2a95))

### Bug Fixes

- trim text ([7cf3ee1](https://github.com/max397574/query-fmt.rs/commit/7cf3ee1a2e5625ff6b9a26a0674f1c3f76e76300))

## [1.3.0](https://github.com/max397574/query-fmt.rs/compare/v1.2.0...v1.3.0) (2023-01-09)

### Features

- **ci:** added build workflow ([e4ade34](https://github.com/max397574/query-fmt.rs/commit/e4ade34bd588e70162af0793b1cc5e92f4f9dc3e))
- **ci:** use latest checkout action ([0865b9d](https://github.com/max397574/query-fmt.rs/commit/0865b9d121b2ef035c410ef445342e648ca885c3))
- start with config module ([1811d67](https://github.com/max397574/query-fmt.rs/commit/1811d671ddc13607fa9382b943e089fd9fdd19e3))

### Bug Fixes

- **ci:** add stuff to `cargo.toml` ([d9d5c3b](https://github.com/max397574/query-fmt.rs/commit/d9d5c3b0185e97896cc3382f6a27a265808b2853))
- **ci:** checkout submodules ([207b12d](https://github.com/max397574/query-fmt.rs/commit/207b12df5cbc3754f509974ac9afb72604549c48))
- **ci:** remove windows build for now ([aa942f7](https://github.com/max397574/query-fmt.rs/commit/aa942f7b06072b9b8837fcb3c47d03ffe639e28c))
- correctly format comments ([5b45802](https://github.com/max397574/query-fmt.rs/commit/5b45802aa0ffb1d732a2e122171c553fe590e575))
- **windows ci:** add missing file ([84e0f35](https://github.com/max397574/query-fmt.rs/commit/84e0f3561637a8ed225a87215ace21a8bbe79581))

## [1.2.0](https://github.com/max397574/query-fmt.rs/compare/v1.1.0...v1.2.0) (2023-01-08)

### Features

- allow passing arguments ([82715b2](https://github.com/max397574/query-fmt.rs/commit/82715b22c9545102e9b1d2f8edb612eb5f2473a0))
- take input from file ([189e70a](https://github.com/max397574/query-fmt.rs/commit/189e70aac608b0604bf6503fdfcaac64b1279835))

### Bug Fixes

- include folder in cargo.toml ([5a81b35](https://github.com/max397574/query-fmt.rs/commit/5a81b3591c4b782ba62fc67a124b704f41d89a18))

## [1.1.0](https://github.com/max397574/query-fmt.rs/compare/v1.0.0...v1.1.0) (2023-01-08)

### Features

- add space before captures ([dbb20dd](https://github.com/max397574/query-fmt.rs/commit/dbb20dd6b382566edcb64159914606efc5a2680c))
- add support for lists ([ac5effd](https://github.com/max397574/query-fmt.rs/commit/ac5effdf28998e7e1e9fcc97f9f74f97ad23f21c))
- better newlines and indent ([67ce1db](https://github.com/max397574/query-fmt.rs/commit/67ce1db6febce471ca628c195edd89d202e908ca))
- some more edge cases with anonymous nodes ([e65cad1](https://github.com/max397574/query-fmt.rs/commit/e65cad10edb497ff86e461603a4b4ebe5e2d43b4))

### Bug Fixes

- double space in predicate with capture ([7407151](https://github.com/max397574/query-fmt.rs/commit/7407151d54a924b600f5d228d6b5552b29611f4c))

## 1.0.0 (2023-01-07)

### ⚠ BREAKING CHANGES

- completely rewrite and start first formatting

### Features

- add space in predicates ([c094d37](https://github.com/max397574/query-fmt.rs/commit/c094d37dbbc5464d07f5012cf5036e13603f6393))
- completely rewrite and start first formatting ([6d2ad85](https://github.com/max397574/query-fmt.rs/commit/6d2ad85ff0074ea7ec0e7d06dc4fc95226857155))
- more complex input ([d0268c5](https://github.com/max397574/query-fmt.rs/commit/d0268c5e19a4d83bbd331b0dc5cd958828e59ded))
- newline before predicates ([5b911aa](https://github.com/max397574/query-fmt.rs/commit/5b911aa6165288091be498f4e7301ebe1d3dda09))

### Bug Fixes

- **ci:** add `jobs:` ([eb9f83b](https://github.com/max397574/query-fmt.rs/commit/eb9f83bb41ed2d22cb12ed6d4fcc082ff543a893))
- **ci:** remove requirements ([1f951a7](https://github.com/max397574/query-fmt.rs/commit/1f951a79f4010e56192a566d5f10df730c879b55))
- **ci:** update package-name ([78b3d2d](https://github.com/max397574/query-fmt.rs/commit/78b3d2dd77277fc5304ffa25aa10f0c306a5d043))
