use windows::{
    core::HSTRING,
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

use crate::{
    error::{Error, Result, XmlErr},
    utils,
};



impl Toast {
    /// This can be used if you do not have a AppUserModelID.
    ///
    /// However, the toast will erroniously report its origin as powershell.
    pub const POWERSHELL_APP_ID: &'static str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\
                                                 \\WindowsPowerShell\\v1.0\\powershell.exe";

    pub fn new() -> Result<Toast> {
        let doc = XmlDocument::new()?;

        doc.LoadXml(&HSTRING::from(
            "<toast> 
                <visual>
                    <binding template=\"ToastGeneric\">
                        <text id=\"0\"></text>
                    </binding>
                </visual>
            </toast>",
        ))?;

        Ok(Toast {
            doc,
            app_id: Toast::POWERSHELL_APP_ID.into(),
        })
    }

    /// Set the duration of the toast notification
    pub fn duration(&self, duration: utils::toast::Duration) -> Result<()> {
        let duration = match duration {
            utils::toast::Duration::Short => "short",
            utils::toast::Duration::Long => "long",
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
    pub fn scenario(&self, scenario: utils::toast::Scenarios) -> Result<()> {
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



    /// Set the text of the toast notification.
    /// If the id already exists, it will be overwritten.
    /// id 0 is reserved for the title.
    pub fn text(&mut self, text: &str, id: i32) -> Result<()> {
        if id == 0 {
            return Err(Error::XmlErr(XmlErr::InvatedArg(
                "id 0 is reserved for the title".into(),
            )));
        }
        if let Ok(node) = self
            .doc
            .SelectSingleNode(&format!("//*[@id='{}']", id).into())
        {
            return Ok(node.SetInnerText(&text.into())?);
        } else {
            let text_node = self.doc.CreateElement(&"text".into()).unwrap();
            text_node.SetInnerText(&text.into()).unwrap();
            text_node.SetAttribute(&"id".into(), &id.to_string().into())?;
            self.doc
                .SelectSingleNode(&"/toast/visual/binding".into())?
                .AppendChild(&text_node)?;
            return Ok(());
        }
    }

    /// Set the bottom text of the toast notification.
    /// only one will will be shown at a time.
    pub fn bottom_text(&mut self, text: &str, id: i32) -> Result<()> {
        if id == 0 {
            return Err(Error::XmlErr(XmlErr::InvatedArg(
                "id 0 is reserved for the title".into(),
            )));
        }
        if let Ok(node) = self
            .doc
            .SelectSingleNode(&format!("//*[@id='{}']", id).into())
        {
            return Ok(node.SetInnerText(&text.into())?);
        } else {
            let text_node = self.doc.CreateElement(&"text".into()).unwrap();
            text_node.SetInnerText(&text.into()).unwrap();
            text_node.SetAttribute(&"id".into(), &id.to_string().into())?;
            text_node.SetAttribute(&"placement".into(), &"attribution".into())?;
            self.doc
                .SelectSingleNode(&"/toast/visual/binding".into())?
                .AppendChild(&text_node)?;
            return Ok(());
        }
    }

    /// Set the AppUserModelID for the toast.
    pub fn app_id(&mut self, app_id: &str) -> Result<()> {
        self.app_id = app_id.into();
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        self.print();
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&self.app_id)?;
        let toast = ToastNotification::CreateToastNotification(&self.doc)?;
        let res = notifier.Show(&toast);

        std::thread::sleep(std::time::Duration::from_millis(10));
        res?;

        Ok(())
    }

    pub fn print(&self) {
        println!("{}", self.doc.GetXml().unwrap());
    }
}
