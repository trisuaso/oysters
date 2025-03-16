#![doc = include_str!("../../../README.md")]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]
pub mod oyster;
pub mod pearl;

#[cfg(feature = "lru")]
mod time;

#[cfg(feature = "lru")]
mod lru;

#[cfg(feature = "persistance")]
mod persistance;

pub use oyster::Oyster;
pub use pearl::Pearl;
