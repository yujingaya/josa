# Josa
> Idiomatic [josa](https://en.wikipedia.org/wiki/Korean_postpositions) selector.

![crates.io version][version] ![crates.io license][license] ![crates.io download][download]

Josa is a [Rust] library to select appropriate josas for korean language.

## Overview
### `push_josa` method
Josa is an [extension trait] implemented for [`String`] type.
Its API works just like [`push_str`] method on [`String`].

```rust
use josa::JosaExt;
use josa::{EunNeun, IGa};

let mut user = "유진".to_owned();
let mut mackerel = "고등어".to_owned();

user.push_josa(EunNeun);
mackerel.push_josa(IGa);

let sentence = format!("{} {} 먹고싶다", user, mackerel);

assert_eq!(sentence, "유진은 고등어가 먹고싶다");
```

> :warning: Like [`push_str`] does, `push_josa` expects [`String`], not [`str`], as its argument.

### `+`, `+=` operator
You can use `+`, `+=` operator to append josa.

```rust
use josa::{EunNeun, IGa};

let user = "유진".to_owned();
let mackerel = "고등어".to_owned();

let sentence = format!("{} {} 먹고싶다", user + EunNeun, mackerel + IGa);

assert_eq!(sentence, "유진은 고등어가 먹고싶다");
```

### `select` method
In case you want to append a josa to formatted text such as `<span>고양이</span>`,
you can use `select` method.

```rust
use josa::select;
use josa::IGa;

let cat = "고양이";
let josa = select(cat, IGa).unwrap();

let cat = format!(r#"<span class="bold">{}</span>{}"#, cat, josa);

assert_eq!(cat, r#"<span class="bold">고양이</span>가"#);
```

## Usage
Add `josa` as a dependency in your `Cargo.toml`.

```toml
[dependencies]
josa = "0.1.2"
```

Now you can use josa crate.
```rust
use josa::*;
// Use here..
```

## Documentation
See [docs.rs][documentation]

## Roadmap
### `select` macro
As soon as [hygiene 2.0 (#54727)][hygiene] arrives stable, we will add support for following macro:

```rust
select!("{}{은} {}{가} 먹고싶다", user, mackerel);
```

which is terser than current syntax:

### Hangul enum variants
As soon as [non-ASCII identifiers (#55467)][ident] arrives stable, we will change the names of josas to Hangul:

```rust
format!("{} {} 먹고싶다", user + 은는, mackerel + 이가);
```

## License
Distributed under the MIT license.


[version]: https://img.shields.io/crates/v/josa
[license]: https://img.shields.io/crates/l/josa
[download]: https://img.shields.io/crates/d/josa

[Rust]: https://rust-lang.org
[extension trait]: https://github.com/rust-lang/rfcs/blob/master/text/0445-extension-trait-conventions.md
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`push_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.push_str

[documentation]: https://docs.rs/josa

[hygiene]: https://github.com/rust-lang/rust/issues/54727
[ident]: https://github.com/rust-lang/rust/issues/55467