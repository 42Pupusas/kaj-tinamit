//! Minimal percent-encoding per RFC 3986, for query-parameter values and
//! URL-encoded path segments (e.g. `"group/project"` → `"group%2Fproject"`).

/// Extension trait for percent-encoding strings.
///
/// Encodes everything outside the unreserved set (`A-Za-z0-9-_.~`).
pub(crate) trait PercentEncode {
    fn percent_encode(&self) -> String;
}

impl PercentEncode for str {
    fn percent_encode(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut out = String::with_capacity(self.len());
        for &b in self.as_bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(b as char);
                }
                _ => {
                    out.push('%');
                    out.push(HEX[(b >> 4) as usize] as char);
                    out.push(HEX[(b & 0xf) as usize] as char);
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::PercentEncode;

    #[test]
    fn unreserved_pass_through() {
        assert_eq!("aZ0-_.~".percent_encode(), "aZ0-_.~");
    }

    #[test]
    fn slash_and_space_encoded() {
        assert_eq!("group/project".percent_encode(), "group%2Fproject");
        assert_eq!("a b".percent_encode(), "a%20b");
    }
}
