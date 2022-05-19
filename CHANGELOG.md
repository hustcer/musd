# Changelog
All notable changes to this project will be documented in this file.

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

