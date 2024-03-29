use kakao_rs::prelude::*;

#[test]
fn simple_text_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let simple_text = SimpleText::new("심플 텍스트 테스트");
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn simple_image_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let simple_img = SimpleImage::new("이미지 링크", "이미지 오류");
    result.add_output(simple_img.build());

    let serialized = r#"{"template":{"outputs":[{"simpleImage":{"imageUrl":"이미지 링크","altText":"이미지 오류"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn carousel_basic_card_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
            );

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}}]}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn carousel_commerce_card_json() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let mut carousel = Carousel::new().set_type(CommerceCard::id());

    for i in 0..5 {
        let commerce_card = CommerceCard::new()
            .set_price(5000 + i)
            .set_desc(format!("{} DESC", i))
            .set_currency("WON")
            .set_thumbnail(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
            );

        carousel.add_card(commerce_card.build_card());
    }

    result.add_output(carousel.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"commerceCard","items":[{"description":"0 DESC","price":5000,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}]},{"description":"1 DESC","price":5001,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}]},{"description":"2 DESC","price":5002,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}]},{"description":"3 DESC","price":5003,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}]},{"description":"4 DESC","price":5004,"currency":"WON","thumbnails":[{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}]}]}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    assert_eq!(serialized, result.to_string());
}

#[test]
fn multiple_outputs_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail("http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg".to_string());

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let simple_text = SimpleText::new("심플 텍스트 테스트".to_string());
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}}]}},{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    // Carousel BasicCards 뒤 SimpleText 발화
    assert_eq!(serialized, result.to_string());
}

#[test]
fn carousel_thumbnail_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("빠른 응답", "빠른 응답 ㅋㅋ"));

    let mut carousel = Carousel::new();
    carousel.set_header("오늘 공지 n개", "n개를 더 불러왔습니다!", "https://");

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
            );

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}}],"header":{"title":"오늘 공지 n개","description":"n개를 더 불러왔습니다!","thumbnail":{"imageUrl":"https://"}}}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}
