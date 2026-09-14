//! AT-23.2 and AT-23.3: the semantic tier's seams, built and proved while the
//! tier itself is staged.
//!
//! **NOTHING CHUNKS A CORPUS AND NOTHING EMBEDS ONE IN THIS CUT**, which is
//! what the design says T3 is: a staged addition whose seams the T1 and T2
//! build proves. So these arms drive the seams -- the interface, the refusal,
//! the endpoint, the comparison and the group -- with the vectors planted,
//! rather than pretending a tier is running.

use crate::common;

use std::io::{Read, Write};

use common::Fixture;
use intentsvcs::embed::{EmbedError, Embedder, Null, Stored};
use intentsvcs::remedy::Remedy;
use intentsvcs::search::{SearchQuery, Tier};
use intentsvcs::store::Store;

/// AT-23.2, the refusal half: a project with no embedder says so, and says what
/// to configure.
#[test]
fn the_null_embedder_refuses_and_its_remedy_names_the_configuration() {
  let refused = Null
    .embed(&["a question".to_string()])
    .expect_err("a project with no embedder cannot answer a semantic query");

  assert!(
    matches!(refused, EmbedError::NotConfigured),
    "got {refused:?}"
  );
  let remedy = refused.remedy();
  for named in ["embed", "endpoint", "model", "config.json"] {
    assert!(
      remedy.contains(named),
      "the remedy must name what to configure, and `{named}` is missing: {remedy}"
    );
  }
}

/// AT-23.2, the endpoint half: the HTTP embedder posts to the endpoint the
/// configuration names and returns the vectors it answered, at the model's
/// width.
///
/// **THE LISTENER IS A DOUBLE AT THE EXTERNAL BOUNDARY**, which is the one
/// place this estate allows one: the thing under test is our request and our
/// reading of the answer, and an OpenAI-compatible service is not ours to run.
#[test]
fn the_http_embedder_asks_the_configured_endpoint_and_reads_its_answer() {
  let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
  let endpoint = format!(
    "http://{}/v1/embeddings",
    listener.local_addr().expect("addr")
  );

  let served = std::thread::spawn(move || {
    let (mut socket, _) = listener.accept().expect("accept");
    let mut seen = [0_u8; 2048];
    let read = socket.read(&mut seen).expect("read the request");
    let request = String::from_utf8_lossy(&seen[..read]).to_string();
    let body = r#"{"data":[{"embedding":[0.5,0.5,0.5,0.5]}]}"#;
    socket
      .write_all(
        format!(
          "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
          body.len()
        )
        .as_bytes(),
      )
      .expect("answer");
    request
  });

  let embedder = intentsvcs::embed::Http::new(&endpoint, "text-embed-small", 4, None);
  let vectors = embedder
    .embed(&["a question".to_string()])
    .expect("the endpoint answered");

  assert_eq!(vectors.len(), 1);
  assert_eq!(
    vectors[0].len(),
    embedder.dims(),
    "the vectors come back at the width the configuration names"
  );

  let request = served.join().expect("the listener thread");
  assert!(
    request.starts_with("POST /v1/embeddings HTTP/1.1"),
    "it posts to the path the endpoint names: {request}"
  );
  assert!(
    request.contains("\"model\":\"text-embed-small\"") && request.contains("a question"),
    "and sends the model and the text: {request}"
  );
}

/// A test-only embedder: one fixed vector, whatever it is asked.
///
/// This is the SEAM being exercised rather than a mock of the thing under test
/// -- `with_embedder` is how a caller supplies its own, and a test is such a
/// caller.
struct Fixed;

impl Embedder for Fixed {
  fn model(&self) -> &str {
    "fixed"
  }

  fn dims(&self) -> usize {
    3
  }

  fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbedError> {
    Ok(texts.iter().map(|_| vec![1.0, 0.0, 0.0]).collect())
  }
}

/// AT-23.3: semantic hits are a tier group, ranked within themselves, and the
/// envelope's shape does not change to admit them.
#[test]
fn semantic_hits_are_a_group_of_their_own_ranked_by_cosine() {
  let fx = Fixture::new();
  {
    let mut store = Store::open(&fx.project().db_path()).expect("store");
    store
      .record_embeddings(&[
        Stored {
          chunk_id: "docs/near.md".to_string(),
          model: "fixed".to_string(),
          vector: vec![0.9, 0.1, 0.0],
        },
        Stored {
          chunk_id: "docs/far.md".to_string(),
          model: "fixed".to_string(),
          vector: vec![0.0, 1.0, 0.0],
        },
        // Another model's vector, which must not be compared with these: two
        // models' spaces are unrelated and a cosine between them is a number
        // with no meaning.
        Stored {
          chunk_id: "docs/other.md".to_string(),
          model: "another-model".to_string(),
          vector: vec![1.0, 0.0, 0.0],
        },
      ])
      .expect("plant the vectors a chunker would have written");
  }

  let facade = fx.facade_on_disk().with_embedder(Box::new(Fixed));
  let answer = facade
    .search_all("a question", &SearchQuery::default())
    .expect("the search answered");

  let semantic = answer
    .groups
    .iter()
    .find(|g| g.tier == Tier::Semantic)
    .expect("the semantic tier is a group of its own");
  assert_eq!(
    semantic
      .hits
      .iter()
      .map(|h| h.path.as_str())
      .collect::<Vec<_>>(),
    vec!["docs/near.md", "docs/far.md"],
    "ranked within the tier, nearest first, and another model's vector is not \
     in the answer at all: {answer:?}"
  );
  assert!(
    semantic.hits[0].score > semantic.hits[1].score,
    "the score is the cosine and it is published, not invented: {semantic:?}"
  );
  assert!(
    answer.groups.iter().any(|g| g.tier == Tier::Lexical)
      && answer.groups.iter().any(|g| g.tier == Tier::Structural),
    "and the tiers that were already built are still groups: {answer:?}"
  );
}

/// The other half of the same claim: a project with no embedder has no semantic
/// group, rather than an empty one.
#[test]
fn a_project_with_no_embedder_has_no_semantic_group() {
  let fx = Fixture::new();
  let answer = fx
    .facade()
    .search_all("a question", &SearchQuery::default())
    .expect("the search answered");

  assert!(
    !answer.groups.iter().any(|g| g.tier == Tier::Semantic),
    "an empty group would claim a tier ran and matched nothing, where the truth \
     is that this project has no such tier: {answer:?}"
  );
}
