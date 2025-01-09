#![forbid(unsafe_code)]

mod data;
pub use data::{Hitokoto, HitokotoType};

pub mod bundles;

#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::{HitokotoTypes, random_hitokoto};
