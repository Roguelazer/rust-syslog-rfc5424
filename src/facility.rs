#[cfg(feature = "serde-serialize")]
use serde::{Serialize, Serializer};

use std::convert::TryFrom;

use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
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

#[derive(Debug, Error)]
pub enum SyslogFacilityError {
    #[error("integer does not correspond to a known facility")]
    InvalidInteger,
}

impl TryFrom<i32> for SyslogFacility {
    type Error = SyslogFacilityError;

    #[inline(always)]
    fn try_from(i: i32) -> Result<SyslogFacility, Self::Error> {
        Ok(match i {
            0 => SyslogFacility::LOG_KERN,
            1 => SyslogFacility::LOG_USER,
            2 => SyslogFacility::LOG_MAIL,
            3 => SyslogFacility::LOG_DAEMON,
            4 => SyslogFacility::LOG_AUTH,
            5 => SyslogFacility::LOG_SYSLOG,
            6 => SyslogFacility::LOG_LPR,
            7 => SyslogFacility::LOG_NEWS,
            8 => SyslogFacility::LOG_UUCP,
            9 => SyslogFacility::LOG_CRON,
            10 => SyslogFacility::LOG_AUTHPRIV,
            11 => SyslogFacility::LOG_FTP,
            12 => SyslogFacility::LOG_NTP,
            13 => SyslogFacility::LOG_AUDIT,
            14 => SyslogFacility::LOG_ALERT,
            15 => SyslogFacility::LOG_CLOCKD,
            16 => SyslogFacility::LOG_LOCAL0,
            17 => SyslogFacility::LOG_LOCAL1,
            18 => SyslogFacility::LOG_LOCAL2,
            19 => SyslogFacility::LOG_LOCAL3,
            20 => SyslogFacility::LOG_LOCAL4,
            21 => SyslogFacility::LOG_LOCAL5,
            22 => SyslogFacility::LOG_LOCAL6,
            23 => SyslogFacility::LOG_LOCAL7,
            _ => return Err(SyslogFacilityError::InvalidInteger),
        })
    }
}

impl SyslogFacility {
    /// Convert an int (as used in the wire serialization) into a `SyslogFacility`
    pub(crate) fn from_int(i: i32) -> Option<Self> {
        Self::try_from(i).ok()
    }

    /// Convert a syslog facility into a unique string representation
    pub fn as_str(self) -> &'static str {
        match self {
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

#[cfg(feature = "serde-serialize")]
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
