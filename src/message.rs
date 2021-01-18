//! In-memory representation of a single Syslog message.

use std::cmp::Ordering;
use std::str::FromStr;
use std::string::String;

#[cfg(feature = "serde-serialize")]
use serde::{Serialize, Serializer};

#[allow(non_camel_case_types)]
pub type time_t = i64;
#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type msgid_t = String;

use crate::facility;
use crate::parser;
use crate::severity;
use crate::structured_data::{BTreeStructuredData, StructuredDataMap};

pub type SDIDType = String;
pub type SDParamIDType = String;
pub type SDParamValueType = String;

#[derive(Clone, Debug, PartialEq, Eq)]
/// `ProcID`s are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcId {
    PID(pid_t),
    Name(String),
}

impl PartialOrd for ProcId {
    fn partial_cmp(&self, other: &ProcId) -> Option<Ordering> {
        match (self, other) {
            (&ProcId::PID(ref s_p), &ProcId::PID(ref o_p)) => Some(s_p.cmp(o_p)),
            (&ProcId::Name(ref s_n), &ProcId::Name(ref o_n)) => Some(s_n.cmp(o_n)),
            _ => None,
        }
    }
}

#[cfg(feature = "serde-serialize")]
impl Serialize for ProcId {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match *self {
            ProcId::PID(ref p) => ser.serialize_i32(*p),
            ProcId::Name(ref n) => ser.serialize_str(n),
        }
    }
}

#[cfg_attr(feature = "serde-serialize", derive(Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A RFC5424-protocol syslog message
pub struct SyslogMessage<SDType: StructuredDataMap = BTreeStructuredData> {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
    pub version: i32,
    pub timestamp: Option<time_t>,
    pub timestamp_nanos: Option<i32>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<ProcId>,
    pub msgid: Option<msgid_t>,
    pub sd: SDType,
    pub msg: String,
}

impl<M: StructuredDataMap> FromStr for SyslogMessage<M> {
    type Err = parser::ParseErr;

    /// Parse a string into a `SyslogMessage`
    ///
    /// Just calls `parser::parse_message`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse_message_with(s)
    }
}

#[cfg(test)]
mod tests {
    use super::SyslogMessage;
    #[cfg(feature = "serde-serialize")]
    use crate::facility::SyslogFacility::*;
    #[cfg(feature = "serde-serialize")]
    use crate::severity::SyslogSeverity::*;
    #[cfg(feature = "serde-serialize")]
    use serde_json;
    use std::collections::BTreeMap;

    #[cfg(feature = "serde-serialize")]
    #[test]
    fn test_serialization_serde() {
        let m = SyslogMessage {
            severity: SEV_INFO,
            facility: LOG_KERN,
            version: 1,
            timestamp: None,
            timestamp_nanos: None,
            hostname: None,
            appname: None,
            procid: None,
            msgid: None,
            sd: BTreeMap::new(),
            msg: String::from(""),
        };

        let encoded = serde_json::to_string(&m).expect("Should encode to JSON");
        // XXX: we don't have a guaranteed order, I don't think, so this might break with minor
        // version changes. *shrug*
        assert_eq!(encoded,
                   "{\"severity\":\"info\",\"facility\":\"kern\",\"version\":1,\"timestamp\":null,\"timestamp_nanos\":null,\"hostname\":null,\"appname\":null,\"procid\":null,\"msgid\":null,\"sd\":{},\"msg\":\"\"}");
    }

    #[test]
    fn test_fromstr() {
        let msg = "<1>1 1985-04-12T23:20:50.52Z host - - - -"
            .parse::<SyslogMessage>()
            .expect("Should parse empty message");
        assert_eq!(msg.timestamp, Some(482196050));
        assert_eq!(msg.sd, BTreeMap::new());
    }

    #[test]
    fn test_fromstr_hashmap() {
        use std::collections::HashMap;

        let msg =
            "<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [foo bar=\"baz\"] some_message"
                .parse::<SyslogMessage<HashMap<_, _>>>()
                .expect("Should parse simple message");
        assert_eq!(msg.sd, {
            let mut m = HashMap::new();
            let mut sm = HashMap::new();
            sm.insert("bar".to_owned(), "baz".to_owned());
            m.insert("foo".to_owned(), sm);
            m
        })
    }

    #[test]
    fn test_fromstr_hashmap_custom_hasher() {
        use std::collections::HashMap;

        let msg =
            "<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [foo bar=\"baz\"] some_message"
                .parse::<SyslogMessage<HashMap<_, _, fxhash::FxBuildHasher>>>()
                .expect("Should parse simple message");
        assert_eq!(msg.sd, {
            let mut m = HashMap::with_hasher(fxhash::FxBuildHasher::default());
            let mut sm = HashMap::with_hasher(fxhash::FxBuildHasher::default());
            sm.insert("bar".to_owned(), "baz".to_owned());
            m.insert("foo".to_owned(), sm);
            m
        })
    }

    #[cfg(feature = "indexmap")]
    #[test]
    fn test_fromstr_indexmap() {
        use indexmap::IndexMap;

        let msg =
            "<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [foo bar=\"baz\"] some_message"
                .parse::<SyslogMessage<IndexMap<_, _>>>()
                .expect("Should parse simple message");
        assert_eq!(msg.sd, {
            let mut m = IndexMap::new();
            let mut sm = IndexMap::new();
            sm.insert("bar".to_owned(), "baz".to_owned());
            m.insert("foo".to_owned(), sm);
            m
        })
    }

    #[cfg(feature = "indexmap")]
    #[test]
    fn test_fromstr_indexmap_custom_hasher() {
        use fxhash::FxBuildHasher;
        use indexmap::IndexMap;

        let msg =
            "<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [foo bar=\"baz\"] some_message"
                .parse::<SyslogMessage<IndexMap<_, _, FxBuildHasher>>>()
                .expect("Should parse simple message");
        assert_eq!(msg.sd, {
            let mut m = IndexMap::with_hasher(FxBuildHasher::default());
            let mut sm = IndexMap::with_hasher(FxBuildHasher::default());
            sm.insert("bar".to_owned(), "baz".to_owned());
            m.insert("foo".to_owned(), sm);
            m
        })
    }
}
