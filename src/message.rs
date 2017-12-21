//! In-memory representation of a single Syslog message.

use std::string::String;

#[allow(non_camel_case_types)]
pub type time_t = i64;
#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type msgid_t = String;
pub type MessageType = String;

use severity;
use facility;

#[derive(Clone,Debug,Serialize)]
pub struct SyslogMessage {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
    pub version: i32,
    pub timestamp: Option<time_t>,
    pub hostname: Option<String>,
    pub tag: Option<String>,
    pub msg: String,
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::SyslogMessage;
    use severity::SyslogSeverity::*;
    use facility::SyslogFacility::*;

    #[test]
    fn test_serialization_serde() {
        let m = SyslogMessage {
            severity: SEV_INFO,
            facility: LOG_KERN,
            version: 1,
            timestamp: None,
            hostname: None,
            tag: None,
            msg: String::from("")
        };

        let encoded = serde_json::to_string(&m).expect("Should encode to JSON");
//        println!("{:?}", encoded);
        // XXX: we don't have a guaranteed order, I don't think, so this might break with minor
        // version changes. *shrug*
        assert_eq!(encoded, "{\"severity\":\"info\",\"facility\":\"kern\",\"version\":1,\"timestamp\":null,\"hostname\":null,\"tag\":null,\"msg\":\"\"}");
    }
}
