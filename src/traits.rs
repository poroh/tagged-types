// SPDX-License-Identifier: MIT

/// Enables TaggedType to implement access to inner data
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

/// Enables TaggedType to implement Deref to inner data
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

/// Enables TaggedType to implement Eq if inner type
/// implements PartialEq.
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

/// Enables TaggedType to implement Eq if inner type
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

/// Enables TaggedType to implement Default if inner type
/// implements Default.
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

/// Enables TaggedType to implement std::fmt::Display trait
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

/// Enables TaggedType to implement std::fmt::Display trait
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

/// Enables TaggedType to implement Clone trait if inner
/// type implements Clone.
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

/// Enables TaggedType to implement Copy trait if inner
/// type implements Copy.
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

/// Enables TaggedType to implement Hash trait if inner
/// type implements Hash.
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

/// Enables parsing of TaggedType to be parsed from string.
///
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, TransparentFromStr};
/// pub type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
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
/// use tagged_types::{TaggedType, TransparentFromInner};
/// pub type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
/// pub enum DefaultGatewayTag {}
/// impl TransparentFromInner for DefaultGatewayTag {};
///
/// let ip: std::net::IpAddr = "192.168.0.1".parse().unwrap();
/// let default_gw: DefaultGateway = ip.into();
/// ```
pub trait TransparentFromInner {}

/// Transparent serde serialize if inner type implemnts
/// serde serialization.
///
#[cfg(feature = "serde_support")]
pub trait TransparentSerialize {}

/// Transparent serde serialize if inner type implemnts
/// serde serialization.
///
#[cfg(feature = "serde_support")]
pub trait TransparentDeserialize {}

/// Helper that gives all traits.
///
/// Automatically implements all traits if Tag implements Permissive
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, Permissive};
/// use std::collections::HashSet;
/// pub type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
/// pub enum DefaultGatewayTag {}
/// impl Permissive for DefaultGatewayTag {};
///
/// // Supports parsing from string
/// let default_gw: DefaultGateway = "192.168.0.1".parse().unwrap();
///
/// // Supports: Display / Debug:
/// format!("{default_gw}, {default_gw:?}");
///
/// // Supports: Deref to inner type:
/// format!("{}", default_gw.is_ipv4());
///
/// // Supports: Hashing:
/// let mut gateways = HashSet::new();
/// gateways.insert(default_gw);
///
/// // Supports cration from inner type:
/// let another_gw_ip: std::net::IpAddr = "192.168.0.1".parse().unwrap();
/// let another_gw: DefaultGateway = another_gw_ip.into();
///
/// // Supports access / moving to inner type:
/// format!("{}", another_gw.inner());
/// let another_gw_ip: std::net::IpAddr = another_gw.into_inner();
///
/// ```
#[cfg(feature = "use_permissive")]
pub trait Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> InnerAccess for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementDeref for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementCopy for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementClone for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementDefault for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementPartialEq for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementEq for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> ImplementHash for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> TransparentDebug for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> TransparentDisplay for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> TransparentFromInner for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
impl<T> TransparentFromStr for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
#[cfg(feature = "serde_support")]
impl<T> TransparentSerialize for T where T: Permissive {}

#[cfg(feature = "use_permissive")]
#[cfg(feature = "serde_support")]
impl<T> TransparentDeserialize for T where T: Permissive {}
