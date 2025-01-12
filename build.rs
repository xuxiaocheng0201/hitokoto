#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = std::env::var("HITOKOTO_BASE_URL")
        .unwrap_or("https://sentences-bundle.hitokoto.cn".to_string());
    let out_dir = std::env::var("OUT_DIR")?;

    // https://sentences-bundle.hitokoto.cn/categories.json
    tokio::try_join!(
        handle_hitokoto(&base_url, &out_dir, 'a', "Anime - 动画"),
        handle_hitokoto(&base_url, &out_dir, 'b', "Comic - 漫画"),
        handle_hitokoto(&base_url, &out_dir, 'c', "Game - 游戏"),
        handle_hitokoto(&base_url, &out_dir, 'd', "Literature - 文学。主要收录现代文学：小说、散文、戏剧。"),
        handle_hitokoto(&base_url, &out_dir, 'e', "Original - 原创"),
        handle_hitokoto(&base_url, &out_dir, 'f', "Internet - 来自网络"),
        handle_hitokoto(&base_url, &out_dir, 'g', "Other - 其他"),
        handle_hitokoto(&base_url, &out_dir, 'h', "Video - 影视"),
        handle_hitokoto(&base_url, &out_dir, 'i', "Poem - 诗词。主要收录中国古代文学，如：诗、歌、词、赋、曲等。"),
        handle_hitokoto(&base_url, &out_dir, 'j', "NCM - 网易云。主要收录网易云音乐热评。"),
        handle_hitokoto(&base_url, &out_dir, 'k', "Philosophy - 哲学"),
        handle_hitokoto(&base_url, &out_dir, 'l', "Funny - 抖机灵"),
    )?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Hitokoto {
    pub id: u32,
    pub uuid: uuid::Uuid,
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub r#type: char,
    pub from: String,
    pub from_who: Option<String>,
    pub creator: String,
    pub creator_uid: u32,
    pub reviewer: u32,
    pub commit_from: String,
    pub created_at: String,
}

async fn handle_hitokoto(base_url: &str, out_dir: &str, category: char, category_doc: &str) -> anyhow::Result<()> {
   let list = if option_env!("DOCS_RS") != Some("1") {
        use anyhow::Context;
        let url = format!("{base_url}/sentences/{category}.json");
        reqwest::get(url).await?.json::<Vec<Hitokoto>>().await.context(format!("decode {category}"))?
    } else {
        // This is for building on docs.rs with no web requests.
        Vec::new()
    };
    let mut rust = format!("\
        /// {category_doc}
        ///
        /// From: https://sentences-bundle.hitokoto.cn/sentences/{category}.json
        pub static HITOKOTOS_{}: &[Hitokoto] = &[", category.to_ascii_uppercase()
    );
    for hitokoto in list {
        rust.push_str(&format!(r##"
            Hitokoto {{
                id: {},
                #[cfg(feature = "uuid")]
                uuid: uuid::Uuid::from_bytes([{}]),
                hitokoto: Cow::Borrowed({:?}),
                r#type: HitokotoType::{},
                from: Cow::Borrowed({:?}),
                from_who: {},
                creator: Cow::Borrowed({:?}),
                creator_uid: {},
                reviewer: {},
                commit_from: Cow::Borrowed({:?}),
                created_at: {},
            }},
        "##,
            hitokoto.id,
            hitokoto.uuid.as_bytes().map(|a| a.to_string()).join(", "),
            hitokoto.hitokoto,
            match hitokoto.r#type {
                'a' => "Anime",
                'b' => "Comic",
                'c' => "Game",
                'd' => "Literature",
                'e' => "Original",
                'f' => "Internet",
                'g' => "Other",
                'h' => "Video",
                'i' => "Poem",
                'j' => "NCM",
                'k' => "Philosophy",
                'l' => "Funny",
                t @ _ => {
                    println!("cargo:warning=Unknown hitokoto type in category {category}, type={t}.");
                    "Anime"
                },
            },
            hitokoto.from,
            match hitokoto.from_who {
                None => "None".to_string(),
                Some(from_who) => format!(r#"Some(Cow::Borrowed({:?}))"#, from_who),
            },
            hitokoto.creator,
            hitokoto.creator_uid,
            hitokoto.reviewer,
            hitokoto.commit_from,
            hitokoto.created_at,
        ));
    }
    rust.push_str("];");
    let file_path = format!("{out_dir}/sentences_{category}.rs");
    tokio::fs::write(&file_path, rust.as_bytes()).await?;
    Ok(())
}
