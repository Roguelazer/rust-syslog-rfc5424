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
