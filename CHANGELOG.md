# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Entry types for known storage models
- `filter` implementation for storage models
- `sorter` implementation for storage models
- `group_with_members` struct as a replacement for `group`
- static variables for group roles
- `GroupWithMembers` struct which includes members, invites and event ids
- `EventWithAttendees` struct which includes attendees and invites
- `ProfileWithRefs` struct which includes references to joined groups and events

### Changed

- Bump `ic-cdk` crate to version `0.15`
- Let group and event use the same struct for joined and invites
- Split `ProfileWithRefs` into seperate structs
- - Split `GroupWithMembers` into seperate structs
- - Split `EventWithAttendees` into seperate structs
- Extract `insert` method from the `StorageClient` trait to the two new traits
  `StorageClientInsertable` and `StorageClientInsertableByKey` to allow working with the incrementing
  keys
