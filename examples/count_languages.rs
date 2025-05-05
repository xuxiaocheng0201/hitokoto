fn main() {
    println!("Bundled version: {}", hitokoto::bundled_version());
    #[cfg(feature = "language")] {
        let mut languages = std::collections::HashMap::new();
        for sentence in hitokoto::get_all_hitokoto() {
            *languages.entry(sentence.language).or_insert(0usize) += 1;
        }
        let mut languages = languages.into_iter().collect::<Vec<_>>();
        languages.sort_by(|(al, a), (bl, b)|
            Ord::cmp(a, b).reverse().then(Ord::cmp(&format!("{al:?}"), &format!("{bl:?}")))
        );
        for (language, count) in languages {
            println!("{:?}: {}", language, count);
        }
    }
}
