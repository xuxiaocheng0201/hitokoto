fn main() {
    let hitokoto = hitokoto::random_hitokoto(hitokoto::HitokotoTypes::all());
    println!(
        "{} ——— {}「{}」",
        hitokoto.hitokoto,
        match hitokoto.from_who { Some(who) => who, None => std::borrow::Cow::Borrowed(""), },
        hitokoto.from,
    );
}
