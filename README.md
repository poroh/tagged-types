# TL;DR

```rust
use tagged_types::TaggedType;

// Introduce Gateway type that has IpAddr as base type.
type Gateway = TaggedType<std::net::IpAddr, GatewayTag>;

// Define all required properties that needed for type.
#[derive(tagged_types::Tag)]
#[implement(Clone, Copy, PartialEq, Eq, Hash)]
#[transparent(Display, Debug, FromStr, Serialize, Deserialize)]
#[capability(inner_access)]
enum GatewayTag {}

// Alternatively to define ALL supported properties. Shortcut can
// be used:
// #[derive(tagged_types::Tag)]
// #[permissive]
// enum GatewayTag {}

// Use type as if it is std::net::Ipaddr
#[derive(serde::Serialize, serde::Deserialize)]
struct Route {
    gateway: Gateway,
}

fn main() {
    let gw = serde_json::from_str::<Route>(r#"{"gateway":"192.168.0.1"}"#)
        .unwrap()
        .gateway;
    println!("gateway is {gw}");
    let another_gw: Gateway = "192.168.0.2".parse().unwrap();
    println!("another gateway is {another_gw}");
}
```

# Crate description

This library contains helpers to enforce stricter types. Possible application
can be introduction of new types with limited capabilities in compare to base
types. 

For example. You can introduce type Password that don't have `Display` and `Debug`
traits. But still can be used for comparison and be deserialized using `serde`:

```rust
use tagged_types::TaggedType;

type Password = TaggedType<String, PasswordTag>;
#[derive(tagged_types::Tag)]
#[implement(Clone, Copy, PartialEq, Eq)]
#[transparent(Deserialize)]
enum PasswordTag {}
```

Another application can be usage of TaggedType to destinguish different
identifiers.

```rust
use tagged_types::TaggedType;

type UserId = TaggedType<uuid::Uuid, UserIdTag>;
#[derive(tagged_types::Tag)]
#[permissive]
enum UserIdTag {}

type GroupId = TaggedType<uuid::Uuid, GroupIdTag>;
#[derive(tagged_types::Tag)]
#[permissive]
enum GroupIdTag {}
```

And of course more complex structures can be wrapped.

```rust
#[derive(Debug, Clone)]
struct Version {
    major: u32,
    minor: u32,
}

type SoftwareVersion = TaggedType<Version, SoftwareVersionTag>;
#[derive(tagged_types::Tag)]
#[permissive]
enum SoftwareVersionTag {}

type FirmwareVersion = TaggedType<Version, FirmwareVersionTag>;
#[derive(tagged_types::Tag)]
#[permissive]
enum FirmwareVersionTag {}

```

## Design considerations

This library is designed to depend only on the standard library.
Optionally, it supports serialization and deserialization for the underlying type
(see the `serde_support` feature).

# TaggedType

## Motivation

In many cases, we want strict types, but we don't want to spend
a lot of time implementing boilerplate around them (serialization / deserialization / parsing / clone / copy, etc.).

This library makes it possible to introduce types with minimal effort
and implements many traits for the new type when the underlying type
implements them.

## Implemented traits

Let:
- `V` is the value type
- `T` is the tag type

Conditionally implemented traits when the trait is implemented by the underlying type `V`:
- `Deref` is implemented if `ImplementDeref` is implemeted for `T`, so all methods of `V` are available on
  `TaggedType<V, T>`.
- `Clone` if `ImplementClone` is implemented for `T`
- `Copy` if `ImplementCopy` is implemented for `T`
- `Hash` if `ImplementHash` is implemented for `T`
- `PartialEq` if `ImplementParitalEq` is implemented for `T`
- `Eq` if `ImplementEq` is implemented for `T`
- `Default` if `ImplementDefault` is implemented for `T`

Conditionally implemented traits when the trait is implemented by the underlying
type `V` and enabled for the tag type `T`:
- `Debug` if `T` implements `TransparentDebug`. In this case, `Debug`
  formats the same as `V`
- `Display` if `T` implements `TransparentDisplay`. In this case,
  `Display` formats the same as `V`
- `FromStr` if `T` implements `TransparentFromStr`. In this case,
  `FromStr` parses the same as `V`

## Conditional feature support

### Feature `support_serde`

Conditionally implemented traits when implemented by the underlying type
`V`:
- `Serialize`
- `Deserialize`

### Feature `provide_permissive`

Provides `Permissive` trait that automatically implements all
defined traits for `T` type.


### Feature `provide_derive`

Provides `#[derive(tagged_type::Tag)]` which provide helpers to avoid
manual implementation of traits. Example of all possible features:

```rust
type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
#[derive(Tag)]
#[implement(Default, Eq, PartialEq, Hash, Clone, Copy)]
#[transparent(Debug, Display, FromStr, Serialize, Deserialize)]
#[capability(inner_access)]
enum DefaultGatewayTag {}
```

Or permissive:

```rust
type DefaultGateway = TaggedType<std::net::IpAddr, DefaultGatewayTag>;
#[derive(Tag)]
#[permissive]
enum DefaultGatewayTag {}
```
