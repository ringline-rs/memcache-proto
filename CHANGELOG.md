# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `DEFAULT_MAX_RESPONSE_LINE_LEN`, plus `Response::parse_with_max_line_len` and
  `ResponseBytes::parse_with_max_line_len` for callers that need a different bound.

### Fixed

- ASCII response parsing is now bounded by a maximum line length, matching the
  command and streaming parsers. `Response::parse` and `ResponseBytes::parse`
  previously reported `Incomplete` for an unterminated line no matter how large
  the buffer grew, so a peer that never sent a newline could make a caller
  buffer without limit. Both now return `Protocol("line too long")` past
  `DEFAULT_MAX_RESPONSE_LINE_LEN` (8 KiB).

## [0.0.3] - 2026-08-19

### Added

- Kani verification for binary header arithmetic and encode/parse round trips

### Changed

- `RequestHeader::value_length()` and `ResponseHeader::value_length()` now return
  `Option<usize>` instead of `usize`; `checked_value_length()` is removed.

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
