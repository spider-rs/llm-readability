# readability

The Rust readability library build for performance, AI, and multiple locales.


## Usage

```toml
[dependencies]
llm_readability = "0"
```

```rust
use llm_readability::extractor;

fn main() {
  match extractor::extract("<html>...</html>", "https://example.com", None) {
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