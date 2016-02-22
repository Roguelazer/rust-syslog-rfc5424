//! In-memory representation of a single Syslog message.

use std::string::String;
use std::collections::BTreeMap;
use std::convert::Into;

use rustc_serialize::{Encodable,Encoder};

#[allow(non_camel_case_types)]
pub type time_t = i64;
#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type msgid_t = String;
pub type MessageType = String;

use severity;
use facility;


#[derive(Clone,Debug,PartialEq,Eq)]
/// ProcIDs are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcIdType {
    PID(pid_t),
    Name(String)
}

impl Encodable for ProcIdType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>
    {
        match *self {
            ProcIdType::PID(ref p) => s.emit_i32(*p),
            ProcIdType::Name(ref n) => s.emit_str(n)
        }
    }
}


pub type SDIDType = String;
pub type SDParamIDType = String;
pub type SDParamValueType = String;


pub type StructuredDataElement = BTreeMap<SDParamIDType, SDParamValueType>;


#[derive(Clone,Debug,PartialEq,Eq)]
/// Container for the StructuredData component of a syslog message.
///
/// This is a map from SD_ID to pairs of SD_ParamID, SD_ParamValue
///
/// The spec does not forbid repeated keys. However, for convenience, we *do* forbid repeated keys.
/// That is to say, if you have a message like
///
/// [foo bar="baz" bar="bing"]
///
/// There's no way to retrieve the original "baz" mapping.
pub struct StructuredData {
    elements: BTreeMap<SDIDType, StructuredDataElement>,
}

impl Encodable for StructuredData {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>
    {
        self.elements.encode(s)
    }
}

impl StructuredData {
    pub fn new_empty() -> StructuredData
    {
        StructuredData {
            elements: BTreeMap::new()
        }
    }

    /// Insert a new (sd_id, sd_param_id) -> sd_value mapping into the StructuredData
    pub fn insert_tuple<SI, SPI, SPV> (&mut self, sd_id: SI, sd_param_id: SPI, sd_param_value: SPV) -> ()
        where SI: Into<SDIDType>, SPI: Into<SDParamIDType>, SPV: Into<SDParamValueType>
    {
        let mut sub_map = self.elements.entry(sd_id.into()).or_insert(BTreeMap::new());
        sub_map.insert(sd_param_id.into(), sd_param_value.into());
    }

    /// Lookup by SDID, SDParamID pair
    pub fn find_tuple<'b>(&'b self, sd_id: &str, sd_param_id: &str) -> Option<&'b SDParamValueType>
    {
        // TODO: use traits to make these based on the public types isntead of &str
        if let Some(sub_map) = self.elements.get(sd_id) {
            if let Some(value) = sub_map.get(sd_param_id) {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Find all param/value mappings for a given SDID
    pub fn find_sdid<'b>(&'b self, sd_id: &str) -> Option<&'b BTreeMap<SDParamIDType, SDParamValueType>>
    {
        self.elements.get(sd_id)
    }

    /// The number of distinct SD_IDs
    pub fn len(&self) -> usize
    {
        self.elements.len()
    }
}


#[derive(Clone,Debug,RustcEncodable)]
pub struct SyslogMessage {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
    pub version: i32,
    pub timestamp: Option<time_t>,
    pub hostname: Option<String>,
    pub application: Option<String>,
    pub procid: Option<ProcIdType>,
    pub msgid: Option<msgid_t>,
    pub sd: StructuredData,
    pub message: String,
}


#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    use super::{StructuredData,SyslogMessage};
    use severity::SyslogSeverity::*;
    use facility::SyslogFacility::*;

    #[test]
    fn test_structured_data_basic() {
        let mut s = StructuredData::new_empty();
        s.insert_tuple("foo", "bar", "baz");
        let v = s.find_tuple("foo", "bar").expect("should find foo/bar");
        assert_eq!(v, "baz");
        assert!(s.find_tuple("foo", "baz").is_none());
    }

    #[test]
    fn test_structured_data_serialization() {
        let mut s = StructuredData::new_empty();
        s.insert_tuple("foo", "bar", "baz");
        s.insert_tuple("foo", "baz", "bar");
        s.insert_tuple("faa", "bar", "baz");
        let encoded = json::encode(&s).expect("Should encode to JSON");
        assert_eq!(encoded, r#"{"faa":{"bar":"baz"},"foo":{"bar":"baz","baz":"bar"}}"#);
    }

    #[test]
    fn test_serialization() {
        let m = SyslogMessage {
            severity: SEV_INFO,
            facility: LOG_KERN,
            version: 1,
            timestamp: None,
            hostname: None,
            application: None,
            procid: None,
            msgid: None,
            sd: StructuredData::new_empty(),
            message: String::from("")
        };

        let encoded = json::encode(&m).expect("Should encode to JSON");
        println!("{:?}", encoded);
        // XXX: we don't have a guaranteed order, I don't think, so this might break with minor
        // version changes. *shrug*
        assert_eq!(encoded, "{\"severity\":\"info\",\"facility\":\"kern\",\"version\":1,\"timestamp\":null,\"hostname\":null,\"application\":null,\"procid\":null,\"msgid\":null,\"sd\":{},\"message\":\"\"}");
    }
}
