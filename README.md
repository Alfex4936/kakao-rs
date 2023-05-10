<div align="center">
<p>
    <img width="680" src="https://raw.githubusercontent.com/Alfex4936/kakaoChatbot-Ajou/main/imgs/chatbot.png">
</p>

[![kakao-rs on crates.io](https://img.shields.io/crates/v/kakao-rs.svg)](https://crates.io/crates/kakao-rs)
[![kakao-rs on docs.rs](https://docs.rs/kakao-rs/badge.svg)](https://docs.rs/kakao-rs/)

<h2>카카오톡 챗봇 빌더 도우미</h2>
<h3>Rust언어 전용</h3>
</div>

# 소개

Rust언어로 카카오 챗봇 서버를 만들 때 좀 더 쉽게 JSON 메시지 응답을 만들 수 있게 도와줍니다.

SimpleText, SimpleImage, ListCard, Carousel, BasicCard, CommerceCard, ItemCard

JSON 데이터를 쉽게 만들 수 있도록 도와줍니다.

# 설치
```toml
[dependencies]
kakao-rs = "0.3"
```

# 응답 타입별 아이템

Button::share (공유 버튼), Button::link (링크 버튼), Button::text (일반 메시지만), Button::call(전화 버튼)

Items: ListItem

# 사용법

## 카카오 JSON 데이터 Bind

예제) 유저 발화문 얻기: json.userRequest.utterance

```rust
#[post("/end", format = "json", data = "<kakao>")]  // Rocket
pub fn test(kakao: Json<Value>) -> String {
    println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    unimplemented!()
}

#[post("/end")]
pub async fn test(kakao: web::Json<Value>) -> impl Responder {  // actix
    println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    unimplemented!()
}
```

## ListCard 예제
```rust
extern crate kakao_rs;

use kakao_rs::prelude::*;

fn main() {
    let mut result = Template::new();

    // 빠른 응답
    result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
    result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

    let mut list_card = ListCard::new("리스트 카드 제목!"); // 제목

    list_card.add_button(Button::text("그냥 텍스트 버튼")); // 메시지 버튼
    list_card.add_button(Button::link("link label", "https://google.com")); // 링크 버튼
    list_card.add_button(Button::share("share label").set_msg("카톡에 보이는 메시지")); // 공유 버튼, 기본적으로 message_text는 없음
    list_card.add_button(Button::call("call label", "010-1234-5679")); // 전화 버튼

    list_card.add_item(
        ListItem::new("title")
            .set_desc("description") // 설명
            .set_link("https://naver.com"),
    );

    result.add_output(list_card.build()); // moved list_card's ownership

    println!(
        "Result: {}",
        serde_json::to_string_pretty(&result).expect("Failed")
    );
}

/*
Result: {
  "template": {
    "outputs": [
      {
        "listCard": {
          "buttons": [
            {
              "label": "그냥 텍스트 버튼",
              "action": "message"
            },
            {
              "label": "link label",
              "action": "webLink",
              "webLinkUrl": "https://google.com"
            },
            {
              "label": "share label",
              "action": "share",
              "messageText": "카톡에 보이는 메시지"
            },
            {
              "label": "call label",
              "action": "phone",
              "phoneNumber": "010-1234-5679"
            }
          ],
          "header": {
            "title": "리스트 카드 제목!"
          },
          "items": [
            {
              "title": "title",
              "description": "description",
              "link": {
                "web": "https://naver.com"
              }
            }
          ]
        }
      }
    ],
    "quickReplies": [
      {
        "action": "message",
        "label": "오늘",
        "messageText": "오늘 공지 보여줘"
      },
      {
        "action": "message",
        "label": "어제",
        "messageText": "어제 공지 보여줘"
      }
    ]
  },
  "version": "2.0"
}
*/
```

## SimpleText, SimpleImage, BasicCard, CommerceCard, Carousel

Carousel에 Card를 추가할 때는 build_card()로 카드를 빌드하세요.

자세한 사용법은 [tests](https://github.com/Alfex4936/kakao-rs/tree/master/tests) 폴더를 참고하세요.

```rust
use kakao_rs::prelude::*;
// 따로 import
// use kakao_rs::components::basics::*;
// use kakao_rs::components::buttons::*;
// use kakao_rs::components::cards::*;
use std::matches;

#[test]
fn simple_text_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답",
        "빠른 응답 ㅋㅋ",
    ));

    let simple_text = SimpleText::new("심플 텍스트 테스트");
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;
    assert_eq!(serialized, result.to_string());
}

#[test]
fn multiple_outputs_test() {
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "빠른 응답",
        "빠른 응답 ㅋㅋ",
    ));

    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for i in 0..5 {
        let basic_card = BasicCard::new()
            .set_title(format!("{}번", i))
            .set_thumbnail(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
            );

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    let simple_text = SimpleText::new("심플 텍스트 테스트");
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"}}]}},{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    // Carousel BasicCards 뒤 SimpleText 발화
    assert_eq!(serialized, result.to_string());
}
```

# TODO

- use PyO3 to export this library in Python


How to make codes more maintainable?

```rust
    pub fn set_field<T: Into<String>>(mut self, field: &str, value: T) -> Self {
        match field {
            "desc" | "description" => self.content.description = Some(value.into()),
            "title" => self.content.title = Some(value.into()),
            "thumbnail" => self.content.thumbnail.image_url = value.into(),
            "link" => self.content.thumbnail.link = Some(Link { web: value.into() }),
            "fixed_ratio" => self.content.thumbnail.fixed_ratio = value.into(),
            "width" => self.content.thumbnail.width = Some(value.into()),
            "height" => self.content.thumbnail.height = Some(value.into()),
            _ => {}
        }
        self
    }
```
