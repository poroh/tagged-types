// SPDX-License-Identifier: MIT

/// Define traits related to `core::cmp` traits.
pub mod cmp;
/// Defines Permissive trait if `provide_permissive` feature is
/// defined.
#[cfg(feature = "provide_permissive")]
pub mod permissive;
/// Defines serde-related traits if `support_serde` feature is
/// defined.
#[cfg(feature = "support_serde")]
pub mod serde;

pub use cmp::ImplementEq;
pub use cmp::ImplementOrd;
pub use cmp::ImplementPartialEq;
pub use cmp::ImplementPartialOrd;

/// Enables `TaggedType` to implement access to inner data
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, InnerAccess};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl InnerAccess for UsernameTag {};
///
/// format!("{}", Username::new("admin".into()).inner());
/// format!("{}", Username::new("admin".into()).into_inner());
/// ```
pub trait InnerAccess {}

/// Enables `TaggedType` to implement `cloned()` method
/// that converts from `TaggedType<&V, T>` to `TaggedType<V, T>`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, Cloned};
/// let user = "admin".to_string();
/// pub type UsernameRef<'a> = TaggedType<&'a String, UsernameTag>;
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl Cloned for UsernameTag {};
///
/// let username: Username = UsernameRef::new(&user).cloned();
/// ```
pub trait Cloned {}

/// Enables `TaggedType` to implement `map` of inner data
///
/// This can be useful if Tag is used as braning mechanism
/// while value type defines storage.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, InnerAccess, ValueMap};
/// pub type Meters<T> = TaggedType<T, MetersTag>;
/// pub enum MetersTag {}
/// impl ValueMap for MetersTag {};
/// impl InnerAccess for MetersTag {};
///
/// let distance = Meters::new(10);
/// let distance = distance.map(|v| v as f64 + 0.5);
/// println!("{}", distance.inner())
///
/// ```
pub trait ValueMap {}

/// Enables `TaggedType` to implement Deref to inner data.
///
/// Note that this is considered bad practice for tagged type
/// to add Deref because of erasure of tag at call site.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, InnerAccess};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl InnerAccess for UsernameTag {};
///
/// format!("{}", Username::new("admin".into()).inner());
/// format!("{}", Username::new("admin".into()).into_inner());
/// ```
pub trait ImplementDeref {}

/// Enables `TaggedType` to implement `Default` if inner type
/// implements `Default`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementDefault};
/// pub type MiddleName = TaggedType<String, MiddleNameTag>;
/// pub enum MiddleNameTag {}
/// impl ImplementDefault for MiddleNameTag {};
/// let empty = MiddleName::default();
/// ```
pub trait ImplementDefault {}

/// Enables `TaggedType` to implement `core::fmt::Display` trait
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, TransparentDebug};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl TransparentDebug for UsernameTag {};
///
/// format!("{:?}", Username::new("admin".into()));
/// ```
pub trait TransparentDebug {}

/// Enables `TaggedType` to implement `core::fmt::Display` trait
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, TransparentDisplay};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl TransparentDisplay for UsernameTag {};
///
/// format!("{}", Username::new("admin".into()));
/// ```
pub trait TransparentDisplay {}

/// Enables `TaggedType` to implement `Clone` trait if inner
/// type implements `Clone`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, TransparentDisplay, ImplementClone};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl TransparentDisplay for UsernameTag {};
/// impl ImplementClone for UsernameTag {};
///
/// let username = Username::new("admin".into());
/// let username_clone = username.clone();
/// format!("user: {username}; copy of user: {username_clone}");
/// ```
pub trait ImplementClone {}

/// Enables `TaggedType` to implement `Copy` trait if inner
/// type implements `Copy`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementCopy, ImplementClone, TransparentDisplay};
/// pub type NetPort = TaggedType<u16, NetPortTag>;
/// pub enum NetPortTag {}
/// impl TransparentDisplay for NetPortTag {};
/// impl ImplementClone for NetPortTag {};
/// impl ImplementCopy for NetPortTag {};
///
/// let ssh_port = NetPort::new(22);
/// let port = ssh_port;
/// format!("port: {ssh_port}; copy of port: {port}");
/// ```
pub trait ImplementCopy {}

/// Enables `TaggedType` to implement `Hash` trait if inner
/// type implements `Hash`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementPartialEq, ImplementEq, ImplementHash};
/// use std::collections::HashSet;
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl ImplementHash for UsernameTag {};
/// impl ImplementPartialEq for UsernameTag {};
/// impl ImplementEq for UsernameTag {};
///
/// let mut users = HashSet::new();
/// users.insert(Username::new("admin".into()));
/// ```
pub trait ImplementHash {}

/// Enables parsing of `TaggedType` to be parsed from string.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, TransparentFromStr};
/// use core::net::IpAddr;
/// pub type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
/// pub enum DefaultGatewayTag {}
/// impl TransparentFromStr for DefaultGatewayTag {};
///
/// let default_gw: DefaultGateway = "192.168.0.1".parse().unwrap();
/// ```
pub trait TransparentFromStr {}

/// Gives possibility to convert from inner type to the tagged type using From/Into.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, FromInner};
/// use core::net::IpAddr;
/// pub type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
/// pub enum DefaultGatewayTag {}
/// impl FromInner for DefaultGatewayTag {};
///
/// let ip: IpAddr = "192.168.0.1".parse().unwrap();
/// let default_gw: DefaultGateway = ip.into();
/// ```
pub trait FromInner {}

/// Backward compatible alias for `FromInner`.
pub trait TransparentFromInner {}

impl<T: TransparentFromInner> FromInner for T {}

/// Implement `core::ops::Add` trait for `TaggedType`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementAdd};
/// pub type CounterU64 = TaggedType<u64, CounterU64Tag>;
/// pub enum CounterU64Tag {}
/// impl ImplementAdd for CounterU64Tag {};
///
/// let counter = CounterU64::new(0);
/// let one: CounterU64 = counter + 1;
/// ```
pub trait ImplementAdd {}

/// Implement `core::ops::Sub` trait for `TaggedType`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementSub, ImplementDefault};
/// pub type Balance = TaggedType<i64, BalanceTag>;
/// pub enum BalanceTag {}
/// impl ImplementDefault for BalanceTag {};
/// impl ImplementSub for BalanceTag {};
///
/// let balance = Balance::default();
/// let credit: Balance = balance - 1;
/// ```
pub trait ImplementSub {}

/// Implement `core::ops::Mul` trait for `TaggedType`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementMul};
/// pub type Capital = TaggedType<f64, CapitalTag>;
/// pub enum CapitalTag {}
/// impl ImplementMul for CapitalTag {};
///
/// let capital = Capital::new(100.0);
/// let next_year_capital: Capital = capital * 1.05;
/// ```
pub trait ImplementMul {}

/// Implement `core::ops::Div` trait for `TaggedType`.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, ImplementDiv, ImplementDefault};
/// pub type Pie = TaggedType<f64, PieTag>;
/// pub enum PieTag {}
/// impl ImplementDiv for PieTag {};
///
/// let pie = Pie::new(5.0);
/// let small_pie: Pie = pie / 5.0;
/// ```
pub trait ImplementDiv {}
