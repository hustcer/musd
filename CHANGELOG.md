# Changelog
All notable changes to this project will be documented in this file.

## [0.2.3] - 2022-05-25

### Features

- Try to do the Github release task by nushell
- Use nushell instead of bash to do the release task
- Add windows msi release package

### Miscellaneous Tasks

- Improve release-pkg script

## [0.2.1] - 2022-05-24

### Features

- Add download m4a format music file support
- Add x86_64-unknown-linux-gnu release target
- Add aarch64-unknown-linux-gnu binary release
- Add armv7-unknown-linux-gnueabihf binary release

## [0.2.0] - 2022-05-21

### Bug Fixes

- Update badge for release.yml

### Documentation

- Add rust min version tip

### Features

- Update github release workflow and enable unknown-linux-musl release
- Add some basic tests
- Add build info table by `-b` option, fix #15 (#18)

### Miscellaneous Tasks

- Remove bad release targets

## [0.1.2] - 2022-05-19

### Bug Fixes

- Fix github release workflow

### Documentation

- Add README.zh-CN.md
- Add build status badge for README.md

### Features

- Add progress bar for downloading status
- Add customizable output path by using `MUSD_OUTPUT` ENV var support
- Add customizable output path by using `--output(-o)` option support
- Add arg_required_else_help support
- Add ci.yaml workflow
- Add github release workflow
- Add usage gif to README.md

### Miscellaneous Tasks

- Update actions for ci.yaml

### Refactor

- Extract configuration constants

### Opt

- Update release config for optimization

## [0.1.0] - 2022-05-17

### Bug Fixes

- Fix music file downloading corruption issue

### Features

- Add search and select music for downloading feature
- Do some code refactor and extract lib.rs
- Add download music feature
- Rename downloaded music by music name and singer
- Highlight output and add no search result tip
- Warn the user if music file already exists
- Add develop related tasks managed by just
- Add CHANGELOG.md

### Miscellaneous Tasks

- Fix rust clippy warnings
- Add LICENSE and empty README.md file

