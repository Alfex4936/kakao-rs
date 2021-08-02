//! # kakao-rs
//!
//! 카카오 챗봇 서버를 만들 때 좀 더 쉽게 JSON 메시지 응답을 만들 수 있게 도와줍니다.
//!
//! 지원하는 메시지 유형
//!  - **SimpleText && SimpleImage **
//!  - **ListCard**
//!  - **Carousel** (BasicCard || CommerceCard || ItemCard)
//!  - **BasicCard**
//!  - **CommerceCard**
//!  - **ItemCard**
//!
//! ### Carousel(BasicCards) + SimpleText 예제
//!
//! ```rust
//! let mut result = Template::new();
//! result.add_qr(QuickReply::new(
//!     "빠른 응답".to_string(),
//!     "빠른 응답 ㅋㅋ".to_string(),
//! ));
//!
//! let mut carousel = Carousel::new().set_type(BasicCard::id());
//!
//! for i in 0..5 {
//!     let basic_card = BasicCard::new()
//!         .set_title(format!("{}번", i))
//!         .set_thumbnail(format!(
//!             "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
//!         ));
//!
//!     carousel.add_card(basic_card.build_card());
//! }
//!
//! result.add_output(carousel.build());
//!
//! let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
//! result.add_output(simple_text.build());
//!
//! ```
//!
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables)]

#[macro_use]
extern crate serde_json;

pub mod components;

pub use crate::components::basics::*;
pub use crate::components::buttons::*;
pub use crate::components::cards::*;
