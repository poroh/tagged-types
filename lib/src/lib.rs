// SPDX-License-Identifier: MIT

//! tagged-types is a library that simplifies implementation of the [New type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
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

pub use traits::FromInner;
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

/// Export `TaggedType` from top level.
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
