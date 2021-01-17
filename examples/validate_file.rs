/// Read a file of newline-delimited messages and count how many are valid
use clap::Arg;
use std::io::{BufRead, BufReader};

use fxhash::FxBuildHasher;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap};

use syslog_rfc5424::SyslogMessage;

#[inline(always)]
fn parse_with_btreemap(s: &str) -> bool {
    s.parse::<SyslogMessage<BTreeMap<_, _>>>().is_ok()
}

#[inline(always)]
fn parse_with_hashmap(s: &str) -> bool {
    s.parse::<SyslogMessage<HashMap<_, _>>>().is_ok()
}

#[inline(always)]
fn parse_with_hashmap_fxhash(s: &str) -> bool {
    s.parse::<SyslogMessage<HashMap<_, _, FxBuildHasher>>>()
        .is_ok()
}

#[inline(always)]
#[cfg(feature = "indexmap")]
fn parse_with_indexmap(s: &str) -> bool {
    s.parse::<SyslogMessage<IndexMap<_, _>>>().is_ok()
}

#[inline(always)]
#[cfg(feature = "indexmap")]
fn parse_with_indexmap_fxhash(s: &str) -> bool {
    s.parse::<SyslogMessage<IndexMap<_, _, FxBuildHasher>>>()
        .is_ok()
}

pub fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("EasyPost <oss@easypost.com>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("map_type")
                .short("m")
                .long("map-type")
                .takes_value(true)
                .possible_values(&[
                    "btreemap",
                    "hashmap",
                    "hashmap+fxhash",
                    "indexmap",
                    "indexmap+fxhash",
                ])
                .default_value("btreemap")
                .help("Map implementation to use"),
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .default_value("-")
                .help("Path to input file (if '-', reads stdin)"),
        )
        .get_matches();

    let s = std::io::stdin();

    let input: Box<dyn BufRead> = match matches.value_of("input").unwrap() {
        "-" => Box::new(s.lock()),
        other => Box::new(BufReader::new(std::fs::File::open(other).unwrap())),
    };

    let f: Box<dyn Fn(&str) -> bool> = match matches.value_of("map_type").unwrap() {
        "btreemap" => Box::new(parse_with_btreemap),
        "hashmap" => Box::new(parse_with_hashmap),
        "hashmap+fxhash" => Box::new(parse_with_hashmap_fxhash),
        #[cfg(feature = "indexmap")]
        "indexmap" => Box::new(parse_with_indexmap),
        #[cfg(feature = "indexmap")]
        "indexmap+fxhash" => Box::new(parse_with_indexmap_fxhash),
        _ => unimplemented!("unknown map type!"),
    };

    let count = input
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| f(line))
        .count();

    println!("count ok: {:?}", count);
}
