// SPDX-License-Identifier: MIT

use std::marker::PhantomData;

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
/// The Display and Debug traits are implemented only when TransparentDisplay / TransparentDebug are implemented:
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
/// The Display and Debug traits are implemented only when TransparentDisplay / TransparentDebug are implemented:
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
    _marker: std::marker::PhantomData<Tag>,
}

impl<V, T> TaggedType<V, T> {
    pub fn new(v: V) -> Self {
        Self {
            v,
            _marker: PhantomData,
        }
    }

    pub fn inner(&self) -> &V {
        &self.v
    }

    pub fn into_inner(self) -> V {
        self.v
    }
}

impl<V, T> std::ops::Deref for TaggedType<V, T> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<V, T> Clone for TaggedType<V, T>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            _marker: PhantomData,
        }
    }
}

impl<V, T> Copy for TaggedType<V, T> where V: Copy {}

impl<V, T> std::hash::Hash for TaggedType<V, T>
where
    V: std::hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.v.hash(state);
    }
}

impl<V, T> PartialEq for TaggedType<V, T>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

impl<V, T> Eq for TaggedType<V, T> where V: Eq {}

impl<V, T> Default for TaggedType<V, T>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            v: V::default(),
        }
    }
}

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

impl<V, T> std::fmt::Debug for TaggedType<V, T>
where
    V: std::fmt::Debug,
    T: TransparentDebug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

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

impl<V, T> std::fmt::Display for TaggedType<V, T>
where
    V: std::fmt::Display,
    T: TransparentDisplay,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

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

impl<V, T> std::str::FromStr for TaggedType<V, T>
where
    V: std::str::FromStr,
    T: TransparentFromStr,
{
    type Err = <V as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            v: V::from_str(s)?,
            _marker: PhantomData,
        })
    }
}

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

impl<V, T> From<V> for TaggedType<V, T>
where
    T: TransparentFromInner,
{
    fn from(v: V) -> Self {
        Self {
            v,
            _marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde_support")]
pub trait TransparentSerialize {}

#[cfg(feature = "serde_support")]
impl<V, T> serde::Serialize for TaggedType<V, T>
where
    V: serde::Serialize,
    T: TransparentSerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.v.serialize(serializer)
    }
}

#[cfg(feature = "serde_support")]
pub trait TransparentDeserialize {}

#[cfg(feature = "serde_support")]
impl<'de, V, T> serde::Deserialize<'de> for TaggedType<V, T>
where
    V: serde::Deserialize<'de>,
    T: TransparentDeserialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        V::deserialize(deserializer).map(TaggedType::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TaggedString<T> = TaggedType<String, T>;
    const URL: &str = "http://example.com";

    #[test]
    fn test_deref() {
        enum UrlStringTag {}
        type UrlString = TaggedString<UrlStringTag>;
        let url = UrlString::new(URL.into());
        assert_eq!(url.to_string(), URL);
        assert!(url.contains("http"));
        assert_eq!(url.as_str(), URL);
    }

    #[test]
    fn test_default() {
        enum CounterU64Tag {}
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        let c = CounterU64::default();
        assert_eq!(*c.inner(), 0);
    }

    #[test]
    fn test_copy() {
        enum CounterU64Tag {}
        type CounterU64 = TaggedType<u64, CounterU64Tag>;
        impl TransparentDebug for CounterU64Tag {}
        let c = CounterU64::default();
        let v = c;
        assert_eq!(v, c);
    }

    #[test]
    fn test_clone() {
        enum UsernameTag {}
        type Username = TaggedType<String, UsernameTag>;
        impl TransparentDebug for UsernameTag {}
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
        type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
        enum DefaultGatewayTag {}
        impl TransparentFromStr for DefaultGatewayTag {}
        const IP: &str = "192.168.0.1";
        let gw: DefaultGateway = IP.parse().unwrap();
        assert_eq!(gw.inner(), &IP.parse::<std::net::IpAddr>().unwrap());
    }
}
