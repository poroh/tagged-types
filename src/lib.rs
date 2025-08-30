// SPDX-License-Identifier: MIT

pub mod tagged_type;
pub mod traits;

pub use traits::ImplementClone;
pub use traits::ImplementCopy;
pub use traits::ImplementDefault;
pub use traits::ImplementDeref;
pub use traits::ImplementEq;
pub use traits::ImplementHash;
pub use traits::ImplementPartialEq;
pub use traits::InnerAccess;
pub use traits::TransparentDebug;
pub use traits::TransparentDisplay;
pub use traits::TransparentFromInner;
pub use traits::TransparentFromStr;

#[cfg(feature = "serde_support")]
pub use traits::TransparentDeserialize;
#[cfg(feature = "serde_support")]
pub use traits::TransparentSerialize;

#[cfg(feature = "use_permissive")]
pub use traits::Permissive;

pub type TaggedType<V, T> = tagged_type::TaggedType<V, T>;
