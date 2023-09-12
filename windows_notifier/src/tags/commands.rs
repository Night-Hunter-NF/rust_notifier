use crate::error::Result;
use crate::Toast;

#[derive(Debug, Clone, Copy)]
pub enum CallCommands {
    Video,
    Voice,
    Decline,
}

#[derive(Debug, Clone, Copy)]
pub enum AlarmCommands {
    Snooze,
    Dismiss,
}

#[derive(Debug, Clone, Copy)]
pub enum Senario {
    Alarm(AlarmCommands),
    IncomingCall(CallCommands),
}

impl Toast {
    /// Senario: IncomingCall
    /// * `command` - Specifies one command from the system-defined command list. These values correspond to available actions that the user can take. Two scenarios are available through the commands element.
    /// * `arguments` - An argument string that can be passed to the associated app to provide specifics about the action that it should execute in response to the user action.
    pub fn add_command(&self, scenario: Senario, arguments: String) -> Result<()> {
        let commands = self.doc.CreateElement(&"commands".into())?;

        let scenario_str = match scenario {
            Senario::Alarm(command) => "alarm",
            Senario::IncomingCall(command) => "incomingCall",
        };
        commands.SetAttribute(&"scenario".into(), &scenario_str.into())?;

        let text_node = self.doc.CreateElement(&"command".into()).unwrap();

        let id = match scenario {
            Senario::Alarm(command) => match command {
                AlarmCommands::Snooze => "snooze",
                AlarmCommands::Dismiss => "dismiss",
            },
            Senario::IncomingCall(command) => match command {
                CallCommands::Video => "video",
                CallCommands::Voice => "voice",
                CallCommands::Decline => "decline",
            },
        };

        text_node.SetAttribute(&"id".into(), &id.into())?;
        text_node.SetAttribute(&"arguments".into(), &arguments.into())?;

        commands.AppendChild(&text_node)?;

        self.doc
            .SelectSingleNode(&"/toast".into())?
            .AppendChild(&commands)?;
        Ok(())
    }
}
