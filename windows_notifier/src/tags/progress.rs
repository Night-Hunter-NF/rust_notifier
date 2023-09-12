use crate::error::Result;
use crate::utils::into_raw::ToXML;
use crate::Toast;

/// The value of the progress bar.
pub enum Value {
    /// This value either be a floating point number between 0.0 and 1.0
    Floating(f32),
    /// results in a loading animation
    Indeterminate,
}

pub struct Progress {
    /// An optional title string.
    title: Option<String>,
    /// A status string that is displayed underneath the progress bar on the left. This string should reflect the status of the operation, like "Downloading..." or "Installing..."
    status: String,
    /// The value of the progress bar.
    value: Value,
    /// An optional string to be displayed instead of the default percentage string. If this isn't provided, something like "70%" will be displayed.
    value_string_override: Option<String>,
}

impl Progress {
    pub fn new(status: &str, value: Value) -> Progress {
        Progress {
            title: None,
            status: status.into(),
            value,
            value_string_override: None,
        }
    }

    /// An optional title string.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }

    /// An optional string to be displayed instead of the default percentage string. If this isn't provided, something like "70%" will be displayed.
    pub fn value_string_override(mut self, value_string_override: &str) -> Self {
        self.value_string_override = Some(value_string_override.into());
        self
    }
}

impl ToXML for Progress {
    fn into_raw(
        self,
        doc: &windows::Data::Xml::Dom::XmlDocument,
    ) -> Result<windows::Data::Xml::Dom::XmlElement> {
        let progress_node = doc.CreateElement(&"progress".into())?;

        if let Some(title) = self.title {
            progress_node.SetAttribute(&"title".into(), &title.into())?;
        }

        progress_node.SetAttribute(&"status".into(), &self.status.into())?;

        match self.value {
            Value::Floating(value) => {
                progress_node.SetAttribute(&"value".into(), &value.to_string().into())?;
            }
            Value::Indeterminate => {
                progress_node.SetAttribute(&"value".into(), &"indeterminate".into())?;
            }
        }

        if let Some(value_string_override) = self.value_string_override {
            progress_node
                .SetAttribute(&"valueStringOverride".into(), &value_string_override.into())?;
        }

        Ok(progress_node)
    }
}


impl Toast {
    pub fn add_progress(&self, progress: Progress) -> Result<()> {
        let progress_node = progress.into_raw(&self.doc)?;
        self.doc
            .SelectSingleNode(&"/toast/visual/binding".into())?
            .AppendChild(&progress_node)?;
        Ok(())
    }
}