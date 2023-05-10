// use kakao_rs::{Button, CallButton, LinkButton, MsgButton, ShareButton};

use kakao_rs::prelude::*;

#[test]
fn button_de_serialize_test() {
    // let a = Button::new(ButtonType::Call)
    //     .set_label("전화하기")
    //     .set_number("911");
    // let b = Button::new(ButtonType::Share).set_label("공유하기");
    // let c = Button::new(ButtonType::Link)
    //     .set_label("링크열기")
    //     .set_link("https://");
    // let d = Button::new(ButtonType::Text)
    //     .set_label("그냥 버튼")
    //     .set_msg("발화문임");

    let data = r#"[{"label":"CALL LABEL","action":"phone","phoneNumber":"0","messageText":"MESSAGE"},{"label":"SHARE LABEL","action":"share"},{"label":"MSG LABEL","action":"message"},{"label":"LABEL","action":"webLink","webLinkUrl":"https://"}]"#;
    let buttons: Vec<Button> = serde_json::from_str(data).unwrap();

    assert_eq!(buttons.len(), 4);
    assert_eq!(serde_json::to_string(&buttons).expect("Failed"), data); // Serialize
}

#[test]
fn button_only_serialize_test() {
    // 일반적으로 deserialize 할 일은 없음
    let buttons: Vec<Button> = vec![
        Button::call("전화하기", "911"),
        Button::share("공유하기"),
        Button::link("링크열기", "https://"),
        Button::text("그냥 버튼").set_msg("발화문임"),
    ];
    // Serialize
    let data = r#"[{"label":"전화하기","action":"phone","phoneNumber":"911"},{"label":"공유하기","action":"share"},{"label":"링크열기","action":"webLink","webLinkUrl":"https://"},{"label":"그냥 버튼","action":"message","messageText":"발화문임"}]"#;
    assert_eq!(
        data,
        serde_json::to_string(&buttons).expect("Failed to serialize")
    );
}
