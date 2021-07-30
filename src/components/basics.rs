use crate::components::buttons::*;
use crate::components::cards::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/***** Items *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub web: String,
}

// Go 버전에서 ListItem, ListItemLink 합침
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ListItem {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<Link>,
}

impl ListItem {
    pub fn new(_title: String) -> Self {
        ListItem {
            title: _title,
            description: None,
            image_url: None,
            link: None,
        }
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn set_image(mut self, url: String) -> Self {
        self.image_url = Some(url);
        self
    }

    pub fn set_link(mut self, _url: String) -> Self {
        self.link = Some(Link { web: _url });
        self
    }
}
/***** Items *****/

/***** Quick Reply *****/
// Go 버전에서 QuickReply
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// [QuickReply](https://i.kakao.com/docs/skill-response-format#quickreplies): action, label, messageText (Optional), blockId (Optional)
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut result = Template::new();
///
/// result.add_qr(QuickReply::new(
///     "빠른 응답".to_string(),
///     "빠른 응답 ㅋㅋ".to_string(),
/// ));
///
/// ```
pub struct QuickReply {
    action: String,
    label: String,
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl QuickReply {
    pub fn new(_label: String, _msg: String) -> Self {
        QuickReply {
            label: _label,
            message_text: _msg,
            action: "message".to_string(),
            block_id: None,
        }
    }

    pub fn set_block_id(mut self, id: String) -> Self {
        self.block_id = Some(id);
        self
    }

    pub fn set_action(mut self, _action: String) -> Self {
        self.action = _action;
        self
    }
}
/***** Quick Reply *****/

/***** Extra *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Title {
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// [ThumbNail 링크](https://i.kakao.com/docs/skill-response-format#thumbnail)
pub struct ThumbNail {
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    pub fixed_ratio: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl ThumbNail {
    pub fn new(url: String) -> Self {
        ThumbNail {
            image_url: url,
            link: None,
            fixed_ratio: false,
            width: None,
            height: None,
        }
    }
    pub fn set_link(mut self, url: String) -> Self {
        self.link = Some(Link { web: url });
        self
    }

    pub fn set_image_url(mut self, url: String) -> Self {
        self.image_url = url;
        self
    }

    pub fn set_fixed_ratio(mut self, fixed: bool) -> Self {
        self.fixed_ratio = fixed;
        self
    }

    pub fn set_width(mut self, _width: i32) -> Self {
        self.width = Some(_width);
        self
    }

    pub fn set_height(mut self, _height: i32) -> Self {
        self.height = Some(_height);
        self
    }
}

/***** Extra *****/

/***** Main *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// 현재 ListCard, BasicCard, SimpleText, SimpleImage, Carousel (Basic/CommerceCard) 지원
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut result = Template::new();
/// result.add_qr(...)
/// result.add_output(...)
/// result.to_string() -> json!(self)
///
/// ```
pub struct Template {
    pub template: Outputs,
    pub version: String,
}

impl Template {
    pub fn new() -> Self {
        Template {
            template: Outputs::new(),
            version: "2.0".to_string(),
        }
    }

    pub fn add_output(&mut self, output: Types) {
        self.template.outputs.push(output);
    }

    pub fn add_qr(&mut self, qr: QuickReply) {
        self.template.quick_replies.push(qr);
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Types {
    List(ListCard),
    Basic(BasicCard),
    Commerce(CommerceCard),
    Item(ItemCard),
    SimpleTxt(SimpleText),
    SimpleImg(SimpleImage),
    Carousel(Carousel),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Outputs {
    pub outputs: Vec<Types>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub quick_replies: Vec<QuickReply>,
}

impl Outputs {
    fn new() -> Self {
        Outputs {
            outputs: Vec::new(),
            quick_replies: Vec::<QuickReply>::new(),
        }
    }
}

/***** Main *****/

/***** Response *****/
/* Supports
    ListCard, SimpleText, Carousel (BasicCard, CommerceCard)
*/

/***** Carousel *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// [Carousel](https://i.kakao.com/docs/skill-response-format#carousel) (BasicCard or CommerceCard): type, items (Optional), header (Optional)
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut carousel = Carousel::new().set_type(BasicCard::id());
///
/// for i in 0..5 {
/// let basic_card = BasicCard::new()
///     .set_title(format!("{}번", i))
///     .set_thumbnail(format!(
///         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
///     ));
///
///     carousel.add_card(Card::Basic(basic_card));
/// }
///
/// result.add_output(carousel.build());
///
/// ```
pub struct Carousel {
    carousel: CarouselContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CarouselContent {
    r#type: String,
    // #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    items: Vec<Card>, // TODO ItemCard, ListCard
    // #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<CarouselHeader>,
}

impl Carousel {
    pub fn new() -> Self {
        Carousel {
            carousel: CarouselContent {
                r#type: "basicCard".to_string(),
                items: Vec::new(),
                header: None,
            },
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.carousel.items.push(card);
        // match &mut card {
        //     Card::Basic(basic) => {
        //         self.carousel.items.push(basic.content);
        //     }
        //     Card::Commerce(com) => {
        //         self.carousel.items.push(com.content);
        //     }
        //     Card::Item(item) => {
        //         self.carousel.items.push(item.content);
        //     }
        // }
    }

    pub fn build(self) -> Types {
        Types::Carousel(self)
    }

    pub fn set_type(mut self, _type: String) -> Self {
        self.carousel.r#type = _type;
        self
    }

    pub fn set_header_title(mut self, title: String) -> Self {
        self.carousel.header.as_mut().unwrap().set_title(title);
        self
    }

    pub fn set_header_desc(mut self, desc: String) -> Self {
        self.carousel.header.as_mut().unwrap().set_desc(desc);
        self
    }

    // fn set_header_thumbnail(mut self, url: String) -> Self {
    //     self.header.thumbnail.set_image_url(url);
    //     self
    // }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CarouselHeader {
    title: String,
    description: String,
    thumbnail: ThumbNail,
}

impl CarouselHeader {
    pub fn new() -> Self {
        CarouselHeader {
            title: "".to_string(),
            description: "".to_string(),
            thumbnail: ThumbNail::new("".to_string()),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_desc(&mut self, desc: String) {
        self.description = desc;
    }
}
/***** Carousel *****/

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/// [ListCard](https://i.kakao.com/docs/skill-response-format#listcard): buttons, header, items
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut list_card = ListCard::new(format!("리스트 카드 제목!")); // 제목
///
/// // 버튼 추가
/// list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼".to_string())));
///
/// list_card.add_button(Button::Link(
///     LinkButton::new("link label".to_string()).set_link("https://google.com".to_string()),
/// ));
/// list_card.add_button(Button::Share(
///     ShareButton::new("share label".to_string()).set_msg("카톡에 보이는 메시지".to_string()),
/// ));
///
/// // 아이템 추가
/// list_card.add_item(
///     ListItem::new("title".to_string())
///         .set_desc("description".to_string())
///         .set_link("https://naver.com".to_string()),
/// );
///
/// // 빌드
/// result.add_output(list_card.build()); // moved list_card's ownership
///
/// ```
pub struct ListCard {
    list_card: ListCardContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListCardContent {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>,
    header: Title,        // 필수
    items: Vec<ListItem>, // 필수
}

impl ListCard {
    pub fn new(_header: String) -> ListCard {
        ListCard {
            list_card: ListCardContent::new(_header),
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.list_card.buttons.push(button);
    }

    pub fn add_item(&mut self, item: ListItem) {
        self.list_card.items.push(item);
    }

    pub fn build(self) -> Types {
        Types::List(self)
    }
}

impl ListCardContent {
    fn new(_title: String) -> ListCardContent {
        ListCardContent {
            buttons: Vec::new(),
            header: Title { title: _title },
            items: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// [SimpleText](https://i.kakao.com/docs/skill-response-format#simpletext): text
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut result = Template::new();
/// result.add_qr(QuickReply::new(
///     "빠른 응답".to_string(),
///     "빠른 응답 ㅋㅋ".to_string(),
/// ));
///
/// let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
/// result.add_output(simple_text.build());
///
/// ```
pub struct SimpleText {
    simple_text: SimpleTextContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimpleTextContent {
    text: String,
}

impl SimpleText {
    pub fn new(_text: String) -> Self {
        SimpleText {
            simple_text: SimpleTextContent { text: _text },
        }
    }

    pub fn set_text(mut self, _text: String) -> Self {
        self.simple_text.text = _text;
        self
    }

    pub fn build(self) -> Types {
        Types::SimpleTxt(self)
    }

    pub fn html(&self) -> String {
        format!("{}", self.simple_text.text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// [SimpleImage](https://i.kakao.com/docs/skill-response-format#simpleimage): imageUrl, altText
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut result = Template::new();
/// result.add_qr(QuickReply::new(
///     "빠른 응답".to_string(),
///     "빠른 응답 ㅋㅋ".to_string(),
/// ));
///
/// let simple_img = SimpleImage::new(
///     format!("http://k.kakaocdn.net/dn/83BvP/bl20duRC1Q1/lj3JUcmrzC53YIjNDkqbWK/i_6piz1p.jpg"),
///     format!("보물상자입니다"));
///
/// result.add_output(simple_img.build());
///
/// ```
pub struct SimpleImage {
    simple_image: SimpleImageContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleImageContent {
    image_url: String,
    alt_text: String,
}

impl SimpleImage {
    pub fn new(_url: String, _text: String) -> Self {
        SimpleImage {
            simple_image: SimpleImageContent {
                image_url: _url,
                alt_text: _text,
            },
        }
    }

    pub fn set_image(mut self, _link: String) -> Self {
        self.simple_image.image_url = _link;
        self
    }

    pub fn set_text(mut self, _text: String) -> Self {
        self.simple_image.alt_text = _text;
        self
    }

    pub fn build(self) -> Types {
        Types::SimpleImg(self)
    }

    pub fn html(&self) -> String {
        format!("{}", self.simple_image.image_url)
    }
}

/***** Response *****/
