#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = std::env::var("HITOKOTO_BASE_URL")
        .unwrap_or("https://sentences-bundle.hitokoto.cn".to_string());
    let out_dir = std::env::var("OUT_DIR")?;

    // https://sentences-bundle.hitokoto.cn/categories.json
    let (a, b, c, d, e, f, g, h, i, j, k, l) = tokio::try_join!(
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
    if a || b || c || d || e || f || g || h || i || j || k || l {
        println!("cargo:warning=If you see any warning, please report this bug to https://github.com/xuxiaocheng0201/hitokoto/ .");
    }

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

async fn handle_hitokoto(base_url: &str, out_dir: &str, category: char, category_doc: &str) -> anyhow::Result<bool> {
    let list = if option_env!("DOCS_RS") != Some("1") {
        use anyhow::Context;
        let url = format!("{base_url}/sentences/{category}.json");
        reqwest::get(url).await?.json::<Vec<Hitokoto>>().await.context(format!("decode {category}"))?
    } else {
        // This is for building on docs.rs with no web requests.
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        Vec::new()
    };
    let mut rust = format!("\
/// {category_doc}
///
/// From: <https://sentences-bundle.hitokoto.cn/sentences/{category}.json>
pub static HITOKOTOS_{}: &[Hitokoto] = &[
", category.to_ascii_uppercase()
    );

    #[cfg(not(feature = "language"))]
    let iter = list.into_iter().map(|hitokoto| (hitokoto, None::<()>));
    #[cfg(feature = "language")]
    let list = tokio::task::spawn_blocking(move || {
        let detector = lingua::LanguageDetectorBuilder::from_all_languages().build();
        let texts = list.iter().map(|hitokoto| hitokoto.hitokoto.as_str()).collect::<Vec<_>>();
        let languages = detector.detect_languages_in_parallel_of(&texts);
        (list, languages)
    }).await?;
    #[cfg(feature = "language")]
    let iter = list.0.into_iter().zip(list.1);

    let mut error = false;
    for (hitokoto, language) in iter {
        rust.push_str("    Hitokoto {");
        rust.push_str(&format!("id: {},", hitokoto.id));

        rust.push_str(r##"#[cfg(feature = "uuid")]"##);
        rust.push_str(&format!(
            "uuid: uuid::Uuid::from_bytes([{}]),",
            hitokoto.uuid.as_bytes().map(|a| a.to_string()).join(", ")
        ));

        rust.push_str(&format!("hitokoto: Cow::Borrowed({:?}),", hitokoto.hitokoto));

        rust.push_str(&format!("r#type: HitokotoType::{},", match hitokoto.r#type {
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
                error = true;
                "Anime"
            },
        }));

        rust.push_str(&format!("from: Cow::Borrowed({:?}),", hitokoto.from));

        rust.push_str(&format!("from_who: {},", match hitokoto.from_who {
            None => "None".to_string(),
            Some(from_who) => format!(r#"Some(Cow::Borrowed({:?}))"#, from_who),
        }));

        rust.push_str(&format!("creator: Cow::Borrowed({:?}),", hitokoto.creator));

        rust.push_str(&format!("creator_uid: {},", hitokoto.creator_uid));

        rust.push_str(&format!("reviewer: {},", hitokoto.reviewer));

        rust.push_str(&format!("commit_from: HitokotoCommitFrom::{},", match hitokoto.commit_from.as_str() {
            "web" => "Web",
            "api" => "Api",
            "app" => "App",
            f @ _ => {
                println!("cargo:warning=Unknown hitokoto commit_from in category {category}, commit_from={f}.");
                error = true;
                "Web"
            }
        }));

        rust.push_str(r##"#[cfg(feature = "time")]"##);
        rust.push_str(&format!(
            "created_at: chrono::DateTime::<chrono::Utc>::from_timestamp_nanos({}),",
            match <i64 as std::str::FromStr>::from_str(&hitokoto.created_at) {
                Ok(time) => match time.checked_mul(1_000_000_000) {
                    Some(time) => time, // Sometimes, the timestamp is in seconds.
                    None => time * 1_000_000, // Sometimes, the timestamp is in milliseconds.
                },
                Err(_) => {
                    println!("cargo:warning=Unknown hitokoto created_at in category {category}, created_at={}.", hitokoto.created_at);
                    error = true;
                    0
                },
            }
        ));

        #[cfg(not(feature = "language"))]
        let _ = language;
        #[cfg(feature = "language")]
        rust.push_str(&format!(
            "language: Language::{},",
            match language {
                Some(language) => format!("{language:?}"),
                None => {
                    println!("cargo:warning=Unknown hitokoto language in category {category}, hitokoto={:?}.", hitokoto.hitokoto);
                    error = true;
                    "Chinese".to_string()
                },
            }
        ));

        rust.push_str("},\n");
    }
    rust.push_str("];");
    let file_path = format!("{out_dir}/sentences_{category}.rs");
    tokio::fs::write(&file_path, rust.as_bytes()).await?;
    Ok(error)
}
