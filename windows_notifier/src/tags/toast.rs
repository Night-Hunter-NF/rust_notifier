use crate::error::Result;
use crate::Toast;

/// Duraction of the toast notification.
#[derive(Debug, Clone, Copy)]
pub enum Duration {
    /// 7 seconds
    Short,

    /// 25 seconds
    Long,
}

/// The scenario your toast is used for, like an alarm or reminder.
#[derive(Debug, Clone, Copy)]
pub enum Scenarios {
    ///  reminder notification.
    /// This will be displayed pre-expanded and stay on the user's screen till dismissed.
    /// Note that this will be silently ignored unless there's a toast button action that activates in background.
    Reminder,
    /// An alarm notification.
    /// This will be displayed pre-expanded and stay on the user's screen till dismissed.
    /// Audio will loop by default and will use alarm audio.
    Alarm,
    /// An incoming call notification.
    /// This will be displayed pre-expanded in a special call format and stay on the user's screen till dismissed.
    /// Audio will loop by default and will use ringtone audio.
    IncomingCall,
    ///  An important notification.
    /// This allows users to have more control over what apps can send them high-priority toast notifications that can break through Focus Assist (Do not Disturb).
    /// This can be modified in the notifications settings.
    Urgent,
}

impl Scenarios {
    /// Converts the scenario to a string.
    pub fn to_string(&self) -> String {
        match self {
            Scenarios::Reminder => "reminder".into(),
            Scenarios::Alarm => "alarm".into(),
            Scenarios::IncomingCall => "incomingCall".into(),
            Scenarios::Urgent => "urgent".into(),
        }
    }
}

impl Toast {
    /// Set the duration of the toast notification
    pub fn duration(&self, duration: Duration) -> Result<()> {
        let duration = match duration {
            Duration::Short => "short",
            Duration::Long => "long",
        };
        self.doc
            .DocumentElement()?
            .SetAttribute(&"duration".into(), &duration.into())?;

        Ok(())
    }

    /// A string that is passed to the application when it is activated by the toast.
    /// The format and contents of this string are defined by the app for its own use.
    /// When the user taps or clicks the toast to launch its associated app,
    /// the launch string provides the context to the app that allows it to show the user a view relevant to the toast content, rather than launching in its default way.
    pub fn launch(&self, launch: &str) -> Result<()> {
        self.doc
            .DocumentElement()?
            .SetAttribute(&"launch".into(), &launch.into())?;

        Ok(())
    }

    /// The scenario your toast is used for, like an alarm or reminder.
    pub fn scenario(&self, scenario: Scenarios) -> Result<()> {
        self.doc
            .DocumentElement()?
            .SetAttribute(&"scenario".into(), &scenario.to_string().into())?;

        Ok(())
    }

    /// Specifies whether styled buttons should be used.
    /// The styling of the button is determined by the **hint-buttonStyle** attribute of the [action](element-action.md) element.
    pub fn styled_button(&self, styled_button: bool) -> Result<()> {
        self.doc
            .DocumentElement()?
            .SetAttribute(&"useButtonStyle".into(), &styled_button.to_string().into())?;

        Ok(())
    }
}
