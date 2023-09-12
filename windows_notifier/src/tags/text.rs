use crate::error::{Error, Result, XmlErr};

use crate::utils::into_raw::ToXML;
use crate::Toast;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Text {
    /// The text element in the toast template that this text is intended for.
    /// If a template has only one text element, then this value is 1.
    /// The number of available text positions is based on the template definition.
    id: Option<i32>,
    /// The text to display.
    text: String,
    /// the text is always displayed at the bottom of your notification, along with your app's identity or the notification's timestamp.
    ///
    /// only one will will be shown at a time.
    bottem_text: bool,
    /// Set to "true" to center the text for incoming call notifications.
    /// This value is only used for notifications with with a scenario value of "incomingCall";
    /// otherwise, it is ignored. For more information, see Toast content.
    hint_call_scenario_center_align: bool,
}

impl Text {
    /// Create a new text element.
    pub fn new(text: impl Into<String>) -> Text {
        Text {
            id: None,
            text: text.into(),
            bottem_text: false,
            hint_call_scenario_center_align: false,
        }
    }

    /// Set the id of the text element.
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    /// the text is always displayed at the bottom of your notification, along with your app's identity or the notification's timestamp.
    /// only one will will be shown at a time.
    pub fn bottem_text(mut self) -> Self {
        self.bottem_text = true;
        self
    }

    /// Set to "true" to center the text for incoming call notifications.
    /// This value is only used for notifications with with a scenario value of "incomingCall";
    /// otherwise, it is ignored. For more information, see Toast content.
    pub fn hint_call_scenario_center_align(mut self) -> Self {
        self.hint_call_scenario_center_align = true;
        self
    }
}

impl ToXML for Text {
    fn into_raw(
        self,
        doc: &windows::Data::Xml::Dom::XmlDocument,
    ) -> Result<windows::Data::Xml::Dom::XmlElement> {
        if self.id == Some(0) {
            return Err(Error::XmlErr(XmlErr::InvatedArg(
                "id 0 is reserved for the title".into(),
            )));
        }
        let text_node = doc.CreateElement(&"text".into()).unwrap();
        text_node.SetInnerText(&self.text.into()).unwrap();
        if let Some(id) = self.id {
            text_node.SetAttribute(&"id".into(), &id.to_string().into())?;
        }
        if self.bottem_text {
            text_node.SetAttribute(&"placement".into(), &"attribution".into())?;
        }
        if self.hint_call_scenario_center_align {
            text_node.SetAttribute(&"hint-callScenarioCenterAlign".into(), &"true".into())?;
        }
        Ok(text_node)
    }
}

impl Toast {
    /// Set the title of the toast notification.
    pub fn title(&mut self, title: &str) -> Result<()> {
        self.doc
            .SelectSingleNode(&"//*[@id='0']".into())?
            .SetInnerText(&title.into())?;
        Ok(())
    }

    /// Set the text of the toast notification.
    /// If the id already exists, it will be overwritten.
    /// id 0 is reserved for the title.
    pub fn add_text(&mut self, text: Text) -> Result<()> {
        self.doc
            .SelectSingleNode(&"/toast/visual/binding".into())?
            .AppendChild(&text.into_raw(&self.doc)?)?;
        return Ok(());
    }
}
