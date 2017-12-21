//! Parser for [RFC 5424](https://tools.ietf.org/html/rfc5424) Syslog messages. Not to be confused
//! with the older [RFC 3164](https://tools.ietf.org/html/rfc3164) BSD Syslog protocol, which many
//! systems still emit.
//!
//! In particular, supports the Structured Data fields.
//!
//! Usually, you'll just call the (re-exported) `parse_message` function with a stringy object.
//!
//! # Example
//!
//! A simple syslog server
//!
//! ```no_run
//! use syslog_rfc3164::parse_message;
//! use std::net::UdpSocket;
//! use std::str;
//!
//! let s = UdpSocket::bind("127.0.0.1:10514").unwrap();
//! let mut buf = [0u8; 2048];
//! loop {
//!     let (data_read, _) = s.recv_from(&mut buf).unwrap();
//!     let msg = parse_message(str::from_utf8(&buf[0..data_read]).unwrap()).unwrap();
//!     println!("{:?} {:?} {:?} {:?}", msg.facility, msg.severity, msg.hostname, msg.msg);
//! }
//! ```
//!
//! # Unimplemented Features
//!
//!  * Theoretically, you can send arbitrary (non-unicode) bytes for the message part of a syslog
//!    message. Rust doesn't have a convenient way to only treat *some* of a buffer as utf-8,
//!    so I'm just not supporting that. Most "real" syslog servers barf on it anway.
//!
#[cfg(test)]
extern crate assert_matches;
extern crate time;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod message;
mod severity;
mod facility;
pub mod parser;

pub use severity::SyslogSeverity;
pub use facility::SyslogFacility;

pub use parser::parse_message;
