Development
===========

Mannerisms
----------

Do not end responses with suggestions for next steps, such as running tests or
adding additional features, unless those suggestions are interesting and
non-obvious.

Documentation
-------------

Always consult the local source code for information about Rust dependencies,
which is guaranteed to be up-to-date for the correct version.

Run `cargo dep NAME` to find the source directory for a dependency:

```console
$ cargo dep serde
/Users/rodarmor/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/
```

Local docs can be built with:

```shell
cargo doc --workspace --document-private-items
```

Which will output documentation for crate `NAME` in
`target/doc/NAME/index.html`.

Read source code and docs as appropriate.

Dependencies
------------

Add dependencies with `cargo add` instead of manually editing `Cargo.toml`.

Imports
-------

Don't rename imports in `use` statements.

```rust bad
use std::io::Result as IoResult;

fn foo() -> IoResult<()>;
```

```rust good
use std::io::Result as IoResult;

fn read() -> io::Result<()>;
```

Imports which are unambiguously named and common should go in the top-level
module and be inherited by child modules with `use super::*`.

Style
-----

Do not write comments. Comments will be added by the user as necessary.

Don't create mutable variables that are initialized in a conditional:

```rust bad
let mut foo = None;

if bar {
  foo = Some("hello");
}
```

Instead, create an immutable variable initialized with an if/else:

```rust good
let foo = if bar {
  Some("hello")
} else {
  None
};
```

Passing primitives into functions creates opportunities for confusion:

```rust bad
struct Config {
  a: bool,
  b: bool,
}

fn foo(a: bool) {
}

let config = Config {
  a: true,
  b: false,
};

foo(config.b);
```

Where possible, pass the object where the primitive originates:

```rust good
struct Config {
  a: bool,
  b: bool,
}

fn foo(config: &Config) {
  // use config.a
}

let config = Config {
  a: true,
  b: false,
};

foo(&config);
```

When converting a value, shadowing the previous variable is often ideal. The
type system prevents confusing the two values, and shadowing the previous
variable prevents it from being unintentionally used later:

```rust bad
let name_string = name.to_string();
```

```rust good
let name = name.to_string();
```

Testing
-------

Tests should use `foo`, `bar`, and similar placeholders in strings to make
clear that the values themselves are not significant.

```rust bad
std::fs::write("file.txt"), "contents").unwrap();
```

```rust good
std::fs::write("foo"), "bar").unwrap();
```

Individual tests should use as little code as possible to exercise the feature
under test.

Testing is white-box style. Write tests according to the implementation. Avoid
writing tests which do not exercise unique code paths.

De-duplicate similar tests with a case function:

```rust bad
#[test]
fn bar() {
  assert_eq!("bar".parse().unwrap(), Foo::Bar);
}

#[test]
fn baz() {
  assert_eq!("baz".parse().unwrap(), Foo::Baz);
}

#[test]
fn bob() {
  assert_eq!("bob".parse().unwrap(), Foo::Bob);
}
```

```rust good
#[test]
fn parsing() {
  #[track_caller]
  fn case(s: &str, expected: Foo) {
    assert_eq!(s.parse().unwrap(), expected);
  }

  case("bar", Foo::Bar);
  case("baz", Foo::Baz);
  case("bob", Foo::Bob);
}
```

Prefer turbofish over type ascription:

```rust bad
let foo: T = foo.parse().unwrap();
let bar: Vec<u8> = foo.into_iter().collect();
```

```rust good
let foo = foo.parse::<T>().unwrap();
let bar = foo.into_iter().collect::<Vec<u8>>();
```

Use modern Rust when available.

Tips
----

If you need a constant for a test, but the value of that constant is hard to
calculate, assert equality with a dummy value:

```rust
assert_eq!(complicated_calculation(), "");
```

Then get the correct value from the error message and insert it:

```rust
assert_eq!(
  complicated_calculation(),
  "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
);
```
