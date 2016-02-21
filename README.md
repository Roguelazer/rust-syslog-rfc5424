This module implements an [RFC 5424](https://tools.ietf.org/html/rfc5424) IETF Syslog Protocol parser in Rust.

[![Build Status](https://travis-ci.org/Roguelazer/rust-syslog-rfc5424.svg?branch=master)](https://travis-ci.org/Roguelazer/rust-syslog-rfc5424)

[Documentation](http://roguelazer.github.io/rust-syslog-rfc5424/syslog_rfc5424/)

## Performance

On a recent system, a release build takes approximately 7µs to parse a message without significant complexity, and approximately 37µs to parse a complicated message. In debug mode, those times are about 25x worse. A production server therefore should be able to parse at least 25k messages per second per thread.

This compares very favorably to the 700µs/message for the [python syslog-rfc5424-parser](https://github.com/EasyPost/syslog-rfc5424-parser) module which I also wrote.
