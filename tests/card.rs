use kakao_rs::prelude::*;
use std::matches;

#[test]
fn basic_card_test() {
    let mut result = Template::new();

    let basic_card = BasicCard::new()
        .set_title("제목입니다.")
        .set_thumbnail("http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg");

    result.add_output(basic_card.build());

    let serialized = r#"{"template":{"outputs":[{"basicCard":{"title":"제목입니다.","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}}}]},"version":"2.0"}"#;
    assert_eq!(serialized, serde_json::to_string(&result).expect("Failed"));

    let deserialized: Template = serde_json::from_str(result.to_string().as_str()).unwrap();
    assert!(matches!(
        deserialized.template.outputs[0],
        Types::Basic { .. }
    ));
}

#[test]
fn item_card_test() {
    let mut result = Template::new();

    let item_card = ItemCard::new()
        .set_title("title")
        .set_desc("desc")
        .set_thumbnail("http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png".to_string())
        .set_thumbnail_width(800)
        .set_thumbnail_height(800)
        .set_image_title("DOFQTK")
        .set_image_desc("Boarding Number")
        .set_item_list_alignment("right")
        .set_item_list_summary("total", "$4,032.54")
        .add_button(
            Button::new(ButtonType::Link)
                .set_label("View Boarding Pass")
                .set_link("https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)"),
        )
        .set_button_layout("vertical");

    result.add_output(item_card.build());

    let serialized = r#"{"template":{"outputs":[{"itemCard":{"thumbnail":{"imageUrl":"http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png","width":800,"height":800},"imageTitle":{"title":"DOFQTK","description":"Boarding Number"},"itemList":[],"itemListAlignment":"right","itemListSummary":{"title":"total","description":"$4,032.54"},"title":"title","description":"desc","buttons":[{"label":"View Boarding Pass","action":"webLink","webLinkUrl":"https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)"}],"buttonLayout":"vertical"}}]},"version":"2.0"}"#;
    assert_eq!(serialized, serde_json::to_string(&result).expect("Failed"));

    let deserialized: Template = serde_json::from_str(result.to_string().as_str()).unwrap();
    assert!(matches!(
        deserialized.template.outputs[0],
        Types::Item { .. }
    ));
}
