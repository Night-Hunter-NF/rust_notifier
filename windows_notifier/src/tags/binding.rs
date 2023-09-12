use windows::Data::Xml::Dom::XmlElement;

use crate::error::Result;
use crate::Toast;

impl Toast {
    fn binding_get_element(&self) -> Result<XmlElement> {
        let res = self.doc.GetElementById(&"binding".into())?;
        Ok(res)
    }

    /// Set to "true" to allow Windows to append a query string to the image URI supplied in the toast notification. Use this attribute if your server hosts images and can handle query strings, either by retrieving an image variant based on the query strings or by ignoring the query string and returning the image as specified without the query string. This query string specifies scale, contrast setting, and language; for instance, a value of
    ///
    /// "www.website.com/images/hello.png"
    ///
    /// given in the notification becomes
    ///
    /// "www.website.com/images/hello.png?ms-scale=100&ms-contrast=standard&ms-lang=en-us"
    pub fn binding_add_image_query(&self) -> Result<()> {
        let element = self.binding_get_element()?;
        element.SetAttribute(&"addImageQuery".into(), &"true".into())?;
        Ok(())
    }

    /// A default base URI that is combined with relative URIs in image source attributes.
    pub fn binding_base_uri(&self, base_uri: &str) -> Result<()> {
        let element = self.binding_get_element()?;
        element.SetAttribute(&"baseUri".into(), &base_uri.into())?;
        Ok(())
    }

    /// A template to use if the primary template cannot be found, for use with backward compatibility.
    pub fn binding_fallback(&self, fallback: &str) -> Result<()> {
        let element = self.binding_get_element()?;
        element.SetAttribute(&"fallback".into(), &fallback.into())?;
        Ok(())
    }
}
