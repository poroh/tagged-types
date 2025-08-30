# Module description

This library contains helpers to enforce stricter types.

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

Always implemented traits:
- `Deref` is implemented, so all methods of `V` are available on
  `TaggedType<V, T>`.

Conditionally implemented traits when the trait is implemented by the underlying type `V`:
- `Clone`
- `Copy`
- `Hash`
- `PartialEq`
- `Eq`
- `Default`

Conditionally implemented traits when the trait is implemented by the underlying
type `V` and enabled for the tag type `T`:
- `Debug` if `T` implements `TransparentDebug`. In this case, `Debug`
  formats the same as `V`
- `Display` if `T` implements `TransparentDisplay`. In this case,
  `Display` formats the same as `V`
- `FromStr` if `T` implements `TransparentFromStr`. In this case,
  `FromStr` parses the same as `V`

Conditional feature support:

### Feature `serde_support`:

Conditionally implemented traits when implemented by the underlying type
`V`:
- `Serialize`
- `Deserialize`

