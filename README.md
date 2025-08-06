# TL;DR.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/tldr-traits)](https://crates.io/crates/tldr-traits)
[![Documentation](https://docs.rs/tldr-traits/badge.svg)](https://docs.rs/tldr-traits)

[Rust] abstractions for [TL;DR] summarization using the [five Ws].

## ✨ Features

- Provides the [`Tldr`](#tldr) trait for generating TL;DR summaries.
- Provides the [`ToTldr`](#totldr) trait for converting objects into TL;DR
  summaries.
- Supports TL;DR generation for multiple natural languages.
- Zero required dependencies, only optional integrations.
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add tldr-traits --rename tldr
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
tldr = { version = "0", package = "tldr-traits" }
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
tldr = { version = "0", package = "tldr-traits", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust,ignore
use tldr::{Tldr, TldrContext, TldrLanguage, TldrResult, TldrSummary, ToTldr};
```

### Implementing the Trait

```rust,ignore
struct Rectangle {
    width: u32,
    height: u32,
}

impl Tldr for Rectangle {
    type Error = Box<dyn Error>;

    fn what(&self, _ctx: &TldrContext) -> TldrResult<T> {
        Ok(Some(format!("A rectangle with a width of {} and a height of {}.", self.width, self.height)))
    }
}
```

## 📚 Reference

- [`Tldr`](#tldr)
- [`TldrContext`](#tldrcontext)
- [`TldrLanguage`](#tldrlanguage)
- [`TldrResult`](#tldrresult)
- [`TldrSummary`](#tldrsummary)
- [`ToTldr`](#totldr)

### [`Tldr`]

```rust,ignore
pub trait Tldr<T = String> {
    fn who(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn what(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn when(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn where(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn why(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn whence(&self, ctx: &TldrContext) -> TldrResult<T>;
    fn how(&self, ctx: &TldrContext) -> TldrResult<T>;
}
```

### [`TldrContext`]

```rust,ignore
pub struct TldrContext {
    pub language: TldrLanguage,
}
```

### [`TldrLanguage`]

```rust,ignore
pub enum TldrLanguage {
    #[default]
    English,
    // ...
    Other(String),
}
```

### [`TldrResult`]

```rust,ignore
pub type TldrResult<T = String, E = Box<dyn Error>> =
    Result<Option<T>, E>;
```

### [`TldrSummary`]

```rust,ignore
pub struct TldrSummary<T = String> {
    pub who: Option<T>,
    pub what: Option<T>,
    pub when: Option<T>,
    pub where: Option<T>,
    pub why: Option<T>,
    pub whence: Option<T>,
    pub how: Option<T>,
}
```

### [`ToTldr`]

```rust,ignore
pub trait ToTldr<T = String> {
    fn to_tldr(&self) -> Box<dyn Tldr<T>>;
}
```

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/tldr.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/tldr.rs&text=TL;DR.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/tldr.rs&title=TL;DR.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/tldr.rs&t=TL;DR.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/tldr.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/tldr.rs)

[Rust]: https://rust-lang.org
[TL;DR]: https://en.wikipedia.org/wiki/TL;DR
[five Ws]: https://en.wikipedia.org/wiki/Five_Ws
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[`Tldr`]: https://docs.rs/tldr-traits/latest/tldr_traits/trait.Tldr.html
[`TldrContext`]: https://docs.rs/tldr-traits/latest/tldr_traits/struct.TldrContext.html
[`TldrLanguage`]: https://docs.rs/tldr-traits/latest/tldr_traits/enum.TldrLanguage.html
[`TldrResult`]: https://docs.rs/tldr-traits/latest/tldr_traits/type.TldrResult.html
[`TldrSummary`]: https://docs.rs/tldr-traits/latest/tldr_traits/struct.TldrSummary.html
[`ToTldr`]: https://docs.rs/tldr-traits/latest/tldr_traits/trait.ToTldr.html
