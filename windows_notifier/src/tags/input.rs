use crate::error::Result;
use crate::utils::into_raw::ToXML;
use crate::Toast;

#[derive(Debug, Clone)]
pub struct Selection {
    /// The ID associated with the selection.
    id: String,
    /// The text displayed for the selection.
    content: String,
}

#[derive(Debug, Clone)]
pub enum InputType {
    Text,
    Selection(Vec<Selection>),
}

pub enum Input {
    Text {
        /// The ID associated with the input.
        id: String,
        /// Text displayed as a label for the input.
        title: Option<String>,
        /// The placeholder displayed for text input.
        place_holder_content: Option<String>,
    },
    Selection {
        /// The ID associated with the input.
        id: String,
        /// Text displayed as a label for the input.
        title: Option<String>,
        /// The ID of the selection that is selected by default.
        default_selection: Option<String>,
        /// A list of selections that the user can choose from.
        selections: Vec<Selection>,
    },
}

impl Input {
    pub fn new_text(id: impl Into<String>, place_holder_content: Option<impl Into<String>>) -> Input {
        Input::Text {
            id: id.into(),
            title: None,
            place_holder_content: place_holder_content.map(|s| s.into()),
        }
    }

    pub fn new_selection(
        id: impl Into<String>,
        selections: Vec<Selection>,
        default_selection: Option<impl Into<String>>,
    ) -> Input {
        Input::Selection {
            id: id.into(),
            title: None,
            default_selection: default_selection.map(|s| s.into()),
            selections,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        match self {
            Input::Text {
                id,
                place_holder_content,
                ..
            } => {
                self = Input::Text {
                    id,
                    title: Some(title.into()),
                    place_holder_content,
                };
            }
            Input::Selection {
                id,
                default_selection,
                selections,
                ..
            } => {
                self = Input::Selection {
                    id,
                    title: Some(title.into()),
                    default_selection,
                    selections,
                };
            }
        }
        self
    }
}

impl ToXML for Input {
    fn into_raw(
        self,
        doc: &windows::Data::Xml::Dom::XmlDocument,
    ) -> Result<windows::Data::Xml::Dom::XmlElement> {
        match self {
            Input::Text {
                id,
                title,
                place_holder_content,
            } => {
                let text_input_node = doc.CreateElement(&"input".into()).unwrap();
                text_input_node.SetAttribute(&"id".into(), &id.into())?;
                text_input_node.SetAttribute(&"type".into(), &"text".into())?;

                if let Some(title) = title {
                    text_input_node.SetAttribute(&"title".into(), &title.into())?;
                }

                if let Some(place_holder_content) = place_holder_content {
                    text_input_node
                        .SetAttribute(&"placeHolderContent".into(), &place_holder_content.into())?;
                }

                return Ok(text_input_node);
            }
            Input::Selection {
                id,
                title,
                default_selection,
                selections,
            } => {
                let selection_input_node = doc.CreateElement(&"input".into()).unwrap();
                selection_input_node.SetAttribute(&"id".into(), &id.into())?;
                selection_input_node.SetAttribute(&"type".into(), &"selection".into())?;

                if let Some(title) = title {
                    selection_input_node.SetAttribute(&"title".into(), &title.into())?;
                }

                if let Some(default_selection) = default_selection {
                    selection_input_node
                        .SetAttribute(&"defaultInput".into(), &default_selection.into())?;
                }

                for selection in selections {
                    let selection_node = doc.CreateElement(&"selection".into()).unwrap();
                    selection_node.SetAttribute(&"id".into(), &selection.id.into())?;
                    selection_node.SetAttribute(&"content".into(), &selection.content.into())?;

                    selection_input_node.AppendChild(&selection_node)?;
                }

                return Ok(selection_input_node);
            }
        }
    }
}

impl<S> Toast<S> {
    pub fn add_input(&self, input: Input) -> Result<()> {
        let input_node = input.into_raw(&self.doc)?;
        self.doc
            .SelectSingleNode(&"/toast/actions".into())?
            .AppendChild(&input_node)?;
        Ok(())
    }
}
