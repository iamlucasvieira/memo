# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0 - 2024-01-27

### Changed

- Moved data model from data.rs to models.rs
- Moved implementation of data model from data.rs to impls.rs
- Made impls.rs public in lib.rs API
- Those changes aims to make more clear the separation of data trait in `data.rs`, the default data model in `models.rs` and the implementations of this model in `impls.rs`

### Added

- Included more unittests

<!-- next-header -->

## [Unreleased] - ReleaseDate

## 0.3.1 - 2024-01-25

### changed

- Replaced `Box<dyn>` arguments by `impl`. This is more applicable for the application and more efficieny.

## 0.3.0 - 2024-01-24

## Changed

- Turned as main argument rather than argument of a subcommand
- Turned Init into a flag
- Turned List into a flag
- Turned delete into an opttion

## 0.2.0 - 2024-01-24

### Added

- Include command to Add items to memo
- Include command to Remove items from memo by id

### Changed

- Improve writing to file. Now, a temp file is written to, then this file is renamed as the data file.
