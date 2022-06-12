use crate::components::basics::*;
use crate::components::buttons::Button;
use serde::{Deserialize, Serialize};

/***** Buttons *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Card {
    Basic(BasicCardContent), // 144 bytes
    Commerce(CommerceCardContent),
    Item(ItemCardContent), // 360 bytes: Too big?
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
/// use kakao_rs::components::basics::Template;
/// use kakao_rs::components::cards::BasicCard;
///
/// let mut result = Template::new();
///
/// let basic_card = BasicCard::new()
///     .set_title("제목입니다.")
///     .set_thumbnail(
///         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
///     );
///
/// result.add_output(basic_card.build());
///
/// ```
pub struct BasicCard {
    #[serde(rename = "basicCard")]
    pub content: BasicCardContent,
}

impl Default for BasicCard {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn set_desc<S: Into<String>>(mut self, desc: S) -> Self {
        self.content.description = Some(desc.into());
        self
    }
    pub fn set_thumbnail<S: Into<String>>(mut self, url: S) -> Self {
        self.content.thumbnail.image_url = url.into();
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

    pub fn set_title<S: Into<String>>(mut self, title: S) -> Self {
        self.content.title = Some(title.into());
        self
    }

    pub fn set_description<S: Into<String>>(mut self, desc: S) -> Self {
        self.content.description = Some(desc.into());
        self
    }
    pub fn set_link<S: Into<String>>(mut self, link: S) -> Self {
        self.content.thumbnail.link = Some(Link { web: link.into() });
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
/// use kakao_rs::components::basics::Template;
/// use kakao_rs::components::cards::CommerceCard;
///
/// let mut result = Template::new();
///
/// let commerce_card = CommerceCard::new()
///     .set_price(5000)
///     .set_desc("1 DESC")
///     .set_currency("WON")
///     .set_thumbnail(
///         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
///     );
///
/// result.add_output(commerce_card.build());
/// ```
pub struct CommerceCard {
    #[serde(rename = "commerceCard")]
    pub content: CommerceCardContent,
}

impl Default for CommerceCard {
    fn default() -> Self {
        Self::new()
    }
}

impl CommerceCard {
    /// CommerceCard를 초기화 합니다.
    #[inline]
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

    pub fn set_desc<S: Into<String>>(mut self, desc: S) -> Self {
        self.content.description = desc.into();
        self
    }
    pub fn set_thumbnail<S: Into<String>>(mut self, url: S) -> Self {
        self.content.thumbnails.push(ThumbNail::new(url.into()));
        self
    }

    #[inline]
    pub fn build(self) -> Types {
        Types::Commerce(self)
    }
    /// Carousel에 추가할 때 사용하세요.
    #[inline]
    pub fn build_card(self) -> Card {
        Card::Commerce(self.content)
    }

    pub fn set_price(mut self, price: i32) -> Self {
        self.content.price = price;
        self
    }

    pub fn set_currency<S: Into<String>>(mut self, currency: S) -> Self {
        self.content.currency = currency.into();
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
/// use kakao_rs::components::basics::Template;
/// use kakao_rs::components::cards::ItemCard;
/// use kakao_rs::components::buttons::{Button, ButtonType};
///
/// let mut result = Template::new();
///
/// let item_card = ItemCard::new()
///     .set_title("title")
///     .set_desc("desc")
///     .set_thumbnail(
///         "http://dev-mk.kakao.com/dn/bot/scripts/with_barcode_blue_1x1.png"
///     )
///     .set_thumbnail_width(800)
///     .set_thumbnail_height(800)
///     .set_image_title("DOFQTK")
///     .set_image_desc("Boarding Number")
///     .set_item_list_alignment("right")
///     .set_item_list_summary("total", "$4,032.54")
///     .add_button(Button::new(ButtonType::Link)
///             .set_label("View Boarding Pass")
///             .set_link("https://namu.wiki/w/%EB%82%98%EC%97%B0(TWICE)"),
///     )
///     .set_button_layout("vertical");
///
/// result.add_output(item_card.build());
///
/// ```
pub struct ItemCard {
    #[serde(rename = "itemCard")]
    pub content: ItemCardContent,
}

impl Default for ItemCard {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemCard {
    /// ItemCard를 초기화 합니다.
    #[inline]
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
    pub fn set_title<S: Into<String>>(mut self, _title: S) -> Self {
        self.content.title = Some(_title.into());
        self
    }
    pub fn set_desc<S: Into<String>>(mut self, desc: S) -> Self {
        self.content.description = Some(desc.into());
        self
    }
    pub fn set_button_layout<S: Into<String>>(mut self, layout: S) -> Self {
        self.content.button_layout = Some(layout.into());
        self
    }
    pub fn set_item_list_alignment<S: Into<String>>(mut self, align: S) -> Self {
        self.content.item_list_alignment = Some(align.into());
        self
    }
    pub fn set_item_list_summary<S: Into<String>>(mut self, _title: S, _desc: S) -> Self {
        self.content.item_list_summary = Some(ItemListSummary {
            title: _title.into(),
            description: _desc.into(),
        });
        self
    }
    pub fn set_thumbnail<S: Into<String>>(mut self, url: S) -> Self {
        self.content.thumbnail = Some(ThumbNail::new(url));
        self
    }

    pub fn set_head<S: Into<String>>(mut self, _title: S) -> Self {
        self.content.head = Some(Head {
            title: _title.into(),
        });
        self
    }

    pub fn set_image_title<S: Into<String>>(mut self, _title: S) -> Self {
        self.content.image_title = Some(ImageTitle::new(_title.into()));
        self
    }

    pub fn set_image_desc<S: Into<String>>(mut self, _desc: S) -> Self {
        if let Some(image) = &mut self.content.image_title {
            image.description = Some(_desc.into());
        }

        self
    }

    pub fn set_image<S: Into<String>>(mut self, _url: S) -> Self {
        if let Some(image) = &mut self.content.image_title {
            image.image_url = Some(_url.into());
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
    /// ImageTitle을 초기화 합니다.
    #[inline]
    pub fn new<S: Into<String>>(_title: S) -> Self {
        ImageTitle {
            title: _title.into(),
            description: None,
            image_url: None,
        }
    }

    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();
    }

    pub fn set_desc<S: Into<String>>(&mut self, desc: S) {
        self.description = Some(desc.into());
    }

    pub fn set_image<S: Into<String>>(&mut self, url: S) {
        self.image_url = Some(url.into());
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
