// use kakao_rs::{Button, CallButton, LinkButton, MsgButton, ShareButton};

use kakao_rs::prelude::*;

use std::matches;

#[test]
fn button_de_serialize_test() {
    let data = r#"[{"label":"CALL LABEL","action":"phone","phoneNumber":"0","messageText":"MESSAGE"},{"label":"SHARE LABEL","action":"share"},{"label":"MSG LABEL","action":"message"},{"label":"LABEL","action":"webLink","webLinkUrl":"https://"}]"#;
    let buttons: Vec<Button> = serde_json::from_str(data).unwrap();

    // println!("Deserialize: {:?}", buttons);

    assert_eq!(buttons.len(), 4);
    assert!(matches!(buttons[1], Button::Share { .. }));
    assert_eq!(serde_json::to_string(&buttons).expect("Failed"), data); // Serialize
}

#[test]
fn button_only_serialize_test() {
    // 일반적으로 deserialize 할 일은 없음
    let mut buttons: Vec<Button> = vec![];
    buttons.push(Button::init_call_button("010-1234-5678", "전화하기"));
    buttons.push(Button::init_msg_button("그냥 텍스트 버튼"));
    buttons.push(Button::init_link_button("https://google.com", "라벨"));
    buttons.push(Button::init_share_button("라벨"));

    buttons.push(Button::Link(
        LinkButton::new("label").set_link("https://google.com"),
    ));
    buttons.push(Button::Share(ShareButton::new("label").set_msg("발화문임")));
    // Serialize
    let data = r#"[{"label":"전화하기","action":"phone","phoneNumber":"010-1234-5678"},{"label":"그냥 텍스트 버튼","action":"message"},{"label":"라벨","action":"webLink","webLinkUrl":"https://google.com"},{"label":"라벨","action":"share"},{"label":"label","action":"webLink","webLinkUrl":"https://google.com"},{"label":"label","action":"share","messageText":"발화문임"}]"#;
    assert_eq!(
        data,
        serde_json::to_string(&buttons).expect("Failed to serialize")
    );
}
