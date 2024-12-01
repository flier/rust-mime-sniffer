## mime-sniffer [![ci](https://github.com/flier/rust-mime-sniffer/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/flier/rust-mime-sniffer/actions/workflows/ci.yml) [![crate](https://img.shields.io/crates/v/mime_sniffer.svg)](https://crates.io/crates/mime_sniffer)

Detecting mime types base on content sniffer.

[Document](https://docs.rs/mime-sniffer/)

***The detection workflow was copied from [Chromium](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/net/base/mime_sniffer.cc)***

For more detail, please read [How Mozilla determines MIME Types](https://developer.mozilla.org/en-US/docs/Mozilla/How_Mozilla_determines_MIME_Types).

## Usage

To use `mime-sniffer`, first add this to your `Cargo.toml`:

```toml
[dependencies]
mime-sniffer = "^0.1"
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
extern crate url;
#[macro_use]
extern crate mime;
extern crate mime_sniffer;

use url::Url;

use mime_sniffer::{HttpRequest, MimeTypeSniffer, MimeTypeSniffable, MimeTypeSnifferExt};

let url = Url::parse("http://localhost/notes.ppt").unwrap();
let req = HttpRequest {
    content: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1",
    url: &url,
    type_hint: "text/plain",
};

assert!(req.should_sniff_mime_type());
assert_eq!(req.sniff_mime_type(), Some("application/vnd.ms-powerpoint"));
assert_eq!(req.sniff_mime_type_ext().unwrap().type_(), mime::APPLICATION);
```

## Related

* To recognize binary file type, you may need [libmagic](https://linux.die.net/man/3/libmagic) with rust binding [rust-magic](https://github.com/robo9k/rust-magic) crate. [![crate](https://img.shields.io/crates/v/magic.svg)](https://crates.io/crates/magic)
* To guess MIME type by file extension, you may need [mime_guess](https://github.com/abonander/mime_guess) crate. [![crate](https://img.shields.io/crates/v/mime_guess.svg)](https://crates.io/crates/mime_guess)
* To manage MIME type as strong types, you may need [mime.rs](https://github.com/hyperium/mime.rs) crate. [![crate](https://img.shields.io/crates/v/mime.svg)](https://crates.io/crates/mime)
