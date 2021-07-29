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
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut list_card = ListCard::new(format!("리스트 카드 제목!")); // 제목
///
/// list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼".to_string())));
///
/// list_card.add_button(Button::Link(
///     LinkButton::new("link label".to_string()).set_link("https://google.com".to_string()),
/// ));
/// list_card.add_button(Button::Share(
///     ShareButton::new("share label".to_string()).set_msg("카톡에 보이는 메시지".to_string()),
/// ));
///
/// ```
pub enum Button {
    Call(CallButton),
    Link(LinkButton),
    Share(ShareButton),
    Msg(MsgButton),
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
/// # Examples
///
/// Basic usage:
///
/// ```
///
/// Button::Call(CallButton::new("전화하기".to_string()).set_number("010-1234-5678".to_string()));
///
/// ```
pub struct CallButton {
    label: String,
    action: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl CallButton {
    pub fn set_number(mut self, number: String) -> Self {
        self.phone_number = number;
        self
    }

    pub fn new(label: String) -> Self {
        CallButton {
            label: label,
            action: "phone".to_string(),
            phone_number: "0".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// MsgButton: 그냥 텍스트 버튼
///
/// # Examples
///
/// Basic usage:
///
/// ```
///
/// Button::Msg(MsgButton::new("그냥 텍스트 버튼".to_string()));
///
/// ```
pub struct MsgButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl MsgButton {
    pub fn new(label: String) -> Self {
        MsgButton {
            label: label,
            action: "message".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// LinkButton: 버튼 누르면 지정된 링크로 이동
///
/// # Examples
///
/// Basic usage:
///
/// ```
///
/// Button::Link(LinkButton::new("label".to_string()).set_link("https://google.com".to_string()))
///
/// ```
pub struct LinkButton {
    label: String,
    action: String,
    web_link_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl LinkButton {
    pub fn set_link(mut self, link: String) -> Self {
        self.web_link_url = link;
        self
    }

    pub fn new(label: String) -> Self {
        LinkButton {
            label: label,
            action: "webLink".to_string(),
            web_link_url: "".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// ShareButton: 버튼 누르면 공유 옵션
///
/// # Examples
///
/// Basic usage:
///
/// ```
///
/// Button::Share(ShareButton::new("label".to_string()).set_msg("발화문임".to_string()))
///
/// ```
pub struct ShareButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl ShareButton {
    pub fn new(label: String) -> Self {
        ShareButton {
            label: label,
            action: "share".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

/***** Buttons *****/
