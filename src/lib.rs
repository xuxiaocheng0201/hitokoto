#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

mod data;
pub use data::{Hitokoto, HitokotoType};

pub mod bundles;

#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::{HitokotoTypes, random_hitokoto, random_hitokoto_option};
