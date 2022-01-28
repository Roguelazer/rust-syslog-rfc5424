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
//! use syslog_rfc5424::SyslogMessage;
//! use std::net::UdpSocket;
//! use std::str;
//!
//! let s = UdpSocket::bind("127.0.0.1:10514").unwrap();
//! let mut buf = [0u8; 2048];
//! loop {
//!     let (data_read, _) = s.recv_from(&mut buf).unwrap();
//!     let msg = str::from_utf8(&buf[0..data_read]).unwrap().parse::<SyslogMessage>().unwrap();
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
mod facility;
pub mod message;
pub mod parser;
mod severity;

pub use facility::SyslogFacility;
pub use severity::SyslogSeverity;

pub use message::SyslogMessage;
pub use parser::parse_message;
