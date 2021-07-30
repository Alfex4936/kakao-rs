use kakao_rs::components::basics::*;
use kakao_rs::components::cards::*;

#[test]
fn simple_text_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답".to_string(),
        "빠른 응답 ㅋㅋ".to_string(),
    ));

    let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn simple_image_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답".to_string(),
        "빠른 응답 ㅋㅋ".to_string(),
    ));

    let simple_img = SimpleImage::new(format!("이미지 링크"), format!("이미지 오류"));
    result.add_output(simple_img.build());

    let serialized = r#"{"template":{"outputs":[{"simpleImage":{"imageUrl":"이미지 링크","altText":"이미지 오류"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn carousel_basic_card_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답".to_string(),
        "빠른 응답 ㅋㅋ".to_string(),
    ));

    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail(format!(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
            ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}}]}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn carousel_commerce_card_json() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답".to_string(),
        "빠른 응답 ㅋㅋ".to_string(),
    ));

    let mut carousel = Carousel::new().set_type(CommerceCard::id());

    for i in 0..5 {
        let commerce_card = CommerceCard::new()
            .set_price(5000 + i)
            .set_desc(format!("{} DESC", i))
            .set_currency("WON".to_string())
            .set_thumbnail(format!(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
            ));

        carousel.add_card(commerce_card.build_card());
    }

    result.add_output(carousel.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"commerceCard","items":[{"description":"0 DESC","price":5000,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}]},{"description":"1 DESC","price":5001,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}]},{"description":"2 DESC","price":5002,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}]},{"description":"3 DESC","price":5003,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}]},{"description":"4 DESC","price":5004,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}]}]}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    assert_eq!(serialized, result.to_string());
}

#[test]
fn multiple_outputs_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답".to_string(),
        "빠른 응답 ㅋㅋ".to_string(),
    ));

    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail(format!(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
            ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}}]}},{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    // Carousel BasicCards 뒤 SimpleText 발화
    assert_eq!(serialized, result.to_string());
}
