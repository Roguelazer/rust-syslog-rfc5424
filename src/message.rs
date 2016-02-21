//! In-memory representation of a single Syslog message.

use std::string::String;
use std::collections::BTreeMap;
use std::convert::Into;

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


pub type SDIDType = String;
pub type SDParamIDType = String;
pub type SDParamValueType = String;


#[derive(Clone,Debug,PartialEq,Eq)]
/// A single key-value SD pair
pub struct StructuredDataParam {
    pub param_id: SDParamIDType,
    pub param_value: SDParamValueType,
}


#[derive(Clone,Debug,PartialEq,Eq)]
pub struct StructuredDataElement {
    pub sd_id: SDIDType,
    pub params: Vec<StructuredDataParam>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct StructuredData {
    pub elements: Vec<StructuredDataElement>
}

/// Map that StructuredData can be converted into
pub type StructuredDataMap = BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>;

impl StructuredData {
    /// Length of this SD (just counts the number of elements, not the number of (element,
    /// param_id) pairs.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Convert the SD into a nested tree of the form
    ///
    /// {
    ///     "sd_id": {
    ///         "param_id_1": "param_value",
    ///         "param_id_2": "param_value'
    ///     }
    /// }
    pub fn as_btree(&self) -> StructuredDataMap {
        let mut res: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

        for sde in self.elements.iter() {
            let mut sub_map = res.entry(sde.sd_id.clone()).or_insert(BTreeMap::new());
            for param in sde.params.iter() {
                sub_map.insert(param.param_id.clone(), param.param_value.clone());
            }
        }
        res
    }
}

impl Into<StructuredDataMap> for StructuredData {
    /// Optimized conversion to a BTreeMap which consumes the StructuredData
    fn into(self) -> StructuredDataMap {
        let mut res: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

        for sde in self.elements.into_iter() {
            let mut sub_map = res.entry(sde.sd_id).or_insert(BTreeMap::new());
            for param in sde.params.into_iter() {
                sub_map.insert(param.param_id, param.param_value);
            }
        }
        res
    }
}


#[derive(Clone,Debug)]
pub struct SyslogMessage {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
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
    use super::{StructuredData,StructuredDataElement,StructuredDataParam};

    #[test]
    fn test_as_btree() {
        let sd = StructuredData {
            elements: vec![
                StructuredDataElement {
                    sd_id: String::from("id1"),
                    params: vec![
                        StructuredDataParam {
                            param_id: String::from("k1"),
                            param_value: String::from("v1"),
                        }
                    ]
                }
            ]
        };

        let bt = sd.as_btree();

        assert_eq!(bt.len(), 1);
        assert_eq!(bt.keys().collect::<Vec<&String>>(), vec!["id1"]);

        let sm1 = bt.get("id1").expect("should unwrap");

        assert_eq!(sm1.keys().collect::<Vec<&String>>(), vec!["k1"]);
    }
}
