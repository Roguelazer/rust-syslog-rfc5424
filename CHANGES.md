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
