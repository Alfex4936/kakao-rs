use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;
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

    let serialized = r#"{"template":{"outputs":[{"basicCard":{"title":"제목입니다.","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}}}]},"version":"2.0"}"#;
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
        .set_title("title".to_string())
        .set_desc("desc".to_string())
        .set_thumbnail(format!(
            "http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png"
        ))
        .set_thumbnail_width(800)
        .set_thumbnail_height(800)
        .set_image_title("DOFQTK".to_string())
        .set_image_desc("Boarding Number".to_string())
        .set_item_list_alignment("right".to_string())
        .set_item_list_summary("total".to_string(), "$4,032.54".to_string())
        .add_button(Button::Link(
            LinkButton::new("View Boarding Pass".to_string())
                .set_link("https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)".to_string()),
        ))
        .set_button_layout("vertical".to_string());

    result.add_output(item_card.build());

    let serialized = r#"{"template":{"outputs":[{"itemCard":{"thumbnail":{"imageUrl":"http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png","fixedRatio":false,"width":800,"height":800},"imageTitle":{"title":"DOFQTK","description":"Boarding Number"},"itemList":[],"itemListAlignment":"right","itemListSummary":{"title":"total","description":"$4,032.54"},"title":"title","description":"desc","buttons":[{"label":"View Boarding Pass","action":"webLink","webLinkUrl":"https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)"}],"buttonLayout":"vertical"}}]},"version":"2.0"}"#;
    assert_eq!(serialized, serde_json::to_string(&result).expect("Failed"));

    let deserialized: Template = serde_json::from_str(result.to_string().as_str()).unwrap();
    assert!(matches!(
        deserialized.template.outputs[0],
        Types::Item { .. }
    ));
}
