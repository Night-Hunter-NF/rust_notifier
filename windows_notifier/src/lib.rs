use crate::error::Result;
use utils::into_raw::ToXML;
use windows::{
    core::HSTRING,
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

pub mod error;
// pub mod new;
pub mod tags;
mod utils;
mod windows_check;

pub struct Toast {
    doc: XmlDocument,
    app_id: HSTRING,
}

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
                <visual id=\"visual\">
                    <binding template=\"ToastGeneric\">
                        <text id=\"0\"></text>
                    </binding>
                </visual>
                <actions>
                </actions>
            </toast>",
        ))?;

        Ok(Toast {
            doc,
            app_id: Toast::POWERSHELL_APP_ID.into(),
        })
    }

    /// Set the AppUserModelID for the toast.
    pub fn app_id(&mut self, app_id: &str) -> Result<()> {
        self.app_id = app_id.into();
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        println!("{:?}", self.into_raw()?);
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&self.app_id)?;
        let toast = ToastNotification::CreateToastNotification(&self.doc)?;
        let res = notifier.Show(&toast);

        std::thread::sleep(std::time::Duration::from_millis(10));

        Ok(res?)
    }

    pub fn into_raw(&self) -> Result<HSTRING> {
        Ok(self.doc.GetXml()?)
    }
}
