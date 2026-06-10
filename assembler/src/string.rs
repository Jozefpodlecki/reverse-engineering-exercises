use core::fmt;
use core::str;

#[derive(Clone)]
pub struct StackString<const CAP: usize> {
    buf: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> PartialEq for StackString<CAP> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.buf[..self.len] == other.buf[..other.len]
    }
}

impl<const CAP: usize> PartialEq<str> for StackString<CAP> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const CAP: usize> PartialEq<&str> for StackString<CAP> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<const CAP: usize> StackString<CAP> {
    pub fn new() -> Self {
        Self {
            buf: [0; CAP],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn remaining(&self) -> usize {
        CAP - self.len
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.as_str().starts_with(prefix)
    }

    pub fn push(&mut self, c: char) -> Result<(), &'static str> {
        let mut buf = [0u8; 4];
        let encoded = c.encode_utf8(&mut buf).as_bytes();

        if self.len + encoded.len() > CAP {
            return Err("capacity exceeded");
        }

        self.buf[self.len..self.len + encoded.len()]
            .copy_from_slice(encoded);

        self.len += encoded.len();
        Ok(())
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), &'static str> {
        if self.len + s.len() > CAP {
            return Err("capacity exceeded");
        }

        self.buf[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const CAP: usize> fmt::Display for StackString<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const CAP: usize> fmt::Debug for StackString<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackString")
            .field("capacity", &CAP)
            .field("len", &self.len)
            .field("value", &self.as_str())
            .finish()
    }
}

impl<const CAP: usize> StackString<CAP> {

    pub fn parse_hex(&self) -> Option<i64> {
        let s = self.as_str();
        if s.starts_with("0x") || s.starts_with("0X") {
            i64::from_str_radix(&s[2..], 16).ok()
        } else {
            None
        }
    }

    pub fn parse_decimal(&self) -> Option<i64> {
        self.as_str().parse().ok()
    }
}