#[derive(Debug, serde::Deserialize)]
struct Category {
    // id: u32,
    name: String,
    desc: String,
    key: String,
    created_at: String,
    updated_at: String,
    // path: String,
}

#[derive(Debug, serde::Deserialize)]
struct Sentence {
    id: u32,
    #[cfg(feature = "uuid")]
    uuid: uuid::Uuid,
    hitokoto: String,
    // #[serde(rename = "type")]
    // r#type: char,
    from: String,
    from_who: Option<String>,
    creator: String,
    creator_uid: u32,
    reviewer: u32,
    commit_from: String,
    #[cfg(feature = "time")]
    created_at: String,
    // length: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if option_env!("DOCS_RS") == Some("1") {
        // This is for building on docs.rs with no web requests.
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        docs_rs_main().await
    } else {
        real_main().await
    }
}

async fn docs_rs_main() -> anyhow::Result<()> {
    let mut rust = String::new();
    rust.push_str("#[doc = \"Visit `https://sentences-bundle.hitokoto.cn/version.json` to check this is the latest version.\"]\n");
    rust.push_str(&format!("pub fn bundled_version() -> &'static str {{ \"DOCS-RS\" }}\n\n"));
    for category in [
        Category { /*id: 1,*/ name: "动画".to_string(), desc: "Anime - 动画".to_string(), key: "a".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/a.json".to_string(),*/ },
        Category { /*id: 2,*/ name: "漫画".to_string(), desc: "Comic - 漫画".to_string(), key: "b".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/b.json".to_string(),*/ },
        Category { /*id: 3,*/ name: "游戏".to_string(), desc: "Game - 游戏".to_string(), key: "c".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/c.json".to_string(),*/ },
        Category { /*id: 4,*/ name: "文学".to_string(), desc: "Literature - 文学。主要收录现代文学：小说、散文、戏剧。".to_string(), key: "d".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/d.json".to_string(),*/ },
        Category { /*id: 5,*/ name: "原创".to_string(), desc: "Original - 原创".to_string(), key: "e".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/e.json".to_string(),*/ },
        Category { /*id: 6,*/ name: "网络".to_string(), desc: "Internet - 来自网络".to_string(), key: "f".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/f.json".to_string(),*/ },
        Category { /*id: 7,*/ name: "其他".to_string(), desc: "Other - 其他".to_string(), key: "g".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/g.json".to_string(),*/ },
        Category { /*id: 8,*/ name: "影视".to_string(), desc: "Video - 影视".to_string(), key: "h".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/h.json".to_string(),*/ },
        Category { /*id: 9,*/ name: "诗词".to_string(), desc: "Poem - 诗词。主要收录中国古代文学，如：诗、歌、词、赋、曲等。".to_string(), key: "i".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/i.json".to_string(),*/ },
        Category { /*id: 10,*/ name: "网易云".to_string(), desc: "NCM - 网易云。主要收录网易云音乐热评。".to_string(), key: "j".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/j.json".to_string(),*/ },
        Category { /*id: 11,*/ name: "哲学".to_string(), desc: "Philosophy - 哲学".to_string(), key: "k".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/k.json".to_string(),*/ },
        Category { /*id: 12,*/ name: "抖机灵".to_string(), desc: "Funny - 抖机灵".to_string(), key: "l".to_string(), created_at: "Unknown".to_string(), updated_at: "Unknown".to_string(), /*path: "./sentences/l.json".to_string(),*/ },
    ] {
        rust.push_str(&format!("include!(concat!(env!(\"OUT_DIR\"), \"/sentences_{}.rs\"));\n", category.key));
        handle_sentences(category, Vec::new()).await?;
    }
    let out_dir = std::env::var("OUT_DIR")?;
    std::fs::write(&format!("{out_dir}/sentences.rs"), rust.as_bytes())?;
    Ok(())
}

async fn real_main() -> anyhow::Result<()> {
    const USER_AGENT: &str = "hitokoto-rs(https://crates.io/crates/hitokoto)";

    println!("Getting latest tag from Github.");
    let client = reqwest::Client::new();
    let json = client
        .get("https://api.github.com/repos/hitokoto-osc/sentences-bundle/tags?per_page=1")
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", USER_AGENT)
        .send().await?
        .error_for_status()?
        .json::<serde_json::Value>().await?;
    let version = json[0]["name"].as_str().expect("Error in Github Api.");
    let zip_url = json[0]["zipball_url"].as_str().expect("Error in Github Api.");

    println!("Checking downloaded sentences.");
    let out_dir = std::env::var("OUT_DIR")?;
    let tag_file = format!("{out_dir}/.tag");
    let zip_file = format!("{out_dir}/.zip");
    let should_download = match std::fs::read_to_string(&tag_file) {
        Ok(old_version) if old_version == version => false,
        Ok(_old_version) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
        Err(error) => return Err(error.into()),
    };
    let bytes = if should_download {
        println!("Downloading sentences from Github.");
        // This should be the same as https://sentences-bundle.hitokoto.cn
        let bytes = client.get(zip_url)
            .header("User-Agent", USER_AGENT)
            .send().await?
            .error_for_status()?
            .bytes().await?;
        std::fs::write(&zip_file, bytes.clone())?;
        std::fs::write(&tag_file, version)?;
        bytes
    } else {
        std::fs::read(&zip_file)?.into()
    };

    println!("Getting categories from zip.");
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let root_dir = zip.name_for_index(0).expect("Error in Github Zip Ball.").to_string();
    let categories = zip.by_name(&format!("{root_dir}categories.json"))?;
    let categories = serde_json::from_reader::<_, Vec<Category>>(categories)?;

    let mut rust = String::new();
    rust.push_str("#[doc = \"Visit `https://sentences-bundle.hitokoto.cn/version.json` to check this is the latest version.\"]\n");
    rust.push_str(&format!("pub fn bundled_version() -> &'static str {{ {version:?} }}\n\n"));
    let mut join_set = tokio::task::JoinSet::new();
    for category in categories {
        println!("Getting sentences {} from zip.", category.key);
        let sentences = zip.by_name(&format!("{root_dir}sentences/{}.json", category.key))?;
        let sentences = serde_json::from_reader::<_, Vec<Sentence>>(sentences)?;
        rust.push_str(&format!("include!(concat!(env!(\"OUT_DIR\"), \"/sentences_{}.rs\"));\n", category.key));
        join_set.spawn(async move { handle_sentences(category, sentences).await });
    }
    std::fs::write(&format!("{out_dir}/sentences.rs"), rust.as_bytes())?;

    let mut error = false;
    while let Some(result) = join_set.join_next().await {
        error |= result??;
    }
    if error {
        println!("cargo:warning=If you see any warning, please report the bug to https://github.com/xuxiaocheng0201/hitokoto/issues");
    }

    Ok(())
}


impl Category {
    // HitokotoType::{}
    fn get_type(&self) -> Option<&'static str> {
        match self.key.as_str() {
            "a" => Some("Anime"),
            "b" => Some("Comic"),
            "c" => Some("Game"),
            "d" => Some("Literature"),
            "e" => Some("Original"),
            "f" => Some("Internet"),
            "g" => Some("Other"),
            "h" => Some("Video"),
            "i" => Some("Poem"),
            "j" => Some("NCM"),
            "k" => Some("Philosophy"),
            "l" => Some("Funny"),
            _ => None,
        }
    }
}

impl Sentence {
    // HitokotoCommitFrom::{}
    fn get_commit_from(&self) -> Option<&'static str> {
        match self.commit_from.as_str() {
            "web" => Some("Web"),
            "api" => Some("Api"),
            "app" => Some("App"),
            _ => None,
        }
    }
}

async fn handle_sentences(category: Category, sentences: Vec<Sentence>) -> anyhow::Result<bool> {
    let mut rust = String::new();
    rust.push_str(&format!("#[doc = \"Bundled sentences in category {}\"]\n", category.name));
    rust.push_str("#[doc = \"\"]\n");
    rust.push_str(&format!("#[doc = \"Description: {}\"]\n", category.desc));
    rust.push_str(&format!("#[doc = \"Create at: {}\"]\n", category.created_at));
    rust.push_str(&format!("#[doc = \"Update at: {}\"]\n", category.updated_at));
    rust.push_str(&format!("#[doc = \"Sentence count: {}\"]\n", sentences.len()));
    rust.push_str("#[doc = \"\"]\n");
    rust.push_str(&format!("#[doc = \"Link: https://sentences-bundle.hitokoto.cn/sentences/{}.json\"]\n", category.key));
    rust.push_str(&format!("pub static HITOKOTO_{}: &[Hitokoto] = &[\n", category.key.to_ascii_uppercase()));

    #[cfg(not(feature = "language"))]
    let iter = sentences.into_iter().map(|sentence| (sentence, None::<()>));
    #[cfg(feature = "language")]
    let iter = tokio::task::spawn_blocking(move || {
        let detector = lingua::LanguageDetectorBuilder::from_all_languages().build();
        let texts = sentences.iter().map(|sentence| sentence.hitokoto.as_str()).collect::<Vec<_>>();
        let languages = detector.detect_languages_in_parallel_of(&texts);
        sentences.into_iter().zip(languages)
    }).await?;

    let mut error = false;
    let r#type = match category.get_type() { Some(t) => t, None => {
        println!("cargo:warning=Unknown hitokoto type for category {}.", category.key);
        error = true;
        "Other" // g
    } };
    for (sentence, _language) in iter {
        rust.push_str("  Hitokoto {");
        rust.push_str(&format!("id: {},", sentence.id));
        #[cfg(feature = "uuid")]
        rust.push_str(&format!("uuid: uuid::Uuid::from_bytes([{}]),", sentence.uuid.as_bytes().map(|a| a.to_string()).join(",")));
        rust.push_str(&format!("hitokoto: Cow::Borrowed({:?}),", sentence.hitokoto));
        rust.push_str(&format!("r#type: HitokotoType::{type},"));
        rust.push_str(&format!("from: Cow::Borrowed({:?}),", sentence.from));
        rust.push_str(&format!("from_who: {},", match sentence.from_who.as_ref() {
            None => "None".to_string(),
            Some(from_who) => format!(r#"Some(Cow::Borrowed({:?}))"#, from_who),
        }));
        rust.push_str(&format!("creator: Cow::Borrowed({:?}),", sentence.creator));
        rust.push_str(&format!("creator_uid: {},", sentence.creator_uid));
        rust.push_str(&format!("reviewer: {},", sentence.reviewer));
        rust.push_str(&format!("commit_from: HitokotoCommitFrom::{},", match sentence.get_commit_from() { Some(f) => f, None => {
            println!("cargo:warning=Unknown hitokoto commit_from in category {}, commit_from={}.", category.key, sentence.commit_from);
            error = true;
            "Web"
        }}));
        #[cfg(feature = "time")]
        rust.push_str(&format!(
            "created_at: chrono::DateTime::<chrono::Utc>::from_timestamp_nanos({}),",
            match <i64 as std::str::FromStr>::from_str(&sentence.created_at) {
                Ok(time) => match time.checked_mul(1_000_000_000) {
                    Some(time) => time, // Sometimes, the timestamp is in seconds.
                    None => time * 1_000_000, // Sometimes, the timestamp is in milliseconds.
                },
                Err(_) => {
                    println!("cargo:warning=Unknown hitokoto created_at in category {}, created_at={}.", category.key, sentence.created_at);
                    error = true;
                    0
                },
            }
        ));
        #[cfg(feature = "language")]
        rust.push_str(&format!(
            "language: Language::{},",
            match _language {
                Some(language) => format!("{language:?}"),
                None => {
                    println!("cargo:warning=Unknown hitokoto language in category {}, hitokoto={:?}.", category.key, sentence.hitokoto);
                    error = true;
                    "Chinese".to_string()
                },
            }
        ));
        rust.push_str("},\n");
    }

    rust.push_str("];\n");
    let out_dir = std::env::var("OUT_DIR")?;
    std::fs::write(&format!("{out_dir}/sentences_{}.rs", category.key), rust)?;
    Ok(error)
}
