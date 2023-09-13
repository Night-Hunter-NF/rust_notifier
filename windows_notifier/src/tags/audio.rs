use std::marker::PhantomData;

use windows::Data::Xml::Dom::XmlDocument;

use crate::error::Result;
use crate::utils::into_raw::ToXML;
use crate::Toast;

#[derive(Debug, Clone, Copy)]
pub enum Notification {
    /// ms-winsoundevent:Notification.Default
    Default,
    /// ms-winsoundevent:Notification.IM
    IM,
    /// ms-winsoundevent:Notification.Mail
    Mail,
    /// ms-winsoundevent:Notification.Reminder
    Reminder,
    /// ms-winsoundevent:Notification.SMS
    SMS,
    /// ms-winsoundevent:Notification.Looping.Alarm
    LoopingAlarm,
    /// ms-winsoundevent:Notification.Looping.Alarm2
    LoopingAlarm2,
    /// ms-winsoundevent:Notification.Looping.Alarm3
    LoopingAlarm3,
    /// ms-winsoundevent:Notification.Looping.Alarm4
    LoopingAlarm4,
    /// ms-winsoundevent:Notification.Looping.Alarm5
    LoopingAlarm5,
    /// ms-winsoundevent:Notification.Looping.Alarm6
    LoopingAlarm6,
    /// ms-winsoundevent:Notification.Looping.Alarm7
    LoopingAlarm7,
    /// ms-winsoundevent:Notification.Looping.Alarm8
    LoopingAlarm8,
    /// ms-winsoundevent:Notification.Looping.Alarm9
    LoopingAlarm9,
    /// ms-winsoundevent:Notification.Looping.Alarm10
    LoopingAlarm10,
    /// ms-winsoundevent:Notification.Looping.Call
    LoopingCall,
    /// ms-winsoundevent:Notification.Looping.Call2
    LoopingCall2,
    /// ms-winsoundevent:Notification.Looping.Call3
    LoopingCall3,
    /// ms-winsoundevent:Notification.Looping.Call4
    LoopingCall4,
    /// ms-winsoundevent:Notification.Looping.Call5
    LoopingCall5,
    /// ms-winsoundevent:Notification.Looping.Call6
    LoopingCall6,
    /// ms-winsoundevent:Notification.Looping.Call7
    LoopingCall7,
    /// ms-winsoundevent:Notification.Looping.Call8
    LoopingCall8,
    /// ms-winsoundevent:Notification.Looping.Call9
    LoopingCall9,
    /// ms-winsoundevent:Notification.Looping.Call10
    LoopingCall10,
}

impl Notification {
    fn to_string(&self) -> String {
        match self {
            Notification::Default => "ms-winsoundevent:Notification.Default".into(),
            Notification::IM => "ms-winsoundevent:Notification.IM".into(),
            Notification::Mail => "ms-winsoundevent:Notification.Mail".into(),
            Notification::Reminder => "ms-winsoundevent:Notification.Reminder".into(),
            Notification::SMS => "ms-winsoundevent:Notification.SMS".into(),
            Notification::LoopingAlarm => "ms-winsoundevent:Notification.Looping.Alarm".into(),
            Notification::LoopingAlarm2 => "ms-winsoundevent:Notification.Looping.Alarm2".into(),
            Notification::LoopingAlarm3 => "ms-winsoundevent:Notification.Looping.Alarm3".into(),
            Notification::LoopingAlarm4 => "ms-winsoundevent:Notification.Looping.Alarm4".into(),
            Notification::LoopingAlarm5 => "ms-winsoundevent:Notification.Looping.Alarm5".into(),
            Notification::LoopingAlarm6 => "ms-winsoundevent:Notification.Looping.Alarm6".into(),
            Notification::LoopingAlarm7 => "ms-winsoundevent:Notification.Looping.Alarm7".into(),
            Notification::LoopingAlarm8 => "ms-winsoundevent:Notification.Looping.Alarm8".into(),
            Notification::LoopingAlarm9 => "ms-winsoundevent:Notification.Looping.Alarm9".into(),
            Notification::LoopingAlarm10 => "ms-winsoundevent:Notification.Looping.Alarm10".into(),
            Notification::LoopingCall => "ms-winsoundevent:Notification.Looping.Call".into(),
            Notification::LoopingCall2 => "ms-winsoundevent:Notification.Looping.Call2".into(),
            Notification::LoopingCall3 => "ms-winsoundevent:Notification.Looping.Call3".into(),
            Notification::LoopingCall4 => "ms-winsoundevent:Notification.Looping.Call4".into(),
            Notification::LoopingCall5 => "ms-winsoundevent:Notification.Looping.Call5".into(),
            Notification::LoopingCall6 => "ms-winsoundevent:Notification.Looping.Call6".into(),
            Notification::LoopingCall7 => "ms-winsoundevent:Notification.Looping.Call7".into(),
            Notification::LoopingCall8 => "ms-winsoundevent:Notification.Looping.Call8".into(),
            Notification::LoopingCall9 => "ms-winsoundevent:Notification.Looping.Call9".into(),
            Notification::LoopingCall10 => "ms-winsoundevent:Notification.Looping.Call10".into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Audio {
    /// Set to true if the sound should repeat as long as the toast is shown; false to play only once.
    /// If this attribute is set to true, the duration attribute in the toast element must also be set.
    /// There are specific sounds provided to be used when looping.
    /// Note that UWP apps support neither looping audio nor long-duration toasts.
    loop_: bool,
    /// True to mute the sound; false to allow the toast notification sound to play.
    silent: bool,
    /// The media file to play in place of the default sound.
    src: Notification,
}

impl Audio {
    pub fn new(src: Notification) -> Self {
        Self {
            loop_: false,
            silent: false,
            src,
        }
    }

    /// Set to true if the sound should repeat as long as the toast is shown; false to play only once.
    /// If this attribute is set to true, the duration attribute in the toast element must also be set.
    /// There are specific sounds provided to be used when looping.
    /// Note that UWP apps support neither looping audio nor long-duration toasts.
    pub fn loop_(mut self) -> Self {
        self.loop_ = true;
        self
    }

    /// True to mute the sound; false to allow the toast notification sound to play.
    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }
}

impl ToXML for Audio {
    fn into_raw(self, doc: &XmlDocument) -> Result<windows::Data::Xml::Dom::XmlElement> {
        let audio_node = doc.CreateElement(&"audio".into()).unwrap();
        audio_node.SetAttribute(&"src".into(), &self.src.to_string().into())?;

        if self.loop_ {
            audio_node.SetAttribute(&"loop".into(), &"true".into())?;
        }

        if self.silent {
            audio_node.SetAttribute(&"silent".into(), &"true".into())?;
        }

        return Ok(audio_node);
    }
}

pub struct HasAudio;

impl Toast<()> {
    /// add custom audio to the toast notification.
    pub fn add_audio(self, audio: Audio) -> Result<Toast<HasAudio>> {
        self.doc
            .SelectSingleNode(&"/toast".into())?
            .AppendChild(&audio.into_raw(&self.doc)?)?;
        return Ok(Toast {
            doc: self.doc,
            app_id: self.app_id,
            phantom: PhantomData,
        });
    }
}
