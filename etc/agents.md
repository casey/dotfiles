Development
===========

Documentation
-------------

Prefer consulting the local source code for information about Rust
dependencies.

Run `cargo dep NAME` to find the source directory for a dependency:

```console
$ cargo dep serde
/Users/rodarmor/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/
```

Dependencies
------------

Add dependencies with `cargo add` instead of manually editing `Cargo.toml`.

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

Testing is white-box style. Write tests needed to exercise the implementation.
Add only the minimum number of tests needed to cover new and changed code.

Individual tests should use as little code as possible to exercise the feature
under test.

Tests should use `foo`, `bar`, and similar placeholders in strings to make
clear that the values themselves are not significant.

```rust bad
std::fs::write("file.txt"), "contents").unwrap();
```

```rust good
std::fs::write("foo"), "bar").unwrap();
```

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

Prefer asserting the entire contents of values:

```rust bad
assert!(path.ends_with("foo/bar"));
```

```rust good
assert_eq!(path, "/home/user/foo/bar");
```

Prefer asserting error messages exactly:

```rust bad
assert!(Regex::new("I/O failed").unwrap().is_match(error));
```

```rust good
assert_eq!(message, "error: I/O failed at `foo/bar/baz`");
````

If nondeterminism forces you to match an error message with a regular
expression, always match the entire message:

```rust bad
assert!(Regex::new("bad thing").unwrap().is_match(error));
```

```rust good
assert!(Regex::new("^error: bad thing ID [0-9]+$").unwrap().is_match(error));
````

Prefer matching complete patterns:

```rust bad
assert_matches!(result, Err(Error::Foo { .. }));
```

```rust good
assert_matches!(result, Err(Error::Foo { message: "bar" }));
```

Git
---

Do not commit changes or amend git history unless explicitly asked.

Mannerisms
----------

If you notice issues unrelated to the current task, mention them.

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
