# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3](https://github.com/jdrouet/tmdb-api/compare/v0.5.2...v0.5.3) - 2024-01-06

### Added
- add popular for tv show

### Other
- fix the workflow to be triggered by PRs
- remove unused import

## [0.5.2](https://github.com/jdrouet/tmdb-api/compare/v0.5.1...v0.5.2) - 2023-11-28

### Added
- add support for tv show images and latest

### Other
- remove triggers
- update triggers and cancel duplicates
- apply cargo fmt
- change invalid id in tests
- flaky tvshow search
- update github action to be triggered by external PRs

## [0.5.1](https://github.com/jdrouet/tmdb-api/compare/v0.5.0...v0.5.1) - 2023-10-24

### Added
- add watch providers for TV shows
- add client builder

### Fixed
- update test docs
- documentation for movie watch providers

### Other
- update github action config
- add release-plz
- *(deps)* bump rustls-webpki from 0.101.1 to 0.101.4
- use non deprecated fields
- apply clippy suggestions
