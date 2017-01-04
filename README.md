## mime-sniffer [![travis build](https://travis-ci.org/flier/rust-mime-sniffer.svg?branch=master)](https://travis-ci.org/flier/rust-mime-sniffer) [![crate](https://img.shields.io/crates/v/mime_sniffer.svg)](https://crates.io/crates/mime_sniffer)

Detecting mime types base on content sniffer.

***The detection workflow was copied from [Chromium](https://src.chromium.org/viewvc/chrome/trunk/src/net/base/mime_sniffer.cc)***

[Document](https://flier.github.io/rust-mime-sniffer/docs/v0.1.0/mime_sniffer/index.html)

## Usage

To use `mime-sniffer`, first add this to your `Cargo.toml`:

```toml
[dependencies]
mime_sniffer = "^0.1"
```

Then, add this to your crate root:

```rust
extern crate mime_sniffer;

use mime_sniffer::MimeTypeSniffer;
```

And then, use hash function with module or hasher

```rust
use mime_sniffer::MimeTypeSniffer;

assert_eq!(Some("application/pdf"), b"%PDF-1.5".sniff_mime_type());
```

## Examples

```rust
use mime_sniffer::{MimeTypeSniffer, HttpRequest};

let req = HttpRequest {
    content: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1",
    url: "http://localhost/notes.ppt",
    type_hint: "plain/text",
};

assert_eq!(Some("application/vnd.ms-powerpoint"), req.sniff_mime_type());
```