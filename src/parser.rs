use std::borrow::Cow;
use std::num;
use std::str;
use std::str::FromStr;
use std::string;

use thiserror::Error;
use time;

use crate::facility;
use crate::message::{ProcId, SyslogMessage};
use crate::severity;
use crate::structured_data::{BTreeStructuredData, StructuredDataMap};

#[derive(Debug, Error)]
pub enum ParseErr {
    #[error("regular expression does not parse")]
    RegexDoesNotMatchErr,
    #[error("bad severity in message")]
    BadSeverityInPri,
    #[error("bad facility in message")]
    BadFacilityInPri,
    #[error("unexpected eof")]
    UnexpectedEndOfInput,
    #[error("too few digits in numeric field")]
    TooFewDigits,
    #[error("too many digits in numeric field")]
    TooManyDigits,
    #[error("invalid UTC offset")]
    InvalidUTCOffset,
    #[error("unicode error: {0}")]
    BaseUnicodeError(#[from] str::Utf8Error),
    #[error("unicode error: {0}")]
    UnicodeError(#[from] string::FromUtf8Error),
    #[error("unexpected input at character {0}")]
    ExpectedTokenErr(char),
    #[error("integer conversion error: {0}")]
    IntConversionErr(#[from] num::ParseIntError),
    #[error("missing field {0}")]
    MissingField(&'static str),
}

// We parse with this super-duper-dinky hand-coded recursive descent parser because we don't really
// have much other choice:
//
//  - Regexp is much slower (at least a factor of 4), and we still end up having to parse the
//    somewhat-irregular SD
//  - LALRPOP requires non-ambiguous tokenization
//  - Rust-PEG doesn't work on anything except nightly
//
// So here we are. The macros make it a bit better.
//
// General convention is that the parse state is represented by a string slice named "rest"; the
// macros will update that slice as they consume tokens.

macro_rules! maybe_expect_char {
    ($s:expr, $e: expr) => {
        match $s.chars().next() {
            Some($e) => Some(&$s[1..]),
            _ => None,
        }
    };
}

macro_rules! take_item {
    ($e:expr, $r:expr) => {{
        let (t, r) = $e?;
        $r = r;
        t
    }};
}

type ParseResult<T> = Result<T, ParseErr>;

macro_rules! take_char {
    ($e: expr, $c:expr) => {{
        $e = match $e.chars().next() {
            Some($c) => &$e[1..],
            Some(_) => {
                return Err(ParseErr::ExpectedTokenErr($c));
            }
            None => {
                return Err(ParseErr::UnexpectedEndOfInput);
            }
        }
    }};
}

fn take_while<F>(input: &str, f: F, max_chars: usize) -> (&str, Option<&str>)
where
    F: Fn(char) -> bool,
{
    for (idx, chr) in input.char_indices() {
        if !f(chr) {
            return (&input[..idx], Some(&input[idx..]));
        }
        if idx == max_chars {
            return (&input[..idx], Some(&input[idx..]));
        }
    }
    ("", None)
}

fn parse_sd_id(input: &str) -> ParseResult<(String, &str)> {
    let (res, rest) = take_while(input, |c| c != ' ' && c != '=' && c != ']', 128);
    Ok((
        String::from(res),
        match rest {
            Some(s) => s,
            None => return Err(ParseErr::UnexpectedEndOfInput),
        },
    ))
}

/** Parse a `param_value`... a.k.a. a quoted string */
fn parse_param_value(input: &str) -> ParseResult<(Cow<str>, &str)> {
    let mut rest = input;
    take_char!(rest, '"');
    // Can't do a 0-copy &str slice here because we need to un-escape escaped quotes
    // in the string. :-(
    let mut result = String::new();

    let mut saw_any_escapes = false;
    let mut escaped = false;

    for (idx, chr) in rest.char_indices() {
        if escaped {
            escaped = false
        } else {
            if chr == '\\' {
                escaped = true;
                if !saw_any_escapes {
                    result.push_str(&rest[..idx]);
                }
                saw_any_escapes = true;
                continue;
            }
            if chr == '"' {
                let res_cow = if saw_any_escapes {
                    Cow::Owned(result)
                } else {
                    Cow::Borrowed(&rest[..idx])
                };
                return Ok((res_cow, &rest[(idx + 1)..]));
            }
        }
        if saw_any_escapes {
            result.push(chr);
        }
    }

    Err(ParseErr::UnexpectedEndOfInput)
}

type ParsedSDParams = Vec<(String, String)>;

fn parse_sd_params(input: &str) -> ParseResult<(ParsedSDParams, &str)> {
    let mut params = Vec::new();
    let mut top = input;
    loop {
        if let Some(rest2) = maybe_expect_char!(top, ' ') {
            let mut rest = rest2;
            let param_name = take_item!(parse_sd_id(rest), rest);
            take_char!(rest, '=');
            let param_value = take_item!(parse_param_value(rest), rest);
            // is there an uglier modifier than &*
            params.push((param_name, String::from(&*param_value)));
            top = rest;
        } else {
            return Ok((params, top));
        }
    }
}

fn parse_sde(sde: &str) -> ParseResult<((String, ParsedSDParams), &str)> {
    let mut rest = sde;
    take_char!(rest, '[');
    let id = take_item!(parse_sd_id(rest), rest);
    let params = take_item!(parse_sd_params(rest), rest);
    take_char!(rest, ']');
    Ok(((id, params), rest))
}

fn parse_sd<S: StructuredDataMap>(structured_data_raw: &str) -> ParseResult<(S, &str)> {
    let mut sd = Default::default();
    if structured_data_raw.starts_with('-') {
        return Ok((sd, &structured_data_raw[1..]));
    }
    let mut rest = structured_data_raw;
    while !rest.is_empty() {
        let (sd_id, params) = take_item!(parse_sde(rest), rest);
        for (sd_param_id, sd_param_value) in params {
            sd.insert_tuple(sd_id.clone(), sd_param_id, sd_param_value);
        }
        if rest.starts_with(' ') {
            break;
        }
    }
    Ok((sd, rest))
}

fn parse_pri_val(pri: i32) -> ParseResult<(severity::SyslogSeverity, facility::SyslogFacility)> {
    let sev = severity::SyslogSeverity::from_int(pri & 0x7).ok_or(ParseErr::BadSeverityInPri)?;
    let fac = facility::SyslogFacility::from_int(pri >> 3).ok_or(ParseErr::BadFacilityInPri)?;
    Ok((sev, fac))
}

fn parse_num(s: &str, min_digits: usize, max_digits: usize) -> ParseResult<(i32, &str)> {
    let (res, rest1) = take_while(s, |c| c >= '0' && c <= '9', max_digits);
    let rest = rest1.ok_or(ParseErr::UnexpectedEndOfInput)?;
    if res.len() < min_digits {
        Err(ParseErr::TooFewDigits)
    } else if res.len() > max_digits {
        Err(ParseErr::TooManyDigits)
    } else {
        Ok((
            i32::from_str(res).map_err(ParseErr::IntConversionErr)?,
            rest,
        ))
    }
}

fn parse_decimal(d: &str, min_digits: usize, max_digits: usize) -> ParseResult<(i32, &str)> {
    parse_num(d, min_digits, max_digits).map(|(val, s)| {
        let mut multiplicand = 1;
        let z = 10 - (d.len() - s.len());

        for _i in 1..(z) {
            multiplicand *= 10;
        }
        (val * multiplicand, s)
    })
}

fn parse_timestamp(m: &str) -> ParseResult<(Option<time::Timespec>, &str)> {
    let mut rest = m;
    if rest.starts_with('-') {
        return Ok((None, &rest[1..]));
    }
    let mut tm = time::empty_tm();
    tm.tm_year = take_item!(parse_num(rest, 4, 4), rest) - 1900;
    take_char!(rest, '-');
    tm.tm_mon = take_item!(parse_num(rest, 2, 2), rest) - 1;
    take_char!(rest, '-');
    tm.tm_mday = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, 'T');
    tm.tm_hour = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    tm.tm_min = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    tm.tm_sec = take_item!(parse_num(rest, 2, 2), rest);
    if rest.starts_with('.') {
        take_char!(rest, '.');
        tm.tm_nsec = take_item!(parse_decimal(rest, 1, 6), rest);
    }
    // Tm::utcoff is totally broken, don't use it.
    let utc_offset_mins = match rest.chars().next() {
        None => 0,
        Some('Z') => {
            rest = &rest[1..];
            0
        }
        Some(c) => {
            let (sign, irest) = match c {
                // Note: signs are backwards as per RFC3339
                '-' => (1, &rest[1..]),
                '+' => (-1, &rest[1..]),
                _ => {
                    return Err(ParseErr::InvalidUTCOffset);
                }
            };
            let hours = i32::from_str(&irest[0..2]).map_err(ParseErr::IntConversionErr)?;
            let minutes = i32::from_str(&irest[3..5]).map_err(ParseErr::IntConversionErr)?;
            rest = &irest[5..];
            minutes * sign + hours * 60 * sign
        }
    };
    tm = tm + time::Duration::minutes(i64::from(utc_offset_mins));
    tm.tm_isdst = -1;
    Ok((Some(tm.to_utc().to_timespec()), rest))
}

fn parse_term(
    m: &str,
    min_length: usize,
    max_length: usize,
) -> ParseResult<(Option<String>, &str)> {
    if m.starts_with('-') && (m.len() <= 1 || m.as_bytes()[1] == 0x20) {
        return Ok((None, &m[1..]));
    }
    let byte_ary = m.as_bytes();
    for (idx, chr) in byte_ary.iter().enumerate() {
        if *chr < 33 || *chr > 126 {
            if idx < min_length {
                return Err(ParseErr::TooFewDigits);
            }
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseErr::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
        if idx >= max_length {
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseErr::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
    }
    Err(ParseErr::UnexpectedEndOfInput)
}

fn parse_message_s<M: StructuredDataMap>(m: &str) -> ParseResult<SyslogMessage<M>> {
    let mut rest = m;
    take_char!(rest, '<');
    let prival = take_item!(parse_num(rest, 1, 3), rest);
    take_char!(rest, '>');
    let (sev, fac) = parse_pri_val(prival)?;
    let version = take_item!(parse_num(rest, 1, 2), rest);
    take_char!(rest, ' ');
    let event_time = take_item!(parse_timestamp(rest), rest);
    take_char!(rest, ' ');
    let hostname = take_item!(parse_term(rest, 1, 255), rest);
    take_char!(rest, ' ');
    let appname = take_item!(parse_term(rest, 1, 48), rest);
    take_char!(rest, ' ');
    let procid_r = take_item!(parse_term(rest, 1, 128), rest);
    let procid = match procid_r {
        None => None,
        Some(s) => Some(match i32::from_str(&s) {
            Ok(n) => ProcId::PID(n),
            Err(_) => ProcId::Name(s),
        }),
    };
    take_char!(rest, ' ');
    let msgid = take_item!(parse_term(rest, 1, 32), rest);
    take_char!(rest, ' ');
    let sd = take_item!(parse_sd::<M>(rest), rest);
    rest = match maybe_expect_char!(rest, ' ') {
        Some(r) => r,
        None => rest,
    };
    let msg = String::from(rest);

    Ok(SyslogMessage {
        severity: sev,
        facility: fac,
        version,
        timestamp: event_time.map(|t| t.sec),
        timestamp_nanos: event_time.map(|t| t.nsec),
        hostname,
        appname,
        procid,
        msgid,
        sd,
        msg,
    })
}

/// Parse a string into a `SyslogMessage` object
///
/// # Arguments
///
///  * `s`: Anything convertible to a string
///
/// # Returns
///
///  * `ParseErr` if the string is not parseable as an RFC5424 message
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
pub fn parse_message_with<S: AsRef<str>, M: StructuredDataMap>(
    s: S,
) -> ParseResult<SyslogMessage<M>> {
    parse_message_s(s.as_ref())
}

pub fn parse_message<S: AsRef<str>>(s: S) -> ParseResult<SyslogMessage<BTreeStructuredData>> {
    parse_message_with(s)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::mem;

    use super::{parse_message, ParseErr};
    use crate::message;

    use crate::facility::SyslogFacility;
    use crate::severity::SyslogSeverity;
    use crate::structured_data::StructuredDataMap;

    #[test]
    fn test_simple() {
        let msg = parse_message("<1>1 - - - - - -").expect("Should parse empty message");
        assert!(msg.facility == SyslogFacility::LOG_KERN);
        assert!(msg.severity == SyslogSeverity::SEV_ALERT);
        assert!(msg.timestamp.is_none());
        assert!(msg.hostname.is_none());
        assert!(msg.appname.is_none());
        assert!(msg.procid.is_none());
        assert!(msg.msgid.is_none());
        assert!(msg.sd.len() == 0);
    }

    #[test]
    fn test_with_time_zulu() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00Z host - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420070400));
    }

    #[test]
    fn test_with_time_offset() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00+00:00 - - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420070400));
    }

    #[test]
    fn test_with_time_offset_nonzero() {
        let msg = parse_message("<1>1 2015-01-01T00:00:00-10:00 - - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1420106400));
        // example from RFC 3339
        let msg1 = parse_message("<1>1 2015-01-01T18:50:00-04:00 - - - - -")
            .expect("Should parse empty message");
        let msg2 = parse_message("<1>1 2015-01-01T22:50:00Z - - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg1.timestamp, msg2.timestamp);
        // example with fractional minutes
        let msg1 = parse_message("<1>1 2019-01-20T00:46:39+05:45 - - - - -")
            .expect("Should parse empty message");
        let msg2 = parse_message("<1>1 2019-01-19T11:01:39-08:00 - - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg1.timestamp, msg2.timestamp);
    }

    #[test]
    fn test_complex() {
        let msg = parse_message("<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [meta sequenceId=\"29\"] some_message").expect("Should parse complex message");
        assert_eq!(msg.facility, SyslogFacility::LOG_CRON);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.hostname, Some(String::from("host1")));
        assert_eq!(msg.appname, Some(String::from("CROND")));
        assert_eq!(msg.procid, Some(message::ProcId::PID(10391)));
        assert_eq!(msg.msg, String::from("some_message"));
        assert_eq!(msg.timestamp, Some(1452816241));
        assert_eq!(msg.sd.len(), 1);
        let v = msg
            .sd
            .find_tuple("meta", "sequenceId")
            .expect("Should contain meta sequenceId");
        assert_eq!(v, "29");
    }

    #[test]
    fn test_sd_features() {
        let msg = parse_message("<78>1 2016-01-15T00:04:01Z host1 CROND 10391 - [meta sequenceId=\"29\" sequenceBlah=\"foo\"][my key=\"value\"][meta bar=\"baz=\"] some_message").expect("Should parse complex message");
        assert_eq!(msg.facility, SyslogFacility::LOG_CRON);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.hostname, Some(String::from("host1")));
        assert_eq!(msg.appname, Some(String::from("CROND")));
        assert_eq!(msg.procid, Some(message::ProcId::PID(10391)));
        assert_eq!(msg.msg, String::from("some_message"));
        assert_eq!(msg.timestamp, Some(1452816241));
        assert_eq!(msg.sd.len(), 2);
        assert_eq!(
            msg.sd.find_sdid("meta").expect("should contain meta").len(),
            3
        );
    }

    #[test]
    fn test_sd_with_escaped_quote() {
        let msg_text = r#"<1>1 - - - - - [meta key="val\"ue"] message"#;
        let msg = parse_message(msg_text).expect("should parse");
        assert_eq!(
            msg.sd
                .find_tuple("meta", "key")
                .expect("Should contain meta key"),
            r#"val"ue"#
        );
    }

    #[test]
    fn test_other_message() {
        let msg_text = r#"<190>1 2016-02-21T01:19:11+00:00 batch6sj - - - [meta sequenceId="21881798" x-group="37051387"][origin x-service="tracking"] metascutellar conversationalist nephralgic exogenetic graphy streng outtaken acouasm amateurism prenotice Lyonese bedull antigrammatical diosphenol gastriloquial bayoneteer sweetener naggy roughhouser dighter addend sulphacid uneffectless ferroprussiate reveal Mazdaist plaudite Australasian distributival wiseman rumness Seidel topazine shahdom sinsion mesmerically pinguedinous ophthalmotonometer scuppler wound eciliate expectedly carriwitchet dictatorialism bindweb pyelitic idic atule kokoon poultryproof rusticial seedlip nitrosate splenadenoma holobenthic uneternal Phocaean epigenic doubtlessly indirection torticollar robomb adoptedly outspeak wappenschawing talalgia Goop domitic savola unstrafed carded unmagnified mythologically orchester obliteration imperialine undisobeyed galvanoplastical cycloplegia quinquennia foremean umbonal marcgraviaceous happenstance theoretical necropoles wayworn Igbira pseudoangelic raising unfrounced lamasary centaurial Japanolatry microlepidoptera"#;
        parse_message(msg_text).expect("should parse as text");
    }

    #[test]
    fn test_bad_pri() {
        let msg = parse_message("<4096>1 - - - - - -");
        assert!(msg.is_err());
    }

    #[test]
    fn test_bad_match() {
        // we shouldn't be able to parse RFC3164 messages
        let msg = parse_message("<134>Feb 18 20:53:31 haproxy[376]: I am a message");
        assert!(msg.is_err());
    }

    #[test]
    fn test_example_timestamps() {
        // these are the example timestamps in the rfc

        let msg = parse_message("<1>1 1985-04-12T23:20:50.52Z host - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(482196050));
        assert_eq!(msg.timestamp_nanos, Some(520000000));

        let msg = parse_message("<1>1 1985-04-12T19:20:50.52+04:00 host - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(482167250));
        assert_eq!(msg.timestamp_nanos, Some(520000000));

        let msg = parse_message("<1>1 1985-04-12T19:20:50+04:00 host - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(482167250));
        assert_eq!(msg.timestamp_nanos, Some(0));

        let msg = parse_message("<1>1 2003-08-24T05:14:15.000003+07:00 host - - - -")
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(1061676855));
        assert_eq!(msg.timestamp_nanos, Some(3000));

        let msg = parse_message("<1>1 2003-08-24T05:14:15.000000003+07:00 host - - - -");
        assert!(msg.is_err(), "expected parse fail");
    }

    #[test]
    fn test_empty_sd_value() {
        let msg = parse_message(r#"<29>1 2018-05-14T08:23:01.520Z leyal_test4 mgd 13894 UI_CHILD_EXITED [junos@2636.1.1.1.2.57 pid="14374" return-value="5" core-dump-status="" command="/usr/sbin/mustd"]"#).expect("must parse");
        assert_eq!(msg.facility, SyslogFacility::LOG_DAEMON);
        assert_eq!(msg.severity, SyslogSeverity::SEV_NOTICE);
        assert_eq!(msg.hostname, Some(String::from("leyal_test4")));
        assert_eq!(msg.appname, Some(String::from("mgd")));
        assert_eq!(msg.procid, Some(message::ProcId::PID(13894)));
        assert_eq!(msg.msg, String::from(""));
        assert_eq!(msg.timestamp, Some(1526286181));
        assert_eq!(msg.timestamp_nanos, Some(520000000));
        assert_eq!(msg.sd.len(), 1);
        let sd = msg
            .sd
            .as_btreemap()
            .get("junos@2636.1.1.1.2.57")
            .map(|s| s.to_owned())
            .expect("should contain root SD");
        let expected = {
            let mut expected = BTreeMap::new();
            expected.insert("pid", "14374");
            expected.insert("return-value", "5");
            expected.insert("core-dump-status", "");
            expected.insert("command", "/usr/sbin/mustd");
            expected
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<_, _>>()
        };
        assert_eq!(sd, expected);
    }

    #[test]
    fn test_fields_start_with_dash() {
        let msg = parse_message("<39>1 2018-05-15T20:56:58+00:00 -web1west -201805020050-bc5d6a47c3-master - - [meta sequenceId=\"28485532\"] 25450-uWSGI worker 6: getaddrinfo*.gaih_getanswer: got type \"DNAME\"").expect("should parse");
        assert_eq!(msg.hostname, Some("-web1west".to_string()));
        assert_eq!(
            msg.appname,
            Some("-201805020050-bc5d6a47c3-master".to_string())
        );
        assert_eq!(
            msg.sd.find_tuple("meta", "sequenceId"),
            Some(&"28485532".to_string())
        );
        assert_eq!(
            msg.msg,
            "25450-uWSGI worker 6: getaddrinfo*.gaih_getanswer: got type \"DNAME\"".to_string()
        );
    }

    #[test]
    fn test_truncated() {
        let err =
            parse_message("<39>1 2018-05-15T20:56:58+00:00 -web1west -").expect_err("should fail");
        assert_eq!(
            mem::discriminant(&err),
            mem::discriminant(&ParseErr::UnexpectedEndOfInput)
        );
    }
}
