// SPDX-License-Identifier: MIT

use crate::AsRef;
use crate::Cloned;
use crate::FromInner;
use crate::ImplementAdd;
use crate::ImplementClone;
use crate::ImplementCopy;
use crate::ImplementDefault;
use crate::ImplementDeref;
use crate::ImplementDiv;
use crate::ImplementHash;
use crate::ImplementMul;
use crate::ImplementSub;
use crate::InnerAccess;
use crate::TransparentDebug;
use crate::TransparentDisplay;
use crate::TransparentFromStr;
use crate::ValueMap;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::hash::Hash;
use core::hash::Hasher;
use core::marker::PhantomData;
use core::ops::Add;
use core::ops::Deref;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;
use core::str::FromStr;

/// Implmentation of comparison traits for `TaggedType`.
pub mod cmp;

/// Implmentation of `serde::Serialize` and `serde::Deserialize` for
/// `support_serde` feature.
#[cfg(feature = "support_serde")]
pub mod serde;

/// Example for a password type:
/// ```rust
/// use tagged_types::TaggedType;
/// pub type Password = TaggedType<String, PasswordTag>;
/// pub enum PasswordTag {}
///
/// let password = Password::new("my-secret".into());
/// ```
///
/// Cannot assign to the base type:
/// ```rust,compile_fail
/// use tagged_types::TaggedType;
/// pub type Password = TaggedType<String, PasswordTag>;
/// pub enum PasswordTag {}
///
/// let password = Password::new("supersecret".into());
/// let copy: String = password; // does not compile: expected String
/// ```
///
/// Cannot assign between types with different tags:
/// ```rust,compile_fail
/// use tagged_types::TaggedType;
/// pub type Password = TaggedType<String, PasswordTag>;
/// pub enum PasswordTag {}
///
/// use tagged_types::TaggedType;
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
///
/// let password = Password::new("my-secret".into());
/// fn foo(user: &Username, password: &Password) {
///    todo!();
/// }
///
/// // Does not compile: invalid order of arguments:
/// foo(&Password::new("supersecret".into()), &Username::new("admin".into()))
/// ```
///
/// The Display and Debug traits are implemented only when `TransparentDisplay` / `TransparentDebug` are implemented:
/// ```rust,compile_fail
/// use tagged_types::TaggedType;
/// pub type Password = TaggedType<String, PasswordTag>;
/// pub enum PasswordTag {}
///
/// let password = Password::new("my-secret".into());
/// format!("{}", password); // does not compile because TransparentDisplay is not implemented
/// format!("{:?}", password); // does not compile because TransparentDebug is not implemented
/// ```
///
/// The Display and Debug traits are implemented only when `TransparentDisplay` / `TransparentDebug` are implemented:
/// ```rust
/// use tagged_types::{TaggedType, TransparentDebug, TransparentDisplay};
/// pub type Username = TaggedType<String, UsernameTag>;
/// pub enum UsernameTag {}
/// impl TransparentDebug for UsernameTag {};
/// impl TransparentDisplay for UsernameTag {};
///
/// format!("{:?}", Username::new("admin".into()));
/// format!("{}", Username::new("admin".into()));
/// ```
pub struct TaggedType<Value, Tag> {
    v: Value,
    _marker: PhantomData<Tag>,
}

impl<V, T> TaggedType<V, T> {
    /// Create `TaggedType` from inner type.
    #[inline]
    pub const fn new(v: V) -> Self {
        Self {
            v,
            _marker: PhantomData,
        }
    }
}

impl<V, T: InnerAccess> TaggedType<V, T> {
    /// Provides reference to inner data.
    #[inline]
    pub const fn inner(&self) -> &V {
        &self.v
    }

    /// Convert `TaggedType` to inner data.
    #[inline]
    pub fn into_inner(self) -> V {
        self.v
    }
}

impl<V: Clone, T: Cloned> TaggedType<&V, T> {
    /// Transform to owning `TaggedType`.
    #[inline]
    #[must_use]
    pub fn cloned(self) -> TaggedType<V, T> {
        TaggedType::new(self.v.clone())
    }
}

impl<V, T: ValueMap> TaggedType<V, T> {
    /// Converts inner type using function f.
    #[inline]
    #[must_use]
    pub fn map<F, U>(self, f: F) -> TaggedType<U, T>
    where
        F: FnOnce(V) -> U,
    {
        TaggedType::<U, T>::new(f(self.v))
    }

    /// Converts inner type using function f that returns Result.
    ///
    /// # Errors
    ///
    /// Will return E the same as Result of f.
    #[inline]
    pub fn try_map<F, U, E>(self, f: F) -> Result<TaggedType<U, T>, E>
    where
        F: FnOnce(V) -> Result<U, E>,
    {
        f(self.v).map(TaggedType::<U, T>::new)
    }
}

impl<V, T: AsRef> TaggedType<V, T> {
    /// Converts from `&TaggedType<V, T>` to `TaggedType<&V, T>`.
    ///
    /// Example:
    /// ```rust
    /// use tagged_types::{TaggedType, AsRef, TransparentDisplay};
    /// pub type Username = TaggedType<String, UsernameTag>;
    /// pub type UsernameRef<'a> = TaggedType<&'a String, UsernameTag>;
    /// pub enum UsernameTag {}
    /// impl AsRef for UsernameTag {};
    /// impl TransparentDisplay for UsernameTag {};
    ///
    /// pub fn print_username(username: UsernameRef<'_>) {
    ///     println!("username is {username}");
    /// }
    ///
    /// let username = Username::new("admin".into());
    /// print_username(username.as_ref());
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> TaggedType<&V, T> {
        TaggedType::<&V, T>::new(&self.v)
    }
}

impl<V, T: ImplementDeref> Deref for TaggedType<V, T> {
    type Target = V;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<V: Clone, T: ImplementClone> Clone for TaggedType<V, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            _marker: PhantomData,
        }
    }
}

impl<V: Copy, T: ImplementCopy + ImplementClone> Copy for TaggedType<V, T> {}

impl<V: Hash, T: ImplementHash> Hash for TaggedType<V, T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
    }
}

impl<V: Default, T: ImplementDefault> Default for TaggedType<V, T> {
    #[inline]
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            v: V::default(),
        }
    }
}

impl<V: Debug, T: TransparentDebug> Debug for TaggedType<V, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.v.fmt(f)
    }
}

impl<V: Display, T: TransparentDisplay> Display for TaggedType<V, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.v.fmt(f)
    }
}

impl<V: FromStr, T: TransparentFromStr> FromStr for TaggedType<V, T> {
    type Err = <V as FromStr>::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            v: V::from_str(s)?,
            _marker: PhantomData,
        })
    }
}

impl<V, T: FromInner> From<V> for TaggedType<V, T> {
    #[inline]
    fn from(v: V) -> Self {
        Self {
            v,
            _marker: PhantomData,
        }
    }
}

impl<Rhs, V: Add<Rhs, Output = V>, T: ImplementAdd> Add<Rhs> for TaggedType<V, T> {
    type Output = Self;
    #[inline]
    fn add(self, v: Rhs) -> Self {
        Self {
            v: self.v + v,
            _marker: PhantomData,
        }
    }
}

impl<Rhs, V: Sub<Rhs, Output = V>, T: ImplementSub> Sub<Rhs> for TaggedType<V, T> {
    type Output = Self;
    #[inline]
    fn sub(self, v: Rhs) -> Self {
        Self {
            v: self.v - v,
            _marker: PhantomData,
        }
    }
}

impl<Rhs, V: Mul<Rhs, Output = V>, T: ImplementMul> Mul<Rhs> for TaggedType<V, T> {
    type Output = Self;
    #[inline]
    fn mul(self, v: Rhs) -> Self {
        Self {
            v: self.v * v,
            _marker: PhantomData,
        }
    }
}

impl<Rhs, V: Div<Rhs, Output = V>, T: ImplementDiv> Div<Rhs> for TaggedType<V, T> {
    type Output = Self;
    #[inline]
    fn div(self, v: Rhs) -> Self {
        Self {
            v: self.v / v,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::net::IpAddr;
    type TaggedString<T> = TaggedType<String, T>;
    const URL: &str = "http://example.com";

    #[test]
    fn test_deref() {
        enum UrlStringTag {}
        type UrlString = TaggedString<UrlStringTag>;
        impl ImplementDeref for UrlStringTag {}
        let url = UrlString::new(URL.into());
        assert_eq!(url.to_string(), URL);
        assert!(url.contains("http"));
        assert_eq!(url.as_str(), URL);
    }

    #[test]
    fn test_default() {
        enum CounterU64Tag {}
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        impl InnerAccess for CounterU64Tag {}
        impl ImplementDefault for CounterU64Tag {}
        let c = CounterU64::default();
        assert_eq!(*c.inner(), 0);
    }

    #[test]
    fn test_copy() {
        enum CounterU64Tag {}
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        impl ImplementCopy for CounterU64Tag {}
        impl ImplementClone for CounterU64Tag {}
        impl TransparentDebug for CounterU64Tag {}
        impl ImplementDefault for CounterU64Tag {}
        impl ImplementPartialEq for CounterU64Tag {}
        let c = CounterU64::default();
        let v = c;
        assert_eq!(v, c);
    }

    #[test]
    fn test_clone() {
        enum UsernameTag {}
        type Username = TaggedType<String, UsernameTag>;
        impl TransparentDebug for UsernameTag {}
        impl ImplementPartialEq for UsernameTag {}
        impl ImplementClone for UsernameTag {}
        let c = Username::new("admin".into());
        let v = c.clone();
        assert_eq!(v, c);
    }

    #[test]
    fn test_transparent_display() {
        enum UrlStringTag {}
        impl TransparentDisplay for UrlStringTag {}
        type UrlString = TaggedString<UrlStringTag>;
        let url = UrlString::new(URL.into());
        assert_eq!(format!("url: {url}"), format!("url: {URL}"));
    }

    #[test]
    fn test_transparent_debug() {
        enum UrlStringTag {}
        impl TransparentDebug for UrlStringTag {}
        type UrlString = TaggedString<UrlStringTag>;
        let url = UrlString::new(URL.into());
        assert_eq!(format!("url: {url:?}"), format!("url: {URL:?}"));
    }

    #[test]
    fn test_transparent_from_str() {
        type DefaultGateway = TaggedType<IpAddr, DefaultGatewayTag>;
        enum DefaultGatewayTag {}
        impl InnerAccess for DefaultGatewayTag {}
        impl TransparentFromStr for DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let gw: DefaultGateway = IP.parse().unwrap();
        assert_eq!(gw.inner(), &IP.parse::<IpAddr>().unwrap());
    }
}
