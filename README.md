This module implements an [RFC 5424](https://tools.ietf.org/html/rfc5424) IETF Syslog Protocol parser in Rust.

[![Build Status](https://travis-ci.org/Roguelazer/rust-syslog-rfc5424.svg?branch=master)](https://travis-ci.org/Roguelazer/rust-syslog-rfc5424)

[Documentation](http://roguelazer.github.io/rust-syslog-rfc5424/syslog_rfc5424/)

## Performance

On a recent system<sup>[1](#sysfootnote)</sup>, a release build takes approximately 8µs to parse an average message and approximately 300ns to parse the smallest legal message. Debug timings are a bit worse -- about 60µs for an average message and about 8µs for the minimal message. A single-threaded Syslog server should be able to parse at least 100,000 messages/s, as long as you run a separate thread for the parser.

This compares *very* favorably to [python syslog-rfc5424-parser](https://github.com/EasyPost/syslog-rfc5424-parser)<sup>[2](#fn2)</sup>, which takes about 300µs for a minimal message, and more than 700µs for an average message.

## Footnotes

* <a name="sysfootnote">1</a>:  An Intel i7-4850HQ in a 2013 rMBP
* <a name="fnt2">2</a>: Which I also wrote
