Unreleased
-----------
- Use AsRef in the message parser instead of Into, since we do not *need* ownership

0.3.0 (2016-05-30)
------------------
- add Deref to StructuredMessage (#4, via @pzol)
- return more references instead of strings in the parser (#3)

0.2.0 (2016-02-22)
------------------
- add rustc_serialize integration
- store structured data in a map instead in nested structs
