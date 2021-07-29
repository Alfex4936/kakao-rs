use kakao_rs::components::basics::*;
use kakao_rs::components::cards::*;
use std::matches;

#[test]
fn basic_card_test() {
    let mut result = Template::new();

    let basic_card = BasicCard::new()
        .set_title("제목입니다.".to_string())
        .set_thumbnail(format!(
            "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
        ));

    result.add_output(basic_card.build());

    let serialized = r#"{"template":{"outputs":[{"title":"제목입니다.","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}}]},"version":"2.0"}"#;
    assert_eq!(serialized, serde_json::to_string(&result).expect("Failed"));

    let deserialized: Template = serde_json::from_str(result.to_string().as_str()).unwrap();
    assert!(matches!(
        deserialized.template.outputs[0],
        Types::Basic { .. }
    ));
}
