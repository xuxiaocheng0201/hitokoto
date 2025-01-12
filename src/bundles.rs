//! This is the bundled sentences from [sentences-bundle.hitokoto.cn](https://sentences-bundle.hitokoto.cn/).
#![allow(unused_imports)]

use alloc::borrow::Cow;
use crate::{Hitokoto, HitokotoType};
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
