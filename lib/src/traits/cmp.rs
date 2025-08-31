// SPDX-License-Identifier: MIT

/// Enables `TaggedType` to implement `PartialEq` if inner type
/// implements `PartialEq`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementPartialEq};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl ImplementPartialEq for UsernameTag {};
/// let admin = Username::new("admin".into());
/// let root = Username::new("root".into());
///
/// format!("{:?}", admin != root);
/// ```
pub trait ImplementPartialEq {}

/// Enables `TaggedType` to implement `Eq` if inner type
/// implements Eq.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementEq, ImplementPartialEq};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl ImplementPartialEq for UsernameTag {};
/// impl ImplementEq for UsernameTag {};
/// let admin = Username::new("admin".into());
/// let root = Username::new("root".into());
///
/// format!("{:?}", admin != root);
/// ```
pub trait ImplementEq {}

/// Enables `TaggedType` to implement `PartialOrd` if inner type
/// implements `PartialOrd`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementPartialOrd, ImplementPartialEq};
/// pub type Priority = TaggedType<u32, PriorityTag>;
/// pub enum PriorityTag {}
/// impl ImplementPartialEq for PriorityTag {};
/// impl ImplementPartialOrd for PriorityTag {};
/// let p0 = Priority::new(0);
/// let p1 = Priority::new(1);
///
/// format!("{:?}", p0 < p1);
/// ```
pub trait ImplementPartialOrd {}

/// Enables `TaggedType` to implement `Ord` if inner type
/// implements Ord.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementPartialEq, ImplementEq, ImplementOrd, ImplementPartialOrd};
/// pub type Priority = TaggedType<u32, PriorityTag>;
/// pub enum PriorityTag {}
/// impl ImplementPartialEq for PriorityTag {};
/// impl ImplementEq for PriorityTag {};
/// impl ImplementPartialOrd for PriorityTag {};
/// impl ImplementOrd for PriorityTag {};
/// let p0 = Priority::new(0);
/// let p1 = Priority::new(1);
///
/// format!("{:?}", p0 < p1);
/// ```
pub trait ImplementOrd {}
