# TL;DR.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/tldr-traits)](https://crates.io/crates/tldr-traits)
[![Documentation](https://docs.rs/tldr-traits/badge.svg)](https://docs.rs/tldr-traits)

[Rust] abstractions for [TL;DR] summarization.

## ✨ Features

- Provides the trait [`Tldr`] to generate TL;DR summaries.
- Provides the trait [`ToTldr`] to convert objects into TL;DR summaries.
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

```rust
use tldr::*;
```

## 📚 Reference

### [`ToTldr`]

```rust
pub trait ToTldr {
    fn to_tldr(&self) -> Box<dyn Tldr>;
}
```

### [`Tldr`]

```rust
pub trait Tldr {
    fn who(&self, ctx: &TldrContext) -> Option<String>;
    fn what(&self, ctx: &TldrContext) -> Option<String>;
    fn when(&self, ctx: &TldrContext) -> Option<String>;
    fn r#where(&self, ctx: &TldrContext) -> Option<String>;
    fn why(&self, ctx: &TldrContext) -> Option<String>;
    fn whence(&self, ctx: &TldrContext) -> Option<String>;
    fn how(&self, ctx: &TldrContext) -> Option<String>;
}
```

### [`TldrSummary`]

```rust
pub struct TldrSummary {
    pub who: Option<String>,
    pub what: Option<String>,
    pub when: Option<String>,
    pub r#where: Option<String>,
    pub why: Option<String>,
    pub whence: Option<String>,
    pub how: Option<String>,
}
```

### [`TldrContext`]

```rust
pub struct TldrContext {
    pub language: String,
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
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[`Tldr`]: #
[`TldrContext`]: #
[`TldrSummary`]: #
[`ToTldr`]: #
