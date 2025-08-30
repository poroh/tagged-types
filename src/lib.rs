// SPDX-License-Identifier: MIT

pub mod tagged_type;

pub use tagged_type::TransparentDebug;
pub use tagged_type::TransparentDisplay;
pub use tagged_type::TransparentFromInner;
pub use tagged_type::TransparentFromStr;

pub type TaggedType<V, T> = tagged_type::TaggedType<V, T>;
