use crate::error::Result;
use windows::Data::Xml::Dom::{XmlDocument, XmlElement};

pub trait ToXML {
    fn into_raw(self, doc: &XmlDocument) -> Result<XmlElement>;
}
