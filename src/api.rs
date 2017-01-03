use magic::{sniff_mime_type, sniff_mime_type_from_local_data};

pub trait MimeTypeSniffer {
    fn sniff_mime_type(&self) -> Option<&str>;
}

impl<T: AsRef<[u8]>> MimeTypeSniffer for T {
    fn sniff_mime_type(&self) -> Option<&str> {
        sniff_mime_type_from_local_data(self)
    }
}

pub struct HttpRequest<'a, T: 'a + AsRef<[u8]>> {
    pub content: &'a T,
    pub url: &'a str,
    pub type_hint: &'a str,
}

impl<'a, T: AsRef<[u8]>> MimeTypeSniffer for HttpRequest<'a, T> {
    fn sniff_mime_type(&self) -> Option<&str> {
        sniff_mime_type(self.content, self.url, self.type_hint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_type_sniffer() {
        assert_eq!(Some("application/pdf"), b"%PDF-1.5".sniff_mime_type());
    }

    #[test]
    fn test_request_sniffer() {
        let req = HttpRequest {
            content: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1",
            url: "http://node.ppt",
            type_hint: "plain/text",
        };

        assert_eq!(Some("application/vnd.ms-powerpoint"), req.sniff_mime_type());
    }
}
