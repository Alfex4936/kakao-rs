/* Constructs below json

{
    "buttons": [
        {
            "label": "CALL LABEL",
            "action": "phone",
            "phoneNumber": "0",
            "messageText": "MESSAGE"
        },
        {
            "label": "label",
            "action": "share"
        }
    ]
}
*/
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/***** Buttons *****/
#[derive(Serialize, Debug, PartialEq)]
#[serde(untagged)]
/// [Button](https://i.kakao.com/docs/skill-response-format#button): CallButton, MsgButton, LinkButton, ShareButton
///
/// # Example
/// ```
/// // 리스트 카드
/// let mut list_card = ListCard::new("리스트 카드 제목!");
///
/// // 메시지 버튼
/// list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼")));
///
/// // 링크 버튼
/// list_card.add_button(Button::Link(
///     LinkButton::new("link label").set_link("https://google.com"),
/// ));
///
/// // 공유 버튼
/// list_card.add_button(Button::Share(
///     ShareButton::new("share label").set_msg("카톡에 보이는 메시지"),
/// ));
///
/// // 전화 버튼
/// list_card.add_button(Button::Call(
///     CallButton::new("전화하기").set_number("010-1234-5678"),
/// ));
///
/// ```
pub enum Button {
    ///
    /// ### 사용법
    /// ```
    /// Button::Call(CallButton::new(label).set_number(number))
    /// ```
    Call(CallButton),
    ///
    /// ### 사용법
    /// ```
    /// Button::Link(LinkButton::new(label).set_link(link))
    /// ```
    Link(LinkButton),
    ///
    /// ### 사용법
    /// ```
    /// Button::Share(ShareButton::new(label)).set_msg(발화문))
    /// ```
    Share(ShareButton),
    ///
    /// ### 사용법
    /// ```
    /// Button::Msg(MsgButton::new(label)));
    /// ```
    Msg(MsgButton),
}

// 버튼 간단히 만들기 위한 함수들
impl Button {
    // pub fn set_msg<S: Into<String>>(&mut self, msg: S) {
    //     match self {
    //         Self::Link(link) => match link {
    //             LinkButton {
    //                 ref mut message_text,
    //                 ..
    //             } => *message_text = Some(msg.into()),
    //         },
    //         Self::Share(share) => match share {
    //             ShareButton {
    //                 ref mut message_text,
    //                 ..
    //             } => *message_text = Some(msg.into()),
    //         },
    //         Self::Msg(msgs) => match msgs {
    //             MsgButton {
    //                 ref mut message_text,
    //                 ..
    //             } => *message_text = Some(msg.into()),
    //         },
    //         Self::Call(call) => match call {
    //             CallButton {
    //                 ref mut message_text,
    //                 ..
    //             } => *message_text = Some(msg.into()),
    //         },
    //     }
    // }

    ///
    /// # CallButton(라벨, 번호)
    ///
    #[inline]
    pub fn init_call_button<S: Into<String>>(number: S, label: S) -> Button {
        Button::Call(
            CallButton::new(label.into()).set_number(number.into()),
            // .set_msg(msg.unwrap_or(label.to_owned()).into()),
        )
    }

    ///
    /// # LinkButton(라벨, 링크)
    ///
    #[inline]
    pub fn init_link_button<S: Into<String>>(link: S, label: S) -> Button {
        Button::Link(LinkButton::new(label.into()).set_link(link.into()))
    }

    ///
    /// # ShareButton(라벨)
    ///
    #[inline]
    pub fn init_share_button<S: Into<String>>(label: S) -> Button {
        Button::Share(ShareButton::new(label))
    }

    ///
    /// # MsgButton(라벨)
    ///
    #[inline]
    pub fn init_msg_button<S: Into<String>>(label: S) -> Button {
        Button::Msg(MsgButton::new(label))
    }
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text: Map<String, Value> = Map::deserialize(deserializer)?;
        let mut keys = HashMap::new();
        for (key, value) in &text {
            let _value = value.as_str().unwrap();
            keys.insert(key.to_owned(), _value.to_string());
        }

        let mut button: Button = match text.get("action").unwrap().as_str() {
            Some("webLink") => Self::Link(LinkButton::new("label".to_string())),
            Some("share") => Self::Share(ShareButton::new("label".to_string())),
            Some("message") => Self::Msg(MsgButton::new("label".to_string())),
            Some("phone") => Self::Call(CallButton::new("label".to_string())),
            _ => Self::Msg(MsgButton::new("label".to_string())),
        };

        for (key, value) in &text {
            let _value = value.as_str().unwrap();
            match &mut button {
                Self::Link(link) => match link {
                    LinkButton {
                        ref mut label,
                        ref action,
                        ref mut web_link_url,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("webLinkUrl") {
                            *web_link_url = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Share(share) => match share {
                    ShareButton {
                        ref mut label,
                        ref action,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Msg(msg) => match msg {
                    MsgButton {
                        ref mut label,
                        ref action,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Call(call) => match call {
                    CallButton {
                        ref mut label,
                        ref action,
                        ref mut phone_number,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("phoneNumber") {
                            *phone_number = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
            }
        }

        Ok(button)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// CallButton: 전화번호 action
///
/// Fn: new(label), set_number(number), set_label(label), set_msg(msg)
///
/// # Example
/// ```
/// Button::Call(CallButton::new("전화하기").set_number("010-1234-5678"));
/// ```
pub struct CallButton {
    label: String,
    action: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl CallButton {
    /// new(라벨)
    #[inline]
    pub fn new<S: Into<String>>(label: S) -> Self {
        CallButton {
            label: label.into(),
            action: "phone".to_string(),
            phone_number: "0".to_string(),
            message_text: None,
        }
    }

    pub fn set_number<S: Into<String>>(mut self, number: S) -> Self {
        self.phone_number = number.into();
        self
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_msg<S: Into<String>>(mut self, msg: S) -> Self {
        self.message_text = Some(msg.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// # MsgButton
///
/// 그냥 텍스트 버튼
///
/// (메시지가 없으면 라벨이 발화문)
///
/// Fn: new(label), set_label(label), set_msg(msg)
///
/// # Example

/// ```
/// Button::Msg(MsgButton::new("그냥 텍스트 버튼"));
/// ```
pub struct MsgButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl MsgButton {
    /// new(라벨)
    #[inline]
    pub fn new<S: Into<String>>(label: S) -> Self {
        MsgButton {
            label: label.into(),
            action: "message".to_string(),
            message_text: None,
        }
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_msg<S: Into<String>>(mut self, msg: S) -> Self {
        self.message_text = Some(msg.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// # LinkButton
///
/// 버튼 누르면 지정된 링크로 이동
///
/// (메시지가 없으면 라벨이 발화문)
///
/// Fn: new(label), set_link(link), set_label(label), set_msg(msg)
///
/// # Example
/// ```
/// Button::Link(LinkButton::new("label").set_link("https://google.com"))
/// ```
pub struct LinkButton {
    label: String,
    action: String,
    web_link_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl LinkButton {
    /// new(라벨)
    #[inline]
    pub fn new<S: Into<String>>(label: S) -> Self {
        LinkButton {
            label: label.into(),
            action: "webLink".to_string(),
            web_link_url: "".to_string(),
            message_text: None,
        }
    }

    pub fn set_link<S: Into<String>>(mut self, link: S) -> Self {
        self.web_link_url = link.into();
        self
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_msg<S: Into<String>>(mut self, msg: S) -> Self {
        self.message_text = Some(msg.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// # ShareButton
///
/// 버튼 누르면 공유 옵션
///
/// (메시지가 없으면 라벨이 발화문)
///
/// Fn: new(label), set_label(label), set_msg(msg)
///
/// # Example
/// ```
/// Button::Share(ShareButton::new("label").set_msg("발화문임"))
/// ```
pub struct ShareButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl ShareButton {
    /// new(라벨)
    #[inline]
    pub fn new<S: Into<String>>(label: S) -> Self {
        ShareButton {
            label: label.into(),
            action: "share".to_string(),
            message_text: None,
        }
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_msg<S: Into<String>>(mut self, msg: S) -> Self {
        self.message_text = Some(msg.into());
        self
    }
}

/***** Buttons *****/

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OneButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

pub enum ButtonType {
    Call,
    Share,
    Link,
    Text,
}

impl OneButton {
    #[inline]
    fn new(r#type: ButtonType) -> Self {
        let action = match r#type {
            ButtonType::Call => "phone",
            ButtonType::Share => "share",
            ButtonType::Link => "webLink",
            ButtonType::Text => "message",
        };
        OneButton {
            action: action.to_string(),
            ..Default::default()
        }
    }

    pub fn set_number<S: Into<String>>(mut self, number: S) -> Self {
        self.phone_number = Some(number.into());
        self
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_msg<S: Into<String>>(mut self, message: S) -> Self {
        self.message_text = Some(message.into());
        self
    }

    pub fn set_link<S: Into<String>>(mut self, link: S) -> Self {
        self.web_link_url = Some(link.into());
        self
    }
}

impl<'de> Deserialize<'de> for OneButton {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text: Map<String, Value> = Map::deserialize(deserializer)?;
        let mut keys = HashMap::new();
        for (key, value) in &text {
            let _value = value.as_str().unwrap();
            keys.insert(key.to_owned(), _value.to_string());
        }

        let mut button: OneButton = match text.get("action").unwrap().as_str() {
            Some("webLink") => OneButton::new(ButtonType::Link),
            Some("share") => OneButton::new(ButtonType::Share),
            Some("message") => OneButton::new(ButtonType::Text),
            Some("phone") => OneButton::new(ButtonType::Call),
            _ => panic!("Unknown button type"),
        };

        if let Some(l) = keys.get("label") {
            button.label = l.to_string();
        }
        if let Some(l) = keys.get("webLinkUrl") {
            button.web_link_url = Some(l.to_string());
        }
        if let Some(l) = keys.get("messageText") {
            button.message_text = Some(l.to_string());
        }
        if let Some(l) = keys.get("phoneNumber") {
            button.phone_number = Some(l.to_string());
        }

        Ok(button)
    }
}

#[test]
fn test_button() {
    // let a = OneButton::new(ButtonType::Call)
    //     .set_label("전화하기")
    //     .set_number("911");
    // let b = OneButton::new(ButtonType::Share).set_label("공유하기");
    // let c = OneButton::new(ButtonType::Link)
    //     .set_label("링크열기")
    //     .set_link("https://");
    // let d = OneButton::new(ButtonType::Text)
    //     .set_label("그냥 버튼")
    //     .set_msg("발화문임");

    let data = r#"[{"label":"CALL LABEL","action":"phone","phoneNumber":"0","messageText":"MESSAGE"},{"label":"SHARE LABEL","action":"share"},{"label":"MSG LABEL","action":"message"},{"label":"LABEL","action":"webLink","webLinkUrl":"https://"}]"#;
    let buttons: Vec<OneButton> = serde_json::from_str(data).unwrap();

    assert_eq!(buttons.len(), 4);
    assert_eq!(buttons[0].action, "phone");
    assert_eq!(serde_json::to_string(&buttons).expect("Failed"), data); // Serialize
}
