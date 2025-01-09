use std::borrow::Cow;

pub mod bundles;

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
    /// 网易云。主要收录网易云音乐热
    NCM,
    /// 哲学
    Philosophy,
    /// 抖机灵
    Funny,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hitokoto {
    /// 一言标识
    pub id: u32,
    /// 一言唯一标识
    #[cfg(feature = "uuid")]
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
    pub commit_from: Cow<'static, str>,
    /// 添加时间
    pub created_at: u64,
}
