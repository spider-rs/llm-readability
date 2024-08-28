# llm_readability

The Rust readability library built for performance, AI, and multiple locales.
The library is used on [Spider Cloud](https://spider.cloud) for data cleaning.

## Usage

```toml
[dependencies]
llm_readability = "0"
```

```rust
use llm_readability::extractor;

fn main() {
  match extractor::extract(&mut "<html>...</html>".as_bytes(), "https://example.com", None) {
      Ok(product) => {
          println!("------- html ------");
          println!("{}", product.content);
          println!("---- plain text ---");
          println!("{}", product.text);
      },
      Err(_) => println!("error occured"),
  }
}
```

This project is a rewrite of `readability-rs` for performance with major bug fixes.