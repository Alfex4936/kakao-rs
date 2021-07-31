use criterion::{criterion_group, criterion_main, Bencher, Criterion};

extern crate kakao_rs;

use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;
use kakao_rs::components::cards::*;

fn bench_listcard_0(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
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
                LinkButton::new("link label".to_string())
                    .set_link("https://google.com".to_string()),
            ));
            list_card.add_button(Button::Share(
                ShareButton::new("share label".to_string())
                    .set_msg("카톡에 보이는 메시지".to_string()),
            ));

            list_card.add_item(
                ListItem::new("title".to_string())
                    .set_desc("description".to_string())
                    .set_link("https://naver.com".to_string()),
            );

            result.add_output(list_card.build()); // moved list_card's ownership
        }
    });
}

fn bench_multiple_output_0(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
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
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("listcard_0", bench_listcard_0);
    c.bench_function("multiple_0", bench_multiple_output_0);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
