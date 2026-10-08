//! Minimal NBT (Named Binary Tag) *writer* for Jade's `receive_data` payloads.
//!
//! Jade expects a single unnamed-compound NBT encoded as:
//! `0x0A | u16 name_len (=0) | compound-payload | 0x00 (TAG_End)`.
//!
//! We only implement what Jade providers actually use: a flat or shallow
//! `Compound` with primitive fields (bytes, ints, strings, byte arrays) plus
//! nested compounds. This is *not* a general NBT library — it's just enough
//! to encode provider responses.
//!
//! TODO(pumpkin-api): Pumpkin's plugin API doesn't bundle an NBT encoder.
//! If upstream exposes an `NbtIo`-style helper, switch to it and drop this.

use crate::modules::bridges::jade::buf::Buf;

/// NBT tag ids as defined by the spec.
const TAG_END: u8 = 0;
const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_COMPOUND: u8 = 10;

/// A single NBT tag in a compound.
///
/// Some variants are unused until more providers are ported — marked
/// `#[allow(dead_code)]` since they're part of the public surface of this
/// helper and shouldn't produce warnings while Jade is being filled in.
#[allow(dead_code)]
enum TagValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Compound(Vec<(String, TagValue)>),
}

/// A builder for a flat-or-shallow unnamed-compound NBT payload.
///
/// Use [`NbtCompound::new`] to start, chain `put_*` calls, then
/// [`NbtCompound::encode`] to get the network bytes (with `TAG_End`).
#[derive(Default)]
pub(crate) struct NbtCompound {
    /// Entries in insertion order; NBT is order-preserving.
    entries: Vec<(String, TagValue)>,
}

impl NbtCompound {
    /// Starts a fresh compound.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Appends a `TAG_Byte` field.
    #[allow(dead_code)]
    pub(crate) fn put_byte(mut self, name: &str, value: i8) -> Self {
        self.entries.push((name.to_owned(), TagValue::Byte(value)));
        self
    }

    /// Appends a `TAG_Short` field.
    #[allow(dead_code)]
    pub(crate) fn put_short(mut self, name: &str, value: i16) -> Self {
        self.entries.push((name.to_owned(), TagValue::Short(value)));
        self
    }

    /// Appends a `TAG_Int` field.
    pub(crate) fn put_int(mut self, name: &str, value: i32) -> Self {
        self.entries.push((name.to_owned(), TagValue::Int(value)));
        self
    }

    /// Appends a `TAG_Long` field.
    #[allow(dead_code)]
    pub(crate) fn put_long(mut self, name: &str, value: i64) -> Self {
        self.entries.push((name.to_owned(), TagValue::Long(value)));
        self
    }

    /// Appends a `TAG_Float` field.
    #[allow(dead_code)]
    pub(crate) fn put_float(mut self, name: &str, value: f32) -> Self {
        self.entries.push((name.to_owned(), TagValue::Float(value)));
        self
    }

    /// Appends a nested `TAG_Compound` field.
    #[allow(dead_code)]
    pub(crate) fn put_compound(mut self, name: &str, value: NbtCompound) -> Self {
        self.entries
            .push((name.to_owned(), TagValue::Compound(value.entries)));
        self
    }

    /// Encodes the compound into network-byte-order bytes.
    ///
    /// The result is a complete unnamed-compound payload (starts with
    /// `0x0A`, ends with `TAG_End`) suitable for `receive_data` channels.
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = Buf::new();
        buf.write_byte(TAG_COMPOUND);
        // Empty root name: u16 length = 0.
        buf.write_byte(0);
        buf.write_byte(0);
        Self::encode_entries(&self.entries, &mut buf);
        buf.write_byte(TAG_END);
        buf.into_vec()
    }

    /// Helper: encode a list of `(name, value)` entries (no leading/wrapping tags).
    fn encode_entries(entries: &[(String, TagValue)], buf: &mut Buf) {
        for (name, value) in entries {
            // Tag id
            buf.write_byte(tag_id(value));
            // Field name: u16 byte-length + UTF-8 bytes.
            let name_bytes = name.as_bytes();
            let name_len = u16::try_from(name_bytes.len()).unwrap_or(u16::MAX);
            buf.write_byte((name_len >> 8) as u8);
            buf.write_byte((name_len & 0xFF) as u8);
            buf.write_bytes(name_bytes);

            match value {
                TagValue::Byte(v) => buf.write_byte(v.to_be_bytes()[0]),
                TagValue::Short(v) => buf.write_bytes(&v.to_be_bytes()),
                TagValue::Int(v) => buf.write_i32(*v),
                TagValue::Long(v) => buf.write_i64(*v),
                TagValue::Float(v) => buf.write_f32(*v),
                TagValue::Compound(entries) => {
                    Self::encode_entries(entries, buf);
                    buf.write_byte(TAG_END);
                }
            }
        }
    }
}

/// Returns the NBT tag id for a given value variant.
const fn tag_id(v: &TagValue) -> u8 {
    match v {
        TagValue::Byte(_) => TAG_BYTE,
        TagValue::Short(_) => TAG_SHORT,
        TagValue::Int(_) => TAG_INT,
        TagValue::Long(_) => TAG_LONG,
        TagValue::Float(_) => TAG_FLOAT,
        TagValue::Compound(_) => TAG_COMPOUND,
    }
}
