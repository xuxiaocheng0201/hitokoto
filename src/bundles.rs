//! This is the bundled sentences from [sentences-bundle.hitokoto.cn](https://sentences-bundle.hitokoto.cn/).
#![allow(unused_imports)]

use alloc::borrow::Cow;
use crate::{Hitokoto, HitokotoType, HitokotoCommitFrom};
#[cfg(feature = "language")]
use crate::Language;

include!(concat!(env!("OUT_DIR"), "/sentences.rs"));

/// Get hitokoto by id. If not found, return `None`.
pub fn get_hitokoto_by_id(id: u32) -> Option<Hitokoto> {
    for h in HITOKOTO_A { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_B { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_C { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_D { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_E { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_F { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_G { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_H { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_I { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_J { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_K { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTO_L { if h.id == id { return Some(h.clone()) } }
    None
}
