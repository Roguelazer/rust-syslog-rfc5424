// It'd be great to use a real parser here. unfortunately, rust-peg doesn't work on any
// stable releases, and lalrpop can't hangle un-tokenizable grammars like this one. so we
// are instead going to use a regexp to pull out the parts that are regular and then a hand-coded
// parser for the rest. Hurray. :-/

use std::str::FromStr;
use std::num;
use std::char;

use time;


use regex::Regex;

use severity;
use facility;
use message::{SyslogMessage,ProcIdType,StructuredData,StructuredDataElement,StructuredDataParam};

#[derive(Debug, Clone)]
pub enum ParseErr {
    RegexDoesNotMatchErr,
    BadSeverityInPri,
    BadFacilityInPri,
    UnexpectedEndOfInput,
    ExpectedTokenErr(char),
    IntConversionErr(num::ParseIntError),
    MissingField(&'static str)
}

fn parse_pri(pri: i32) -> Result<(severity::SyslogSeverity, facility::SyslogFacility), ParseErr> {
    let sev = try!(severity::SyslogSeverity::from_int(pri & 0x7).ok_or(ParseErr::BadSeverityInPri));
    let fac = try!(facility::SyslogFacility::from_int(pri >> 3).ok_or(ParseErr::BadFacilityInPri));
    Ok((sev, fac))
}

macro_rules! extract_field {
    ($md:expr, $f:expr) => (match $md.name($f) {
        Some("-") => None,
        Some(f) => Some(String::from(f)),
        None => { return Err(ParseErr::MissingField($f)) }
    })
}

macro_rules! try_field_as_i32 {
    ($md:expr, $f: expr) => (match $md.name($f) {
        Some(f) => try!(i32::from_str(f).map_err(ParseErr::IntConversionErr)),
        None => { return Err(ParseErr::MissingField($f)) }
    })
}

macro_rules! maybe_expect_char {
    ($s:expr, $e: expr) => (match $s.chars().next() {
        Some($e) => Some(&$s[1..]),
        _ => None,
    })
}

macro_rules! expect_char {
    ($s:expr, $e: expr) => (match $s.chars().next() {
        Some($e) => &$s[1..],
        _ => { return Err(ParseErr::ExpectedTokenErr($e)); }
    })
}

fn take_while<F>(input: &str, f: F) -> (String, Option<&str>)
    where F: Fn(char) -> bool {
    let mut result = String::new();

    for (idx, chr) in input.char_indices() {
        if !f(chr) {
            return (result, Some(&input[idx..]));
        }
        result.push(chr);
    }
    (result, None)
}

fn parse_sd_id(input: &str) -> Result<(String, &str), ParseErr> {
    let (res, rest) = take_while(input, |c| c != ' ' && c != '=' && c != ']');
    Ok((res, match rest {
        Some(s) => s,
        None => { return Err(ParseErr::UnexpectedEndOfInput); }
    }))
}

fn parse_param_value(input: &str) -> Result<(String, &str), ParseErr> {
    let rest1 = expect_char!(input, '"');
    let mut result = String::new();

    let mut escaped = false;

    for (idx, chr) in rest1.char_indices() {
        if escaped {
            escaped = false
        } else {
            if chr == '\\' {
                escaped = true
            }
            if chr == '"' {
                return Ok((result, &input[(idx + 2)..]));
            }
        }
        result.push(chr)
    }

    return Err(ParseErr::UnexpectedEndOfInput);
}

fn parse_sd_params(input: &str) -> Result<(Vec<StructuredDataParam>, &str), ParseErr> {
    let mut params = Vec::new();
    let mut top = input;
    loop {
        if let Some(rest) = maybe_expect_char!(top, ' ') {
            let (param_name, rest2) = try!(parse_sd_id(rest));
            let rest3 = expect_char!(rest2, '=');
            let (param_value, rest4) = try!(parse_param_value(rest3));
            params.push(StructuredDataParam {
                param_id: String::from(param_name),
                param_value: String::from(param_value)
            });
            top = rest4;
        } else {
            return Ok((params, top));
        }
    }
}

fn parse_sde(sde: &str) -> Result<(StructuredDataElement, &str), ParseErr> {
    let (id, rest1) = try!(parse_sd_id(expect_char!(sde, '[')));
    let (params, rest2) = try!(parse_sd_params(rest1));
    let rest3 = expect_char!(rest2, ']');
    Ok((StructuredDataElement {
        sd_id: id,
        params: params
    }, rest3))
}

/// SUPER-DINKY recursive-descent parser to parse structured data
fn parse_sd(structured_data_raw: String) -> Result<StructuredData, ParseErr> {
    let mut elements = Vec::new();
    let mut rest = &structured_data_raw[..];
    loop {
        let (element, rest2) = try!(parse_sde(rest));
        elements.push(element);
        if rest2.len() == 0 {
            return Ok(StructuredData { elements: elements });
        }
        rest = rest2;
    }
}


/// Parse a string into a SyslogMessage object
///
/// # Arguments
///
///  * `s`: Anything convertible to a string
///
/// # Returns
///
///  * ParseErr if the string is not parseable as an RFC5424 message
///
/// # Example
///
/// ```
/// use syslog_rfc5424::parse_message;
///
/// let message = parse_message("<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [meta sequenceId=\"29\"] some_message").unwrap();
///
/// assert!(message.hostname.unwrap() == "host1");
/// ```
pub fn parse_message<S> (s: S) -> Result<SyslogMessage, ParseErr> 
    where S: Into<String> {

    lazy_static! {
        static ref SYSLOG_RE:Regex = Regex::new(r#"(?x)
        ^
        <(?P<pri>[0-9]+)>
        (?P<version>[0-9]+)
        \x20
        (?P<timestamp>-|(?:(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})T(?P<hour>[0-9]{2}):(?P<minute>[0-9]{2}):(?P<second>[0-9]{2})(?P<fracsec>\.[0-9]{1,6})?(?P<offset>Z|(?:[+-]?[0-9]{2}:[0-9]{2}))))
        \x20
        (?P<hostname>-|(?:\S{1,255}))
        \x20
        (?P<appname>-|(?:\S{1,48}))
        \x20
        (?P<procid>-|(?:\S{1,128}))
        \x20
        (?P<msgid>-|(?:\S{1,32}))
        \x20
        (?P<sd>-|(?: \[ [^\x20\]"]{1,32} (?: \x20 [^=\x20\]]{1,32} = "[^"]+")*\])+)
        \x20?
        (?P<message>.*)$
        "#).unwrap();
        static ref ALL_DIGIT_RE: Regex = Regex::new("^[0-9]+$").unwrap();
    }

    let s_s = s.into();

    let md = try!(SYSLOG_RE.captures(&s_s).ok_or(ParseErr::RegexDoesNotMatchErr));

    let pri_val = try!(md.name("pri").ok_or(ParseErr::MissingField("pri")));
    let (sev, fac) = try!(parse_pri(try!(i32::from_str(pri_val).map_err(ParseErr::IntConversionErr))));

    let procid = match extract_field!(md, "procid") {
        Some(ss) => { match ALL_DIGIT_RE.is_match(&ss) {
            true => Some(ProcIdType::PID(i32::from_str(&ss).unwrap())),
            false => Some(ProcIdType::Name(ss))
        }},
        None => None,
    };

    let timestamp = match md.name("timestamp") {
        Some("-") => None,
        None => None,
        Some(_) => {
            let mut tm = time::empty_tm();
            tm.tm_year = try_field_as_i32!(md, "year") - 1900;
            tm.tm_mon = try_field_as_i32!(md, "month") - 1;
            tm.tm_mday = try_field_as_i32!(md, "day");
            tm.tm_hour = try_field_as_i32!(md, "hour");
            tm.tm_min = try_field_as_i32!(md, "minute");
            tm.tm_sec = try_field_as_i32!(md, "second");
            // Tm::utcoff is totally broken, don't use it.
            let utc_offset_mins = match md.name("offset") {
                None => 0,
                Some("Z") => 0,
                Some(other) => {
                    let front = other[0..1].as_bytes()[0];
                    let (sign, rest) = match char::from_u32(front as u32) {
                        Some('+') => (1, &other[1..]),
                        Some('-') => (-1, &other[1..]),
                        _ => (1, other)
                    };
                    let hours = try!(i32::from_str(&rest[0..2]).map_err(ParseErr::IntConversionErr));
                    let minutes = try!(i32::from_str(&rest[3..5]).map_err(ParseErr::IntConversionErr));
                    println!("hours:{:?}, minutes:{:?}, sign:{:?}", hours, minutes, sign);
                    minutes + hours * 60 * sign
                }
            };
            tm = tm + time::Duration::minutes(utc_offset_mins as i64);
            println!("offset={:?}", tm.tm_utcoff);
            tm.tm_isdst = -1;
            Some(tm.to_utc().to_timespec().sec)
        }
    };

    Ok(SyslogMessage {
        severity: sev,
        facility: fac,
        hostname: extract_field!(md, "hostname"),
        application: extract_field!(md, "appname"),
        procid: procid,
        msgid: extract_field!(md, "msgid"),
        sd: try!(extract_field!(md, "sd").map(parse_sd).unwrap_or(Ok(StructuredData { elements: Vec::new() }))),
        timestamp: timestamp,
        message: String::from(md.name("message").unwrap_or(""))
    })
}


#[cfg(test)]
mod tests {
    use super::{parse_message, ParseErr};
    use message;

    use facility::SyslogFacility;
    use severity::SyslogSeverity;

    #[test]
    fn test_simple() {
        let msg = parse_message("<1>1 - - - - - -").expect("Should parse empty message");
        assert!(msg.facility == SyslogFacility::LOG_KERN);
        assert!(msg.severity == SyslogSeverity::SEV_ALERT);
        assert!(msg.timestamp.is_none());
        assert!(msg.hostname.is_none());
        assert!(msg.application.is_none());
        assert!(msg.procid.is_none());
        assert!(msg.msgid.is_none());
        assert!(msg.sd.len() == 0);
    }

    #[test]
    fn test_with_time_zulu() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00Z - - - - -").expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420070400));
    }

    #[test]
    fn test_with_time_offset() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00+00:00 - - - - -").expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420070400));
    }

    #[test]
    fn test_with_time_offset_nonzero() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00+10:00 - - - - -").expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420106400));
    }

    #[test]
    fn test_complex() {
        let msg = parse_message("<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [meta sequenceId=\"29\"] some_message").expect("Should parse complex message");
        assert_eq!(msg.facility, SyslogFacility::LOG_CRON);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.hostname, Some(String::from("host1")));
        assert_eq!(msg.application, Some(String::from("CROND")));
        assert_eq!(msg.procid, Some(message::ProcIdType::PID(10391)));
        assert_eq!(msg.message, String::from("some_message"));
        assert_eq!(msg.timestamp, Some(1452816241));
        assert_eq!(msg.sd.len(), 1);
        assert_eq!(msg.sd.elements, vec![message::StructuredDataElement { sd_id: String::from("meta"), params: vec![message::StructuredDataParam { param_id: String::from("sequenceId"), param_value: String::from("29") }]}]);
    }

    #[test]
    fn test_sd_features() {
        let msg = parse_message("<78>1 2016-01-15T00:04:01Z host1 CROND 10391 - [meta sequenceId=\"29\" sequenceBlah=\"foo\"][my key=\"value\"] some_message").expect("Should parse complex message");
        assert_eq!(msg.facility, SyslogFacility::LOG_CRON);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.hostname, Some(String::from("host1")));
        assert_eq!(msg.application, Some(String::from("CROND")));
        assert_eq!(msg.procid, Some(message::ProcIdType::PID(10391)));
        assert_eq!(msg.message, String::from("some_message"));
        assert_eq!(msg.timestamp, Some(1452816241));
        assert_eq!(msg.sd.len(), 2);
        assert_eq!(msg.sd, message::StructuredData { elements: vec![
            message::StructuredDataElement {
                sd_id: String::from("meta"),
                params: vec![
                    message::StructuredDataParam {
                        param_id: String::from("sequenceId"), param_value: String::from("29") 
                    },
                    message::StructuredDataParam {
                        param_id: String::from("sequenceBlah"), param_value: String::from("foo") 
                    }
                ],
            }, message::StructuredDataElement {
                sd_id: String::from("my"),
                params: vec![
                    message::StructuredDataParam {
                        param_id: String::from("key"), param_value: String::from("value")
                    }
                ]
            }
        ] }); 
    }

    #[test]
    fn test_other_message() { 
        let msg_text = r#"<190>1 2016-02-21T01:19:11+00:00 batch6sj - - - [meta sequenceId="21881798" x-group="37051387"][origin x-service="tracking"] metascutellar conversationalist nephralgic exogenetic graphy streng outtaken acouasm amateurism prenotice Lyonese bedull antigrammatical diosphenol gastriloquial bayoneteer sweetener naggy roughhouser dighter addend sulphacid uneffectless ferroprussiate reveal Mazdaist plaudite Australasian distributival wiseman rumness Seidel topazine shahdom sinsion mesmerically pinguedinous ophthalmotonometer scuppler wound eciliate expectedly carriwitchet dictatorialism bindweb pyelitic idic atule kokoon poultryproof rusticial seedlip nitrosate splenadenoma holobenthic uneternal Phocaean epigenic doubtlessly indirection torticollar robomb adoptedly outspeak wappenschawing talalgia Goop domitic savola unstrafed carded unmagnified mythologically orchester obliteration imperialine undisobeyed galvanoplastical cycloplegia quinquennia foremean umbonal marcgraviaceous happenstance theoretical necropoles wayworn Igbira pseudoangelic raising unfrounced lamasary centaurial Japanolatry microlepidoptera"#;
        let msg = parse_message(msg_text).expect("should parse as text");
    }

    #[test]
    fn test_bad_pri() {
        let msg = parse_message("<4096>1 - - - - - -");
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err(), ParseErr::BadFacilityInPri);
    }

    #[test]
    fn test_bad_match() {
        // we shouldn't be able to parse RFC3164 messages
        let msg = parse_message("<134>Feb 18 20:53:31 haproxy[376]: I am a message");
        assert!(msg.is_err());
        assert_matches!(msg.unwrap_err(), ParseErr::RegexDoesNotMatchErr);
    }
}
