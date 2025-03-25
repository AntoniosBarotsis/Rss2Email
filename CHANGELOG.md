# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### In Progress

- [Styling HTML output](https://github.com/AntoniosBarotsis/Rss2Email/issues/6)

  See [this](https://github.com/AntoniosBarotsis/Rss2Email/pull/37) pull request.

  Explore the idea of letting the user inject their own CSS through environment variables and predefined "themes".

- [Add post descriptions](https://github.com/AntoniosBarotsis/Rss2Email/issues/38)

## [2.1.1] - 2025-03-25

- Bump dependencies

## [2.1.0] - 2024-03-07

### Added

- Add ability to include the number of posts fetched in the subject
- Add ability to skip sending if no posts fetched

### Changed

- The email now lets the user know that no new posts were found instead of being empty.

## [2.0.0] - 2024-02-06

### Added

- Can now send emails to multiple recipients

### Changed

- `RECIPIENT_ADDRESSES` and `SUBJECT` environment variables are now required

## [1.1.1] - 2023-08-23

### Fixed

- Fixed a bug where parsing would break if a feed would end on a newline

### Changed

- Feeds are now first read from the env variable and if that does not exist, the file is checked

## [1.1.0] - 2023-07-10

### Added

- Added support for [Resend](https://resend.com/)

## [1.0.1] - 2023-01-03

### Added

- Error reporting now includes details on which post errored and what the error was specifically.
- Post's date now defaults to `publication` date and fallbacks to the `update` date

## [1.0.0] - 2022-12-06

Initial release 🎉

[unreleased]: https://github.com/AntoniosBarotsis/rss2email/compare/v2.1.1...HEAD
[2.1.1]: https://github.com/AntoniosBarotsis/rss2email/compare/v2.1.1...HEAD
[2.1.0]: https://github.com/AntoniosBarotsis/rss2email/compare/v2.1.0...HEAD
[2.0.0]: https://github.com/AntoniosBarotsis/rss2email/releases/tag/v2.0.0
[1.1.1]: https://github.com/AntoniosBarotsis/rss2email/releases/tag/v1.1.1
[1.1.0]: https://github.com/AntoniosBarotsis/rss2email/releases/tag/v1.1.0
[1.0.1]: https://github.com/AntoniosBarotsis/rss2email/releases/tag/v1.0.1
[1.0.0]: https://github.com/AntoniosBarotsis/rss2email/releases/tag/v1.0.0
