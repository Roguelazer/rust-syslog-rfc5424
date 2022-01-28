0.8.0 (2022-01-28)
------------------
- Upgrade `time` dependency to 0.3 to resolve cargo audits
- Switch from Travis-CI to Github Actions for CI

0.7.0 (2020-09-24)
------------------
- Bump to Rust 2018 edition
- Bump MSRV to 1.34
- Add public `TryFrom` implementations for severity and facility (requested in #16)
- rustfmt/clippyize/etc

0.6.1 (2019-01-19)
------------------
- Fix sign error in numeric timezone offsets (thanks to @main-- for reporting this on GitHub)

0.6.0 (2018-07-14)
------------------
- Parse subsecond part of timestamps and include it as the `timestamp_nanos` field (thanks @bwtril-justin)

0.5.1 (2018-05-15)
------------------
- Allow terms (hostnames, appnames) to start with a hyphen

0.5.0 (2018-05-15)
------------------
- Remove `Severity::from_int`
- Rename `ProcIdType` to `ProcId`
- Remove rustc-serialize
- Implement `FromStr` for `SyslogMessage`, allowing more idiomatic parsing
- Implement Ord/PartialOrd/Eq/PartialEq in more places
- Make clippy and rustfmt happy

0.4.2 (2018-05-15)
------------------
- Make `docs.rs` build with all features

0.4.1 (2018-05-15)
------------------
- Fix bug parsing message with non-empty SD fields but empty message body

0.4.0 (2017-10-24)
----------
- Make `rustc-serialize` support optional behind the self-named feature flag
- Add optional `serde` support behind the `serde-serialize` feature flag

0.3.1 (2017-10-24)
-----------
- Use AsRef in the message parser instead of Into, since we do not *need* ownership
- Support sub-second timestamp resolution (Fixes #5 / #6)
- Add more tests
- Fix various clippy concerns

0.3.0 (2016-05-30)
------------------
- add Deref to StructuredMessage (#4, via @pzol)
- return more references instead of strings in the parser (#3)

0.2.0 (2016-02-22)
------------------
- add rustc_serialize integration
- store structured data in a map instead in nested structs
