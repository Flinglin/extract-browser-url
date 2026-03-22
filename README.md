# extract-browser-url

A Rust library for extracting the browser's URL using native platform APIs.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
extract-browser-url = "0.1.0"
```

Or via the CLI:

```bash
cargo add extract-browser-url
```

##  Quick Start

```rust
 use extract_browser_url::extract_url;
 fn main(){
     let url=extract_url(3000,false).unwrap();
     println!("the url is: {}",url)
 }
```

## Supported Browsers

| Browser | Windows |
|---------| ------- |
| Chrome  | ✅      |
| Firefox | ✅      |
| Edge    | ✅      |

## 📄 License

Licensed under the MIT License. See [LICENSE](https://mit-license.org/) for details.

---

<div align="center">
  <sub>Built with ❤️ by <a href="https://github.com/flinglin">flinglin</a></sub>
</div>