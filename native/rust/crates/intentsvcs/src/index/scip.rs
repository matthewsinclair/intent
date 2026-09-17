//! The parts of a SCIP index Rust's level 3 reads (ST0076 WP-05, vc decision 25
//! (5)): rust-analyzer's `scip` export, decoded from its protobuf bytes.
//!
//! **HAND-WRITTEN, AND ONLY THE FIELDS LEVEL 3 READS.** A generated decoder
//! would bring a protobuf toolchain into the build for five fields. The field
//! numbers are scip.proto's, named once below; every other field is skipped by
//! its wire type, so a newer exporter that adds fields still decodes.
//!
//! **PURE (IN-AG-PFIC-001).** Bytes in, values out. Running the export and
//! reading the file are the Rust reader's.

/// scip.proto's field numbers, for the messages and fields this reads.
mod field {
  /// `Index.metadata`
  pub const INDEX_METADATA: u32 = 1;
  /// `Index.documents`
  pub const INDEX_DOCUMENTS: u32 = 2;
  /// `Metadata.project_root`
  pub const METADATA_PROJECT_ROOT: u32 = 3;
  /// `Document.relative_path`
  pub const DOCUMENT_RELATIVE_PATH: u32 = 1;
  /// `Document.occurrences`
  pub const DOCUMENT_OCCURRENCES: u32 = 2;
  /// `Document.position_encoding`
  pub const DOCUMENT_POSITION_ENCODING: u32 = 6;
  /// `Occurrence.range`
  pub const OCCURRENCE_RANGE: u32 = 1;
  /// `Occurrence.symbol`
  pub const OCCURRENCE_SYMBOL: u32 = 2;
  /// `Occurrence.symbol_roles`
  pub const OCCURRENCE_SYMBOL_ROLES: u32 = 3;
}

/// Protobuf's wire types, as the key's low three bits carry them.
mod wire {
  pub const VARINT: u8 = 0;
  pub const FIXED64: u8 = 1;
  pub const LEN: u8 = 2;
  pub const FIXED32: u8 = 5;
}

/// scip.proto's `SymbolRole.Definition` bit in `Occurrence.symbol_roles`.
pub const DEFINITION: i32 = 0x1;

/// scip.proto's `PositionEncoding.UTF8CodeUnitOffsetFromLineStart`: a column
/// is a byte offset into its line, which is what rust-analyzer writes.
pub const UTF8_COLUMNS: i32 = 1;

/// An index, as far as level 3 reads one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Index {
  /// The URI the export was run over; each document's path is relative to it.
  pub project_root: String,
  pub documents: Vec<Document>,
}

/// One source file in the index.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Document {
  /// Relative to [`Index::project_root`], as the export wrote it.
  pub relative_path: String,
  /// What a column counts in; [`UTF8_COLUMNS`] is bytes. 0 where the exporter
  /// did not say.
  pub position_encoding: i32,
  pub occurrences: Vec<Occurrence>,
}

/// One occurrence of a symbol in a document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Occurrence {
  /// The line the occurrence starts on, 0-based as SCIP counts.
  pub line: u32,
  /// The column it starts at, 0-based, in the document's position encoding.
  pub column: u32,
  /// Where it ends, as [`Self::line`] and [`Self::column`] count.
  pub end_line: u32,
  pub end_column: u32,
  /// The symbol, as the exporter spells it: `rust-analyzer cargo <crate>
  /// <version> <descriptors>`, or `local <n>`.
  pub symbol: String,
  pub roles: i32,
}

impl Occurrence {
  /// Is this the definition of its symbol, rather than a reference to it?
  pub fn is_definition(&self) -> bool {
    self.roles & DEFINITION != 0
  }
}

/// Bytes that are not a SCIP index this reader can read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("the SCIP index is malformed at byte {offset}: {what}")]
pub struct DecodeError {
  pub offset: usize,
  pub what: &'static str,
}

/// Decode an index.
pub fn decode(bytes: &[u8]) -> Result<Index, DecodeError> {
  let mut index = Index::default();
  let mut r = Reader::new(bytes, 0);
  while !r.done() {
    let (number, wire_type) = r.key()?;
    match (number, wire_type) {
      (field::INDEX_METADATA, wire::LEN) => {
        let mut m = r.message()?;
        while !m.done() {
          match m.key()? {
            (field::METADATA_PROJECT_ROOT, wire::LEN) => index.project_root = m.string()?,
            (_, other) => m.skip(other)?,
          }
        }
      }
      (field::INDEX_DOCUMENTS, wire::LEN) => index.documents.push(document(r.message()?)?),
      (_, other) => r.skip(other)?,
    }
  }
  Ok(index)
}

fn document(mut r: Reader<'_>) -> Result<Document, DecodeError> {
  let mut doc = Document::default();
  while !r.done() {
    match r.key()? {
      (field::DOCUMENT_RELATIVE_PATH, wire::LEN) => doc.relative_path = r.string()?,
      (field::DOCUMENT_OCCURRENCES, wire::LEN) => doc.occurrences.push(occurrence(r.message()?)?),
      (field::DOCUMENT_POSITION_ENCODING, wire::VARINT) => {
        doc.position_encoding = r.varint()? as i32
      }
      (_, other) => r.skip(other)?,
    }
  }
  Ok(doc)
}

fn occurrence(mut r: Reader<'_>) -> Result<Occurrence, DecodeError> {
  let start = r.offset();
  let mut occ = Occurrence::default();
  let mut range: Vec<i32> = Vec::new();
  while !r.done() {
    match r.key()? {
      // Packed, as proto3 writes a repeated scalar; one element per key is
      // legal too, so both are read.
      (field::OCCURRENCE_RANGE, wire::LEN) => {
        let mut packed = r.message()?;
        while !packed.done() {
          range.push(packed.varint()? as i32);
        }
      }
      (field::OCCURRENCE_RANGE, wire::VARINT) => range.push(r.varint()? as i32),
      (field::OCCURRENCE_SYMBOL, wire::LEN) => occ.symbol = r.string()?,
      (field::OCCURRENCE_SYMBOL_ROLES, wire::VARINT) => occ.roles = r.varint()? as i32,
      (_, other) => r.skip(other)?,
    }
  }
  // A range is `[line, column, end column]` on one line or `[line, column, end
  // line, end column]`, and anything else is not an occurrence anyone can place.
  let (line, column, end_line, end_column) = match range.as_slice() {
    [line, column, end_column] => (*line, *column, *line, *end_column),
    [line, column, end_line, end_column] => (*line, *column, *end_line, *end_column),
    _ => (-1, -1, -1, -1),
  };
  match [line, column, end_line, end_column].map(u32::try_from) {
    [Ok(line), Ok(column), Ok(end_line), Ok(end_column)] => {
      occ.line = line;
      occ.column = column;
      occ.end_line = end_line;
      occ.end_column = end_column;
      Ok(occ)
    }
    _ => Err(DecodeError {
      offset: start,
      what: "an occurrence's range is not three or four non-negative numbers",
    }),
  }
}

/// A cursor over one message's bytes. `base` is where they start in the whole
/// index, so an error names a byte of the file rather than of the message.
struct Reader<'a> {
  bytes: &'a [u8],
  pos: usize,
  base: usize,
}

impl<'a> Reader<'a> {
  fn new(bytes: &'a [u8], base: usize) -> Self {
    Reader {
      bytes,
      pos: 0,
      base,
    }
  }

  fn done(&self) -> bool {
    self.pos >= self.bytes.len()
  }

  fn offset(&self) -> usize {
    self.base + self.pos
  }

  fn fail<T>(&self, what: &'static str) -> Result<T, DecodeError> {
    Err(DecodeError {
      offset: self.offset(),
      what,
    })
  }

  fn varint(&mut self) -> Result<u64, DecodeError> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
      let Some(&byte) = self.bytes.get(self.pos) else {
        return self.fail("a number runs past the end of its message");
      };
      self.pos += 1;
      value |= u64::from(byte & 0x7f) << shift;
      if byte & 0x80 == 0 {
        return Ok(value);
      }
    }
    self.fail("a number is longer than ten bytes")
  }

  fn key(&mut self) -> Result<(u32, u8), DecodeError> {
    let key = self.varint()?;
    let number = u32::try_from(key >> 3).or_else(|_| self.fail("a field number is too large"))?;
    Ok((number, (key & 0x7) as u8))
  }

  fn len_delimited(&mut self) -> Result<(usize, &'a [u8]), DecodeError> {
    let len = self.varint()?;
    let start = self.pos;
    match usize::try_from(len)
      .ok()
      .and_then(|len| start.checked_add(len))
    {
      Some(end) if end <= self.bytes.len() => {
        self.pos = end;
        Ok((self.base + start, &self.bytes[start..end]))
      }
      _ => self.fail("a length runs past the end of its message"),
    }
  }

  fn message(&mut self) -> Result<Reader<'a>, DecodeError> {
    let (base, bytes) = self.len_delimited()?;
    Ok(Reader::new(bytes, base))
  }

  fn string(&mut self) -> Result<String, DecodeError> {
    let (_, bytes) = self.len_delimited()?;
    match std::str::from_utf8(bytes) {
      Ok(text) => Ok(text.to_string()),
      Err(_) => self.fail("a string is not UTF-8"),
    }
  }

  fn skip(&mut self, wire_type: u8) -> Result<(), DecodeError> {
    let width = match wire_type {
      wire::VARINT => return self.varint().map(|_| ()),
      wire::LEN => return self.len_delimited().map(|_| ()),
      wire::FIXED64 => 8,
      wire::FIXED32 => 4,
      _ => return self.fail("a field has a wire type this reader does not read"),
    };
    match self.pos.checked_add(width) {
      Some(end) if end <= self.bytes.len() => {
        self.pos = end;
        Ok(())
      }
      _ => self.fail("a fixed-width field runs past the end of its message"),
    }
  }
}
