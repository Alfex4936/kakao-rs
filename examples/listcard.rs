extern crate kakao_rs;

use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;

fn main() {
  let mut result = Template::new();

  // 빠른 응답
  result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
  result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

  let mut list_card = ListCard::new("리스트 카드 제목!"); // 제목

  list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼"))); // 메시지 버튼

  list_card.add_button(Button::Link(
    LinkButton::new("link label").set_link("https://google.com"), // 링크 버튼
  ));
  list_card.add_button(Button::Share(
    ShareButton::new("share label").set_msg("카톡에 보이는 메시지"), // 공유 버튼
  ));

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
