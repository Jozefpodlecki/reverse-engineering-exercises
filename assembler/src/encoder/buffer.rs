#[derive(Debug, Clone, Copy)]
pub struct InstrBuf {
    bytes: [u8; 15],
    len: u8,
}

impl InstrBuf {
    pub fn new() -> Self {
        Self {
            bytes: [0; 15],
            len: 0,
        }
    }

    pub const fn from_byte(byte: u8) -> Self {
        Self {
            bytes: [byte, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            len: 1,
        }
    }

    pub const fn from_array<const N: usize>(bytes: [u8; N]) -> Self {
        let mut buf = [0; 15];
        let mut i = 0;
        while i < N && i < 15 {
            buf[i] = bytes[i];
            i += 1;
        }
        Self {
            bytes: buf,
            len: i as u8,
        }
    }

    pub fn push(&mut self, b: u8) -> &mut Self {
        self.bytes[self.len as usize] = b;
        self.len += 1;
        self
    }

    pub fn push_rex(&mut self, rex: Option<u8>) -> &mut Self {
        if let Some(r) = rex {
            self.push(r);
        }
        self
    }

    pub fn push_u16(&mut self, v: u16) -> &mut Self {
        self.push((v & 0xFF) as u8);
        self.push((v >> 8) as u8);
        self
    }

    pub fn push_u32(&mut self, v: u32) -> &mut Self {
        self.push((v & 0xFF) as u8);
        self.push(((v >> 8) & 0xFF) as u8);
        self.push(((v >> 16) & 0xFF) as u8);
        self.push(((v >> 24) & 0xFF) as u8);
        self
    }

    pub fn push_u64(&mut self, v: u64) -> &mut Self {
        self.push_u32(v as u32);
        self.push_u32((v >> 32) as u32);
        self
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}

impl AsRef<[u8]> for InstrBuf {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}