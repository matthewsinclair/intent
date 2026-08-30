//! The ONLY way a request handler reaches a project's data.
//!
//! **THIS MODULE EXISTS TO MAKE `AC-08.11`'s STRUCTURAL OBLIGATION HOLD BY
//! CONSTRUCTION RATHER THAN BY DISCIPLINE.** The row requires that no blocking
//! store call ever occupies an async worker thread, and the reason is precise:
//! per-connection tasks order operations WITHIN a connection, while the
//! starvation that defeats the client's deadline happens BETWEEN them. The
//! accept loop is itself a task on the same runtime, so N concurrent blocking
//! `rusqlite` calls on N workers leave the probe's connection sitting in the
//! kernel backlog until its deadline expires -- and the client then routes
//! in-process against a store this daemon owns, which is the exact failure the
//! routing rule exists to prevent.
//!
//! **SO THE `Facade` NEVER LEAVES THIS FILE.** It is owned by a plain
//! `std::thread` and reached only through [`ProjectHandle`], whose methods are
//! all `async` and none of which can block a worker: the send is asynchronous,
//! the wait is a `oneshot`, and the blocking work happens on a thread the
//! runtime does not schedule. A handler cannot reach a blocking handle because
//! it is never given a type that has one.
//!
//! **THE COMPILER IS THEREFORE THE WITNESS, WHICH IS WHAT THE ROW ASKED FOR.**
//! A latency test would have measured the machine rather than the code and
//! passed with every line of this discipline deleted. What holds here is that
//! `Facade` does not appear in any other module of this crate -- checked
//! mechanically by `tests/one_store_door.rs`, in the shape `dep_graph_guard.rs`
//! uses for D06, so the arrangement cannot be quietly undone by someone who
//! reasonably wanted a store handle where they were working.
//!
//! **A THREAD PER PROJECT RATHER THAN `spawn_blocking`, AND THE REASON IS
//! OWNERSHIP.** `Facade`'s mutating methods take `&mut self`, so the handle
//! cannot be shared across tasks under any design -- something has to own it
//! exclusively. A dedicated thread makes that ownership the thread's, which
//! also serialises every operation on one project without a lock anywhere:
//! requests queue in the channel and are served in order. `spawn_blocking`
//! would have needed a mutex around the facade to achieve the same thing, and
//! a mutex held across a blocking call is the shape this module exists to avoid.

use std::path::{Path, PathBuf};

use intentsvcs::facade::{Facade, FacadeContext};
use intentsvcs::project::Project;
use intentsvcs::remedy::Remedy;
use intentsvcs::wire::{Op, Response, ThreadSummary};
use tokio::sync::{mpsc, oneshot};

/// How many requests may be queued for one project before callers wait.
///
/// **BOUNDED, SO A CLIENT CANNOT MAKE THE DAEMON BUY MEMORY ON ITS BEHALF.** An
/// unbounded queue turns a slow store into unbounded growth; a bounded one
/// turns it into backpressure, where the caller waits and the daemon stays the
/// size it was. The wait is asynchronous, so a full queue delays the requests
/// for THAT project and no others -- and never the accept loop.
const QUEUE_DEPTH: usize = 64;

/// One unit of work for a project's store thread.
///
/// **TWO ARMS BECAUSE THERE ARE TWO ORIGINS, AND CONFLATING THEM WOULD BREAK
/// THE ONE DISCRIMINATOR `AC-08.2` RESTS ON.** A client request is counted,
/// answered and attributable; a watcher-driven ingest has no client, nobody to
/// answer, and must NOT move [`ProjectHandle::dispatched`] -- that counter says
/// *how many ops a CLIENT routed here*, and a background ingest landing inside
/// a harness's before-and-after bracket would report `+2` for a single verb and
/// make the conformance test flake against a mechanism it does not know about.
///
/// **THE SEPARATION IS STRUCTURAL RATHER THAN A BRANCH.** The count lives in
/// [`ProjectHandle::call`], which is the client door; the watcher reaches
/// [`ProjectHandle::ingest`], which is a different door and cannot increment
/// anything. A `skip if this was internal` branch inside one shared path is
/// exactly where such an exemption goes quietly wrong.
enum Work {
  /// A client asked something and is waiting for the answer.
  Client {
    op: Op,
    reply: oneshot::Sender<Response>,
  },
  /// The watcher saw the project's tree change (`AC-08.5`).
  Ingest,
}

/// A handle to one project's store.
///
/// Cloneable and shareable: it is a channel sender, not a database handle. That
/// difference is the whole design -- see the module note.
/// **NOT `Clone`, AND THE COUNTER IS WHY IT MUST NOT BE.** The handle is always
/// reached through an `Arc` -- the registry stores `Arc<ProjectHandle>` and hands
/// out clones of THAT -- so deriving `Clone` here was never used. It is removed
/// rather than left harmless: **a cloned handle would carry a fresh
/// [`ProjectHandle::dispatched`] while addressing the same store**, so one
/// project would have two independent counts and the registry would report
/// whichever copy it happened to hold. A discriminator that can be halved by an
/// `#[derive]` is not one.
#[derive(Debug)]
pub struct ProjectHandle {
  tx: mpsc::Sender<Work>,
  root: PathBuf,
  /// Ops this handle has dispatched to its store.
  ///
  /// **IT LIVES ON THE HANDLE AND IS INCREMENTED INSIDE [`ProjectHandle::call`],
  /// WHICH MAKES `counts dispatched ops and nothing else` STRUCTURAL RATHER THAN
  /// A RULE SOMEBODY FOLLOWS.** Counting in the connection handler would have
  /// needed a branch to skip liveness probes and the registry, and a branch is
  /// exactly where that exemption goes quietly wrong -- **if the probe
  /// incremented, every fallthrough would increment and the discriminator would
  /// be vacuous**, which is the defect the counter exists to catch, arriving
  /// through the mechanism built to avoid it.
  ///
  /// Nothing but a dispatched op reaches this line: probes are answered before
  /// dispatch, and `Op::Registry` never reaches a handle at all. See
  /// `intentsvcs::wire::UNCOUNTED`, which declares that set so the harness can
  /// form an expectation from it rather than from a list of its own.
  dispatched: std::sync::atomic::AtomicU64,
  /// Ingests this project's tree has driven (`AC-08.5`).
  ///
  /// **INCREMENTED WHERE THE WORK IS DONE, NOT WHERE IT IS REQUESTED, AND THAT
  /// IS THE OPPOSITE CHOICE FROM [`ProjectHandle::dispatched`].** The two
  /// counters answer different questions, so they are counted at different
  /// points on purpose. `dispatched` answers *did a client route here*, which
  /// is true the moment the request is sent and stays true if the store then
  /// fails. This one answers *how many times did the tree actually get
  /// re-read*, which is a claim about work performed -- and counting it at the
  /// send would make a full queue look like completed ingests.
  ingested: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl ProjectHandle {
  /// Open a project and start the thread that owns its store.
  ///
  /// **THE OPEN HAPPENS ON THE STORE THREAD, NOT HERE.** `Facade::open` runs
  /// migrations and reads canon; doing it on the caller's task would block a
  /// worker for exactly as long as the thing this module exists to keep off
  /// them. So the thread is started first and reports back whether it got a
  /// facade, which also means the failure path returns a `Response` a client
  /// can be told rather than an error only a log would see.
  pub async fn open(root: PathBuf) -> Result<ProjectHandle, Response> {
    let (tx, mut rx) = mpsc::channel::<Work>(QUEUE_DEPTH);
    let (ready_tx, ready_rx) = oneshot::channel::<Result<(), Response>>();
    let thread_root = root.clone();
    // **SHARED WITH THE STORE THREAD BECAUSE THAT IS WHERE THE WORK HAPPENS.**
    // The handle reports the count and the thread increments it, so the two
    // must be the same cell -- a counter on the handle alone could only be
    // incremented at the send, which is the meaning this one deliberately does
    // not have.
    let ingested = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let thread_ingested = std::sync::Arc::clone(&ingested);

    std::thread::spawn(move || {
      let mut facade = match open_facade(&thread_root) {
        Ok(facade) => {
          if ready_tx.send(Ok(())).is_err() {
            // Nobody is waiting any more: the caller went away between the
            // spawn and the open. Drop the facade rather than serve a queue no
            // one will read.
            return;
          }
          facade
        }
        Err(refusal) => {
          let _ = ready_tx.send(Err(refusal));
          return;
        }
      };

      // `blocking_recv` is correct HERE and would be a defect anywhere else in
      // this crate: this is not a runtime worker, it is a thread whose entire
      // job is to block.
      while let Some(work) = rx.blocking_recv() {
        match work {
          Work::Client { op, reply } => {
            let response = serve(&mut facade, op);
            // A dropped receiver means the client disconnected mid-request.
            // The work is already done and there is nobody to tell, which is a
            // state rather than a fault.
            let _ = reply.send(response);
          }
          Work::Ingest => {
            ingest(&mut facade, &thread_root);
            // **AFTER THE WORK, AND AFTER IT EVEN WHEN IT FAILED.** The
            // question this answers is *how many times did the watcher make
            // this store re-read the tree*, which is what `AC-08.5`'s
            // debounce and scope claims are about -- and a failed ingest was
            // still a trigger that fired. Counting only successes would make
            // a self-triggering loop over unreadable files invisible.
            thread_ingested.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
          }
        }
      }
    });

    match ready_rx.await {
      Ok(Ok(())) => Ok(ProjectHandle {
        tx,
        root,
        dispatched: std::sync::atomic::AtomicU64::new(0),
        ingested,
      }),
      Ok(Err(refusal)) => Err(refusal),
      // The thread ended without answering: it panicked. Report it as a
      // refusal rather than letting the caller wait on a channel nothing will
      // ever write to.
      Err(_) => Err(Response::error(
        format!(
          "the store thread for `{}` stopped before it could open the project",
          root.display()
        ),
        "this is a fault in intentd rather than in the project. Restart the daemon; if it recurs, the daemon's log names the panic.",
      )),
    }
  }

  /// How many ops have been dispatched to this store.
  pub fn dispatched(&self) -> u64 {
    self.dispatched.load(std::sync::atomic::Ordering::Relaxed)
  }

  /// How many times this project's tree has been re-read after a change.
  pub fn ingested(&self) -> u64 {
    self.ingested.load(std::sync::atomic::Ordering::Relaxed)
  }

  /// Ask the store, without occupying a worker while it answers.
  pub async fn call(&self, op: Op) -> Response {
    // Counted BEFORE the send, so a request the store never answers still
    // registers as having been dispatched: the harness's question is whether
    // the CLIENT routed, and a store that then failed does not un-route it.
    self
      .dispatched
      .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (reply_tx, reply_rx) = oneshot::channel();
    if self
      .tx
      .send(Work::Client {
        op,
        reply: reply_tx,
      })
      .await
      .is_err()
    {
      return Response::error(
        format!(
          "the store for `{}` is no longer running",
          self.root.display()
        ),
        "the project's store thread has ended, which it does only on a panic. Restart the daemon.",
      );
    }
    match reply_rx.await {
      Ok(response) => response,
      Err(_) => Response::error(
        format!(
          "the store for `{}` stopped while serving this request",
          self.root.display()
        ),
        "the request was accepted and the store thread ended before answering. Restart the daemon; the log names the panic.",
      ),
    }
  }

  /// Ask the store to re-read the project from disk (`AC-08.5`).
  ///
  /// **SYNCHRONOUS, BECAUSE ITS ONLY CALLER IS NOT ON THE RUNTIME.** The
  /// debouncer owns its own thread, so `blocking_send` is correct here for the
  /// same reason `blocking_recv` is correct on the store thread and would be a
  /// defect anywhere else in this crate: neither is a runtime worker.
  ///
  /// **IT DOES NOT TOUCH [`ProjectHandle::dispatched`], AND THAT IS THE POINT
  /// OF ITS BEING A SEPARATE DOOR.** See [`Work`].
  ///
  /// **BLOCKING RATHER THAN DROPPING WHEN THE QUEUE IS FULL.** A duplicate
  /// ingest costs a scan and writes nothing, because the sync is driven by
  /// content hashes; a DROPPED one leaves the store behind the disk until
  /// somebody happens to edit again. The wait delays the next batch for this
  /// project and nothing else.
  /// **THE REFUSAL IS A `Response`, NOT A `String`** (`IN-RS-CODE-004`, caught
  /// by the pre-commit critic). It is the same shape every other fallible entry
  /// in this crate returns, so the message and the remedy stay two fields
  /// rather than one string a caller has to take apart to report.
  pub fn ingest(&self) -> Result<(), Response> {
    self.tx.blocking_send(Work::Ingest).map_err(|_| {
      Response::error(
        format!(
          "the store for `{}` is no longer running, so external edits are not being ingested",
          self.root.display()
        ),
        "the project's store thread has ended, which it does only on a panic. Restart the daemon.",
      )
    })
  }
}

/// Re-read the project from disk, on the store thread.
///
/// **THE SAME CALL `intent sync --to-store` MAKES, WHICH IS D32 APPLIED TO A
/// TRIGGER NOBODY TYPED.** A daemon with its own ingest path would be a second
/// sync engine in the literal sense the design warns about -- not two processes
/// racing, but two implementations that agree today.
///
/// **A FAILURE IS PRINTED BECAUSE THERE IS NOBODY TO RETURN IT TO, AND SILENCE
/// IS THE ONE THING IT MUST NOT BE** (`IN-AG-NO-SILENT-001`). An ingest that
/// failed quietly leaves a daemon that looks healthy and a store that is
/// behind the disk, and the operator's first symptom is a stale answer to an
/// unrelated question. `AC-08.4` names where daemon logs live; until that
/// lands this is stderr, which is where `intent daemon run` puts it in front
/// of whoever started it.
fn ingest(facade: &mut Facade, root: &Path) {
  if let Err(e) = facade.sync_from_disk(&intentsvcs::sync::Scope::All) {
    eprintln!(
      "intentd: ingesting `{}` after an external edit failed: {}\n  remedy: {}",
      root.display(),
      e.render(),
      e.remedy()
    );
  }
}

/// Open the facade for a root, turning every failure into something sayable.
fn open_facade(root: &Path) -> Result<Facade, Response> {
  let project = Project::open(root).map_err(|e| {
    Response::error(
      format!("{e}"),
      format!(
        "`{}` is registered with this daemon and is not an Intent project. Run `intent init` there, or the directory has been replaced since the daemon first opened it.",
        root.display()
      ),
    )
  })?;
  let ctx = FacadeContext {
    // **THE DAEMON IS NOT THE AUTHOR AND MUST NOT CLAIM TO BE.** `principal`
    // reaches the event log, so a daemon writing "local" would attribute every
    // client's work to itself. Naming the daemon is honest today, when no
    // request carries an identity; when one does, it comes off the request.
    principal: "intentd".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Facade::open(project, ctx).map_err(|e| Response::error(format!("{e}"), e.remedy()))
}

/// Serve one project-scoped operation.
///
/// **THE ONLY FUNCTION IN THIS CRATE THAT TOUCHES A `Facade`, AND IT RUNS ON
/// THE STORE THREAD.** Adding an operation means adding an arm here; there is
/// nowhere else it could be added, which is the property the door is for.
fn serve(facade: &mut Facade, op: Op) -> Response {
  match op {
    Op::ThreadList => Response::Threads {
      threads: facade
        .st_list()
        .into_iter()
        .map(|thread| ThreadSummary {
          id: thread.id.clone(),
          title: thread.title.clone(),
          status: thread.status,
          created: thread.created.clone(),
          completed: thread.completed.clone(),
        })
        .collect(),
    },
    // **UNREACHABLE BY DISPATCH AND ANSWERED ANYWAY.** The registry is not
    // scoped to a project, so the connection handler answers it before any
    // handle is involved. If that dispatch ever changes, a caller arriving here
    // gets a refusal that names the mistake rather than a panic or a wrong
    // answer -- the arm costs two lines and removes a whole class of silent
    // misrouting.
    Op::Registry => Response::error(
      "the registry is not a project-scoped operation and reached a project's store",
      "this is a routing fault inside intentd. The registry is answered by the daemon itself, not by any project.",
    ),
  }
}
