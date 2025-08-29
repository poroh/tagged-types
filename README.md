# Module description

This library contains helpers to enforce more strict types.

## Design considiration

This library doesn't suppose to depend on only standard types.

# TaggedType

## Motivation

In many cases we want to have strict types but we don't want to spend
a lot of time on implementing a lot of boilerplate around these types
(serialization / deserialization / parsing / clone / copy etc.).

This library gives possibility to introduce types with minimal efforts
and implements many properties for new type if underlying type
implements it.

## Implemented property

Let:
- `V` is value type
- `T` is tag type

Always implemented traits:
- `Deref` is implemented. So all functions of `V` are avaiable for
  `TaggedType<V, T>`

Coditionally impleneted traits if trait implemented by underlying type `V`
- `Clone`
- `Copy`
- `Hash`
- `PartialEq`
- `Eq`
- `Default`

Conditionally implemented trait if trait implemented by underlying
type `V` and enabled for tag type `T`:
- `Debug` if `T` implements `TransparentDebug`. In this case Debug
  works as if `V` printed
- `Display` if `T` implements `TransparentDisplay`. In this case
  Display works as if `V` printed
- `FromStr` if `T` implements `TransparentFromStr`. In this case
  Display works as if `V` printed

Conditional feature support:

### serde_support feature:

Coditionally impleneted traits if trait implemented by underlying type
`V`:
- `Serialize`
- `Deserialize`

