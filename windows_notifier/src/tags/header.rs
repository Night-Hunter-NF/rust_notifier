use crate::Toast;
use crate::error::Result;

/// The type of activation this header will use when clicked.
#[derive(Debug, Clone, Copy, Default)]
pub enum ActivationType {
    /// Default value. Your foreground app is launched.
    #[default]
    Forground,
    /// Launch a different app using protocol activation.
    Protocol,
}

impl ActivationType {
    fn to_string(&self) -> String {
        match self {
            Self::Forground => "foreground".to_string(),
            Self::Protocol => "protocol".to_string(),
        }
    }
}

pub struct Header {
    /// A developer-created identifier that uniquely identifies this header.
    /// If two notifications have the same header id, they will be displayed underneath the same header in Action Center.
    id: String,
    /// A title for the header.
    title: String,
    /// A developer-defined string of arguments that is returned to the app when the user clicks this header. Cannot be null.
    arguments: String,
    /// The type of activation this header will use when clicked.
    activation_type: Option<ActivationType>,
}

impl Header {
    pub fn new(id: &str, title: &str, arguments: &str) -> Header {
        Header {
            id: id.into(),
            title: title.into(),
            arguments: arguments.into(),
            activation_type: None,
        }
    }

    /// The type of activation this header will use when clicked.
    pub fn activation_type(mut self, activation_type: ActivationType) -> Self {
        self.activation_type = Some(activation_type);
        self
    }
}

impl Toast {
    pub fn add_header(&self, header: Header) -> Result<()> {
        let header_node = self.doc.CreateElement(&"header".into())?;
        header_node.SetAttribute(&"id".into(), &header.id.into())?;
        header_node.SetAttribute(&"title".into(), &header.title.into())?;
        header_node.SetAttribute(&"arguments".into(), &header.arguments.into())?;
        if let Some(activation_type) = header.activation_type {
            header_node.SetAttribute(
                &"activationType".into(),
                &activation_type.to_string().into(),
            )?;
        }
        self.doc
            .SelectSingleNode(&"/toast".into())?
            .AppendChild(&header_node)?;
        Ok(())
    }
}
