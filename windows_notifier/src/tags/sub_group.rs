use crate::error::Result;
use crate::utils::into_raw::ToXML;
use crate::Toast;

use super::image::Image;
use super::text::Text;

pub enum Child {
    Text(Text),
    Image(Image),
}

impl Toast {
    /// Specifies vertical columns that can contain text and images.
    pub fn add_sub_group(&self, children: Vec<Child>) -> Result<()> {
        if let Err(_) = self
            .doc
            .SelectSingleNode(&"/toast/visual/binding/group".into())
        {
            self.doc
                .SelectSingleNode(&"/toast/visual/binding".into())?
                .AppendChild(&self.doc.CreateElement(&"group".into())?)?;
        }

        let sub_group = self.doc.CreateElement(&"subgroup".into())?;

        for child in children {
            match child {
                Child::Text(text) => {
                    sub_group.AppendChild(&text.into_raw(&self.doc)?)?;
                }
                Child::Image(image) => {
                    sub_group.AppendChild(&image.into_raw(&self.doc)?)?;
                }
            }
        }

        self.doc
            .SelectSingleNode(&"/toast/visual/binding/group".into())?
            .AppendChild(&sub_group)?;
        return Ok(());
    }
}
