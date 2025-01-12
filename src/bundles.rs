//! This is the bundled sentences from [sentences-bundle.hitokoto.cn](https://sentences-bundle.hitokoto.cn/).
#![allow(unused_imports)]

use alloc::borrow::Cow;
use crate::{Hitokoto, HitokotoType, HitokotoCommitFrom};
#[cfg(feature = "language")]
use crate::Language;

include!(concat!(env!("OUT_DIR"), "/sentences_a.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_b.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_c.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_d.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_e.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_f.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_g.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_h.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_i.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_j.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_k.rs"));
include!(concat!(env!("OUT_DIR"), "/sentences_l.rs"));

/// Get hitokoto by id. If not found, return `None`.
pub fn get_hitokoto_by_id(id: u32) -> Option<Hitokoto> {
    for h in HITOKOTOS_A { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_B { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_C { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_D { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_E { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_F { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_G { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_H { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_I { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_J { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_K { if h.id == id { return Some(h.clone()) } }
    for h in HITOKOTOS_L { if h.id == id { return Some(h.clone()) } }
    None
}
