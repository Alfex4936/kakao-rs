use kakao_rs::Button;
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
