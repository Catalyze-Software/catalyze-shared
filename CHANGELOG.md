# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Entry types for known storage models
- `filter` implementation for storage models
- `sorter` implementation for storage models
- `get_paginated` and `filter_paginated` method implementations for the storage client

### Changed

- Bump `ic-cdk` crate to version `0.15`
- Extract `insert` method from the `StorageClient` trait to the two new traits
  `StorageClientInsertable` and `StorageClientInsertableByKey` to allow working with the incrementing
  keys
