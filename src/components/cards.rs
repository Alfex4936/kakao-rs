use crate::components::basics::*;
use crate::components::buttons::Button;
use serde::{Deserialize, Serialize};

/***** Buttons *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Card {
    Basic(BasicCardContent),
    Commerce(CommerceCardContent),
    Item(ItemCardContent),
}

/***** BasicCard *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// [BasicCard](https://i.kakao.com/docs/skill-response-format#basiccard): title, description, thumbnail, buttons
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let basic_card = BasicCard::new()
///     .set_title("제목입니다.".to_string())
///     .set_thumbnail(format!(
///         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
///     ));
///
/// result.add_output(basic_card.build());
///
/// ```
pub struct BasicCard {
    #[serde(rename = "basicCard")]
    pub content: BasicCardContent,
}

impl BasicCard {
    pub fn new() -> Self {
        BasicCard {
            content: BasicCardContent {
                title: None,
                description: None,
                thumbnail: ThumbNail::new("".to_string()),
                buttons: Vec::new(),
            },
        }
    }
    pub fn add_button(mut self, btn: Button) -> Self {
        self.content.buttons.push(btn);
        self
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.content.description = Some(desc);
        self
    }
    pub fn set_thumbnail(mut self, url: String) -> Self {
        self.content.thumbnail.image_url = url;
        self
    }

    // 단독이면 build, Carousel은 build_card
    pub fn build(self) -> Types {
        Types::Basic(self)
    }
    /// Carousel에 추가할 때 사용하세요.
    pub fn build_card(self) -> Card {
        Card::Basic(self.content)
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.content.title = Some(title);
        self
    }

    pub fn set_description(mut self, desc: String) -> Self {
        self.content.description = Some(desc);
        self
    }
    pub fn set_link(mut self, link: String) -> Self {
        self.content.thumbnail.link = Some(Link { web: link });
        self
    }
    pub fn set_fixed_ratio(mut self, fixed: bool) -> Self {
        self.content.thumbnail.fixed_ratio = fixed;
        self
    }

    pub fn set_width(mut self, _width: i32) -> Self {
        self.content.thumbnail.width = Some(_width);
        self
    }

    pub fn set_height(mut self, _height: i32) -> Self {
        self.content.thumbnail.height = Some(_height);
        self
    }

    #[inline]
    pub fn id() -> String {
        "basicCard".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BasicCardContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    thumbnail: ThumbNail, // 필수
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>,
}

/***** BasicCard *****/

/***** CommerceCard *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// [CommerceCard](https://i.kakao.com/docs/skill-response-format#commercecard): description, price, currency, discount, discountRate, discountedPrice, thumbnails, buttons
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let commerce_card = CommerceCard::new()
///     .set_price(5000)
///     .set_desc(format!("1 DESC"))
///     .set_currency("WON".to_string())
///     .set_thumbnail(format!(
///         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
///     ));
///
/// ```
pub struct CommerceCard {
    #[serde(rename = "commerceCard")]
    pub content: CommerceCardContent,
}

impl CommerceCard {
    pub fn new() -> Self {
        CommerceCard {
            content: CommerceCardContent {
                description: "".to_string(),
                price: 0,
                currency: "".to_string(),
                discount: None,
                discount_rate: None,
                discounted_price: None,
                thumbnails: Vec::new(),
                buttons: Vec::new(),
            },
        }
    }
    pub fn add_button(mut self, btn: Button) -> Self {
        self.content.buttons.push(btn);
        self
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.content.description = desc;
        self
    }
    pub fn set_thumbnail(mut self, url: String) -> Self {
        self.content.thumbnails.push(ThumbNail::new(url));
        self
    }

    pub fn build(self) -> Types {
        Types::Commerce(self)
    }
    /// Carousel에 추가할 때 사용하세요.
    pub fn build_card(self) -> Card {
        Card::Commerce(self.content)
    }

    pub fn set_price(mut self, price: i32) -> Self {
        self.content.price = price;
        self
    }

    pub fn set_currency(mut self, currency: String) -> Self {
        self.content.currency = currency;
        self
    }

    pub fn set_discount(mut self, discount: i32) -> Self {
        self.content.discount = Some(discount);
        self
    }

    pub fn set_discount_rate_price(mut self, rate: i32, priced: i32) -> Self {
        self.content.discount_rate = Some(rate);
        self.content.discounted_price = Some(priced);
        self
    }

    #[inline]
    pub fn id() -> String {
        "commerceCard".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CommerceCardContent {
    description: String,
    price: i32,
    currency: String, // 필수
    #[serde(skip_serializing_if = "Option::is_none")]
    discount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounted_price: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    thumbnails: Vec<ThumbNail>, // 필수, 1개만 지원
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>, // 필수
}

/***** CommerceCard *****/

/***** ItemCard *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
/// [ItemCard](https://i.kakao.com/docs/skill-response-format#itemcard): thumbnail, head, imageTitle, itemList,
///     itemListAlignment, itemListSummary, title, description, buttons, buttonLayout
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let item_card = ItemCard::new()
///     .set_title("title".to_string())
///     .set_desc("desc".to_string())
///     .set_thumbnail(format!(
///         "http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png"
///     ))
///     .set_thumbnail_width(800)
///     .set_thumbnail_height(800)
///     .set_image_title("DOFQTK".to_string())
///     .set_image_desc("Boarding Number".to_string())
///     .set_item_list_alignment("right".to_string())
///     .set_item_list_summary("total".to_string(), "$4,032.54".to_string())
///     .add_button(Button::Link(
///         LinkButton::new("View Boarding Pass".to_string())
///             .set_link("https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)".to_string()),
///     ))
///     .set_button_layout("vertical".to_string());
///
/// ```
pub struct ItemCard {
    #[serde(rename = "itemCard")]
    pub content: ItemCardContent,
}

impl ItemCard {
    pub fn new() -> Self {
        ItemCard {
            content: ItemCardContent {
                thumbnail: None,
                head: None,
                image_title: None,
                item_list: Vec::new(),
                item_list_alignment: None,
                item_list_summary: None,
                title: None,
                description: None,
                buttons: Vec::new(),
                button_layout: None,
            },
        }
    }

    pub fn build(self) -> Types {
        Types::Item(self)
    }
    /// Carousel에 추가할 때 사용하세요.
    pub fn build_card(self) -> Card {
        Card::Item(self.content)
    }

    pub fn add_button(mut self, btn: Button) -> Self {
        self.content.buttons.push(btn);
        self
    }
    pub fn set_title(mut self, _title: String) -> Self {
        self.content.title = Some(_title);
        self
    }
    pub fn set_desc(mut self, desc: String) -> Self {
        self.content.description = Some(desc);
        self
    }
    pub fn set_button_layout(mut self, layout: String) -> Self {
        self.content.button_layout = Some(layout);
        self
    }
    pub fn set_item_list_alignment(mut self, align: String) -> Self {
        self.content.item_list_alignment = Some(align);
        self
    }
    pub fn set_item_list_summary(mut self, _title: String, _desc: String) -> Self {
        self.content.item_list_summary = Some(ItemListSummary {
            title: _title,
            description: _desc,
        });
        self
    }
    pub fn set_thumbnail(mut self, url: String) -> Self {
        self.content.thumbnail = Some(ThumbNail::new(url));
        self
    }

    pub fn set_head(mut self, _title: String) -> Self {
        self.content.head = Some(Head { title: _title });
        self
    }

    pub fn set_image_title(mut self, _title: String) -> Self {
        self.content.image_title = Some(ImageTitle::new(_title));
        self
    }

    pub fn set_image_desc(mut self, _desc: String) -> Self {
        if let Some(image) = &mut self.content.image_title {
            image.description = Some(_desc);
        }

        self
    }

    pub fn set_image(mut self, _url: String) -> Self {
        if let Some(image) = &mut self.content.image_title {
            image.image_url = Some(_url);
        }

        self
    }

    pub fn set_thumbnail_height(mut self, _height: i32) -> Self {
        if let Some(thumb) = &mut self.content.thumbnail {
            thumb.height = Some(_height);
        }
        self
    }

    pub fn set_thumbnail_width(mut self, _width: i32) -> Self {
        if let Some(thumb) = &mut self.content.thumbnail {
            thumb.width = Some(_width);
        }
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ItemCardContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<ThumbNail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<Head>,
    // Profile 현재 미지원
    #[serde(skip_serializing_if = "Option::is_none")]
    image_title: Option<ImageTitle>,
    item_list: Vec<ItemList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_list_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_list_summary: Option<ItemListSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button_layout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageTitle {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl ImageTitle {
    pub fn new(_title: String) -> Self {
        ImageTitle {
            title: _title,
            description: None,
            image_url: None,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_desc(&mut self, desc: String) {
        self.description = Some(desc);
    }

    pub fn set_image(&mut self, url: String) {
        self.image_url = Some(url);
    }

    #[inline]
    pub fn id() -> String {
        "itemCard".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemList {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemListSummary {
    title: String,
    description: String,
}
/***** ItemCard *****/
