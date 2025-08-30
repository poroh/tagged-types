// SPDX-License-Identifier: MIT

pub mod tagged_type;

pub use tagged_type::TransparentDebug;
pub use tagged_type::TransparentDisplay;
pub type TaggedType<V, T> = tagged_type::TaggedType<V, T>;
