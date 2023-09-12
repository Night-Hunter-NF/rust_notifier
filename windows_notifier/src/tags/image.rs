use crate::error::Result;
use crate::utils::into_raw::ToXML;
use crate::Toast;

/// The placement of the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Placement {
    /// The image replaces your app's logo in the toast notification.
    AppLogoOverride,
    ///  The image is displayed as a hero image.
    Hero,
}

impl Placement {
    fn to_string(&self) -> String {
        match self {
            Placement::AppLogoOverride => "appLogoOverride".into(),
            Placement::Hero => "hero".into(),
        }
    }
}

/// The cropping of the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Crop {
    /// The image is not cropped and displayed as a square.
    None,
    /// The image is cropped into a circle.
    Circle,
}

impl Crop {
    fn to_string(&self) -> String {
        match self {
            Crop::None => "".into(),
            Crop::Circle => "circle".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    /// Set to "true" to allow Windows to append a query string to the image URI supplied in the toast notification. Use this attribute if your server hosts images and can handle query strings, either by retrieving an image variant based on the query strings or by ignoring the query string and returning the image as specified without the query string. This query string specifies scale, contrast setting, and language; for instance, a value of
    ///
    /// "www.website.com/images/hello.png"
    ///
    /// given in the notification becomes
    ///
    /// "www.website.com/images/hello.png?ms-scale=100&ms-contrast=standard&ms-lang=en-us"
    add_image_query: bool,
    /// A description of the image, for users of assistive technologies.
    alt: String,
    /// The image element in the toast template that this image is intended for. If a template has only one image, then this value is 1. The number of available image positions is based on the template definition.
    id: Option<i32>,
    /// The URI of the image source, using one of these protocol handlers:
    ///
    ///    http:// or https://
    ///
    ///    A web-based image.
    ///
    ///    ms-appx:///
    ///
    ///    An image included in the app package.
    ///
    ///    ms-appdata:///local/
    ///
    ///    An image saved to local storage.
    ///
    ///    file:///
    ///
    ///    A local image. (Supported only for desktop apps. This protocol cannot be used by UWP apps.)
    src: String,
    /// The placement of the image.
    placement: Placement,
    /// The cropping of the image.
    hint_crop: Crop,
}

impl Image {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            add_image_query: false,
            alt: "".into(),
            id: None,
            src: src.into(),
            placement: Placement::AppLogoOverride,
            hint_crop: Crop::None,
        }
    }

    pub fn set_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn set_alt(mut self, alt: String) -> Self {
        self.alt = alt;
        self
    }

    pub fn set_placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn set_hint_crop(mut self, hint_crop: Crop) -> Self {
        self.hint_crop = hint_crop;
        self
    }
}

impl ToXML for Image {
    fn into_raw(
        self,
        doc: &windows::Data::Xml::Dom::XmlDocument,
    ) -> Result<windows::Data::Xml::Dom::XmlElement> {
        let text_node = doc.CreateElement(&"image".into()).unwrap();
        text_node.SetAttribute(
            &"addImageQuery".into(),
            &self.add_image_query.to_string().into(),
        )?;
        text_node.SetAttribute(&"alt".into(), &self.alt.into())?;
        if let Some(id) = self.id {
            text_node.SetAttribute(&"id".into(), &id.to_string().into())?;
        }
        text_node.SetAttribute(&"src".into(), &self.src.into())?;
        text_node.SetAttribute(&"placement".into(), &self.placement.to_string().into())?;
        text_node.SetAttribute(&"hint-crop".into(), &self.hint_crop.to_string().into())?;
        return Ok(text_node);
    }
}

impl Toast {
    pub fn add_image(&self, image: Image) -> Result<()> {
        self.doc
            .SelectSingleNode(&"/toast/visual/binding".into())?
            .AppendChild(&image.into_raw(&self.doc)?)?;
        return Ok(());
    }
}
