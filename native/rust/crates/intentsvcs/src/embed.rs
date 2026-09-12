//! The semantic tier's seam: what turns text into vectors, and what a vector
//! can be compared with.
//!
//! **THE TIER IS SPECIFIED AND STAGED, NOT SHIPPED WORKING.** Nothing chunks a
//! corpus and nothing writes an embedding in this cut; what is here is the
//! interface the tier plugs into, an implementation that REFUSES, one that
//! talks to an OpenAI-compatible endpoint, and the comparison. That is the
//! claim the design makes about T3 -- a staged addition whose seams the T1 and
//! T2 build proves -- and the way to keep it honest is to build the seam and
//! not to pretend the tier is there.
//!
//! # Why the Null implementation refuses rather than returning zeros
//!
//! A zero vector is not an absence: it has a cosine with every other vector,
//! so an unconfigured project would get a ranked list of nothing, ordered
//! confidently. **A wrong answer that scores is worse than a refusal**, and it
//! is the exact defect this estate keeps finding in other forms -- a clean
//! report over a corpus nobody read.

use std::io::{Read, Write};

use crate::remedy::Remedy;

/// What turns text into vectors.
///
/// **`model` AND `dims` ARE PART OF THE INTERFACE AND NOT METADATA.** A vector
/// whose model is unknown cannot be compared with another: two models' spaces
/// are unrelated, so a cosine between them is a number with no meaning. Every
/// stored vector carries both, and a reader that meets a vector of another
/// model must skip it rather than score it.
/// **`Send + Sync` IS PART OF THE INTERFACE AND NOT A DETAIL.** The facade is
/// carried across threads -- the terminal loads one behind a progress spinner,
/// the daemon holds one per project -- so an embedder that could not go with it
/// would make the facade unsendable and the failure would land in `render.rs`
/// rather than here. An embedder is a client, and a client that cannot be
/// shared is one nobody can hold.
pub trait Embedder: Send + Sync {
  /// The model's name, as it is stored beside every vector it produced.
  fn model(&self) -> &str;
  /// The width every vector from this model has.
  fn dims(&self) -> usize;
  /// The vectors for these texts, in the order given.
  fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbedError>;
}

/// Why no vectors came back.
///
/// **EVERY VARIANT IS A REFUSAL WITH A REMEDY AND NONE IS AN EMPTY RESULT**
/// (IN-AG-NO-SILENT-001). An embedder that answered an unreachable endpoint
/// with no vectors would leave a search reporting that nothing matched.
#[derive(Debug, thiserror::Error)]
pub enum EmbedError {
  #[error("this project has no embedder configured, so it cannot answer a semantic query")]
  NotConfigured,
  #[error("the embedder at `{endpoint}` could not be reached: {detail}")]
  Unreachable { endpoint: String, detail: String },
  #[error("the embedder at `{endpoint}` refused the request: {status}")]
  Refused { endpoint: String, status: String },
  #[error("the embedder at `{endpoint}` answered something this build cannot read: {detail}")]
  Unreadable { endpoint: String, detail: String },
  #[error("`{model}` answered vectors {got} wide where the configuration says {expected}")]
  WrongWidth {
    model: String,
    expected: usize,
    got: usize,
  },
}

impl Remedy for EmbedError {
  fn remedy(&self) -> String {
    match self {
      // **THE REMEDY NAMES THE CONFIGURATION, WHICH IS THE WHOLE POINT OF THE
      // REFUSAL.** An operator who asked a semantic question of a project that
      // has no embedder needs the key and the two values, not the news that
      // the answer is empty.
      Self::NotConfigured => "add an `embed` block to `intent/.config/config.json` with `endpoint` and `model` -- an OpenAI-compatible endpoint. Without one, the lexical and structural tiers answer and the semantic tier is absent.".to_string(),
      Self::Unreachable { endpoint, .. } => format!(
        "check that {endpoint} is running and reachable from this machine, then ask again"
      ),
      Self::Refused { endpoint, .. } => format!(
        "check the credentials and the model name the `embed` block sends to {endpoint}"
      ),
      Self::Unreadable { .. } => {
        "the endpoint is answering something other than an OpenAI-compatible embeddings response -- check that the `endpoint` in the `embed` block is the embeddings path and not the chat one".to_string()
      }
      Self::WrongWidth { model, expected, .. } => format!(
        "set `dims` in the `embed` block to the width `{model}` actually returns, or point `model` at the one whose width is {expected}"
      ),
    }
  }
}

/// The embedder for a project with none configured.
///
/// **IT REFUSES AND IT IS NOT A FAILURE MODE.** A project without an `embed`
/// block is the normal case in this cut, and the refusal is how a semantic
/// question gets an answer that names the configuration rather than an empty
/// list that looks like "nothing matched".
pub struct Null;

impl Embedder for Null {
  fn model(&self) -> &str {
    ""
  }

  fn dims(&self) -> usize {
    0
  }

  fn embed(&self, _texts: &[String]) -> Result<Vec<Vec<f32>>, EmbedError> {
    Err(EmbedError::NotConfigured)
  }
}

/// An embedder against an OpenAI-compatible `/embeddings` endpoint.
///
/// **NO HTTP CRATE, AND THAT IS A CHOICE WITH A REASON.** This is one POST of
/// one JSON document to an endpoint the operator named, and the workspace's
/// dependency rule asks for a written rationale per crate; a client library
/// would bring a TLS stack and an async runtime into a crate that has neither,
/// for a request that is twelve lines of `std::net`. **The limit is stated
/// rather than discovered: this speaks `http://` only.** An `https://`
/// endpoint is refused by name, with the remedy, rather than silently
/// downgraded -- a key sent in clear to an endpoint that expected TLS is worse
/// than a refusal.
pub struct Http {
  endpoint: String,
  model: String,
  dims: usize,
  key: Option<String>,
}

impl Http {
  pub fn new(endpoint: &str, model: &str, dims: usize, key: Option<String>) -> Self {
    Self {
      endpoint: endpoint.to_string(),
      model: model.to_string(),
      dims,
      key,
    }
  }

  /// `(host:port, path)` from the endpoint, or the refusal that says why not.
  fn address(&self) -> Result<(String, String), EmbedError> {
    let rest = self
      .endpoint
      .strip_prefix("http://")
      .ok_or_else(|| EmbedError::Unreachable {
        endpoint: self.endpoint.clone(),
        detail: "this build speaks `http://` only".to_string(),
      })?;
    let (host, path) = match rest.find('/') {
      Some(at) => (&rest[..at], &rest[at..]),
      None => (rest, "/"),
    };
    let host = if host.contains(':') {
      host.to_string()
    } else {
      format!("{host}:80")
    };
    Ok((host, path.to_string()))
  }
}

impl Embedder for Http {
  fn model(&self) -> &str {
    &self.model
  }

  fn dims(&self) -> usize {
    self.dims
  }

  fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbedError> {
    let (host, path) = self.address()?;
    let body = serde_json::json!({ "model": self.model, "input": texts }).to_string();
    let mut request = format!(
      "POST {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
      body.len()
    );
    if let Some(key) = &self.key {
      request.push_str(&format!("Authorization: Bearer {key}\r\n"));
    }
    request.push_str("\r\n");
    request.push_str(&body);

    let unreachable = |detail: String| EmbedError::Unreachable {
      endpoint: self.endpoint.clone(),
      detail,
    };
    let mut stream = std::net::TcpStream::connect(&host).map_err(|e| unreachable(e.to_string()))?;
    stream
      .write_all(request.as_bytes())
      .map_err(|e| unreachable(e.to_string()))?;
    let mut answer = String::new();
    stream
      .read_to_string(&mut answer)
      .map_err(|e| unreachable(e.to_string()))?;

    parse_answer(&answer, &self.endpoint, &self.model, self.dims)
  }
}

/// The vectors in an OpenAI-shaped answer, or the refusal that says what came
/// back instead.
///
/// Pure, so the parsing is driven on strings rather than through a socket: the
/// socket is the part a test doubles, and the shape of the answer is the part
/// that actually goes wrong.
pub fn parse_answer(
  answer: &str,
  endpoint: &str,
  model: &str,
  dims: usize,
) -> Result<Vec<Vec<f32>>, EmbedError> {
  let unreadable = |detail: &str| EmbedError::Unreadable {
    endpoint: endpoint.to_string(),
    detail: detail.to_string(),
  };
  let (head, body) = answer
    .split_once("\r\n\r\n")
    .ok_or_else(|| unreadable("no HTTP body"))?;
  let status = head.lines().next().unwrap_or_default();
  if !status.contains(" 200") {
    return Err(EmbedError::Refused {
      endpoint: endpoint.to_string(),
      status: status.to_string(),
    });
  }
  let parsed: serde_json::Value =
    serde_json::from_str(body).map_err(|e| unreadable(&e.to_string()))?;
  let rows = parsed
    .get("data")
    .and_then(|d| d.as_array())
    .ok_or_else(|| unreadable("no `data` array in the answer"))?;

  let mut out = Vec::new();
  for row in rows {
    let vector: Vec<f32> = row
      .get("embedding")
      .and_then(|e| e.as_array())
      .ok_or_else(|| unreadable("a `data` row carries no `embedding`"))?
      .iter()
      .filter_map(|n| n.as_f64().map(|f| f as f32))
      .collect();
    if vector.len() != dims {
      return Err(EmbedError::WrongWidth {
        model: model.to_string(),
        expected: dims,
        got: vector.len(),
      });
    }
    out.push(vector);
  }
  Ok(out)
}

/// One stored vector: what it covers, which model produced it, and the vector.
///
/// **`dims` IS NOT A FIELD HERE AND IS A COLUMN IN THE STORE**, which is not a
/// contradiction: in memory the width is the vector's own length and a second
/// copy could disagree with it, while on disk the BLOB has no length of its own
/// until something says how wide a value is.
#[derive(Debug, Clone, PartialEq)]
pub struct Stored {
  /// The indexed unit this vector covers. A path today; a chunk when something
  /// chunks.
  pub chunk_id: String,
  pub model: String,
  pub vector: Vec<f32>,
}

/// The cosine of two vectors, or `None` when they cannot be compared.
///
/// **DIFFERENT WIDTHS ARE `None` AND NOT ZERO.** Zero is a real cosine -- two
/// vectors at right angles -- so returning it for an unanswerable comparison
/// would put an incomparable pair in the ranking at a plausible position. A
/// zero-length vector is the same case: it has no direction, so there is
/// nothing to compare.
pub fn cosine(a: &[f32], b: &[f32]) -> Option<f32> {
  if a.len() != b.len() || a.is_empty() {
    return None;
  }
  let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
  let norm = |v: &[f32]| v.iter().map(|x| x * x).sum::<f32>().sqrt();
  let (na, nb) = (norm(a), norm(b));
  if na == 0.0 || nb == 0.0 {
    return None;
  }
  Some(dot / (na * nb))
}

/// The embedder a project's configuration asks for.
///
/// **A PROJECT WITH NO `embed` BLOCK GETS THE ONE THAT REFUSES**, which is the
/// normal case in this cut and is not an error until something asks a semantic
/// question.
pub fn from_config(config: &crate::project::EmbedConfig) -> Box<dyn Embedder> {
  match (&config.endpoint, &config.model) {
    (Some(endpoint), Some(model)) => {
      Box::new(Http::new(endpoint, model, config.dims, config.key.clone()))
    }
    _ => Box::new(Null),
  }
}
