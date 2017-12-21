use serde::{Serializer, Serialize};

#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
/// Syslog facilities. Taken From RFC 5424, but I've heard that some platforms mix these around.
/// Names are from Linux.
pub enum SyslogFacility {
    LOG_KERN = 0,
    LOG_USER = 1,
    LOG_MAIL = 2,
    LOG_DAEMON = 3,
    LOG_AUTH = 4,
    LOG_SYSLOG = 5,
    LOG_LPR = 6,
    LOG_NEWS = 7,
    LOG_UUCP = 8,
    LOG_CRON = 9,
    LOG_AUTHPRIV = 10,
    LOG_FTP = 11,
    LOG_NTP = 12,
    LOG_AUDIT = 13,
    LOG_ALERT = 14,
    LOG_CLOCKD = 15,
    LOG_LOCAL0 = 16,
    LOG_LOCAL1 = 17,
    LOG_LOCAL2 = 18,
    LOG_LOCAL3 = 19,
    LOG_LOCAL4 = 20,
    LOG_LOCAL5 = 21,
    LOG_LOCAL6 = 22,
    LOG_LOCAL7 = 23,
}

impl SyslogFacility {
    /// Convert an int (as used in the wire serialization) into a `SyslogFacility`
    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            0 => Some(SyslogFacility::LOG_KERN),
            1 => Some(SyslogFacility::LOG_USER),
            2 => Some(SyslogFacility::LOG_MAIL),
            3 => Some(SyslogFacility::LOG_DAEMON),
            4 => Some(SyslogFacility::LOG_AUTH),
            5 => Some(SyslogFacility::LOG_SYSLOG),
            6 => Some(SyslogFacility::LOG_LPR),
            7 => Some(SyslogFacility::LOG_NEWS),
            8 => Some(SyslogFacility::LOG_UUCP),
            9 => Some(SyslogFacility::LOG_CRON),
            10 => Some(SyslogFacility::LOG_AUTHPRIV),
            11 => Some(SyslogFacility::LOG_FTP),
            12 => Some(SyslogFacility::LOG_NTP),
            13 => Some(SyslogFacility::LOG_AUDIT),
            14 => Some(SyslogFacility::LOG_ALERT),
            15 => Some(SyslogFacility::LOG_CLOCKD),
            16 => Some(SyslogFacility::LOG_LOCAL0),
            17 => Some(SyslogFacility::LOG_LOCAL1),
            18 => Some(SyslogFacility::LOG_LOCAL2),
            19 => Some(SyslogFacility::LOG_LOCAL3),
            20 => Some(SyslogFacility::LOG_LOCAL4),
            21 => Some(SyslogFacility::LOG_LOCAL5),
            22 => Some(SyslogFacility::LOG_LOCAL6),
            23 => Some(SyslogFacility::LOG_LOCAL7),
            _ => None,
        }
    }

    /// Convert a syslog facility into a unique string representation
    pub fn as_str(&self) -> &'static str {
        match *self {
            SyslogFacility::LOG_KERN => "kern",
            SyslogFacility::LOG_USER => "user",
            SyslogFacility::LOG_MAIL => "mail",
            SyslogFacility::LOG_DAEMON => "daemon",
            SyslogFacility::LOG_AUTH => "auth",
            SyslogFacility::LOG_SYSLOG => "syslog",
            SyslogFacility::LOG_LPR => "lpr",
            SyslogFacility::LOG_NEWS => "news",
            SyslogFacility::LOG_UUCP => "uucp",
            SyslogFacility::LOG_CRON => "cron",
            SyslogFacility::LOG_AUTHPRIV => "authpriv",
            SyslogFacility::LOG_FTP => "ftp",
            SyslogFacility::LOG_NTP => "ntp",
            SyslogFacility::LOG_AUDIT => "audit",
            SyslogFacility::LOG_ALERT => "alert",
            SyslogFacility::LOG_CLOCKD => "clockd",
            SyslogFacility::LOG_LOCAL0 => "local0",
            SyslogFacility::LOG_LOCAL1 => "local1",
            SyslogFacility::LOG_LOCAL2 => "local2",
            SyslogFacility::LOG_LOCAL3 => "local3",
            SyslogFacility::LOG_LOCAL4 => "local4",
            SyslogFacility::LOG_LOCAL5 => "local5",
            SyslogFacility::LOG_LOCAL6 => "local6",
            SyslogFacility::LOG_LOCAL7 => "local7",
        }
    }
}

impl Serialize for SyslogFacility {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.as_str())
    }
}


#[cfg(test)]
mod tests {
    use super::SyslogFacility;

    #[test]
    fn test_deref() {
        assert_eq!(SyslogFacility::LOG_KERN.as_str(), "kern");
    }
}
