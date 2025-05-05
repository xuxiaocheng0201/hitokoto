#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

mod data;
pub use data::{Hitokoto, HitokotoCommitFrom, HitokotoType};
#[cfg(feature = "language")]
pub use data::Language;

mod bundles;
pub use bundles::{bundled_version, get_all_hitokoto, get_hitokoto_by_id};
pub mod bundled {
    //! This is the bundled sentences from [sentences-bundle.hitokoto.cn](https://sentences-bundle.hitokoto.cn/).
    pub use crate::bundles::HITOKOTO_A;
    pub use crate::bundles::HITOKOTO_B;
    pub use crate::bundles::HITOKOTO_C;
    pub use crate::bundles::HITOKOTO_D;
    pub use crate::bundles::HITOKOTO_E;
    pub use crate::bundles::HITOKOTO_F;
    pub use crate::bundles::HITOKOTO_G;
    pub use crate::bundles::HITOKOTO_H;
    pub use crate::bundles::HITOKOTO_I;
    pub use crate::bundles::HITOKOTO_J;
    pub use crate::bundles::HITOKOTO_K;
    pub use crate::bundles::HITOKOTO_L;
}

#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::{random_hitokoto, random_hitokoto_option, HitokotoTypes};
