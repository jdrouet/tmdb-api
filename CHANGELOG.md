# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-alpha.1](https://github.com/jdrouet/tmdb-api/compare/v0.9.1...v1.0.0-alpha.1) - 2025-05-04

### Added

- rewrite library to only use functions on the client ([#102](https://github.com/jdrouet/tmdb-api/pull/102))

### Other

- update readme

## [0.9.1](https://github.com/jdrouet/tmdb-api/compare/v0.9.0...v0.9.1) - 2025-04-14

### Fixed

- set default base_url ([#97](https://github.com/jdrouet/tmdb-api/pull/97))

## [0.9.0](https://github.com/jdrouet/tmdb-api/compare/v0.8.0...v0.9.0) - 2025-04-13

### Added

- implement tv show keywords ([#80](https://github.com/jdrouet/tmdb-api/pull/80))
- implementing tv show content rating ([#77](https://github.com/jdrouet/tmdb-api/pull/77))
- implement tv show aggregate credits ([#95](https://github.com/jdrouet/tmdb-api/pull/95))
- bump version to 2024
- add Debug on Client
- *(genre)* add function to specify country when listing ([#79](https://github.com/jdrouet/tmdb-api/pull/79))
- *(configuration)* implement commands ([#83](https://github.com/jdrouet/tmdb-api/pull/83))
- *(genre)* add method to specify language ([#86](https://github.com/jdrouet/tmdb-api/pull/86))

### Fixed

- update documentation ([#78](https://github.com/jdrouet/tmdb-api/pull/78))
- Episode air date is nullable ([#90](https://github.com/jdrouet/tmdb-api/pull/90))
- *(tvshow)* make episode number nullable ([#84](https://github.com/jdrouet/tmdb-api/pull/84))

### Other

- *(deps)* bump ring from 0.17.8 to 0.17.13 ([#92](https://github.com/jdrouet/tmdb-api/pull/92))
- *(deps)* bump tokio from 1.36.0 to 1.38.2 ([#93](https://github.com/jdrouet/tmdb-api/pull/93))
- format code with clippy
- remove use of async_trait
- *(client)* remove with_base_url
- *(deps)* remove patch versions
- *(deps)* bump rustls from 0.22.2 to 0.22.4 ([#89](https://github.com/jdrouet/tmdb-api/pull/89))
- *(deps)* bump h2 from 0.3.25 to 0.3.26 ([#88](https://github.com/jdrouet/tmdb-api/pull/88))
- *(certification)* add some usage examples ([#82](https://github.com/jdrouet/tmdb-api/pull/82))

## [0.8.0](https://github.com/jdrouet/tmdb-api/compare/v0.7.0...v0.8.0) - 2024-03-24

### Added
- *(client)* move reqwest as an executor
- *(error)* use thiserror to implement StdError
- *(tokio-rate-limit)* remove feature

### Fixed
- many deserialization fixes
- *(credits)* known for department is nullable

### Other
- remove mention of tokio rate limit feature
- update test job to use several rust version
- removed array deref
- lint
- update the use of NaiveDateTime
- update dependencies
- remove integration tests

## [0.7.0](https://github.com/jdrouet/tmdb-api/compare/v0.6.0...v0.7.0) - 2024-01-26

### Added
- *(tokio-rate-limit)* add a rate limit feature using the tokio runtime
- *(watch-provider)* add watch provider list

### Fixed
- *(tokio-rate-limit)* lower default rate limit
- *(movies)* deserialize empty string as None
- *(collection)* nullable overview field
- *(tokio-rate-limit)* missing feature macro
- *(collection)* missing overview field
- *(movies)* credits `country` url param corrected to `language`
- serde global macro use
- clippy and forbid unsafe code enforcement

### Other
- disable non working endpoint for /tv/latest
- lint
- empty rustfmt file
- *(watch-provider)* typo fix

## [0.6.0](https://github.com/jdrouet/tmdb-api/compare/v0.5.3...v0.6.0) - 2024-01-21

### Added
- add collections

### Other
- lint
- add quotes to the refresh script variables
- *(collection)* add collection details to the refresh script
- typo fix
- add .env file to .gitignore
- *(deps)* bump h2 from 0.3.20 to 0.3.24

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
