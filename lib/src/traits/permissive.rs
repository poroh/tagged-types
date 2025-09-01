// SPDX-License-Identifier: MIT

use crate::traits::cmp::ImplementEq;
use crate::traits::cmp::ImplementOrd;
use crate::traits::cmp::ImplementPartialEq;
use crate::traits::cmp::ImplementPartialOrd;
use crate::traits::ImplementAdd;
use crate::traits::ImplementClone;
use crate::traits::ImplementCopy;
use crate::traits::ImplementDefault;
use crate::traits::ImplementDiv;
use crate::traits::ImplementHash;
use crate::traits::ImplementMul;
use crate::traits::ImplementSub;
use crate::traits::InnerAccess;
use crate::traits::TransparentDebug;
use crate::traits::TransparentDisplay;
use crate::traits::TransparentFromInner;
use crate::traits::TransparentFromStr;

#[cfg(feature = "support_serde")]
use crate::traits::serde::TransparentDeserialize;
#[cfg(feature = "support_serde")]
use crate::traits::serde::TransparentSerialize;

/// Helper that gives all traits.
///
/// Automatically implements all traits if Tag implements Permissive
/// Example:
/// ```rust
/// use tagged_types::{TaggedType, Permissive};
/// use std::collections::HashSet;
/// use core::net::IpAddr;
/// pub type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
/// pub enum DefaultGatewayTag {}
/// impl Permissive for DefaultGatewayTag {};
///
/// // Supports parsing from string
/// let default_gw: DefaultGateway = "192.168.0.1".parse().unwrap();
///
/// // Supports: Display / Debug:
/// format!("{default_gw}, {default_gw:?}");
///
/// // Supports: access to inner type:
/// format!("{}", default_gw.inner().is_ipv4());
///
/// // Supports: Hashing:
/// let mut gateways = HashSet::new();
/// gateways.insert(default_gw);
///
/// // Supports cration from inner type:
/// let another_gw_ip: IpAddr = "192.168.0.1".parse().unwrap();
/// let another_gw: DefaultGateway = another_gw_ip.into();
///
/// // Supports access / moving to inner type:
/// format!("{}", another_gw.inner());
/// let another_gw_ip: IpAddr = another_gw.into_inner();
///
/// ```
pub trait Permissive {}

impl<T> InnerAccess for T where T: Permissive {}
impl<T> ImplementCopy for T where T: Permissive {}
impl<T> ImplementClone for T where T: Permissive {}
impl<T> ImplementDefault for T where T: Permissive {}
impl<T> ImplementPartialEq for T where T: Permissive {}
impl<T> ImplementEq for T where T: Permissive {}
impl<T> ImplementPartialOrd for T where T: Permissive {}
impl<T> ImplementOrd for T where T: Permissive {}
impl<T> ImplementHash for T where T: Permissive {}
impl<T> ImplementAdd for T where T: Permissive {}
impl<T> ImplementSub for T where T: Permissive {}
impl<T> ImplementMul for T where T: Permissive {}
impl<T> ImplementDiv for T where T: Permissive {}
impl<T> TransparentDebug for T where T: Permissive {}
impl<T> TransparentDisplay for T where T: Permissive {}
impl<T> TransparentFromInner for T where T: Permissive {}
impl<T> TransparentFromStr for T where T: Permissive {}

#[cfg(feature = "support_serde")]
impl<T> TransparentSerialize for T where T: Permissive {}

#[cfg(feature = "support_serde")]
impl<T> TransparentDeserialize for T where T: Permissive {}
