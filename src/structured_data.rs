#[cfg(feature = "indexmap")]
use indexmap::IndexMap;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasher;

pub type SDIDType = String;
pub type SDParamIDType = String;
pub type SDParamValueType = String;

pub trait StructuredDataElement: Default + std::fmt::Debug {}

impl StructuredDataElement for BTreeMap<SDParamIDType, SDParamValueType> {}

impl<H: Default + Clone + BuildHasher> StructuredDataElement
    for HashMap<SDParamIDType, SDParamValueType, H>
{
}

#[cfg(feature = "indexmap")]
impl<H: Default> StructuredDataElement for IndexMap<SDParamIDType, SDParamValueType, H> {}

pub trait StructuredDataMap: Clone + PartialEq + Eq + std::fmt::Debug + Default {
    type StructuredDataElementMap: StructuredDataElement;

    fn find_tuple<'b>(&'b self, sd_id: &str, sd_param_id: &str) -> Option<&'b SDParamValueType>;
    fn find_sdid<'b>(&'b self, sd_id: &str) -> Option<&'b Self::StructuredDataElementMap>;
    fn as_btreemap(&self) -> Cow<BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>>;
    fn insert_tuple<SI, SPI, SPV>(
        &mut self,
        sd_id: SI,
        sd_param_id: SPI,
        sd_param_value: SPV,
    ) -> ()
    where
        SI: Into<SDIDType>,
        SPI: Into<SDParamIDType>,
        SPV: Into<SDParamValueType>;
}

pub type BTreeStructuredData = BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>;

impl StructuredDataMap for BTreeStructuredData {
    type StructuredDataElementMap = BTreeMap<SDParamIDType, SDParamValueType>;

    fn find_tuple<'b>(&'b self, sd_id: &str, sd_param_id: &str) -> Option<&'b SDParamValueType> {
        self.get(sd_id).and_then(|submap| submap.get(sd_param_id))
    }

    fn find_sdid<'b>(&'b self, sd_id: &str) -> Option<&'b Self::StructuredDataElementMap> {
        self.get(sd_id)
    }

    fn insert_tuple<SI, SPI, SPV>(&mut self, sd_id: SI, sd_param_id: SPI, sd_param_value: SPV) -> ()
    where
        SI: Into<SDIDType>,
        SPI: Into<SDParamIDType>,
        SPV: Into<SDParamValueType>,
    {
        let sub_map = self.entry(sd_id.into()).or_insert_with(Default::default);
        sub_map.insert(sd_param_id.into(), sd_param_value.into());
    }

    fn as_btreemap(
        &self,
    ) -> std::borrow::Cow<BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>> {
        Cow::Borrowed(&self)
    }
}

impl<H: Default + Clone + BuildHasher> StructuredDataMap
    for HashMap<SDIDType, HashMap<SDParamIDType, SDParamValueType, H>, H>
{
    type StructuredDataElementMap = HashMap<SDParamIDType, SDParamValueType, H>;

    fn find_tuple<'b>(&'b self, sd_id: &str, sd_param_id: &str) -> Option<&'b SDParamValueType> {
        self.get(sd_id).and_then(|submap| submap.get(sd_param_id))
    }

    fn find_sdid<'b>(&'b self, sd_id: &str) -> Option<&'b Self::StructuredDataElementMap> {
        self.get(sd_id)
    }

    fn insert_tuple<SI, SPI, SPV>(&mut self, sd_id: SI, sd_param_id: SPI, sd_param_value: SPV) -> ()
    where
        SI: Into<SDIDType>,
        SPI: Into<SDParamIDType>,
        SPV: Into<SDParamValueType>,
    {
        let sub_map = self.entry(sd_id.into()).or_insert_with(Default::default);
        sub_map.insert(sd_param_id.into(), sd_param_value.into());
    }

    fn as_btreemap(
        &self,
    ) -> std::borrow::Cow<BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>> {
        Cow::Owned(
            self.into_iter()
                .map(|(k, sm)| {
                    (
                        k.to_owned(),
                        sm.into_iter()
                            .map(|(p, v)| (p.to_owned(), v.to_owned()))
                            .collect::<BTreeMap<_, _>>(),
                    )
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

#[cfg(feature = "indexmap")]
impl<H: Clone + Default + BuildHasher> StructuredDataMap
    for IndexMap<SDIDType, IndexMap<SDParamIDType, SDParamValueType, H>, H>
{
    type StructuredDataElementMap = IndexMap<SDParamIDType, SDParamValueType, H>;

    fn find_tuple<'b>(&'b self, sd_id: &str, sd_param_id: &str) -> Option<&'b SDParamValueType> {
        self.get(sd_id).and_then(|submap| submap.get(sd_param_id))
    }

    fn find_sdid<'b>(&'b self, sd_id: &str) -> Option<&'b Self::StructuredDataElementMap> {
        self.get(sd_id)
    }

    fn insert_tuple<SI, SPI, SPV>(&mut self, sd_id: SI, sd_param_id: SPI, sd_param_value: SPV) -> ()
    where
        SI: Into<SDIDType>,
        SPI: Into<SDParamIDType>,
        SPV: Into<SDParamValueType>,
    {
        let sub_map = self.entry(sd_id.into()).or_insert_with(Default::default);
        sub_map.insert(sd_param_id.into(), sd_param_value.into());
    }

    fn as_btreemap(
        &self,
    ) -> std::borrow::Cow<BTreeMap<SDIDType, BTreeMap<SDParamIDType, SDParamValueType>>> {
        Cow::Owned(
            self.into_iter()
                .map(|(k, sm)| {
                    (
                        k.to_owned(),
                        sm.into_iter()
                            .map(|(p, v)| (p.to_owned(), v.to_owned()))
                            .collect::<BTreeMap<_, _>>(),
                    )
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}
