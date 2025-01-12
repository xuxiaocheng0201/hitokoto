use crate::{Hitokoto, HitokotoType};

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HitokotoTypes: u16 {
        /// 动画
        const Anime = 1 << 0;
        /// 漫画
        const Comic = 1 << 1;
        /// 游戏
        const Game = 1 << 2;
        /// 文学。主要收录现代文学：小说、散文、戏剧。
        const Literature = 1 << 3;
        /// 原创
        const Original = 1 << 4;
        /// 来自网络
        const Internet = 1 << 5;
        /// 其他
        const Other = 1 << 6;
        /// 影视
        const Video = 1 << 7;
        /// 诗词。主要收录中国古代文学，如：诗、歌、词、赋、曲等。
        const Poem = 1 << 8;
        /// 网易云。主要收录网易云音乐热评。
        const NCM = 1 << 9;
        /// 哲学
        const Philosophy = 1 << 10;
        /// 抖机灵
        const Funny = 1 << 11;
    }
}

impl From<HitokotoType> for HitokotoTypes {
    fn from(value: HitokotoType) -> Self {
        match value {
            HitokotoType::Anime => Self::Anime,
            HitokotoType::Comic => Self::Comic,
            HitokotoType::Game => Self::Game,
            HitokotoType::Literature => Self::Literature,
            HitokotoType::Original => Self::Original,
            HitokotoType::Internet => Self::Internet,
            HitokotoType::Other => Self::Other,
            HitokotoType::Video => Self::Video,
            HitokotoType::Poem => Self::Poem,
            HitokotoType::NCM => Self::NCM,
            HitokotoType::Philosophy => Self::Philosophy,
            HitokotoType::Funny => Self::Funny,
        }
    }
}

impl<T: IntoIterator<Item=HitokotoType>> From<T> for HitokotoTypes {
    fn from(value: T) -> Self {
        let mut bitset = Self::empty();
        for v in value {
            bitset |= Self::from(v);
        }
        bitset
    }
}

/// This is equivalent to requesting 'https://v1.hitokoto.cn/?c='
///
/// # Panic
/// If `types` is empty, this function will panic.
// https://developer.hitokoto.cn/sentence/demo.html
pub fn random_hitokoto(types: HitokotoTypes) -> Hitokoto {
    assert_ne!(types, HitokotoTypes::empty(), "Random hitokoto types should not be empty.");
    let a_len = if types.contains(HitokotoTypes::Anime) { crate::bundles::HITOKOTOS_A.len() } else { 0 };
    let b_len = if types.contains(HitokotoTypes::Comic) { crate::bundles::HITOKOTOS_B.len() } else { 0 };
    let c_len = if types.contains(HitokotoTypes::Game) { crate::bundles::HITOKOTOS_C.len() } else { 0 };
    let d_len = if types.contains(HitokotoTypes::Literature) { crate::bundles::HITOKOTOS_D.len() } else { 0 };
    let e_len = if types.contains(HitokotoTypes::Original) { crate::bundles::HITOKOTOS_E.len() } else { 0 };
    let f_len = if types.contains(HitokotoTypes::Internet) { crate::bundles::HITOKOTOS_F.len() } else { 0 };
    let g_len = if types.contains(HitokotoTypes::Other) { crate::bundles::HITOKOTOS_G.len() } else { 0 };
    let h_len = if types.contains(HitokotoTypes::Video) { crate::bundles::HITOKOTOS_H.len() } else { 0 };
    let i_len = if types.contains(HitokotoTypes::Poem) { crate::bundles::HITOKOTOS_I.len() } else { 0 };
    let j_len = if types.contains(HitokotoTypes::NCM) { crate::bundles::HITOKOTOS_J.len() } else { 0 };
    let k_len = if types.contains(HitokotoTypes::Philosophy) { crate::bundles::HITOKOTOS_K.len() } else { 0 };
    let l_len = if types.contains(HitokotoTypes::Funny) { crate::bundles::HITOKOTOS_L.len() } else { 0 };
    let total_len = a_len + b_len + c_len + d_len + e_len + f_len + g_len + h_len + i_len + j_len + k_len + l_len;
    let mut random_index = rand::Rng::gen_range(&mut rand::thread_rng(), 0..total_len);
    if random_index < a_len { return crate::bundles::HITOKOTOS_A[random_index].clone(); } random_index -= a_len;
    if random_index < b_len { return crate::bundles::HITOKOTOS_B[random_index].clone(); } random_index -= b_len;
    if random_index < c_len { return crate::bundles::HITOKOTOS_C[random_index].clone(); } random_index -= c_len;
    if random_index < d_len { return crate::bundles::HITOKOTOS_D[random_index].clone(); } random_index -= d_len;
    if random_index < e_len { return crate::bundles::HITOKOTOS_E[random_index].clone(); } random_index -= e_len;
    if random_index < f_len { return crate::bundles::HITOKOTOS_F[random_index].clone(); } random_index -= f_len;
    if random_index < g_len { return crate::bundles::HITOKOTOS_G[random_index].clone(); } random_index -= g_len;
    if random_index < h_len { return crate::bundles::HITOKOTOS_H[random_index].clone(); } random_index -= h_len;
    if random_index < i_len { return crate::bundles::HITOKOTOS_I[random_index].clone(); } random_index -= i_len;
    if random_index < j_len { return crate::bundles::HITOKOTOS_J[random_index].clone(); } random_index -= j_len;
    if random_index < k_len { return crate::bundles::HITOKOTOS_K[random_index].clone(); } random_index -= k_len;
    if random_index < l_len { return crate::bundles::HITOKOTOS_L[random_index].clone(); } random_index -= l_len;
    unreachable!("This should a bug. types={types:?}, rest_random={random_index}");
}
