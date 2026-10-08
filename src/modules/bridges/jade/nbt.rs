//! Minimal NBT (Named Binary Tag) *writer* for Jade's `receive_data` payloads.
//!
//! Jade expects a single unnamed-compound NBT encoded as:
//! `0x0A | u16 name_len (=0) | compound-payload | 0x00 (TAG_End)`.
//!
//! We only implement what Jade providers actually use: primitive scalars,
//! byte arrays (every provider payload shape), strings, and nested compounds.
//! This is *not* a general NBT library.
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
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

/// A single NBT tag value.
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
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(u8, Vec<TagValue>),
    Compound(Vec<(String, TagValue)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// A mutable NBT compound. Methods take `&mut self` so multiple providers
/// can write into the same tag while responding to a single request.
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
    pub(crate) fn put_byte(&mut self, name: &str, value: i8) -> &mut Self {
        self.entries.push((name.to_owned(), TagValue::Byte(value)));
        self
    }

    /// Appends a `TAG_Short` field.
    #[allow(dead_code)]
    pub(crate) fn put_short(&mut self, name: &str, value: i16) -> &mut Self {
        self.entries.push((name.to_owned(), TagValue::Short(value)));
        self
    }

    /// Appends a `TAG_Int` field.
    pub(crate) fn put_int(&mut self, name: &str, value: i32) -> &mut Self {
        self.entries.push((name.to_owned(), TagValue::Int(value)));
        self
    }

    /// Appends a `TAG_Long` field.
    #[allow(dead_code)]
    pub(crate) fn put_long(&mut self, name: &str, value: i64) -> &mut Self {
        self.entries.push((name.to_owned(), TagValue::Long(value)));
        self
    }

    /// Appends a `TAG_Float` field.
    ///
    /// Currently unused — every Jade provider writes its float payload as bytes,
    /// not as a raw NBT `TAG_Float`. Kept on the builder for parity with the
    /// NBT spec.
    #[allow(dead_code)]
    pub(crate) fn put_float(&mut self, name: &str, value: f32) -> &mut Self {
        self.entries.push((name.to_owned(), TagValue::Float(value)));
        self
    }

    /// Appends a `TAG_Double` field.
    #[allow(dead_code)]
    pub(crate) fn put_double(&mut self, name: &str, value: f64) -> &mut Self {
        self.entries
            .push((name.to_owned(), TagValue::Double(value)));
        self
    }

    /// Appends a `TAG_Byte_Array` field — the shape every Jade provider
    /// uses for its key → encoded payload.
    pub(crate) fn put_byte_array(&mut self, name: &str, value: Vec<u8>) -> &mut Self {
        self.entries
            .push((name.to_owned(), TagValue::ByteArray(value)));
        self
    }

    /// Appends a `TAG_String` field.
    // TODO(block-id): used by the bridge's `BlockId` field once available;
    // providers may also use it for short string data.
    #[allow(dead_code)]
    pub(crate) fn put_string(&mut self, name: &str, value: &str) -> &mut Self {
        self.entries
            .push((name.to_owned(), TagValue::String(value.to_owned())));
        self
    }

    /// Appends a nested `TAG_Compound` field.
    #[allow(dead_code)]
    pub(crate) fn put_compound(&mut self, name: &str, value: NbtCompound) -> &mut Self {
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
            buf.write_byte(tag_id(value));
            let name_bytes = name.as_bytes();
            let name_len = u16::try_from(name_bytes.len()).unwrap_or(u16::MAX);
            buf.write_byte((name_len >> 8) as u8);
            buf.write_byte((name_len & 0xFF) as u8);
            buf.write_bytes(name_bytes);
            value.encode_payload(buf);
        }
    }
}

impl TagValue {
    /// Encodes just this value's payload (no tag id, no name).
    fn encode_payload(&self, buf: &mut Buf) {
        match self {
            TagValue::Byte(v) => buf.write_byte(v.to_be_bytes()[0]),
            TagValue::Short(v) => buf.write_bytes(&v.to_be_bytes()),
            TagValue::Int(v) => buf.write_i32(*v),
            TagValue::Long(v) => buf.write_i64(*v),
            TagValue::Float(v) => buf.write_f32(*v),
            TagValue::Double(v) => buf.write_bytes(&v.to_be_bytes()),
            TagValue::ByteArray(v) => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                buf.write_i32(v.len() as i32);
                buf.write_bytes(v);
            }
            TagValue::String(v) => {
                let bytes = v.as_bytes();
                let len = u16::try_from(bytes.len()).unwrap_or(u16::MAX);
                buf.write_byte((len >> 8) as u8);
                buf.write_byte((len & 0xFF) as u8);
                buf.write_bytes(bytes);
            }
            TagValue::List(elem_id, elems) => {
                buf.write_byte(*elem_id);
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                buf.write_i32(elems.len() as i32);
                for e in elems {
                    e.encode_payload(buf);
                }
            }
            TagValue::Compound(entries) => {
                NbtCompound::encode_entries(entries, buf);
                buf.write_byte(TAG_END);
            }
            TagValue::IntArray(v) => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                buf.write_i32(v.len() as i32);
                for i in v {
                    buf.write_i32(*i);
                }
            }
            TagValue::LongArray(v) => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                buf.write_i32(v.len() as i32);
                for i in v {
                    buf.write_i64(*i);
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
        TagValue::Double(_) => TAG_DOUBLE,
        TagValue::ByteArray(_) => TAG_BYTE_ARRAY,
        TagValue::String(_) => TAG_STRING,
        TagValue::List(..) => TAG_LIST,
        TagValue::Compound(_) => TAG_COMPOUND,
        TagValue::IntArray(_) => TAG_INT_ARRAY,
        TagValue::LongArray(_) => TAG_LONG_ARRAY,
    }
}
