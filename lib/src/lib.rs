// SPDX-License-Identifier: MIT

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod tagged_type;
pub mod traits;

pub use traits::ImplementClone;
pub use traits::ImplementCopy;
pub use traits::ImplementDefault;
pub use traits::ImplementDeref;
pub use traits::ImplementEq;
pub use traits::ImplementHash;
pub use traits::ImplementOrd;
pub use traits::ImplementPartialEq;
pub use traits::ImplementPartialOrd;
pub use traits::InnerAccess;
pub use traits::TransparentDebug;
pub use traits::TransparentDisplay;
pub use traits::TransparentFromInner;
pub use traits::TransparentFromStr;

#[cfg(feature = "support_serde")]
pub use traits::serde::TransparentDeserialize;
#[cfg(feature = "support_serde")]
pub use traits::serde::TransparentSerialize;

#[cfg(feature = "provide_permissive")]
pub use traits::permissive::Permissive;

pub type TaggedType<V, T> = tagged_type::TaggedType<V, T>;

#[cfg(feature = "provide_derive")]
pub use tagged_types_derive::Tag;

#[cfg(feature = "provide_derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_all() {
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        #[derive(Tag)]
        #[implement(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
        #[transparent(Debug, Display, FromStr)]
        #[capability(inner_access)]
        enum CounterU64Tag {}

        let c = CounterU64::default();
        assert_eq!(*c.inner(), 0);
    }

    #[test]
    fn test_derive_permissive() {
        #[derive(Tag)]
        #[permissive]
        enum CounterU64Tag {}
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        let c = CounterU64::default();
        assert_eq!(*c.inner(), 0);
    }
}
