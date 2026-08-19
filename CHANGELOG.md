# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Fallible binary request and response value-length APIs
- Kani verification for binary header arithmetic and encode/parse round trips

### Fixed

- Reject inconsistent binary header lengths before waiting for body data

- ASCII line framing no longer stalls on a bare `\r`. `find_crlf` inspected only
  the first `\r` in the buffer and gave up if it was not followed by `\n`, so a
  complete CRLF-terminated line containing a bare `\r` reported `Incomplete`
  forever in the response parser, and reported `Protocol("line too long")` in the
  command and streaming parsers once the line-length guard tripped. All three now
  scan for a real CRLF.

## [0.0.1] - 2026-02-21

### Added

- Initial release extracted from ringline workspace
- ASCII memcache protocol parsing and encoding
- Binary memcache protocol parsing and encoding
- Streaming parser for incremental decoding
