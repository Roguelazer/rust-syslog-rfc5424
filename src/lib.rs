#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate time;

pub mod message;
pub mod severity;
pub mod facility;
mod parser;

pub use parser::parse_message;
