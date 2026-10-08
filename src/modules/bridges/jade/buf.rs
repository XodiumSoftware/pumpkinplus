//! Small write-buffer helpers for encoding Jade payload wire format.
//!
//! Jade uses Minecraft's network codec conventions (`VarInt` for length-prefixed
//! ints, big-endian for fixed-width primitives, Modified-UTF-8 for strings).
//! These helpers cover the shapes used by `JadePayloads.kt` and friends.

/// A growable byte buffer with helpers matching Minecraft's net-codec conventions.
#[derive(Default)]
pub(crate) struct Buf {
    bytes: Vec<u8>,
}

impl Buf {
    /// Creates a fresh empty buffer.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Appends a single byte.
    pub(crate) fn write_byte(&mut self, b: u8) {
        self.bytes.push(b);
    }

    /// Appends a boolean as 0/1 byte.
    ///
    /// Used by future Jade payload paths (e.g., `ItemStack` codec presence bits).
    #[allow(dead_code)]
    pub(crate) fn write_bool(&mut self, v: bool) {
        self.write_byte(u8::from(v));
    }

    /// Appends a big-endian `f32`.
    pub(crate) fn write_f32(&mut self, v: f32) {
        self.bytes.extend_from_slice(&v.to_be_bytes());
    }

    /// Appends a big-endian `i32`.
    pub(crate) fn write_i32(&mut self, v: i32) {
        self.bytes.extend_from_slice(&v.to_be_bytes());
    }

    /// Appends a big-endian `i64`.
    pub(crate) fn write_i64(&mut self, v: i64) {
        self.bytes.extend_from_slice(&v.to_be_bytes());
    }

    /// Appends a `VarInt` (7 bits per byte, MSB = continuation).
    pub(crate) fn write_var_int(&mut self, mut v: i32) {
        loop {
            #[allow(clippy::cast_sign_loss)]
            let mut byte = (v & 0x7F) as u8;
            v >>= 7;
            if v != 0 {
                byte |= 0x80;
            }
            self.bytes.push(byte);
            if v == 0 {
                break;
            }
        }
    }

    /// Appends a VarInt-prefixed UTF-8 string.
    pub(crate) fn write_utf(&mut self, s: &str) {
        let bytes = s.as_bytes();
        // Saturating cast is fine: strings this large would OOM anyway.
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        self.write_var_int(bytes.len() as i32);
        self.bytes.extend_from_slice(bytes);
    }

    /// Appends raw bytes.
    pub(crate) fn write_bytes(&mut self, bs: &[u8]) {
        self.bytes.extend_from_slice(bs);
    }

    /// Consumes the buffer and returns the underlying `Vec<u8>`.
    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

/// Small read-buffer for decoding Jade client→server payloads.
///
/// Jade's `request_block` / `request_entity` payloads contain a few leading
/// fields before the `VarInt` indices; we parse them off here.
pub(crate) struct BufReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> BufReader<'a> {
    /// Wraps a byte slice for reading.
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    /// Reads a single byte; returns 0 if past end.
    pub(crate) fn read_u8(&mut self) -> u8 {
        self.bytes.get(self.pos).copied().map_or(0, |b| {
            self.pos += 1;
            b
        })
    }

    /// Reads a boolean (any non-zero byte is true).
    pub(crate) fn read_bool(&mut self) -> bool {
        self.read_u8() != 0
    }

    /// Reads a `VarInt` (up to 5 bytes); returns 0 on truncation.
    pub(crate) fn read_var_int(&mut self) -> i32 {
        let mut result = 0i32;
        for shift in (0..=28).step_by(7) {
            let b = self.read_u8();
            result |= i32::from(b & 0x7F) << shift;
            if b & 0x80 == 0 {
                return result;
            }
        }
        result
    }

    /// Reads a big-endian `f32`.
    pub(crate) fn read_f32(&mut self) -> f32 {
        let mut buf = [0u8; 4];
        for b in &mut buf {
            *b = self.read_u8();
        }
        f32::from_be_bytes(buf)
    }

    /// Reads a big-endian `i64`.
    pub(crate) fn read_packed_i64(&mut self) -> i64 {
        let mut buf = [0u8; 8];
        for b in &mut buf {
            *b = self.read_u8();
        }
        i64::from_be_bytes(buf)
    }

    /// Returns `true` if at least one more byte is readable.
    ///
    /// Reserved for future use: skipping optional trailing accessor NBT when
    /// porting the full request codec.
    #[allow(dead_code)]
    pub(crate) fn is_readable(&self) -> bool {
        self.pos < self.bytes.len()
    }
}
