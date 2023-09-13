use crate::error::Result;
use crate::Toast;

/// Decides the type of activation that will be used when the user interacts with a specific action.
#[derive(Debug, Clone, Copy, Default)]
pub enum ActivationType {
    /// Default value. Your foreground app is launched.
    #[default]
    Forground,
    ///  Your corresponding background task is triggered, and you can execute code in the background without interrupting the user.
    Background,
    /// Launch a different app using protocol activation.
    Protocol,
}

impl ActivationType {
    fn to_string(&self) -> String {
        match self {
            Self::Forground => "foreground".to_string(),
            Self::Background => "background".to_string(),
            Self::Protocol => "protocol".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
    Success,
    Critical,
}

impl ButtonStyle {
    fn to_string(&self) -> String {
        match self {
            Self::Success => "Success".to_string(),
            Self::Critical => "Critical".to_string(),
        }
    }
}

pub struct Action {
    ///  The content displayed on the button.
    content: String,
    /// App-defined string of arguments that the app will later receive if the user clicks this button.
    arguments: String,
    /// Decides the type of activation that will be used when the user interacts with a specific action.
    activation_type: Option<ActivationType>,
    /// When set to true, the action becomes a context menu action added to the toast notification's context menu rather than a traditional toast button.
    context_menu: bool,
    /// The URI of the image source for a toast button icon. These icons are white transparent 16x16 pixel images at 100% scaling and should have no padding included in the image itself. If you choose to provide icons on a toast notification, you must provide icons for ALL of your buttons in the notification, as it transforms the style of your buttons into icon buttons. Use one of the following protocol handlers:
    ///
    /// - http:// or https:// - A web-based image.
    /// - ms-appx:/// - An image included in the app package.
    /// - ms-appdata:///local/ - An image saved to local storage.
    /// - file:/// - A local image. (Supported only for desktop apps. This protocol cannot be used by UWP apps.)
    image_uri: Option<String>,
    /// Set to the Id of an input to position button beside the input.
    hint_input_id: Option<String>,
    /// The button style. useButtonStyle must be set to true in the toast element.
    ///
    /// - "Success" - The button is green
    /// - "Critical" - The button is red.
    hint_button_style: Option<ButtonStyle>,
    /// The tooltip for a button, if the button has an empty content string.
    hint_tooltip: Option<String>,
}

impl Action {
    pub fn new(content: String, arguments: String) -> Self {
        Self {
            content,
            arguments,
            activation_type: None,
            context_menu: false,
            image_uri: None,
            hint_input_id: None,
            hint_button_style: None,
            hint_tooltip: None,
        }
    }

    pub fn activation_type(mut self, activation_type: ActivationType) -> Self {
        self.activation_type = Some(activation_type);
        self
    }

    pub fn context_menu(mut self) -> Self {
        self.context_menu = true;
        self
    }

    pub fn image_uri(mut self, image_uri: String) -> Self {
        self.image_uri = Some(image_uri);
        self
    }

    pub fn hint_input_id(mut self, hint_input_id: String) -> Self {
        self.hint_input_id = Some(hint_input_id);
        self
    }

    pub fn hint_button_style(mut self, hint_button_style: ButtonStyle) -> Self {
        self.hint_button_style = Some(hint_button_style);
        self
    }

    pub fn hint_tooltip(mut self, hint_tooltip: String) -> Self {
        self.hint_tooltip = Some(hint_tooltip);
        self
    }
}

impl<S> Toast<S> {
    /// add custom action to the toast notification.
    pub fn add_action(&self, action: Action) -> Result<()> {
        let action_node = self.doc.CreateElement(&"action".into()).unwrap();
        action_node.SetAttribute(&"content".into(), &action.content.into())?;
        action_node.SetAttribute(&"arguments".into(), &action.arguments.into())?;

        if let Some(activation_type) = action.activation_type {
            action_node.SetAttribute(
                &"activationType".into(),
                &activation_type.to_string().into(),
            )?;
        }

        if action.context_menu {
            action_node.SetAttribute(&"placement".into(), &"contextMenu".into())?;
        }

        if let Some(image_uri) = action.image_uri {
            action_node.SetAttribute(&"imageUri".into(), &image_uri.into())?;
        }

        if let Some(hint_input_id) = action.hint_input_id {
            action_node.SetAttribute(&"hint-inputId".into(), &hint_input_id.into())?;
        }

        if let Some(hint_button_style) = action.hint_button_style {
            action_node.SetAttribute(
                &"hint-buttonStyle".into(),
                &hint_button_style.to_string().into(),
            )?;
        }

        if let Some(hint_tooltip) = action.hint_tooltip {
            action_node.SetAttribute(&"hint-tooltip".into(), &hint_tooltip.into())?;
        }

        self.doc
            .SelectSingleNode(&"/toast/actions".into())?
            .AppendChild(&action_node)?;

        Ok(())
    }
}
