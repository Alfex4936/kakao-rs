<div align="center">
<p>
    <img width="680" src="https://raw.githubusercontent.com/Alfex4936/kakaoChatbot-Ajou/main/imgs/chatbot.png">
</p>

[![kakao-rs on crates.io](https://img.shields.io/crates/v/kakao-rs.svg)](https://crates.io/crates/kakao-rs)
[![kakao-rs on docs.rs](https://docs.rs/kakao-rs/badge.svg)](https://docs.rs/kakao-rs/0.1.0/kakao_rs/)

<h2>카카오톡 챗봇 빌더 도우미</h2>
<h3>Rust언어 전용</h3>
</div>

# 소개

Rust언어로 카카오 챗봇 서버를 만들 때 좀 더 쉽게 JSON 메시지 응답을 만들 수 있게 도와줍니다.

SimpleText, ListCard, Carousel, BasicCard, CommerceCard JSON 데이터를 쉽게 만들 수 있도록 도와줍니다.

# 설치

# 응답 타입별 아이템

Buttons: ShareButton (공유 버튼), LinkButton (링크 버튼), MsgButton (일반 메시지만), CallButton (전화 버튼)

Items: ListItem

# 사용법 (Rocket 프레임워크 기준)

왠만한 필드는 &str이 아닌 String 입니다.

## 카카오 JSON 데이터 Bind

예제) 유저 발화문 얻기: json.userRequest.utterance

```rust
#[post("/end", format = "json", data = "<kakao>")]
pub fn test(kakao: Json<Value>) -> String {
    println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    unimplemented!()
}
```

## ListCard 예제
```rust
extern crate kakao_rs;

use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;
use kakao_rs::components::cards::*;

fn main() {
    let mut result = Template::new();

    // 빠른 응답
    result.add_qr(QuickReply::new(
        "오늘".to_string(),
        "오늘 공지 보여줘".to_string(),
    ));
    result.add_qr(QuickReply::new(
        "어제".to_string(),
        "어제 공지 보여줘".to_string(),
    ));

    let mut list_card = ListCard::new(format!("리스트 카드 제목!")); // 제목

    list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼".to_string())));

    list_card.add_button(Button::Link(
        LinkButton::new("link label".to_string()).set_link("https://google.com".to_string()),
    ));
    list_card.add_button(Button::Share(
        ShareButton::new("share label".to_string()).set_msg("카톡에 보이는 메시지".to_string()),
    ));

    list_card.add_item(
        ListItem::new("title".to_string())
            .set_desc("description".to_string())
            .set_link("https://naver.com".to_string()),
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

## SimpleText, BasicCard, CommerceCard, Carousel

자세한 사용법은 [tests](https://github.com/Alfex4936/kakao-rs/tree/master/tests) 폴더를 참고하세요.

```rust
use kakao_rs::components::basics::*;
use kakao_rs::components::cards::*;
use std::matches;

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

        carousel.add_card(Card::Basic(basic_card));
    }

    result.add_output(carousel.build());

    let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
    result.add_output(simple_text.build());

    let serialized = r#"{"template":{"outputs":[{"carousel":{"type":"basicCard","items":[{"title":"0번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"1번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"2번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"3번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}},{"title":"4번","thumbnail":{"imageUrl":"http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg","fixedRatio":false}}]}},{"simpleText":{"text":"심플 텍스트 테스트"}}],"quickReplies":[{"action":"message","label":"빠른 응답","messageText":"빠른 응답 ㅋㅋ"}]},"version":"2.0"}"#;

    // Carousel BasicCards 뒤 SimpleText 발화
    assert_eq!(serialized, result.to_string());
}
```