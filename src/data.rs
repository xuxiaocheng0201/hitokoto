use alloc::borrow::Cow;

/// Hitokoto type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HitokotoType {
    /// 动画
    Anime,
    /// 漫画
    Comic,
    /// 游戏
    Game,
    /// 文学。主要收录现代文学：小说、散文、戏剧。
    Literature,
    /// 原创
    Original,
    /// 来自网络
    Internet,
    /// 其他
    Other,
    /// 影视
    Video,
    /// 诗词。主要收录中国古代文学，如：诗、歌、词、赋、曲等。
    Poem,
    /// 网易云。主要收录网易云音乐热评。
    NCM,
    /// 哲学
    Philosophy,
    /// 抖机灵
    Funny,
}

/// Hitokoto data.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hitokoto {
    /// 一言标识
    pub id: u32,
    /// 一言唯一标识
    #[cfg(feature = "uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
    pub uuid: uuid::Uuid,
    /// 一言正文
    pub hitokoto: Cow<'static, str>,
    /// 类型
    pub r#type: HitokotoType,
    /// 一言的出处
    pub from: Cow<'static, str>,
    /// 一言的作者
    pub from_who: Option<Cow<'static, str>>,
    /// 添加者
    pub creator: Cow<'static, str>,
    /// 添加者用户标识
    pub creator_uid: u32,
    /// 审核员标识
    pub reviewer: u32,
    /// 提交方式
    pub commit_from: HitokotoCommitFrom,
    /// 添加时间
    #[cfg(feature = "time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 语言
    #[cfg(feature = "language")]
    #[cfg_attr(docsrs, doc(cfg(feature = "language")))]
    pub language: Language,
}

/// Where the hitokoto commit from.
///
/// Currently (hitokoto sentences v1.0.399), only this three versions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HitokotoCommitFrom {
    Web,
    Api,
    App,
}

/// This is a copy from [lingua Language](https://docs.rs/lingua/%5E1.6/lingua/enum.Language.html).
/// In order to reduce dependencies.
///
/// In hitokoto sentences v1.0.399, the count of languages are as follows:
/// ```text
/// Bokmal: 2
/// German: 3
/// Dutch: 1
/// Albanian: 1
/// Latin: 4
/// Romanian: 1
/// Korean: 1
/// Ganda: 1
/// Japanese: 48
/// Finnish: 1
/// Tswana: 1
/// French: 2
/// Chinese: 6795
/// Italian: 1
/// Afrikaans: 2
/// English: 123
/// Bosnian: 1
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(feature = "language")]
#[cfg_attr(docsrs, doc(cfg(feature = "language")))]
#[allow(missing_docs)]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Maori,
    Marathi,
    Mongolian,
    Nynorsk,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Xhosa,
    Yoruba,
    Zulu,
}
