// SPDX-License-Identifier: MIT

//! `tagged-types` is a zero-dependency* library that helps you introduce new types
//! with zero runtime cost.
//!
//! \* - you can opt-in using `serde-support` feature to `serde` dependency if you want
//!     `Serialize`/`Deserialize` implementation for your new type.
//!
//! In many cases, we want strict types, but we don't want to spend a
//! lot of time implementing boilerplate around them (serialization/deserialization/parsing/clone/copy, etc.).
//!
//! This crate provides implementations for you. You can choose between
//! two implementations:
//! - Permissive, which provides automatic implementations of all supported traits.
//! - Fine-grained, inheriting only the traits needed for your New type.
//!
//! Optionally, you can also use [`tagged-types-derive`] to further reduce the verbosity
//! of the implementation.

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf
)]
#![deny(
    clippy::absolute_paths,
    clippy::todo,
    clippy::unimplemented,
    clippy::tests_outside_test_module,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::panic,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unused_trait_names,
    clippy::print_stdout,
    clippy::print_stderr
)]
#![deny(missing_docs)]

/// Definition of `TaggedType`.
pub mod tagged_type;

/// Definitions of crate's traits.
pub mod traits;

#[doc(inline)]
pub use tagged_type::TaggedType;
#[cfg(feature = "provide_derive")]
#[doc(inline)]
pub use tagged_types_derive::Tag;
#[cfg(feature = "provide_permissive")]
#[doc(inline)]
pub use traits::permissive::Permissive;
#[cfg(feature = "support_serde")]
#[doc(inline)]
pub use traits::serde::TransparentDeserialize;
#[cfg(feature = "support_serde")]
#[doc(inline)]
pub use traits::serde::TransparentSerialize;
#[doc(inline)]
pub use traits::AsRef;
#[doc(inline)]
pub use traits::Cloned;
#[doc(inline)]
pub use traits::FromInner;
#[doc(inline)]
pub use traits::ImplementAdd;
#[doc(inline)]
pub use traits::ImplementClone;
#[doc(inline)]
pub use traits::ImplementCopy;
#[doc(inline)]
pub use traits::ImplementDefault;
#[doc(inline)]
pub use traits::ImplementDeref;
#[doc(inline)]
pub use traits::ImplementDiv;
#[doc(inline)]
pub use traits::ImplementEq;
#[doc(inline)]
pub use traits::ImplementHash;
#[doc(inline)]
pub use traits::ImplementMul;
#[doc(inline)]
pub use traits::ImplementOrd;
#[doc(inline)]
pub use traits::ImplementPartialEq;
#[doc(inline)]
pub use traits::ImplementPartialOrd;
#[doc(inline)]
pub use traits::ImplementSub;
#[doc(inline)]
pub use traits::InnerAccess;
#[doc(inline)]
pub use traits::TransparentDebug;
#[doc(inline)]
pub use traits::TransparentDisplay;
#[doc(inline)]
pub use traits::TransparentFromInner;
#[doc(inline)]
pub use traits::TransparentFromStr;
#[doc(inline)]
pub use traits::ValueMap;

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
        #[capability(inner_access, from_inner, value_map, cloned, as_ref)]
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
