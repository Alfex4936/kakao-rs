//! Button
//! struct Button, enum ButtonType
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// [Button](https://i.kakao.com/docs/skill-response-format#button): Call, Text, Link, Share
/// ButtonType::Call, ButtonType::Text, ButtonType::Link, ButtonType::Share
///
/// 라벨 지정은 필수입니다.
///
/// # Example
/// ```
/// use kakao_rs::prelude::*;
///
/// let mut list_card = ListCard::new("리스트 카드 제목!"); // 제목
///
/// list_card.add_button(Button::new(ButtonType::Text).set_label("그냥 텍스트 버튼")); // 메시지 버튼
///
/// list_card.add_button(
///   Button::new(ButtonType::Link)
///     .set_label("link label")
///     .set_link("https://google.com"),
/// ); // 링크 버튼
///
/// list_card.add_button(
///   Button::new(ButtonType::Share)
///     .set_label("share label")
///     .set_msg("카톡에 보이는 메시지"),
/// ); // 공유 버튼
///
/// list_card.add_button(
///   Button::new(ButtonType::Call)
///     .set_label("call label")
///     .set_number("010-1234-5678"),
/// ); // 전화 버튼
///
/// ```
pub struct Button {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

/// 버튼 타입 (Call, Share, Link, Text)
pub enum ButtonType {
    Call,
    Share,
    Link,
    Text,
}

impl Button {
    #[inline]
    /// ButtonType::종류 입력
    pub fn new(r#type: ButtonType) -> Self {
        let action = match r#type {
            ButtonType::Call => "phone",
            ButtonType::Share => "share",
            ButtonType::Link => "webLink",
            ButtonType::Text => "message",
        };
        Button {
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
            Some("webLink") => Button::new(ButtonType::Link),
            Some("share") => Button::new(ButtonType::Share),
            Some("message") => Button::new(ButtonType::Text),
            Some("phone") => Button::new(ButtonType::Call),
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
