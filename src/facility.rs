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
    /// Convert an int (as used in the wire serialization) into a SyslogFacility
    pub fn from_int(i: i32) -> Option<SyslogFacility> {
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
}
