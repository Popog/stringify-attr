# stringify-attr

[![Crate Version](https://img.shields.io/crates/v/stringify-attr.svg)](https://crates.io/crates/stringify-attr)
[![Documentation](https://docs.rs/stringify-attr/badge.svg)](https://docs.rs/stringify-attr)
[![Zlib](https://img.shields.io/badge/license-zlib%20License-blue.svg)](https://opensource.org/licenses/Zlib)

Attribute macros for stringifying. Probably only useful in unit tests and debugging, but who knows.

## Usage

Basically these macros allow you to stringify using attribute macros instead of the normal `stringify!`. Since attribute macros must produce an item. Each macro produces a `result!` macro which expands to the desired string.

You can stringify just attributes:
```rust
use stringify_attr::stringify_attr;

assert_eq!(
    {
        #[stringify_attr(foo)] struct Foo;
        result!()
    },
    "foo"
);
```
Just items:
```rust
use stringify_attr::stringify_item;

assert_eq!(
    {
        #[stringify_item(foo)] struct Foo;
        result!()
    },
    "struct Foo;"
);
```
Or the whole thing:
```rust
use stringify_attr::stringify_all;

assert_eq!(
    {
        #[stringify_all(foo)] struct Foo;
        result!()
    },
    "#[stringify_all(foo)] struct Foo;"
);
```

Since attribute macros cannot differential being invoked with different delimeters, different attributes are provided:
```rust
use stringify_attr::stringify_braces as stringify_all;

assert_eq!(
    {
        #[stringify_all{foo}] struct Foo;
        result!()
    },
    "#[stringify_all{foo}] struct Foo;"
);
```

Note that these attributes still produce the text `"stringify_all"` in the output.