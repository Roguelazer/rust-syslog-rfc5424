use serde::{Serializer, Serialize};

#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
/// Syslog Severities from RFC 5424.
pub enum SyslogSeverity {
    SEV_EMERG = 0,
    SEV_ALERT = 1,
    SEV_CRIT = 2,
    SEV_ERR = 3,
    SEV_WARNING = 4,
    SEV_NOTICE = 5,
    SEV_INFO = 6,
    SEV_DEBUG = 7,
}

impl SyslogSeverity {
    /// Convert an int (as used in the wire serialization) into a `SyslogSeverity`
    ///
    /// Returns an Option, but the wire protocol will only include 0..7, so should
    /// never return None in practical usage.
    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            0 => Some(SyslogSeverity::SEV_EMERG),
            1 => Some(SyslogSeverity::SEV_ALERT),
            2 => Some(SyslogSeverity::SEV_CRIT),
            3 => Some(SyslogSeverity::SEV_ERR),
            4 => Some(SyslogSeverity::SEV_WARNING),
            5 => Some(SyslogSeverity::SEV_NOTICE),
            6 => Some(SyslogSeverity::SEV_INFO),
            7 => Some(SyslogSeverity::SEV_DEBUG),
            _ => None,
        }
    }

    /// Convert a syslog severity into a unique string representation
    pub fn as_str(&self) -> &'static str {
        match *self {
            SyslogSeverity::SEV_EMERG => "emerg",
            SyslogSeverity::SEV_ALERT => "alert",
            SyslogSeverity::SEV_CRIT => "crit",
            SyslogSeverity::SEV_ERR => "err",
            SyslogSeverity::SEV_WARNING => "warning",
            SyslogSeverity::SEV_NOTICE => "notice",
            SyslogSeverity::SEV_INFO => "info",
            SyslogSeverity::SEV_DEBUG => "debug"
        }
    }
}

impl Serialize for SyslogSeverity {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.as_str())
    }
}


#[cfg(test)]
mod tests {
    use super::SyslogSeverity;

    #[test]
    fn test_deref() {
        assert_eq!(SyslogSeverity::SEV_EMERG.as_str(), "emerg");
        assert_eq!(SyslogSeverity::SEV_ALERT.as_str(), "alert");
        assert_eq!(SyslogSeverity::SEV_CRIT.as_str(), "crit");
        assert_eq!(SyslogSeverity::SEV_ERR.as_str(), "err");
        assert_eq!(SyslogSeverity::SEV_WARNING.as_str(), "warning");
        assert_eq!(SyslogSeverity::SEV_NOTICE.as_str(), "notice");
        assert_eq!(SyslogSeverity::SEV_INFO.as_str(), "info");
        assert_eq!(SyslogSeverity::SEV_DEBUG.as_str(), "debug");
    }
}
