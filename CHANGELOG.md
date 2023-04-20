# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/jdrouet/tmdb-api/compare/v0.4.0...v0.4.1) - 2023-04-20

### Added
- *(client)* export base url
- *(movie)* create command for upcoming
- *(movie)* create command for top rated
- *(movie)* create command for popular
- *(movie)* create command for now playing
- *(movie)* create command for latest
- *(movie)* create command for watch providers
- *(movie)* create command for videos
- *(movie)* create command for translations
- *(movie)* create command for reviews
- *(movie)* create command for release dates
- *(movie)* create command for recommendations
- *(movie)* create command for lists
- *(movie)* create command for keywords
- *(movie)* create command for images
- *(movie)* create command for external ids
- *(movie)* create command for credits
- *(movie)* create command for changes
- *(movie)* create command to get alternative titles
- *(changes)* allow to filter and paginate
- create change model and command
- create command to get company images
- create company alternative names
- create certification model and command
- add serialize to PaginatedResult structure
- *(tvshow)* add command to get episode details
- *(tvshow)* add command to get season details
- *(tvshow)* add command to get similar tvshows
- add serialize to every structure
- get similar movies
- fetch people details
- get company details (#1)
- get details of tv show
- get details of movie
- add commands to list genres
- add tvshow search command

### Fixed
- *(tvshow)* overview and first_air_date can be undefined
- stop testing the API
- ensure that movies can be serialized and deserialized
- some string containing a date might be and empty string
- *(test)* movie integration search
- add validation error scheme
- *(tvshow)* some field should have been nullable
- remove println

### Other
- prepare new release script
- bump version to 0.4.0
- remove not required packages
- bump version to 0.3.5
- fix integration tests
- *(deps)* Bump mockito to 1.0
- please clippy
- create script for generating assets
- bump version to 0.3.3
- simplify publish job
- bump version to 0.3.2
- auto publish on tags
- bump version to 0.3.1
- add PartialEq to each structure
- bump version to 0.3.0
- remove reqwest dependency when not using commands feature
- add CI job to check features
- split commands as separate feature
- bump version to 0.2.2
- bump version to 0.2.1
- bump version to 0.2.0
- make common structs public
- *(ci)* execute integration tests for repo owner
- bump version to 0.1.2
- *(movie)* fetch more movies in integration tests
- bump version to 0.1.1
- format code and update documentation
- move genre to common module
- move movie structs and mutualize the code
- move tvshow structs and mutualize the code
- move movie and tvshow search struct
- first commit
